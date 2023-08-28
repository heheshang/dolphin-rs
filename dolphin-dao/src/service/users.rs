use entity::t_ds_user::{self};

use proto::{
    self,
    ds_user::{
        user_service_server::{UserService, UserServiceServer},
        DsUser,
        GetUserRequest,
        UpdateUserRequest,
    },
};
use dolphin_common::core_results::results::GrpcResponse;
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
    async fn get_user(&self, request: tonic::Request<GetUserRequest>) -> GrpcResponse<DsUser> {
        let conn = &self.conn;
        let name = request.into_inner().name;
        let db_user: Option<t_ds_user::Model> = t_ds_user::Entity::find()
           // .column(t_ds_user::Column::UserName)
           .filter(t_ds_user::Column::UserName.eq(name))
           .into_model()
            .one(conn)
                   .await
                   .map_err(|_| tonic::Status::not_found("User not found"))?;
        match db_user {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::not_found("User not found")),
        }
    }

    async fn update_user(
        &self,
        request: tonic::Request<UpdateUserRequest>,
    ) -> Result<tonic::Response<DsUser>, tonic::Status> {
        let conn = &self.conn;
        if let Some(user) = request.into_inner().user {
            let db_user = t_ds_user::Entity::find_by_id(user.id)
                .one(conn)
                .await
                .map_err(|_| tonic::Status::not_found("User not found"))?;
            let mut db_user = db_user.unwrap();

            db_user.email = user.email;
            db_user.phone = user.phone;
            db_user.user_type = user.user_type;
            Ok(tonic::Response::new(db_user.into()))
        } else {
            Ok(tonic::Response::new(DsUser::default()))
        }
    }
}
