use clap::{Parser, ArgAction};
use errors::Error;
use service::Service;
use std::path::PathBuf;
mod context;
mod errors;
mod filters;
mod handlers;
mod presentation;
mod routers;
mod service;

#[derive(Parser, Debug, Clone)]
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
    /// 本番環境モード
    #[clap(long("production"), action = ArgAction::SetFalse)]
    is_development: bool,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};
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
