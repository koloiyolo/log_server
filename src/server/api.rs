use poem::{EndpointExt, Result, Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;

use crate::api::{message::MessageApi, user::UserApi};

pub struct ApiServer {
    pool: SqlitePool,
    address: String,
}

impl ApiServer {
    pub async fn new(address: &str, database_url: &str) -> Result<Self, async_nats::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        let address = address.to_owned();
        Ok(ApiServer { pool, address })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        let version = env!("CARGO_PKG_VERSION");
        let api_service =
            OpenApiService::new((MessageApi, UserApi), "Message and User API", version).server("");

        let docs = api_service.openapi_explorer();

        let route = Route::new()
            .nest("/", api_service)
            .nest("/docs", docs)
            .data(self.pool);

        Server::new(TcpListener::bind(&self.address))
            .run(route)
            .await?;
        Ok(())
    }
}
