use sqlx::{Pool, Postgres};
use std::fs;
use std::path::Path;

pub struct MigrationManager {
    migrations_dir: String,
}

impl MigrationManager {
    pub fn new(migrations_dir: &str) -> Self {
        Self {
            migrations_dir: migrations_dir.to_string(),
        }
    }
    
    pub async fn run_migrations(&self, pool: &Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
        self.ensure_migrations_table(pool).await?;
        
        let applied = self.get_applied_migrations(pool).await?;
        let available = self.get_available_migrations()?;
        
        for migration in available {
            if !applied.contains(&migration.version) {
                self.apply_migration(pool, &migration).await?;
            }
        }
        
        Ok(())
    }
    
    async fn ensure_migrations_table(&self, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS migrations (
                id SERIAL PRIMARY KEY,
                version VARCHAR(50) NOT NULL UNIQUE,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )"
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_applied_migrations(&self, pool: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
        let versions: Vec<String> = sqlx::query_scalar(
            "SELECT version FROM migrations ORDER BY version"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(versions)
    }
    
    fn get_available_migrations(&self) -> Result<Vec<Migration>, Box<dyn std::error::Error>> {
        let mut migrations = Vec::new();
        let dir = Path::new(&self.migrations_dir);
        
        if !dir.exists() {
            fs::create_dir_all(dir)?;
            return Ok(migrations);
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|ext| ext.to_str()) == Some("sql") {
                let filename = path.file_stem()
                    .ok_or("Invalid filename")?
                    .to_string_lossy();
                
                let parts: Vec<&str> = filename.split('_').collect();
                if parts.len() >= 2 {
                    let version = parts[0].to_string();
                    let name = parts[1..].join("_");
                    
                    let sql = fs::read_to_string(&path)?;
                    
                    migrations.push(Migration {
                        version,
                        name,
                        sql,
                    });
                }
            }
        }
        
        migrations.sort_by_key(|m| m.version.clone());
        Ok(migrations)
    }
    
    async fn apply_migration(&self, pool: &Pool<Postgres>, migration: &Migration) -> Result<(), Box<dyn std::error::Error>> {
        let mut transaction = pool.begin().await?;
        
        // Execute migration SQL
        sqlx::query(&migration.sql)
            .execute(&mut *transaction)
            .await?;
        
        // Record migration
        sqlx::query(
            "INSERT INTO migrations (version, name) VALUES ($1, $2)"
        )
        .bind(&migration.version)
        .bind(&migration.name)
        .execute(&mut *transaction)
        .await?;
        
        transaction.commit().await?;
        
        println!("Applied migration: {} - {}", migration.version, migration.name);
        Ok(())
    }
}

struct Migration {
    version: String,
    name: String,
    sql: String,
}