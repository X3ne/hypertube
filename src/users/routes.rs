use crate::error::ApiError;
use crate::infrastructure::authorization::security::{GetUserFromSession, Security};
use crate::infrastructure::authorization::AuthorizationService;
use crate::infrastructure::models::user::{User, UserInsert, UserUpdate};
use crate::users::requests::{PatchUser, RegisterUser};
use actix_web::web;
use apistos::actix::NoContent;
use apistos::api_operation;
use apistos::web::{delete, get, patch, post, resource, scope, ServiceConfig};
use garde::Validate;
use sqlx::SqlitePool;
use tracing::instrument;
use uuid::Uuid;

pub fn config_users(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users").service(
            scope("")
                .service(resource("/@me").route(get().to(get_me)))
                .service(resource("/register").route(post().to(register)))
                .service(
                    scope("/{user_id}").service(
                        resource("")
                            .route(get().to(get_user))
                            .route(patch().to(patch_user))
                            .route(delete().to(delete_user)),
                    ),
                ),
        ),
    );
}

#[api_operation(tag = "users", operation_id = "get_me", summary = "Get current user")]
#[instrument(skip(security, pool))]
pub async fn get_me(security: Security, pool: web::Data<SqlitePool>) -> Result<web::Json<User>, ApiError> {
    let user = security.get_user(&pool).await?;

    Ok(web::Json(user))
}

#[api_operation(tag = "users", operation_id = "register", summary = "Register a new user")]
#[instrument(skip(pool))]
pub async fn register(pool: web::Data<SqlitePool>, body: web::Json<RegisterUser>) -> Result<NoContent, ApiError> {
    let new_user = body.into_inner();
    new_user.validate()?;

    User::create(
        &pool,
        &UserInsert {
            username: new_user.username,
            email: new_user.email,
            password: Some(new_user.password),
            permissions: AuthorizationService::default_permissions(),
        },
    )
    .await?;

    Ok(NoContent)
}

#[api_operation(tag = "users", operation_id = "get_user", summary = "Get a user by ID")]
#[instrument(skip(pool))]
pub async fn get_user(path: web::Path<Uuid>, pool: web::Data<SqlitePool>) -> Result<web::Json<User>, ApiError> {
    let user_id = path.into_inner();

    let user = User::get_by_id(&pool, user_id).await?;

    Ok(web::Json(user))
}

#[api_operation(tag = "users", operation_id = "patch_user", summary = "Patch a user")]
#[instrument(skip(pool))]
pub async fn patch_user(
    path: web::Path<Uuid>,
    pool: web::Data<SqlitePool>,
    body: web::Json<PatchUser>,
) -> Result<NoContent, ApiError> {
    let patch_user = body.into_inner();
    patch_user.validate()?;

    let user_id = path.into_inner();

    User::update(
        &pool,
        user_id,
        &UserUpdate {
            username: patch_user.username,
            email: patch_user.email,
            password: patch_user.password,
            permissions: None,
        },
    )
    .await?;

    Ok(NoContent)
}

#[api_operation(tag = "users", operation_id = "delete_user", summary = "Delete a user")]
#[instrument(skip(pool))]
pub async fn delete_user(path: web::Path<Uuid>, pool: web::Data<SqlitePool>) -> Result<NoContent, ApiError> {
    let user_id = path.into_inner();

    User::delete(&pool, user_id).await?;

    Ok(NoContent)
}
