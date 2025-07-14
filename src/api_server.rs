use poem::{EndpointExt, Result, Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use sqlx::SqlitePool;

use super::message_api::MessageApi;

pub struct ApiServer {
    pool: SqlitePool,
    address: String,
}

impl ApiServer {
    pub async fn new(address: &String, database_url: &String) -> Result<Self, async_nats::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        let address = address.clone();
        Ok(ApiServer { pool, address })
    }

    pub async fn serve(self) -> Result<(), async_nats::Error> {
        let version = env!("CARGO_PKG_VERSION");
        let api_service = OpenApiService::new(MessageApi, "Messages", version).server("");
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
