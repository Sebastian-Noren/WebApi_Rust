mod handlers;

use actix_web::web;
use handlers::*;




pub  fn init_routes(cfg: &mut web::ServiceConfig) {


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
            )
    );

    cfg.service(
        web::scope("/redis")
            .service(
                web::resource("")
                    .route(web::post().to(set_item_in_redis)), // New POST route
            )
            .service(
                web::resource("/{key}")
                    .route(web::get().to(get_item_from_redis))
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

