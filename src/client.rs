// use hello_world::greeter_client::GreeterClient;
// use hello_world::HelloRequest;

// pub mod hello_world {
//     tonic::include_proto!("helloworld");
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = GreeterClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(HelloRequest {
//         name: "Tonic".into(),
//     });

//     let response = client.say_hello(request).await?;

//     println!("Response = {:?}", response);

//     Ok(())
// }

pub mod user {
    tonic::include_proto!("user");
}

use user::{crud_client::CrudClient, UserRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CrudClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(UserRequest {
        id: "steadylearner".into(),
    });

    let response = client.get_user(request).await?;

    println!("RESPONSE={:?}", response);
    let user_date_of_birth = &response.into_inner().date_of_birth;
    println!("{}", user_date_of_birth);

    Ok(())
}
