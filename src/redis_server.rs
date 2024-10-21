use redis::AsyncCommands;
use redis::Client;

use tokio::sync::Mutex;

pub struct RedisClient{
    client: Client,
    connection: Mutex<Option<redis::aio::MultiplexedConnection>>
}

impl RedisClient {
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        let connection = client.get_multiplexed_async_connection().await?;
        Ok(RedisClient {
            client,
            connection: Mutex::new(Some(connection)),
        })
    }


    pub async fn get_item(&self, key: &str) -> redis::RedisResult<String> {
        let mut conn_guard = self.connection.lock().await;
        if conn_guard.is_none() {
            *conn_guard = Some(self.client.get_multiplexed_async_connection().await?);
        }
        let conn = conn_guard.as_mut().unwrap();
        conn.get(key).await
    }

    pub async fn set_item(&self, key: &str, value: &str) -> redis::RedisResult<()> {
        let mut conn_guard = self.connection.lock().await;
        if conn_guard.is_none() {
            *conn_guard = Some(self.client.get_multiplexed_async_connection().await?);
        }
        let conn = conn_guard.as_mut().unwrap();
        conn.set(key, value).await
    }


}