use actix_web::{
    delete,
    error::ErrorInternalServerError,
    get, post, put,
    web::{block, Data, Json, Path},
    Error, HttpResponse,
};
use uuid::Uuid;

use crate::{
    connection::get_pool,
    model::user::{NewUser, UpdateUser, User},
};

use super::DbPool;

#[get("/user")]
pub async fn get_users(pool: Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = block(move || {
        let conn = get_pool(pool);
        User::get_some(&conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/user/{id}")]
pub async fn get_user_by_id(pool: Data<DbPool>, id: Path<Uuid>) -> Result<HttpResponse, Error> {
    let uid = id.into_inner();

    let user = block(move || {
        let conn = get_pool(pool);
        User::get_by_id(uid, &conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {uid}"));
        Ok(res)
    }
}

#[post("/user")]
pub async fn add_user(pool: Data<DbPool>, new_user: Json<NewUser>) -> Result<HttpResponse, Error> {
    let user = block(move || {
        let conn = get_pool(pool);
        User::create_new(new_user.into_inner(), &conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/user/{id}")]
pub async fn update_user(
    pool: Data<DbPool>,
    update_user: Json<UpdateUser>,
    id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let uid = id.into_inner();
    let req_body = update_user.into_inner();

    let user = block(move || {
        let conn = get_pool(pool);
        User::update_by_id(uid, req_body, &conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/user/{id}")]
pub async fn remove_user(pool: Data<DbPool>, id: Path<Uuid>) -> Result<HttpResponse, Error> {
    let uid = id.into_inner();

    block(move || {
        let conn = get_pool(pool);
        User::remove_by_id(uid, &conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(format!("user id {} was deleted", uid.to_string())))
}
