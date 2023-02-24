use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;

pub mod person;

pub type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(url);

    r2d2::Pool::builder().max_size(5).build(manager).unwrap()
}
