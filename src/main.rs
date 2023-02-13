mod api;

use api::task::{
    get_tasks,
    get_task,
    submit_task,
    update_task,
    delete_task,
};
// use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        // api
        // api [ tasks ]
        .service(get_tasks)
        .service(get_task)
        .service(submit_task)
        .service(update_task)
        .service(delete_task)
    })
    .bind(("127.0.0.1", 9001))?
    .run()
    .await
}
