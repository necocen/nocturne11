# Nocturne v11
Simple weblog system for my own.

## Prerequisites
- Rust 1.74
- Bun
- PostgreSQL 13.x?
- ElasticSearch 7.x

## Development
1. まずPostgreSQLサーバーとElasticSearchサーバーを用意する
2. 環境変数はsample.envとか見てください
3. sample.envはfrontend内にもあります
4. frontendディレクトリで`bun run build`する
5. ルートディレクトリで`cargo run --bin migrate`する
6. `cargo run`する
7. localhost:4000で起動するはず


## TODO
- improve test coverage
- CI/CD on GitHub Actions
