use anyhow::Context as _;
use application::models::SearchResult;
use chrono::{DateTime, Local, NaiveDate, TimeZone as _, Utc};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use domain::entities::{date::YearMonth, Post, PostId};
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::{
    indices::{IndicesCreateParts, IndicesGetParts},
    CreateParts, DeleteParts, Elasticsearch, ExistsParts, SearchParts, UpdateParts,
};
use r2d2::{Pool, PooledConnection};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::diesel_helpers::{extract, DatePart, TimezoneCustomizer};
use crate::models::Post as PostModel;

#[derive(Clone)]
pub struct SearchClient {
    client: Elasticsearch,
    pub(crate) conn_pool: Pool<ConnectionManager<PgConnection>>,
    index_name: String,
}

impl SearchClient {
    const DEFAULT_INDEX_NAME: &'static str = "andante";

    pub fn new(es_url: &url::Url, pg_url: &url::Url) -> anyhow::Result<Self> {
        let es_conn_pool = SingleNodeConnectionPool::new(es_url.clone());
        let transport = TransportBuilder::new(es_conn_pool)
            .disable_proxy()
            .build()?;
        let client = Elasticsearch::new(transport);
        let customizer = TimezoneCustomizer {
            offset: *Local::now().offset(),
        };
        let conn_pool = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(ConnectionManager::<PgConnection>::new(pg_url.as_str()))?;

        let index_name = Self::DEFAULT_INDEX_NAME.to_string();

        Ok(Self {
            client,
            conn_pool,
            index_name,
        })
    }

    pub fn with_es_index_name(
        es_url: &url::Url,
        pg_url: &url::Url,
        index_name: impl Into<String>,
    ) -> anyhow::Result<Self> {
        let es_conn_pool = SingleNodeConnectionPool::new(es_url.clone());
        let transport = TransportBuilder::new(es_conn_pool)
            .disable_proxy()
            .build()?;
        let client = Elasticsearch::new(transport);
        let customizer = TimezoneCustomizer {
            offset: *Local::now().offset(),
        };
        let conn_pool = Pool::builder()
            .connection_customizer(Box::new(customizer))
            .build(ConnectionManager::<PgConnection>::new(pg_url.as_str()))?;

        let index_name = index_name.into().to_string();

        Ok(Self {
            client,
            conn_pool,
            index_name,
        })
    }

    fn get_conn(&self) -> anyhow::Result<PooledConnection<ConnectionManager<PgConnection>>> {
        self.conn_pool.get().context("failed to get connection")
    }

    async fn create_index_if_needed(&self) -> anyhow::Result<bool> {
        let response = self
            .client
            .indices()
            .get(IndicesGetParts::Index(&[&self.index_name]))
            .send()
            .await
            .context("Failed to check index")?;
        if response.status_code() == StatusCode::OK {
            return Ok(false);
        }

        self.client
            .indices()
            .create(IndicesCreateParts::Index(&self.index_name))
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

        Ok(true)
    }
}

