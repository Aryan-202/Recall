pub mod migration_manager;

use sqlx::{Pool, Postgres};
use std::fs;
use std::path::Path;

const MIGRATIONS_DIR: &str = "migrations";

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Create migrations table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version BIGINT PRIMARY KEY,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#
    ).execute(pool).await?;
    
    // Get applied migrations
    let applied: Vec<i64> = sqlx::query_scalar(
        "SELECT version FROM schema_migrations ORDER BY version"
    )
    .fetch_all(pool)
    .await?;
    
    // Read migration files
    let migration_dir = Path::new(MIGRATIONS_DIR);
    if !migration_dir.exists() {
        fs::create_dir_all(migration_dir)?;
        create_initial_migrations(migration_dir)?;
    }
    
    let mut migration_files: Vec<_> = fs::read_dir(migration_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "sql" {
                let filename = path.file_stem()?.to_str()?;
                let version = filename.split('_').next()?.parse::<i64>().ok()?;
                Some((version, path))
            } else {
                None
            }
        })
        .collect();
    
    migration_files.sort_by_key(|(version, _)| *version);
    
    // Apply pending migrations
    for (version, path) in migration_files {
        if !applied.contains(&version) {
            println!("Applying migration: {}", path.display());
            
            let migration_sql = fs::read_to_string(&path)?;
            
            let mut transaction = pool.begin().await?;
            
            // Split SQL by semicolons and execute each statement
            for statement in migration_sql.split(';').filter(|s| !s.trim().is_empty()) {
                sqlx::query(statement).execute(&mut *transaction).await?;
            }
            
            // Record migration
            sqlx::query(
                "INSERT INTO schema_migrations (version) VALUES ($1)"
            )
            .bind(version)
            .execute(&mut *transaction)
            .await?;
            
            transaction.commit().await?;
            
            println!("Migration {} applied successfully", version);
        }
    }
    
    Ok(())
}

fn create_initial_migrations(dir: &Path) -> std::io::Result<()> {
    let initial_schema = include_str!("./migrations/0001_initial_schema.sql");
    fs::write(dir.join("0001_initial_schema.sql"), initial_schema)?;
    
    let add_tags = include_str!("./migrations/0002_add_tags.sql");
    fs::write(dir.join("0002_add_tags.sql"), add_tags)?;
    
    Ok(())
}