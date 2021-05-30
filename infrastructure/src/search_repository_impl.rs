use anyhow::{anyhow, Context};
use chrono::prelude::*;
use domain::entities::{Post, PostId, SearchResult};
use domain::repositories::search::{Result, SearchRepository};
use elasticsearch::{
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    http::StatusCode,
    indices::{IndicesCreateParts, IndicesDeleteParts, IndicesGetParts},
    snapshot::{
        SnapshotCreateParts, SnapshotCreateRepositoryParts, SnapshotGetParts,
        SnapshotGetRepositoryParts, SnapshotRestoreParts,
    },
    BulkOperation, BulkParts, CreateParts, DeleteParts, Elasticsearch, SearchParts, UpdateParts,
};
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct SearchRepositoryImpl {
    client: Elasticsearch,
    repository_name: Option<String>,
    s3_bucket_name: Option<String>,
    aws_access_key_id: Option<String>,
    aws_secret_access_key: Option<String>,
}

impl SearchRepositoryImpl {
    const INDEX_NAME: &'static str = "andante";

    pub fn new(es_url: &url::Url) -> Result<SearchRepositoryImpl> {
        let conn_pool = SingleNodeConnectionPool::new(es_url.clone());
        let transport = TransportBuilder::new(conn_pool)
            .disable_proxy()
            .build()
            .with_context(|| format!("Failed to build transport: {}", es_url))?;
        let client = Elasticsearch::new(transport);
        Ok(SearchRepositoryImpl {
            client,
            repository_name: None,
            s3_bucket_name: None,
            aws_access_key_id: None,
            aws_secret_access_key: None,
        })
    }
    pub fn new_for_snapshot(
        es_url: &url::Url,
        repository_name: String,
        s3_bucket_name: Option<String>,
        aws_access_key_id: Option<String>,
        aws_secret_access_key: Option<String>,
    ) -> Result<SearchRepositoryImpl> {
        let conn_pool = SingleNodeConnectionPool::new(es_url.clone());
        let transport = TransportBuilder::new(conn_pool)
            .disable_proxy()
            .build()
            .with_context(|| format!("Failed to build transport: {}", es_url))?;
        let client = Elasticsearch::new(transport);
        Ok(SearchRepositoryImpl {
            client,
            repository_name: Some(repository_name),
            s3_bucket_name,
            aws_access_key_id,
            aws_secret_access_key,
        })
    }

    async fn ensure_repository(&self) -> Result<()> {
        let repository_name = self
            .repository_name
            .as_ref()
            .context("Repository name is not specified.")?;
        let response = self
            .client
            .snapshot()
            .get_repository(SnapshotGetRepositoryParts::Repository(&[repository_name]))
            .send()
            .await
            .context("Failed to get repository")?;
        if response.status_code() == StatusCode::OK {
            info!("Repository '{}' was already registered.", repository_name);
        } else {
            info!("Create repository '{}'.", repository_name);
            self.client
                .snapshot()
                .create_repository(SnapshotCreateRepositoryParts::Repository(repository_name))
                .body(json!({
                    "type": "s3",
                    "settings": {
                        "bucket": self.s3_bucket_name.as_ref().context("S3 bucket name is not specified.")?,
                        "access_key": self.aws_access_key_id.as_ref().context("AWS_ACCESS_KEY_ID is not specified")?,
                        "secret_key": self.aws_secret_access_key.as_ref().context("AWS_SECRET_ACCESS_KEY_ID is not specified")?,
                    },
                }))
                .send()
                .await.context("Failed to create repository")?;
        }
        Ok(())
    }

