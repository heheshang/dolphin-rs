use proto::ds_user::ds_user_bean_service_client::DsUserBeanServiceClient;


macro_rules! get_client {
    ($fn_name:ident, $service_type:ident) => {
        pub async fn $fn_name() -> anyhow::Result<$service_type<tonic::transport::Channel>> {
            dotenvy::dotenv().ok();
            let host = std::env::var("DOLPHIN_DAO_CLIENT_HOST")
                .expect("HOST is not set in .env file")
                .clone();
            let port = std::env::var("DOLPHIN_DAO_CLIENT_PORT")
                .expect("PORT is not set in .env file")
                .clone();
            let addr_str = format!("http://{host}:{port}").clone();
            let addr = tonic::transport::Endpoint::from_shared(addr_str);
            match $service_type::connect(addr.unwrap()).await {
                Ok(client) => Ok(client),
                Err(e) => Err(anyhow::Error::new(e)),
            }
        }
    };
}
macro_rules! init_contanst_client {
    ($contanst_name:ident, $service_type:ident) => {
        pub static $contanst_name: tokio::sync::OnceCell<
            anyhow::Result<$service_type<tonic::transport::Channel>>,
        > = tokio::sync::OnceCell::const_new();
    };
}


// pub static USER_SERVICE_CLIENT: OnceCell<Result<DsUserBeanServiceClient<Channel>>> =
//     OnceCell::const_new();

init_contanst_client!(USER_SERVICE_CLIENT, DsUserBeanServiceClient);
get_client!(get_user_client, DsUserBeanServiceClient);


// pub async fn get_client() -> Result<DsUserBeanServiceClient<Channel>> {
//     dotenvy::dotenv().ok();
//     let host = env::var("DOLPHIN_DAO_CLIENT_HOST")
//         .expect("HOST is not set in .env file")
//         .clone();
//     let port = env::var("DOLPHIN_DAO_CLIENT_PORT")
//         .expect("PORT is not set in .env file")
//         .clone();
//     let addr_str = format!("http://{host}:{port}").clone();
//     let addr = Endpoint::from_shared(addr_str);
//     match DsUserBeanServiceClient::connect(addr.unwrap()).await {
//         Ok(client) => Ok(client),
//         Err(e) => Err(anyhow::Error::new(e)),
//     }
// }