#[async_trait::async_trait]
impl application::adapters::SearchClient for SearchClient {
    async fn find_by_keywords<'a>(
        &self,
        keywords: &'a [&'a str],
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult> {
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
        let body = json!({
            "sort": [
                {
                    "created_at": "desc",
                    "id": "desc"
                }
            ],
            "from" : offset,
            "size" : limit,
            "track_total_hits": true,
            "_source": false,
            "fields": ["id"],
            "query": {
                "bool": {
                    "must": must_queries,
                    "should": should_queries,
                }
            },
        });

        let response = self
            .client
            .search(SearchParts::Index(&[&self.index_name]))
            .body(body)
            .allow_no_indices(true)
            .send()
            .await
            .context("Search failed")?
            .json::<Value>()
            .await
            .context("Failed to parse search result")?;
        let post_ids = response["hits"]["hits"]
            .as_array()
            .context("`hits` was not an array")?
            .iter()
            .map(|v| -> anyhow::Result<PostId> {
                serde_json::from_value(v["fields"]["id"][0].clone()).context("Failed to get PostId")
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let total_count = response["hits"]["total"]["value"]
            .as_u64()
            .context("Returned result does not contain `total`.")?
            as usize;

        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn find_by_year_month(
        &self,
        year_month: &YearMonth,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult> {
        use crate::schema::posts::dsl::{created_at, id, posts};
        let (next_year, next_month) = if year_month.1 == 12 {
            (year_month.0 + 1, 1)
        } else {
            (year_month.0, year_month.1 + 1)
        };
        let created_after = Local
            .with_ymd_and_hms(year_month.0 as i32, year_month.1 as u32, 1, 0, 0, 0)
            .unwrap();
        let created_before = Local
            .with_ymd_and_hms(next_year as i32, next_month as u32, 1, 0, 0, 0)
            .unwrap();
        let post_ids = posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .offset(offset as i64)
            .limit(limit as i64)
            .select(id)
            .get_results::<i32>(&mut self.get_conn()?)
            .context("Failed to get posts")?
            .into_iter()
            .map(PostId)
            .collect();
        let total_count = posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .count()
            .get_result::<i64>(&mut self.get_conn()?)
            .context("Failed to get total count")? as usize;
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn find_by_date(
        &self,
        date: &NaiveDate,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<SearchResult> {
        use crate::schema::posts::dsl::{created_at, id, posts};
        let created_after = Local
            .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
            .unwrap();
        let created_before = Local
            .from_local_datetime(&date.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap())
            .unwrap();

        let post_ids = posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .offset(offset as i64)
            .limit(limit as i64)
            .select(id)
            .get_results::<i32>(&mut self.get_conn()?)
            .context("Failed to get posts")?
            .into_iter()
            .map(PostId)
            .collect();
        let total_count = posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .count()
            .get_result::<i64>(&mut self.get_conn()?)
            .context("Failed to get total count")? as usize;
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn get_year_months(&self) -> anyhow::Result<Vec<YearMonth>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let results = posts
            .select((
                extract(DatePart::Year, created_at),
                extract(DatePart::Month, created_at),
            ))
            .distinct()
            .get_results::<(i32, i32)>(&mut self.get_conn()?)
            .context("Failed to get results")?;
        Ok(results
            .into_iter()
            .map(|(y, m)| YearMonth(y as u16, m as u8))
            .collect())
    }

    async fn get_days_in_year_month(
        &self,
        YearMonth(year, month): &YearMonth,
    ) -> anyhow::Result<Vec<u8>> {
        use crate::schema::posts::dsl::{created_at, posts};
        let (next_year, next_month) = if *month == 12 {
            (*year + 1, 1)
        } else {
            (*year, *month + 1)
        };
        let created_after = Local
            .with_ymd_and_hms((*year) as i32, (*month) as u32, 1, 0, 0, 0)
            .unwrap();
        let created_before = Local
            .with_ymd_and_hms(next_year as i32, next_month as u32, 1, 0, 0, 0)
            .unwrap();
        let results = posts
            .filter(created_at.ge(created_after))
            .filter(created_at.lt(created_before))
            .select(extract(DatePart::Day, created_at))
            .distinct()
            .get_results::<i32>(&mut self.get_conn()?)
            .context("Failed to get results")?
            .into_iter()
            .map(|d| d as u8)
            .collect();
        Ok(results)
    }

    async fn get_latest_posts(&self, offset: usize, limit: usize) -> anyhow::Result<SearchResult> {
        use crate::schema::posts::dsl::{created_at, id, posts};
        let post_ids = posts
            .order_by(created_at.desc())
            .offset(offset as i64)
            .limit(limit as i64)
            .select(id)
            .get_results::<i32>(&mut self.get_conn()?)
            .context("Failed to get posts")?
            .into_iter()
            .map(PostId)
            .collect();
        let total_count = posts
            .count()
            .get_result::<i64>(&mut self.get_conn()?)
            .context("Failed to get total count")? as usize;
        Ok(SearchResult {
            post_ids,
            total_count,
        })
    }

    async fn get_last_updated(&self) -> anyhow::Result<Option<DateTime<Utc>>> {
        use crate::schema::posts::dsl::{posts, updated_at};
        let post = posts
            .order_by(updated_at.desc())
            .first::<PostModel>(&mut self.get_conn()?)
            .optional()
            .context("failed to get result")?;
        Ok(post.map(|p| p.updated_at))
    }

    async fn get_from_date(
        &self,
        from: DateTime<Utc>,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<PostId>> {
        use crate::schema::posts::dsl::{created_at, id, posts};
        let results = posts
            .filter(created_at.ge(from))
            .order_by(created_at.asc())
            .offset(offset as i64)
            .limit(limit as i64)
            .select(id)
            .get_results::<i32>(&mut self.get_conn()?)
            .context("Failed to get results")?
            .into_iter()
            .map(PostId)
            .collect();
        Ok(results)
    }

    async fn get_until_date(
        &self,
        until: DateTime<Utc>,
        offset: usize,
        limit: usize,
    ) -> anyhow::Result<Vec<PostId>> {
        use crate::schema::posts::dsl::{created_at, id, posts};
        let results = posts
            .filter(created_at.lt(until))
            .order_by(created_at.desc())
            .offset(offset as i64)
            .limit(limit as i64)
            .select(id)
            .get_results::<i32>(&mut self.get_conn()?)
            .context("failed to get results")?
            .into_iter()
            .map(PostId)
            .collect();
        Ok(results)
    }

    async fn save(&self, post: &Post) -> anyhow::Result<()> {
        self.create_index_if_needed().await?;

        let response = self
            .client
            .exists(ExistsParts::IndexId(&self.index_name, &post.id.to_string()))
            .send()
            .await
            .context("failed to check existence of specified PostId")?;
        if response.status_code() == StatusCode::OK {
            // Update
            self.client
                .update(UpdateParts::IndexId(&self.index_name, &post.id.to_string()))
                .body(json!({
                    "doc": post,
                }))
                .send()
                .await
                .context("failed to update document")?;
        } else {
            // Insert
            self.client
                .create(CreateParts::IndexId(&self.index_name, &post.id.to_string()))
                .body(post)
                .send()
                .await
                .context("failed to insert document")?;
        }

        Ok(())
    }

    async fn delete(&self, id: &PostId) -> anyhow::Result<()> {
        self.client
            .delete(DeleteParts::IndexId(&self.index_name, &id.to_string()))
            .send()
            .await
            .context("failed to delete document")?;

        Ok(())
    }
}
