use super::super::models::folder::{Folder, FolderWithChildren};
use crate::utils::error::{AppError, Result};
use sqlx::{Pool, Postgres};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct FolderRepository {
    pool: Pool<Postgres>,
}

impl FolderRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn create_folder(&self, dto: CreateFolderDto) -> Result<FolderWithChildren> {
        let folder = sqlx::query_as::<_, Folder>(
            r#"
            INSERT INTO folders (user_id, name, parent_folder_id, color)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(dto.user_id)
        .bind(&dto.name)
        .bind(dto.parent_folder_id)
        .bind(&dto.color)
        .fetch_one(&self.pool)
        .await?;
        
        self.get_folder_with_children(folder.folder_id).await
    }
    
    pub async fn get_folder_by_id(&self, folder_id: i32) -> Result<Option<FolderWithChildren>> {
        let folder = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE folder_id = $1"
        )
        .bind(folder_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(folder) = folder {
            self.get_folder_with_children(folder.folder_id).await.map(Some)
        } else {
            Ok(None)
        }
    }
    
    pub async fn get_user_folders(&self, user_id: i32) -> Result<Vec<FolderWithChildren>> {
        let folders = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE user_id = $1 ORDER BY name"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for folder in folders {
            let with_children = self.get_folder_with_children(folder.folder_id).await?;
            result.push(with_children);
        }
        
        Ok(result)
    }
    
    pub async fn update_folder(&self, dto: UpdateFolderDto) -> Result<FolderWithChildren> {
        // Build update query dynamically
        let mut query = "UPDATE folders SET updated_at = $1".to_string();
        let mut params: Vec<String> = vec![Utc::now().to_string()];
        let mut param_count = 2;
        
        if let Some(name) = &dto.name {
            query.push_str(&format!(", name = ${}", param_count));
            params.push(name.clone());
            param_count += 1;
        }
        
        if let Some(parent_folder_id) = dto.parent_folder_id {
            query.push_str(&format!(", parent_folder_id = ${}", param_count));
            params.push(parent_folder_id.map(|id| id.to_string()).unwrap_or("NULL".to_string()));
            param_count += 1;
        }
        
        if let Some(color) = &dto.color {
            query.push_str(&format!(", color = ${}", param_count));
            params.push(color.clone());
            param_count += 1;
        }
        
        query.push_str(&format!(" WHERE folder_id = ${} RETURNING *", param_count));
        params.push(dto.folder_id.to_string());
        
        // Execute update
        let folder = sqlx::query_as::<_, Folder>(&query);
        let mut query_builder = folder;
        
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        
        let folder = query_builder.fetch_one(&self.pool).await?;
        
        self.get_folder_with_children(folder.folder_id).await
    }
    
    pub async fn delete_folder(&self, folder_id: i32, user_id: i32) -> Result<()> {
        // Check if folder has notes
        let note_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM notes WHERE folder_id = $1 AND user_id = $2 AND is_deleted = FALSE"
        )
        .bind(folder_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        if note_count > 0 {
            return Err(AppError::InvalidInput(
                "Cannot delete folder that contains notes".to_string()
            ));
        }
        
        // Check if folder has subfolders
        let subfolder_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM folders WHERE parent_folder_id = $1 AND user_id = $2"
        )
        .bind(folder_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        if subfolder_count > 0 {
            return Err(AppError::InvalidInput(
                "Cannot delete folder that contains subfolders".to_string()
            ));
        }
        
        sqlx::query("DELETE FROM folders WHERE folder_id = $1 AND user_id = $2")
            .bind(folder_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn get_folder_tree(&self, user_id: i32) -> Result<Vec<FolderWithChildren>> {
        let root_folders = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE user_id = $1 AND parent_folder_id IS NULL ORDER BY name"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut tree = Vec::new();
        for folder in root_folders {
            let with_children = self.build_folder_tree(&folder).await?;
            tree.push(with_children);
        }
        
        Ok(tree)
    }
    
    async fn build_folder_tree(&self, parent: &Folder) -> Result<FolderWithChildren> {
        let children = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE parent_folder_id = $1 ORDER BY name"
        )
        .bind(parent.folder_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut child_trees = Vec::new();
        for child in children {
            let child_tree = self.build_folder_tree(&child).await?;
            child_trees.push(child_tree);
        }
        
        let note_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM notes WHERE folder_id = $1 AND is_deleted = FALSE"
        )
        .bind(parent.folder_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(FolderWithChildren {
            folder: parent.clone(),
            children: child_trees,
            note_count,
        })
    }
    
    async fn get_folder_with_children(&self, folder_id: i32) -> Result<FolderWithChildren> {
        let folder = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE folder_id = $1"
        )
        .bind(folder_id)
        .fetch_one(&self.pool)
        .await?;
        
        let children = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE parent_folder_id = $1 ORDER BY name"
        )
        .bind(folder_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut child_trees = Vec::new();
        for child in children {
            let child_tree = self.get_folder_with_children(child.folder_id).await?;
            child_trees.push(child_tree);
        }
        
        let note_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM notes WHERE folder_id = $1 AND is_deleted = FALSE"
        )
        .bind(folder_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(FolderWithChildren {
            folder,
            children: child_trees,
            note_count,
        })
    }
}

#[derive(Debug)]
pub struct CreateFolderDto {
    pub user_id: i32,
    pub name: String,
    pub parent_folder_id: Option<i32>,
    pub color: Option<String>,
}

#[derive(Debug)]
pub struct UpdateFolderDto {
    pub folder_id: i32,
    pub name: Option<String>,
    pub parent_folder_id: Option<Option<i32>>,
    pub color: Option<String>,
}