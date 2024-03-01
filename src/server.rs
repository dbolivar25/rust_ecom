use sqlx::{query, query_as};
use std::{sync::Arc, time};

use crate::proto::{
    self, admin_server::Admin, storefront_server::Storefront, user_server::User,
    GetAdminAccountResponse, GetUserAccountResponse,
};

#[derive(Debug)]
pub(crate) struct StorefrontService {
    db_pool: Arc<sqlx::PgPool>,
}

#[derive(Debug)]
pub(crate) struct AdminService {
    db_pool: Arc<sqlx::PgPool>,
}

#[derive(Debug)]
pub(crate) struct UserService {
    db_pool: Arc<sqlx::PgPool>,
}

impl StorefrontService {
    pub(crate) fn new(db_pool: Arc<sqlx::PgPool>) -> Self {
        Self { db_pool }
    }
}

impl AdminService {
    pub(crate) fn new(db_pool: Arc<sqlx::PgPool>) -> Self {
        Self { db_pool }
    }
}

impl UserService {
    pub(crate) fn new(db_pool: Arc<sqlx::PgPool>) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl Storefront for StorefrontService {
    async fn get_products(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::GetProductsResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(proto::Product, "SELECT * FROM products;")
            .fetch_all(self.db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("ERROR: {:?}", e);
                tonic::Status::internal("Internal Server Error")
            })?;

        res.iter().for_each(|product| {
            println!("Product: {:?}", product);
        });

        let response = proto::GetProductsResponse { products: res };

        Ok(tonic::Response::new(response))
        // Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn get_product(
        &self,
        request: tonic::Request<proto::GetProductRequest>,
    ) -> Result<tonic::Response<proto::GetProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = $1;",
            request.get_ref().product_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Product: {:?}", res);

        let response = proto::GetProductResponse { product: Some(res) };

        Ok(tonic::Response::new(response))
        // Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn create_user_account(
        &self,
        request: tonic::Request<proto::CreateUserAccountRequest>,
    ) -> Result<tonic::Response<proto::CreateUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::UserAccount,
            "INSERT INTO users (username, password, email, created_at, products, orders) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;",
            request.username,
            request.password,
            request.email,
            time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as f64,
            &vec![],
            &vec![]
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::CreateUserAccountResponse {
            account: Some(GetUserAccountResponse {
                user_id: res.user_id,
                username: res.username,
                email: res.email,
                created_at: res.created_at,
                products: vec![],
                orders: vec![],
            }),
        };

        Ok(tonic::Response::new(response))
        // Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}

#[tonic::async_trait]
impl Admin for AdminService {
    // Products

    async fn get_products(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::GetProductsResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(proto::Product, "SELECT * FROM products;")
            .fetch_all(self.db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("ERROR: {:?}", e);
                tonic::Status::internal("Internal Server Error")
            })?;

        res.iter().for_each(|product| {
            println!("Product: {:?}", product);
        });

        let response = proto::GetProductsResponse { products: res };

