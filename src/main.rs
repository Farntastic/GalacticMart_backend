use actix_web::{get, post, put, delete, web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::{InsertProduct, Product};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

mod schema;
mod models;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[get("/products")]
async fn get_products(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::products::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");
    let result = products.load::<Product>(&mut conn);

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// ✅ GET - ดึงสินค้าตาม ID
#[get("/products/{id}")]
async fn get_product_by_id(pool: web::Data<DbPool>, product_id: web::Path<Uuid>) -> impl Responder {
    use crate::schema::products::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");
    let target_id = product_id.into_inner();

    let result = products.filter(id.eq(target_id)).first::<Product>(&mut conn);

    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::NotFound().json("❌ ไม่พบสินค้านี้!"),
    }
}


#[post("/products")]
async fn add_product(pool: web::Data<DbPool>, item: web::Json<InsertProduct>) -> impl Responder {
    use crate::schema::products::dsl::*;

    let new_product = InsertProduct {
        name: item.name.clone(),
        details: item.details.clone(),
        price: item.price,
        stock: item.stock,
        image: item.image.clone(),
        category: item.category.clone(),
    };

    let mut conn = pool.get().expect("Failed to get DB connection");

    let result = diesel::insert_into(products)
        .values(&new_product)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// ✅ PUT - อัปเดตสินค้า
#[put("/products/{id}")]
async fn update_product(
    pool: web::Data<DbPool>,
    product_id: web::Path<Uuid>,
    item: web::Json<InsertProduct>,
) -> impl Responder {
    use crate::schema::products::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    let target_id = product_id.into_inner();

    let result = diesel::update(products.filter(id.eq(target_id)))
        .set((
            name.eq(item.name.clone()),
            details.eq(item.details.clone()),
            price.eq(item.price),
            stock.eq(item.stock),
            image.eq(item.image.clone()),
            category.eq(item.category.clone()),
        ))
        .execute(&mut conn);

    match result {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json("✅ อัปเดตสินค้าเรียบร้อย!"),
        _ => HttpResponse::NotFound().json("❌ ไม่พบสินค้านี้!"),
    }
}

// ✅ DELETE - ลบสินค้า
#[delete("/products/{id}")]
async fn delete_product(pool: web::Data<DbPool>, product_id: web::Path<Uuid>) -> impl Responder {
    use crate::schema::products::dsl::*;

    let mut conn = pool.get().expect("Failed to get DB connection");

    let target_id = product_id.into_inner();

    let result = diesel::delete(products.filter(id.eq(target_id))).execute(&mut conn);

    match result {
        Ok(rows) if rows > 0 => HttpResponse::Ok().json("✅ ลบสินค้าเรียบร้อย!"),
        _ => HttpResponse::NotFound().json("❌ ไม่พบสินค้านี้!"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create DB pool");

    println!("🚀 Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .app_data(web::Data::new(pool.clone()))
            .service(get_products)
            .service(add_product)
            .service(get_product_by_id) // ✅ เพิ่ม API GET by ID
            .service(update_product) // ✅ เพิ่ม API PUT
            .service(delete_product) // ✅ เพิ่ม API DELETE
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
