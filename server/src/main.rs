use errors::Error;
use service::Service;
use std::path::PathBuf;
mod askama_helpers;
mod context;
mod errors;
mod handlers;
mod presentation;
mod routers;
mod service;

#[derive(clap::Clap, Debug, Clone)]
#[clap(version = "11.0.0", author = "@necocen <necocen@gmail.com>")]
struct Opts {
    /// バインドするアドレス
    #[clap(short, long, default_value = "127.0.0.1")]
    bind: String,
    /// バインドするポート
    #[clap(short, long, default_value = "4000")]
    port: u16,
    /// 静的ファイルの配信元ディレクトリ
    #[clap(long("static"), default_value = "./frontend/build/src")]
    static_path: PathBuf,
    /// 開発モード
    #[clap(long("production"), parse(from_flag = std::ops::Not::not))]
    is_development: bool,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};
    use clap::Clap;
    env_logger::init();
    dotenv::dotenv().ok();
    let opts = Opts::parse();
    let service = Service::new(&opts)?;
    HttpServer::new(move || App::new().configure(routers::routing(service.clone())))
        .bind(format!("{}:{}", &opts.bind, &opts.port))?
        .run()
        .await?;
    Ok(())
}
