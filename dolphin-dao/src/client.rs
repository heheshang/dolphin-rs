use proto::proto_mod::{user_service_client::UserServiceClient, GetUserRequest};
use tonic::{transport::Endpoint, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = Endpoint::from_static("http://0.0.0.0:50051");

    /*
    Client code is not implemented in completely
    as it would just make the code base look too complicated ....
    and interface requires a lot of boilerplate code to implement.

    But a basic implementation is given below ....
    please refer it to implement other ways to make your code pretty
    */

    let mut client = UserServiceClient::connect(addr).await?;
    let request = Request::new(GetUserRequest {
        name: "1".to_string(),
    });
    let response = client.get_user(request).await?;

    eprintln!("RESPONSE={:?}", response);

    Ok(())
}
