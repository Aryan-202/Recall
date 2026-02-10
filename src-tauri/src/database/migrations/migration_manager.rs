use sqlx::{Pool, Postgres};
use std::fs;

#[allow(dead_code)]
pub struct MigrationManager {
    migrations_dir: String,
}

impl MigrationManager {
    #[allow(dead_code)]
    pub fn new(migrations_dir: &str) -> Self {
        Self {
            migrations_dir: migrations_dir.to_string(),
        }
    }

    #[allow(dead_code)]
    pub async fn run_migrations(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_migrations_table(pool).await?;

        let applied_migrations = self.get_applied_migrations(pool).await?;
        let available_migrations = self.get_available_migrations()?;

        for migration in available_migrations {
            if !applied_migrations.contains(&migration.name) {
                self.apply_migration(pool, &migration).await?;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn ensure_migrations_table(&self, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS _migrations (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL UNIQUE,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )",
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    async fn get_applied_migrations(
        &self,
        pool: &Pool<Postgres>,
    ) -> Result<Vec<String>, sqlx::Error> {
        let migrations = sqlx::query_scalar::<_, String>("SELECT name FROM _migrations")
            .fetch_all(pool)
            .await?;

        Ok(migrations)
    }

    #[allow(dead_code)]
    fn get_available_migrations(&self) -> Result<Vec<Migration>, Box<dyn std::error::Error>> {
        let mut migrations = Vec::new();
        let paths = fs::read_dir(&self.migrations_dir)?;

        for path in paths {
            let path = path?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                let name = path.file_name().unwrap().to_str().unwrap().to_string();
                let content = fs::read_to_string(&path)?;
                migrations.push(Migration { name, content });
            }
        }

        migrations.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(migrations)
    }

    #[allow(dead_code)]
    async fn apply_migration(
        &self,
        pool: &Pool<Postgres>,
        migration: &Migration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut tx = pool.begin().await?;

        sqlx::query(&migration.content).execute(&mut *tx).await?;

        sqlx::query("INSERT INTO _migrations (name) VALUES ($1)")
            .bind(&migration.name)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}

#[allow(dead_code)]
struct Migration {
    name: String,
    content: String,
}
