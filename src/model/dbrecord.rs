pub trait DbRecord<Pool>: Sized {
    type Id: Send + Sync;

    async fn insert(self, pool: &Pool) -> Result<Self::Id, sqlx::Error>;
    async fn find_by_id(id: Self::Id, pool: &Pool) -> Result<Option<Self>, sqlx::Error>;
    async fn search(query: String, pool: &Pool) -> Result<Vec<Self>, sqlx::Error>;
    async fn get_limit(limit: usize, pool: &Pool) -> Result<Vec<Self>, sqlx::Error>;
    async fn delete(id: Self::Id, pool: &Pool) -> Result<(), sqlx::Error>;
    async fn update(self, pool: &Pool) -> Result<(), sqlx::Error>;
}
