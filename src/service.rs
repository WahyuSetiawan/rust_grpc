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

        let mut conn = establish_connection();

        let rows: Vec<UserReply> = conn
            .as_mut()
            .query_map(
                format!(
                    "select id, first_name, last_name, date_of_birth from users where id = \'{}\'",
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


        let result = rows.get(0).unwrap().clone();

        Ok(Response::new(result))
    }

    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        println!("Got a request {:#?}", &request);

        let mut conn = establish_connection();

        let rows: Vec<UserReply> = conn
            .query_map(
                "select id, first_name, last_name, date_of_birth from users ",
                |(id, first_name, last_name, date_of_birth)| UserReply {
                    id: id,
                    first_name: first_name,
                    last_name: last_name,
                    date_of_birth: date_of_birth,
                },
            )
            .unwrap();

        Ok(Response::new(Users { users: rows }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request {:#?}", &request);

        let mut conn = establish_connection();

        let CreateUserRequest {
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();

        let result_insert =   conn.as_mut()
        .exec_drop("insert into users (first_name, last_name, date_of_birth) values (:first_name, :last_name, :date_of_birth)", params! {
            "first_name" => &first_name,
            "last_name" => &last_name, 
            "date_of_birth" => &date_of_birth
        });

        Ok(Response::new(CreateUserReply {
            message: "masih dalam pengerjaan".to_string(),
        }))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Got a request {:#?}", &request);

        let mut conn = establish_connection();

        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            date_of_birth,
        } = request.into_inner();

        let result_insert =   conn.as_mut()
        .exec_drop("update users set firstname = :first_name, last_name = :last_name, date_of_birth = date_of_birth where id = :id", params! {
            "id" => &id,
            "first_name" => &first_name,
            "last_name" => &last_name, 
            "date_of_birth" => &date_of_birth
        });

        Ok(Response::new(UpdateUserReply {
            message: "update user request dalam pengerjaan".to_string(),
        }))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request {:#?}", &request);

        let mut conn = establish_connection();
        let UserRequest { id } = request.into_inner();

        let rows = conn.as_mut().exec_drop(
            "delete from users where id = :id",
            params! {
                "id" => &id
            },
        );

        Ok(Response::new(DeleteUserReply {
            message: "delete user masih dalam pengerjaan".to_string(),
        }))
    }

    async fn delete(&self, request: Request<Empty>) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request {:#?}", &request);

        let mut conn = establish_connection();

        let rows: Option<String> = conn.as_mut().query_first("delete from users").unwrap();

        Ok(Response::new(DeleteUserReply {
            message: format!("table from users table success to delete"),
        }))
    }
}
