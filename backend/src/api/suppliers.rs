use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use sqlx::PgPool;

use crate::models::supplier::{Supplier, CreateSupplier, UpdateSupplier};

pub async fn get_all(db: web::Data<PgPool>) -> impl Responder {
    // Placeholder for database query
    HttpResponse::Ok().json(vec![])
}

pub async fn get_by_id(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    // Placeholder for database query
    HttpResponse::Ok().json({})
}

pub async fn create(
    db: web::Data<PgPool>,
    supplier: web::Json<CreateSupplier>,
) -> impl Responder {
    // Placeholder for database query
    HttpResponse::Created().json({})
}

pub async fn update(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
    supplier: web::Json<UpdateSupplier>,
) -> impl Responder {
    // Placeholder for database query
    HttpResponse::Ok().json({})
}

pub async fn delete(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    // Placeholder for database query
    HttpResponse::NoContent().finish()
}