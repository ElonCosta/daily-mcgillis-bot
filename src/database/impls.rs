use sqlx::{Executor, Result, Sqlite, SqliteConnection};

use crate::database::models::ImageDescriptor;

impl ImageDescriptor {
    pub async fn get_random<'e, 'c: 'e, E>(conn: E) -> Result<Self>
    where
        E: 'e + Executor<'c, Database = Sqlite>,
    {
        let descriptor = sqlx::query_as!(
            ImageDescriptor,
            "SELECT * FROM image_descriptors ORDER BY (RANDOM() * weight) LIMIT 1"
        )
        .fetch_one(conn)
        .await?;

        Ok(descriptor)
    }

    pub async fn update_weights(conn: &mut SqliteConnection, descriptor_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE image_descriptors SET weight = weight + 0.02 WHERE descriptor_id <> $1;
            UPDATE image_descriptors SET weight = 0 WHERE descriptor_id = $1;",
        )
        .bind(descriptor_id)
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}
