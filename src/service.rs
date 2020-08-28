use chrono::*;
use uuid::Uuid;

use mysql::prelude::*;

use crate::db_connection::establish_connection;

use tonic::{Request, Response, Status};

use crate::user::{
    crud_server::Crud, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty, UpdateUserReply,
    UpdateUserRequest, UserReply, UserRequest, Users,
};

#[derive(Default)]
pub struct User {}

#[tonic::async_trait]
impl Crud for User {
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Get a requst : {:#?}", &request);

        let UserRequest { id } = &request.into_inner();

        let conn = establish_connection();

        let rows = conn
            .as_mut()
            .query_map(
                format!(
                    "select id, first_name, last_name, date_of_birth from users where id = {}",
                    &id
                ),
                |(id, first_name, last_name, date_of_birth)| UserReply {
                    id: id,
                    first_name: first_name,
                    last_name: last_name,
                    date_of_birth: date_of_birth,
                },
            )
            .unwrap();


        let result = rows.get(0).unwrap();


        Ok(Response::new(UserReply {
            id: "1".to_string(),
            first_name: "alskdf".to_string(),
            last_name: "2".to_string(),
            date_of_birth: "123".to_string(),
        }))
    }
}
