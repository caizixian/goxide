use crate::routes::*;
use actix_files::Files;
use actix_web::dev::Server;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool, debug: bool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(Logger::default())
            .service(web::scope("/api/v1").configure(api_v1_config))
            .configure(|cfg| {
                if !debug {
                    cfg.service(
                        Files::new("/ui", "./dist/")
                            .index_file("index.html")
                            .redirect_to_slash_directory(),
                    );
                }
            })
            // Needs to be the last
            // Paths are resolved in the order they are defined
            .service(web::scope("").configure(|cfg| index_config(cfg, debug)))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
