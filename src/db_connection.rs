use dotenv::dotenv;
use mysql::*;
use std::env;

use crate::env as envFileUse;

pub fn establish_connection() -> PooledConn {
    dotenv().ok();

    let mut envfile = unsafe { envFileUse::envComponentSingleton.getData() };
    let data = &envfile.store;

    let url = format! {"mysql://{}@{}:{}/{}", data["username"], data["database_server"], data["port"], data["database"]};

    println!("{}", url);

    let pool = Pool::new(url).unwrap();

    pool.get_conn().unwrap()
}
