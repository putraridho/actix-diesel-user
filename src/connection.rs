use actix_web::web::Data;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use dotenv::dotenv;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_pool_connection() -> PgPool {
    dotenv().ok();
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_pool(pool: Data<PgPool>) -> PgPooledConnection {
    pool.get().expect("Failed to get pool")
}
