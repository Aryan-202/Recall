use crate::database::models::folder::FolderWithChildren;
use crate::database::repository::folders_repository::{
    CreateFolderDto, FolderRepository, UpdateFolderDto,
};
use crate::utils::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_folder_id: Option<i32>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFolderRequest {
    pub folder_id: i32,
    pub name: Option<String>,
    pub parent_folder_id: Option<Option<i32>>,
    pub color: Option<String>,
}

#[tauri::command]
pub async fn create_folder(
    request: CreateFolderRequest,
    repository: State<'_, FolderRepository>,
) -> Result<FolderWithChildren> {
    let dto = CreateFolderDto {
        user_id: 1, // TODO: Get from auth
        name: request.name,
        parent_folder_id: request.parent_folder_id,
        color: request.color,
    };

    let folder = repository.create_folder(dto).await?;
    Ok(folder)
}

#[tauri::command]
pub async fn get_folder(
    folder_id: i32,
    repository: State<'_, FolderRepository>,
) -> Result<FolderWithChildren> {
    let folder = repository
        .get_folder_by_id(folder_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Folder not found".to_string()))?;
    Ok(folder)
}

#[tauri::command]
pub async fn get_all_folders(
    repository: State<'_, FolderRepository>,
) -> Result<Vec<FolderWithChildren>> {
    let user_id = 1; // TODO: Get from auth
    let folders = repository.get_user_folders(user_id).await?;
    Ok(folders)
}

#[tauri::command]
pub async fn update_folder(
    request: UpdateFolderRequest,
    repository: State<'_, FolderRepository>,
) -> Result<FolderWithChildren> {
    let dto = UpdateFolderDto {
        folder_id: request.folder_id,
        name: request.name,
        parent_folder_id: request.parent_folder_id,
        color: request.color,
    };

    let folder = repository.update_folder(dto).await?;
    Ok(folder)
}

#[tauri::command]
pub async fn delete_folder(
    folder_id: i32,
    repository: State<'_, FolderRepository>,
) -> Result<bool> {
    let user_id = 1; // TODO: Get from auth
    repository.delete_folder(folder_id, user_id).await?;
    Ok(true)
}

#[tauri::command]
pub async fn get_folder_tree(
    repository: State<'_, FolderRepository>,
) -> Result<Vec<FolderWithChildren>> {
    let user_id = 1; // TODO: Get from auth
    let tree = repository.get_folder_tree(user_id).await?;
    Ok(tree)
}
