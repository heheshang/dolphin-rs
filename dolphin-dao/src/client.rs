use proto::{
    ds_access_token::ds_access_token_bean_service_client::DsAccessTokenBeanServiceClient,
    ds_user::{ds_user_bean_service_client::DsUserBeanServiceClient, GetDsUserBeanRequest},
};
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

    let mut client: DsUserBeanServiceClient<tonic::transport::Channel> =
        DsUserBeanServiceClient::connect(addr.clone()).await?;
    let request = Request::new(GetDsUserBeanRequest {
        name: "admin".to_string(),
    });
    let response = client.get_ds_user_bean(request).await?;


    let mut client_acc = DsAccessTokenBeanServiceClient::connect(addr.clone()).await?;

    let resp = client_acc
        .list_ds_access_token_beans(Request::new(
            proto::ds_access_token::ListDsAccessTokenBeansRequest {
                page_size: 5,
                page_num: 1,
            },
        ))
        .await?;

    eprintln!("RESPONSE={:?}", response);
    eprintln!("RESPONSE={:?}", resp);

    Ok(())
}