    async fn create_index(&self) -> Result<bool> {
        let response = self
            .client
            .indices()
            .get(IndicesGetParts::Index(&[Self::INDEX_NAME]))
            .send()
            .await
            .context("Failed to check index")?;
        if response.status_code() == StatusCode::OK {
            debug!("Index exists");
            return Ok(false);
        } else {
            self.client
                .indices()
                .create(IndicesCreateParts::Index(Self::INDEX_NAME))
                .body(json!({
                    "settings": {
                        "analysis": {
                            "char_filter": {
                                "normalize": {
                                    "type": "icu_normalizer",
                                    "name": "nfkc",
                                    "mode": "compose",
                                },
                            },
                            "tokenizer": {
                                "bigram": {
                                    "type": "ngram",
                                    "min_gram": 1,
                                    "max_gram": 2,
                                    "token_chars": [
                                        "letter",
                                        "digit",
                                    ]
                                },
                                "kuromoji": {
                                    "mode": "search",
                                    "type": "kuromoji_tokenizer",
                                    "discard_compound_token": true,
                                }
                            },
                            "filter": {
                                "kana_filter": {
                                    "type": "icu_transform",
                                    "id": "Hiragana-Katakana",
                                }
                            },
                            "analyzer": {
                                "kuromoji_analyzer": {
                                    "type": "custom",
                                    "char_filter": ["normalize"],
                                    "tokenizer": "kuromoji",
                                    "filter": [
                                        "kuromoji_baseform",
                                        "kuromoji_part_of_speech",
                                        "cjk_width",
                                        "ja_stop",
                                        "kuromoji_stemmer",
                                        "lowercase",
                                        "kana_filter",
                                    ],
                                },
                                "bigram_analyzer": {
                                    "type": "custom",
                                    "char_filter": ["normalize"],
                                    "tokenizer": "bigram",
                                    "filter": [
                                        "lowercase",
                                        "kana_filter",
                                    ],
                                }
                            }
                        },
                    },
                    "mappings": {
                        "properties": {
                            "body": {
                                "type": "text",
                                "analyzer": "kuromoji_analyzer",
                                "fields": {
                                    "bigram": {
                                        "type": "text",
                                        "analyzer": "bigram_analyzer",
                                    },
                                },
                            },
                            "title": {
                                "type": "text",
                                "analyzer": "kuromoji_analyzer",
                                "fields": {
                                    "bigram": {
                                        "type": "text",
                                        "analyzer": "bigram_analyzer",
                                    },
                                },
                            },
                            "id": {
                                "type": "integer"
                            }
                        }
                    }
                }))
                .send()
                .await
                .context("Failed to create index")?;
        }
        Ok(true)
    }
}

#[async_trait::async_trait]
impl SearchRepository for SearchRepositoryImpl {
    async fn search(
        &self,
        keywords: &[&str],
        search_after: Option<(u64, u64)>,
        limit: usize,
    ) -> Result<SearchResult> {
        // 本文とタイトルから検索。bigramのマッチはMUST、kuromojiのマッチはSHOULD。
        let must_queries = keywords
            .iter()
            .map(|keyword| {
                json!({
                    "multi_match": {
                        "query": keyword,
                        "fields": ["body.bigram", "title.bigram"],
                        "type": "phrase",
                    },
                })
            })
            .collect::<Vec<_>>();
        let should_queries = keywords
            .iter()
            .map(|keyword| {
                json!({
                    "multi_match": {
                        "query": keyword,
                        "fields": ["body", "title"],
                        "type": "phrase",
                    },
                })
            })
            .collect::<Vec<_>>();

        let mut body = json!({
            "sort": [
                {
                    "created_at": "desc",
                    "id": "desc"
                }
            ],
            "size" : limit,
            "track_total_hits": true,
            "query": {
                "bool": {
                    "must": must_queries,
                    "should": should_queries,
                }
            },
        });

        if let Some(search_after) = search_after {
            body["search_after"] = json!([search_after.0, search_after.1]);
        }

        let response = self
            .client
            .search(SearchParts::Index(&[Self::INDEX_NAME]))
            .body(body)
            .allow_no_indices(true)
            .send()
            .await
            .context("Search failed")?;

        let body = response
            .json::<Value>()
            .await
            .context("Failed to parse search result")?;
        let posts = body["hits"]["hits"]
            .as_array()
            .context("`hits` was not an array")?
            .iter()
            .map(|v| -> Result<Post> {
                Ok(serde_json::from_value(v["_source"].clone()).context("Failed to parse Post")?)
            })
            .collect::<Result<Vec<_>>>()?;
        let total_count = body["hits"]["total"]["value"]
            .as_u64()
            .context("Returned result does not contain `total`.")? as u64;

        let search_after = body["hits"]["hits"]
            .as_array()
            .context("Returned result was not an array.")?
            .last()
            .and_then(|last_post| {
                let mut iter = last_post["sort"]
                    .as_array()?
                    .iter()
                    .flat_map(|v| v.as_u64());
                Some((iter.next()?, iter.next()?))
            });

        Ok(SearchResult {
            posts,
            total_count,
            search_after,
        })
    }

