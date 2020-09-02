use dotenv::dotenv;
use mysql::*;

use crate::env as envFileUse;

pub fn establish_connection() -> PooledConn {
    dotenv().ok();

    let envfile = unsafe { envFileUse::envComponentSingleton.getData() };
    let data = &envfile.store;

    let url = format! {"mysql://{}@{}:{}/{}", data["username"], data["database_server"], data["port"], data["database"]};

    println!("{}", url);

    let pool = Pool::new(url).unwrap();

    pool.get_conn().unwrap()
}
