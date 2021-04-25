use anyhow::Result;
use diesel::prelude::*;
use infrastructure::migration::*;

pub struct TestDatabase {
    pub pg_url: url::Url,
    pg_base_url: url::Url,
    db_name: String,
}

impl TestDatabase {
    fn new(pg_base_url: url::Url, db_name: impl Into<String>) -> Result<Self> {
        let conn = PgConnection::establish(pg_base_url.join("postgres")?.as_str())?;
        let db_name = db_name.into();
        let query = diesel::sql_query(format!("CREATE DATABASE {}", &db_name).as_str());
        query.execute(&conn)?;

        let pg_url = pg_base_url.clone().join(&db_name)?;
        migrate(&pg_url)?;

        Ok(Self {
            pg_url,
            pg_base_url,
            db_name,
        })
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // cf. https://snoozetime.github.io/2019/06/16/integration-test-diesel.html
        let conn = PgConnection::establish(
            self.pg_base_url
                .join("postgres")
                .expect("Cannot parse postgres URL.")
                .as_str(),
        )
        .expect("Cannot connect to postgres database.");

        // 先にコネクションを削除しないとDROP DATABASEできない
        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.db_name
        );
        diesel::sql_query(&disconnect_users).execute(&conn).unwrap();

        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
        query
            .execute(&conn)
            .expect(&format!("Couldn't drop database {}", self.db_name));
    }
}

pub fn test_db(db_name: impl Into<String>) -> Result<TestDatabase> {
    TestDatabase::new(
        url::Url::parse("postgres://root:password@127.0.0.1")?,
        db_name,
    )
}
