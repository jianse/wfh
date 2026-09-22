use axum::Extension;
use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::IntoActiveModel;
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use serde::Deserialize;
use serde::Serialize;

use crate::AppState;
use crate::entities;

use crate::api::BizError;
use crate::api::BizResponse;
use crate::api::BizResult;
use crate::api::CurrentUser;
use crate::api::Pager;
use crate::api::Role;
use crate::api::RequireRoleLayer;
use entities::prelude::*;

pub(crate) fn user_routes(state: AppState) -> Router<AppState> {
    let admin_routes = Router::new()
        .route("/list", get(get_user_list))
        .route("/create", post(create_user))
        .route("/update/{id}", put(update_user))
        .route("/delete/{id}", delete(delete_user))
        .route("/reset-password/{id}", post(reset_password))
        .route("/update-role/{id}", put(update_user_role))
        .layer(RequireRoleLayer::at_least(state, Role::Admin));

    Router::new()
        .route("/current", get(get_current_user))
        .route("/{id}", get(get_user_by_id))
        .nest("/admin", admin_routes)
}

#[derive(Serialize)]
struct UserResp {
    id: i64,
    username: String,
    role: String,
}

impl From<entities::user::Model> for UserResp {
    fn from(value: entities::user::Model) -> Self {
        Self {
            id: value.id.into(),
            username: value.username,
            role: value.role,
        }
    }
}

#[axum::debug_handler]
async fn get_user_by_id(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserResp>, StatusCode> {
    let u = User::find_by_id(id).one(&state.conn).await.unwrap();
    if let Some(u) = u {
        Ok(Json(u.into()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[axum::debug_handler]
async fn get_current_user(
    state: State<AppState>,
    Extension(cur_user): Extension<CurrentUser>,
) -> BizResult<BizResponse<UserResp>> {
    let u = User::find_by_id(cur_user.id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(cur_user.id))?;
    Ok(BizResponse::ok_with(u.into()))
}

#[axum::debug_handler]
async fn get_user_list(
    state: State<AppState>,
    Query(pager): Query<Pager>,
) -> BizResult<BizResponse<super::Page<UserResp>>> {
    let paginator = User::find().paginate(&state.conn, pager.page_size);
    let total = paginator.num_items().await?;
    let users = paginator
        .fetch_page(pager.page - 1)
        .await?
        .into_iter()
        .map(UserResp::from)
        .collect();
    Ok(BizResponse::ok_page(
        pager.page,
        pager.page_size,
        total,
        users,
    ))
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    role: String,
}

#[axum::debug_handler]
async fn create_user(
    state: State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> BizResult<BizResponse<UserResp>> {
    // 检查用户名是否已存在
    let existing_user = User::find()
        .filter(entities::user::Column::Username.eq(&req.username))
        .one(&state.conn)
        .await?;
    if existing_user.is_some() {
        return Err(BizError::custom("Username already exists"));
    }

    // 验证角色
    if req.role.parse::<Role>().is_err() {
        return Err(BizError::custom(
            "Invalid role. Must be one of: admin, maintainer, normal",
        ));
    }

    // 哈希密码
    let hashed_password = settings::get().auth.pwd_hasher.hash(req.password)?;

    // 创建用户
    let user = entities::user::ActiveModel {
        username: sea_orm::ActiveValue::Set(req.username),
        password: sea_orm::ActiveValue::Set(hashed_password),
        role: sea_orm::ActiveValue::Set(req.role),
        ..Default::default()
    };

    let created_user = User::insert(user).exec(&state.conn).await?;

    // 获取创建后的用户信息
    let user_model = User::find_by_id(created_user.last_insert_id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(created_user.last_insert_id))?;

    Ok(BizResponse::ok_with(user_model.into()))
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    username: String,
    role: String,
}

#[axum::debug_handler]
async fn update_user(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRequest>,
) -> BizResult<BizResponse<UserResp>> {
    if id == 0 && req.role != "admin" {
        return Err(BizError::custom("Cannot update user role with ID 0"));
    }

    // 检查用户是否存在
    let existing_user = User::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(id))?;

    // 检查用户名是否已被其他用户使用
    if req.username != existing_user.username {
        let user_with_same_name = User::find()
            .filter(entities::user::Column::Username.eq(&req.username))
            .filter(entities::user::Column::Id.ne(id))
            .one(&state.conn)
            .await?;
        if user_with_same_name.is_some() {
            return Err(BizError::custom("Username already exists"));
        }
    }

    // 验证角色
    if req.role.parse::<Role>().is_err() {
        return Err(BizError::custom(
            "Invalid role. Must be one of: admin, maintainer, normal",
        ));
    }

    // 更新用户信息
    let mut user = existing_user.into_active_model();
    user.username = sea_orm::ActiveValue::Set(req.username);
    user.role = sea_orm::ActiveValue::Set(req.role);

    let updated_user = User::update(user).exec(&state.conn).await?;

    Ok(BizResponse::ok_with(updated_user.into()))
}

#[axum::debug_handler]
async fn delete_user(state: State<AppState>, Path(id): Path<i32>) -> BizResult<BizResponse<()>> {
    // 禁止删除ID为0的用户
    if id == 0 {
        return Err(BizError::custom("Can not delete user with id 0"));
    }

    // 检查用户是否存在
    let _existing_user = User::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(id))?;

    // 删除用户
    User::delete_by_id(id).exec(&state.conn).await?;

    Ok(BizResponse::ok())
}

#[derive(Deserialize)]
struct ResetPasswordRequest {
    password: String,
}

#[axum::debug_handler]
async fn reset_password(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<ResetPasswordRequest>,
) -> BizResult<BizResponse<()>> {
    // 检查用户是否存在
    let existing_user = User::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(id))?;

    // 哈希新密码
    let hashed_password = settings::get().auth.pwd_hasher.hash(req.password)?;

    // 更新密码
    let mut user = existing_user.into_active_model();
    user.password = sea_orm::ActiveValue::Set(hashed_password);

    User::update(user).exec(&state.conn).await?;

    Ok(BizResponse::ok())
}

#[derive(Deserialize)]
struct UpdateUserRoleRequest {
    role: String,
}

#[axum::debug_handler]
async fn update_user_role(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> BizResult<BizResponse<UserResp>> {
    // 检查用户是否存在
    let existing_user = User::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(id))?;

    // 验证角色
    if req.role.parse::<Role>().is_err() {
        return Err(BizError::custom(
            "Invalid role. Must be one of: admin, maintainer, normal",
        ));
    }

    // 更新用户角色
    let mut user = existing_user.into_active_model();
    user.role = sea_orm::ActiveValue::Set(req.role);

    User::update(user).exec(&state.conn).await?;

    // 获取更新后的用户信息
    let user_model = User::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(BizError::NoSuchUserId(id))?;

    Ok(BizResponse::ok_with(user_model.into()))
}