    async fn insert(&self, post: &Post) -> Result<()> {
        self.create_index().await?;
        self.client
            .create(CreateParts::IndexId(Self::INDEX_NAME, &post.id.to_string()))
            .body(post)
            .send()
            .await
            .context("Failed to insert")?;
        Ok(())
    }

    async fn insert_bulk(&self, posts: &[Post]) -> Result<()> {
        self.create_index().await?;
        let posts: Vec<BulkOperation<&Post>> = posts
            .iter()
            .map(|post| BulkOperation::create(post.id.to_string(), post).into())
            .collect();
        self.client
            .bulk(BulkParts::Index(Self::INDEX_NAME))
            .body(posts)
            .send()
            .await
            .context("Failed to bulk insert")?;
        Ok(())
    }

    async fn update(&self, post: &Post) -> Result<()> {
        self.client
            .update(UpdateParts::IndexId(Self::INDEX_NAME, &post.id.to_string()))
            .body(json!({
                "doc": post,
            }))
            .send()
            .await
            .context("Failed to update")?;
        Ok(())
    }

    async fn delete(&self, id: PostId) -> Result<()> {
        self.client
            .delete(DeleteParts::IndexId(Self::INDEX_NAME, &id.to_string()))
            .send()
            .await
            .context("Failed to delete")?;
        Ok(())
    }

    async fn save_snapshot(&self) -> Result<()> {
        self.ensure_repository().await?;

        let repository_name = self
            .repository_name
            .as_ref()
            .context("Repository name is not specified.")?;

        let snapshot_name = Utc::now().to_rfc3339().to_lowercase();
        let response = self
            .client
            .snapshot()
            .create(SnapshotCreateParts::RepositorySnapshot(
                repository_name,
                &snapshot_name,
            ))
            .body(json!({ "indices": Self::INDEX_NAME }))
            .send()
            .await
            .context("Failed to save snapshot")?;

        match response.status_code() {
            StatusCode::OK => Ok(()),
            _ => Err(anyhow!(
                "{}",
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown Error".to_owned())
            )
            .into()),
        }
    }

    async fn reset(&self) -> Result<()> {
        self.client
            .indices()
            .delete(IndicesDeleteParts::Index(&[Self::INDEX_NAME]))
            .send()
            .await
            .context("Failed to reset")?;
        Ok(())
    }

    async fn newest_snapshot_name(&self) -> Result<String> {
        self.ensure_repository().await?;

        let repository_name = self
            .repository_name
            .as_ref()
            .context("Repository name is not specified.")?;

        // 最新のスナップショットを選ぶ
        let mut snapshots = self
            .client
            .snapshot()
            .get(SnapshotGetParts::RepositorySnapshot(
                repository_name,
                &["*"],
            ))
            .send()
            .await
            .context("Failed to get snapshots")?
            .json::<Value>()
            .await
            .context("Failed to parse snapshots data")?["snapshots"]
            .as_array()
            .context("No snapshots were found")?
            .clone();
        snapshots.sort_by_key(|s| s["start_time_in_millis"].as_u64().unwrap());
        Ok(
            snapshots.last().context("No snapshots were found")?["snapshot"]
                .as_str()
                .unwrap()
                .to_string(),
        )
    }

    async fn restore_snapshot(&self, snapshot_name: &str) -> Result<()> {
        self.ensure_repository().await?;

        let repository_name = self
            .repository_name
            .as_ref()
            .context("Repository name is not specified.")?;

        // リストア実行
        let response = self
            .client
            .snapshot()
            .restore(SnapshotRestoreParts::RepositorySnapshot(
                repository_name,
                snapshot_name,
            ))
            .send()
            .await
            .context("Failed to restore snapshot")?;

        match response.status_code() {
            StatusCode::OK => Ok(()),
            _ => Err(anyhow!(
                "{}",
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown Error".to_owned())
            )
            .into()),
        }
    }
}
