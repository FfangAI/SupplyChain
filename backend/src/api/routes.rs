use actix_web::web;

use super::{
    products,
    suppliers,
    orders,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/products")
                    .route("", web::get().to(products::get_all))
                    .route("", web::post().to(products::create))
                    .route("/{id}", web::get().to(products::get_by_id))
                    .route("/{id}", web::put().to(products::update))
                    .route("/{id}", web::delete().to(products::delete))
            )
            .service(
                web::scope("/suppliers")
                    .route("", web::get().to(suppliers::get_all))
                    .route("", web::post().to(suppliers::create))
                    .route("/{id}", web::get().to(suppliers::get_by_id))
                    .route("/{id}", web::put().to(suppliers::update))
                    .route("/{id}", web::delete().to(suppliers::delete))
            )
            .service(
                web::scope("/orders")
                    .route("", web::get().to(orders::get_all))
                    .route("", web::post().to(orders::create))
                    .route("/{id}", web::get().to(orders::get_by_id))
                    .route("/{id}/status", web::patch().to(orders::update_status))
            )
    );
}