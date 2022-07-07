use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub mod user;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
