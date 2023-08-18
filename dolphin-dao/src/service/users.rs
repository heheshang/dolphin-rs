use entity::t_ds_user::{self};

use proto::proto_mod::{
    self,
    user_service_server::{UserService, UserServiceServer},
    GetUserRequest,
};
use sea_orm::{entity::prelude::*, DatabaseConnection};
#[derive(Default)]
pub struct UserServer {
    conn: DatabaseConnection,
}


impl UserServer {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn into_service(self) -> UserServiceServer<Self> {
        UserServiceServer::new(self)
    }
}


#[tonic::async_trait]
impl UserService for UserServer {
    async fn get_user(
        &self,
        _request: tonic::Request<GetUserRequest>,
    ) -> Result<tonic::Response<proto_mod::User>, tonic::Status> {
        let conn = &self.conn;
        let db_user = t_ds_user::Entity::find()
           // .column(t_ds_user::Column::UserName)
            .one(conn)
                   .await
                   .map_err(|_| tonic::Status::not_found("User not found"))?;

        let user = proto_mod::User {
            id: "1".to_string(),
            name: db_user.unwrap().user_name.unwrap(),
            email: "3".to_string(),
            password: "4".to_string(),
            role: "5".to_string(),
            status: "6".to_string(),
            created_at: "7".to_string(),
            updated_at: "8".to_string(),
            deleted_at: "9".to_string(),
        };

        Ok(tonic::Response::new(user))
    }
}
