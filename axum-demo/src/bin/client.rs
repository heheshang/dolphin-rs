mod proto {
    include!(concat!("../pb", "/helloworld.rs"));
}
use crate::proto::{greeter_client::GreeterClient, HelloRequest};
// use once_cell::sync::Lazy;
// static CLIENT: Lazy<T> = Lazy::new(  ||{
//     let channel = GreeterClient::connect("http://127.0.0.1:3000");
//     channel
// });
use lazy_static::lazy_static;

lazy_static! {
    static ref CLIENT: GreeterClient<tonic::transport::Channel> = {
        let channel =
            tonic::transport::Channel::from_static("http://127.0.0.1:3000").connect_lazy();
        GreeterClient::new(channel)
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = CLIENT.clone();
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;
    println!("RESPONSE={:?}", response);


    Ok(())
}
