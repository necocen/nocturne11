use anyhow::Result;
use diesel::prelude::*;
use dotenv::dotenv;
use infrastructure::migration::*;
use std::env;
use uuid::Uuid;

pub struct DatabaseMock {
    pub pg_url: url::Url,
    pg_base_url: url::Url,
    db_name: String,
}

impl DatabaseMock {
    fn new(pg_base_url: url::Url, db_name: impl Into<String>) -> Result<Self> {
        let mut conn = PgConnection::establish(pg_base_url.join("postgres")?.as_str())?;
        let db_name = db_name.into();
        let query = diesel::sql_query(format!(r#"CREATE DATABASE "{}""#, &db_name).as_str());
        query.execute(&mut conn)?;

        let pg_url = pg_base_url.join(&db_name)?;
        migrate(&pg_url)?;

        Ok(Self {
            pg_url,
            pg_base_url,
            db_name,
        })
    }
}

impl Drop for DatabaseMock {
    fn drop(&mut self) {
        // cf. https://snoozetime.github.io/2019/06/16/integration-test-diesel.html
        let mut conn = PgConnection::establish(
            self.pg_base_url
                .join("postgres")
                .expect("Cannot parse postgres URL.")
                .as_str(),
        )
        .expect("Cannot connect to postgres database.");

        // 先にコネクションを削除しないとDROP DATABASEできない
        let disconnect_users = format!(
            r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';"#,
            self.db_name
        );
        diesel::sql_query(disconnect_users)
            .execute(&mut conn)
            .unwrap();

        let query = diesel::sql_query(format!(r#"DROP DATABASE "{}""#, self.db_name).as_str());
        query
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!(r#"Couldn't drop database "{}""#, self.db_name));
    }
}

pub fn mock_db() -> Result<DatabaseMock> {
    dotenv().ok();
    let db_name = Uuid::new_v4().simple().to_string();
    DatabaseMock::new(url::Url::parse(&env::var("POSTGRES_URL")?)?, db_name)
}