        Ok(tonic::Response::new(response))
    }
    async fn get_product(
        &self,
        request: tonic::Request<proto::GetProductRequest>,
    ) -> Result<tonic::Response<proto::GetProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = $1;",
            request.get_ref().product_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Product: {:?}", res);

        let response = proto::GetProductResponse { product: Some(res) };

        Ok(tonic::Response::new(response))
    }

    async fn create_product(
        &self,
        request: tonic::Request<proto::CreateProductRequest>,
    ) -> Result<tonic::Response<proto::CreateProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::Product,
            "INSERT INTO products (name, description, price, created_at) VALUES ($1, $2, $3, $4) RETURNING *;",
            request.name,
            request.description,
            request.price,
            time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as f64
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Product: {:?}", res);

        let response = proto::CreateProductResponse { product: Some(res) };

        Ok(tonic::Response::new(response))
    }

    async fn update_product(
        &self,
        request: tonic::Request<proto::UpdateProductRequest>,
    ) -> Result<tonic::Response<proto::UpdateProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::Product,
            "UPDATE products SET name = $1, description = $2, price = $3 WHERE product_id = $4 RETURNING *;",
            request.name,
            request.description,
            request.price,
            request.product_id
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Product: {:?}", res);

        let response = proto::UpdateProductResponse { product: Some(res) };

        Ok(tonic::Response::new(response))
    }

    async fn delete_product(
        &self,
        request: tonic::Request<proto::DeleteProductRequest>,
    ) -> Result<tonic::Response<proto::DeleteProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::Product,
            "DELETE FROM products WHERE product_id = $1 RETURNING *;",
            request.product_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Product: {:?}", res);

        let response = proto::DeleteProductResponse { product: Some(res) };

        Ok(tonic::Response::new(response))
    }

    // Orders

    async fn get_orders(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::GetOrdersResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(proto::Order, "SELECT * FROM orders;")
            .fetch_all(self.db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("ERROR: {:?}", e);
                tonic::Status::internal("Internal Server Error")
            })?;

        res.iter().for_each(|order| {
            println!("Order: {:?}", order);
        });

        let response = proto::GetOrdersResponse { orders: res };

        Ok(tonic::Response::new(response))
    }
    async fn get_order(
        &self,
        request: tonic::Request<proto::GetOrderRequest>,
    ) -> Result<tonic::Response<proto::GetOrderResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(
            proto::Order,
            "SELECT * FROM orders WHERE order_id = $1;",
            request.get_ref().order_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Order: {:?}", res);

        let response = proto::GetOrderResponse { order: Some(res) };

        Ok(tonic::Response::new(response))
    }

    async fn update_order(
        &self,
        request: tonic::Request<proto::UpdateOrderRequest>,
    ) -> Result<tonic::Response<proto::UpdateOrderResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::Order,
            "UPDATE orders SET user_id = $1, products = $2, total = $3, status = $4 WHERE order_id = $5 RETURNING *;",
            request.user_id,
            &request.products,
            request.total,
            request.status,
            request.order_id
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Order: {:?}", res);

        let response = proto::UpdateOrderResponse { order: Some(res) };

        Ok(tonic::Response::new(response))
    }

    async fn delete_order(
        &self,
        request: tonic::Request<proto::DeleteOrderRequest>,
    ) -> Result<tonic::Response<proto::DeleteOrderResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::Order,
            "DELETE FROM orders WHERE order_id = $1 RETURNING *;",
            request.order_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Order: {:?}", res);

        let response = proto::DeleteOrderResponse { order: Some(res) };

        Ok(tonic::Response::new(response))
    }

    // Admin Accounts

    async fn get_admin_accounts(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::GetAdminAccountsResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(proto::AdminAccount, "SELECT * FROM admins;")
            .fetch_all(self.db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("ERROR: {:?}", e);
                tonic::Status::internal("Internal Server Error")
            })?;

        res.iter().for_each(|admin| {
            println!("Admin Account: {:?}", admin);
        });

        let response = proto::GetAdminAccountsResponse {
            accounts: res
                .iter()
                .map(|admin| GetAdminAccountResponse {
                    admin_id: admin.admin_id,
                    username: admin.username.to_owned(),
                    email: admin.email.to_owned(),
                    created_at: admin.created_at,
                })
                .collect(),
        };

        Ok(tonic::Response::new(response))
    }
    async fn get_admin_account(
        &self,
        request: tonic::Request<proto::GetAdminAccountRequest>,
    ) -> Result<tonic::Response<proto::GetAdminAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(
            proto::AdminAccount,
            "SELECT * FROM admins WHERE admin_id = $1;",
            request.get_ref().admin_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Admin Account: {:?}", res);

        let response = proto::GetAdminAccountResponse {
            admin_id: res.admin_id,
            username: res.username.to_owned(),
            email: res.email.to_owned(),
            created_at: res.created_at,
        };

        Ok(tonic::Response::new(response))
    }

    async fn create_admin_account(
        &self,
        request: tonic::Request<proto::CreateAdminAccountRequest>,
    ) -> Result<tonic::Response<proto::CreateAdminAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::AdminAccount,
            "INSERT INTO admins (username, password, email, created_at) VALUES ($1, $2, $3, $4) RETURNING *;",
            request.username,
            request.password,
            request.email,
            time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as f64
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Admin Account: {:?}", res);

        let response = proto::CreateAdminAccountResponse {
            account: Some(GetAdminAccountResponse {
                admin_id: res.admin_id,
                username: res.username.to_owned(),
                email: res.email.to_owned(),
                created_at: res.created_at,
            }),
        };

        Ok(tonic::Response::new(response))
    }

    // Admin Account

    async fn update_admin_account(
        &self,
        request: tonic::Request<proto::UpdateAdminAccountRequest>,
    ) -> Result<tonic::Response<proto::UpdateAdminAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let headers = request.metadata();
        let request = request.get_ref();

        let admin_id = headers
            .get("admin_id")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let res = query_as!(
            proto::AdminAccount,
            "UPDATE admins SET username = $1, password = $2, email = $3 WHERE admin_id = $4 RETURNING *;",
            request.username,
            request.password,
            request.email,
            admin_id
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Admin Account: {:?}", res);

        let response = proto::UpdateAdminAccountResponse {
            account: Some(GetAdminAccountResponse {
                admin_id: res.admin_id,
                username: res.username.to_owned(),
                email: res.email.to_owned(),
                created_at: res.created_at,
            }),
        };

        Ok(tonic::Response::new(response))
    }

    async fn delete_admin_account(
        &self,
        request: tonic::Request<proto::DeleteAdminAccountRequest>,
    ) -> Result<tonic::Response<proto::DeleteAdminAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::AdminAccount,
            "DELETE FROM admins WHERE admin_id = $1 RETURNING *;",
            request.admin_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("Admin Account: {:?}", res);

        let response = proto::DeleteAdminAccountResponse {
            account: Some(GetAdminAccountResponse {
                admin_id: res.admin_id,
                username: res.username.to_owned(),
                email: res.email.to_owned(),
                created_at: res.created_at,
            }),
        };

        Ok(tonic::Response::new(response))
    }

    // User Accounts

    async fn get_user_accounts(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::GetUserAccountsResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(proto::UserAccount, "SELECT * FROM users;")
            .fetch_all(self.db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("ERROR: {:?}", e);
                tonic::Status::internal("Internal Server Error")
            })?;

        res.iter().for_each(|user| {
            println!("User Account: {:?}", user);
        });

        let response = proto::GetUserAccountsResponse {
            accounts: res
                .iter()
                .map(|user| GetUserAccountResponse {
                    user_id: user.user_id,
                    username: user.username.to_owned(),
                    email: user.email.to_owned(),
                    created_at: user.created_at,
                    products: vec![],
                    orders: vec![],
                })
                .collect(),
        };

        Ok(tonic::Response::new(response))
    }
    async fn get_user_account(
        &self,
        request: tonic::Request<proto::GetUserAccountRequest>,
    ) -> Result<tonic::Response<proto::GetUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(
            proto::UserAccount,
            "SELECT * FROM users WHERE user_id = $1;",
            request.get_ref().user_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::GetUserAccountResponse {
            user_id: res.user_id,
            username: res.username.to_owned(),
            email: res.email.to_owned(),
            created_at: res.created_at,
            products: vec![],
            orders: vec![],
        };

        Ok(tonic::Response::new(response))
    }

    async fn create_user_account(
        &self,
        request: tonic::Request<proto::CreateUserAccountRequest>,
    ) -> Result<tonic::Response<proto::CreateUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::UserAccount,
            "INSERT INTO users (username, password, email, products, orders, created_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;",
            request.username,
            request.password,
            request.email,
            &vec![],
            &vec![],
            time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as f64
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::CreateUserAccountResponse {
            account: Some(GetUserAccountResponse {
                user_id: res.user_id,
                username: res.username.to_owned(),
                email: res.email.to_owned(),
                created_at: res.created_at,
                products: vec![],
                orders: vec![],
            }),
        };

        Ok(tonic::Response::new(response))
    }

    async fn delete_user_account(
        &self,
        request: tonic::Request<proto::DeleteUserAccountRequest>,
    ) -> Result<tonic::Response<proto::DeleteUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::UserAccount,
            "DELETE FROM users WHERE user_id = $1 RETURNING *;",
            request.user_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::DeleteUserAccountResponse {
            account: Some(GetUserAccountResponse {
                user_id: res.user_id,
                username: res.username.to_owned(),
                email: res.email.to_owned(),
                created_at: res.created_at,
                products: vec![],
                orders: vec![],
            }),
        };

        Ok(tonic::Response::new(response))
    }

    async fn get_products_by_user(
        &self,
        request: tonic::Request<proto::GetUserAccountRequest>,
    ) -> Result<tonic::Response<proto::GetProductsResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query!(
            "SELECT products FROM users WHERE user_id = $1;",
            request.get_ref().user_id
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let products: Vec<i32> = res
            .iter()
            .map(|row| *row.products.get(0).unwrap())
            .collect();

        let res = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = ANY($1);",
            &products
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        res.iter().for_each(|product| {
            println!("Product: {:?}", product);
        });

        let response = proto::GetProductsResponse { products: res };

        Ok(tonic::Response::new(response))
    }
    async fn get_orders_by_user(
        &self,
        request: tonic::Request<proto::GetUserAccountRequest>,
    ) -> Result<tonic::Response<proto::GetOrdersResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(
            proto::Order,
            "SELECT * FROM orders WHERE user_id = $1;",
            request.get_ref().user_id
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        res.iter().for_each(|order| {
            println!("Order: {:?}", order);
        });

        let response = proto::GetOrdersResponse { orders: res };

        Ok(tonic::Response::new(response))
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_user_account(
        &self,
        request: tonic::Request<proto::Empty>,
    ) -> Result<tonic::Response<proto::GetUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let res = query_as!(proto::UserAccount, "SELECT * FROM users;")
            .fetch_one(self.db_pool.as_ref())
            .await
            .map_err(|e| {
                println!("ERROR: {:?}", e);
                tonic::Status::internal("Internal Server Error")
            })?;

        println!("User Account: {:?}", res);

        let response = proto::GetUserAccountResponse {
            user_id: res.user_id,
            username: res.username,
            email: res.email,
            created_at: res.created_at,
            products: res.products,
            orders: res.orders,
        };

        Ok(tonic::Response::new(response))
    }

    async fn update_user_account(
        &self,
        request: tonic::Request<proto::UpdateUserAccountRequest>,
    ) -> Result<tonic::Response<proto::UpdateUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let headers = request.metadata();
        let request = request.get_ref();

        let user_id = headers
            .get("user_id")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let res = query_as!(
            proto::UserAccount,
            "UPDATE users SET username = $1, password = $2, email = $3 WHERE user_id = $4 RETURNING *;",
            request.username,
            request.password,
            request.email,
            user_id
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::UpdateUserAccountResponse {
            account: Some(GetUserAccountResponse {
                user_id: res.user_id,
                username: res.username,
                email: res.email,
                created_at: res.created_at,
                products: res.products,
                orders: res.orders,
            }),
        };

        Ok(tonic::Response::new(response))
    }

    async fn delete_user_account(
        &self,
        request: tonic::Request<proto::DeleteUserAccountRequest>,
    ) -> Result<tonic::Response<proto::DeleteUserAccountResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::UserAccount,
            "DELETE FROM users WHERE user_id = $1 RETURNING *;",
            request.user_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::DeleteUserAccountResponse {
            account: Some(GetUserAccountResponse {
                user_id: res.user_id,
                username: res.username,
                email: res.email,
                created_at: res.created_at,
                products: res.products,
                orders: res.orders,
            }),
        };

        Ok(tonic::Response::new(response))
    }

    // Products

    async fn add_product_to_cart(
        &self,
        request: tonic::Request<proto::AddToCartRequest>,
    ) -> Result<tonic::Response<proto::GetProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let find_product = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = $1;",
            request.product_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let res = query_as!(
            proto::UserAccount,
            "UPDATE users SET products = array_append(products, $1) WHERE user_id = $2 RETURNING *",
            request.product_id,
            request.user_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::GetProductResponse {
            product: Some(find_product),
        };

        Ok(tonic::Response::new(response))
    }

    async fn remove_product_from_cart(
        &self,
        request: tonic::Request<proto::RemoveFromCartRequest>,
    ) -> Result<tonic::Response<proto::GetProductResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::UserAccount,
            "UPDATE users SET products = array_remove(products, $1) WHERE user_id = $2 RETURNING *",
            request.product_id,
            request.user_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let find_product = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = $1;",
            request.product_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        println!("User Account: {:?}", res);

        let response = proto::GetProductResponse {
            product: Some(find_product),
        };

        Ok(tonic::Response::new(response))
    }

    async fn checkout(
        &self,
        request: tonic::Request<proto::CheckoutRequest>,
    ) -> Result<tonic::Response<proto::CheckoutResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let headers = request.metadata();
        // let request = request.get_ref();

        let user_id = headers
            .get("user_id")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let res = query_as!(
            proto::UserAccount,
            "SELECT * FROM users WHERE user_id = $1;",
            user_id
        )
        .fetch_one(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let products: Vec<i32> = res.products.iter().map(|product| *product).collect();

        let find_products = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = ANY($1);",
            &products
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let total: f64 = find_products.iter().map(|product| product.price).sum();

        let order = query_as!(
            proto::Order,
            "INSERT INTO orders (user_id, products, total, status) VALUES ($1, $2, $3, $4) RETURNING *;",
            user_id,
            &products,
            total,
            "Pending"
        ).fetch_one(self.db_pool.as_ref()).await.map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let response = proto::CheckoutResponse { order: Some(order) };

        Ok(tonic::Response::new(response))
    }

    async fn get_products(
        &self,
        request: tonic::Request<proto::GetUserAccountRequest>,
    ) -> Result<tonic::Response<proto::GetProductsResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query!(
            "SELECT products FROM users WHERE user_id = $1;",
            request.user_id
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        let products: Vec<i32> = res
            .iter()
            .map(|row| *row.products.get(0).unwrap())
            .collect();

        let res = query_as!(
            proto::Product,
            "SELECT * FROM products WHERE product_id = ANY($1);",
            &products
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        res.iter().for_each(|product| {
            println!("Product: {:?}", product);
        });

        let response = proto::GetProductsResponse { products: res };

        Ok(tonic::Response::new(response))
    }

    async fn get_orders(
        &self,
        request: tonic::Request<proto::GetUserAccountRequest>,
    ) -> Result<tonic::Response<proto::GetOrdersResponse>, tonic::Status> {
        println!("\nREQUEST: {:?}", request);

        let request = request.get_ref();

        let res = query_as!(
            proto::Order,
            "SELECT * FROM orders WHERE user_id = $1;",
            request.user_id
        )
        .fetch_all(self.db_pool.as_ref())
        .await
        .map_err(|e| {
            println!("ERROR: {:?}", e);
            tonic::Status::internal("Internal Server Error")
        })?;

        res.iter().for_each(|order| {
            println!("Order: {:?}", order);
        });

        let response = proto::GetOrdersResponse { orders: res };

        Ok(tonic::Response::new(response))
    }
}
