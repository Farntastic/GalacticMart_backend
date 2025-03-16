use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub details: String,
    pub price: f64,
    pub stock: i32,
    pub image: String,
    pub category: String,
}

#[derive(Insertable, Deserialize, Serialize)]  // ✅ เพิ่ม `Serialize`
#[diesel(table_name = crate::schema::products)]
pub struct InsertProduct {
    pub name: String,
    pub details: String,
    pub price: f64,
    pub stock: i32,
    pub image: String,
    pub category: String,
}
