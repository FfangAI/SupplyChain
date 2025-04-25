use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub supplier_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub price: f64,
    pub stock_quantity: i32,
    pub supplier_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sku: Option<String>,
    pub price: Option<f64>,
    pub stock_quantity: Option<i32>,
    pub supplier_id: Option<Uuid>,
}