mod proto {
    tonic::include_proto!("rust_ecom");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("rust_ecom_descriptor");
}

mod server;

use server::*;
use std::{error::Error, sync::Arc};

use proto::{
    admin_server::AdminServer, storefront_server::StorefrontServer, user_server::UserServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let addr = std::env::var("SERVICE_ADDRESS")
        .expect("SERVICE_ADDRESS must be set")
        .parse()?;
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn_pool = Arc::new(sqlx::PgPool::connect(&db_url).await?);

    sqlx::migrate!("./migrations")
        .run(conn_pool.as_ref())
        .await?;

    let _ = sqlx::query!(
        "INSERT INTO admins (admin_id, username, password, email, created_at) 
            VALUES ($1, $2, $3, $4, $5)",
        0,
        "admin",
        "admin",
        "admin",
        0.0
    )
    .execute(conn_pool.as_ref())
    .await;

    let storefront_service = StorefrontService::new(conn_pool.clone());
    let admin_service = AdminService::new(conn_pool.clone());
    let user_service = UserService::new(conn_pool.clone());

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    println!("Listening on {}\n", addr);

    Server::builder()
        .add_service(reflection_server)
        .add_service(StorefrontServer::new(storefront_service))
        .add_service(AdminServer::new(admin_service))
        .add_service(UserServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
