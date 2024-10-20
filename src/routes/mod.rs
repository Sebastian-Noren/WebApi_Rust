mod handlers;

use actix_web::web;
use handlers::{get_items, get_item, create_item, update_item, delete_item, read_file,get_file_by_name,print_ids,fetch_from_java};


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/items")
            .service(
                web::resource("")
                     // Middleware applied to this resource
                    .route(web::get().to(get_items))
                    .route(web::post().to(create_item)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_item))
                    .route(web::put().to(update_item))
                    .route(web::delete().to(delete_item)),
            ),
    );

    // Routes for /files
    cfg.service(
        web::scope("/files")
            .service(
                web::resource("/read")
                    .route(web::get().to(read_file)),
            )
            .service(
                web::resource("/read/{filename}")
                    .route(web::get().to(get_file_by_name)),
            ),
    );

    cfg.service(print_ids);

    // Route for /java
    cfg.route("/java", web::get().to(fetch_from_java));
}

