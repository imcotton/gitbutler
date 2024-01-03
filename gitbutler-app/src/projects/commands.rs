use std::path;

use tauri::Manager;
use tracing::instrument;

use crate::{
    error::{Code, UserError},
    projects,
};

use super::controller::{self, Controller};

impl From<controller::UpdateError> for UserError {
    fn from(value: controller::UpdateError) -> Self {
        match value {
            controller::UpdateError::NotFound => UserError::User {
                code: Code::Projects,
                message: "Project not found".into(),
            },
            controller::UpdateError::Other(error) => {
                tracing::error!(?error, "failed to update project");
                UserError::Unknown
            }
        }
    }
}

#[tauri::command(async)]
#[instrument(skip(handle))]
pub async fn update_project(
    handle: tauri::AppHandle,
    project: projects::UpdateRequest,
) -> Result<projects::Project, UserError> {
    handle
        .state::<Controller>()
        .update(&project)
        .await
        .map_err(Into::into)
}

impl From<controller::AddError> for UserError {
    fn from(value: controller::AddError) -> Self {
        match value {
            controller::AddError::NotAGitRepository => UserError::User {
                code: Code::Projects,
                message: "Must be a git directory".to_string(),
            },
            controller::AddError::AlreadyExists => UserError::User {
                code: Code::Projects,
                message: "Project already exists".to_string(),
            },
            controller::AddError::OpenProjectRepository(error) => error.into(),
            controller::AddError::NotADirectory => UserError::User {
                code: Code::Projects,
                message: "Not a directory".to_string(),
            },
            controller::AddError::PathNotFound => UserError::User {
                code: Code::Projects,
                message: "Path not found".to_string(),
            },
            controller::AddError::User(error) => error.into(),
            controller::AddError::Other(error) => {
                tracing::error!(?error, "failed to add project");
                UserError::Unknown
            }
        }
    }
}

#[tauri::command(async)]
#[instrument(skip(handle))]
pub async fn add_project(
    handle: tauri::AppHandle,
    path: &path::Path,
) -> Result<projects::Project, UserError> {
    handle.state::<Controller>().add(path).map_err(Into::into)
}

impl From<controller::GetError> for UserError {
    fn from(value: controller::GetError) -> Self {
        match value {
            controller::GetError::NotFound => UserError::User {
                code: Code::Projects,
                message: "Project not found".into(),
            },
            controller::GetError::Other(error) => {
                tracing::error!(?error, "failed to get project");
                UserError::Unknown
            }
        }
    }
}

#[tauri::command(async)]
#[instrument(skip(handle))]
pub async fn get_project(
    handle: tauri::AppHandle,
    id: &str,
) -> Result<projects::Project, UserError> {
    let id = id.parse().map_err(|_| UserError::User {
        code: Code::Validation,
        message: "Malformed project id".into(),
    })?;
    handle.state::<Controller>().get(&id).map_err(Into::into)
}

impl From<controller::ListError> for UserError {
    fn from(value: controller::ListError) -> Self {
        match value {
            controller::ListError::Other(error) => {
                tracing::error!(?error, "failed to list projects");
                UserError::Unknown
            }
        }
    }
}

#[tauri::command(async)]
#[instrument(skip(handle))]
pub async fn list_projects(handle: tauri::AppHandle) -> Result<Vec<projects::Project>, UserError> {
    handle.state::<Controller>().list().map_err(Into::into)
}

impl From<controller::DeleteError> for UserError {
    fn from(value: controller::DeleteError) -> Self {
        match value {
            controller::DeleteError::Other(error) => {
                tracing::error!(?error, "failed to delete project");
                UserError::Unknown
            }
        }
    }
}

#[tauri::command(async)]
#[instrument(skip(handle))]
pub async fn delete_project(handle: tauri::AppHandle, id: &str) -> Result<(), UserError> {
    let id = id.parse().map_err(|_| UserError::User {
        code: Code::Validation,
        message: "Malformed project id".into(),
    })?;
    handle
        .state::<Controller>()
        .delete(&id)
        .await
        .map_err(Into::into)
}