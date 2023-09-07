use dolphin_common::{
    core_error::error::DolphinErrorInfo,
    core_results::results::{GrpcRequest, GrpcResponse},
    core_status::app_status::AppStatus,
};
use entity::t_ds_access_token;
use proto::ds_access_token::{
    ds_access_token_bean_service_server::DsAccessTokenBeanService,
    CreateDsAccessTokenBeanRequest,
    DeleteDsAccessTokenBeanRequest,
    DsAccessTokenBean,
    GetDsAccessTokenBeanRequest,
    ListDsAccessTokenBeansRequest,
    ListDsAccessTokenBeansResponse,
    UpdateDsAccessTokenBeanRequest,
};
use sea_orm::{entity::prelude::*, QueryOrder};

use super::service::DolphinRpcServer;

#[tonic::async_trait]
impl DsAccessTokenBeanService for DolphinRpcServer {
    async fn list_ds_access_token_beans(
        &self,
        req: GrpcRequest<ListDsAccessTokenBeansRequest>,
    ) -> GrpcResponse<ListDsAccessTokenBeansResponse> {
        let page_size = req.get_ref().page_size;
        let page_num = req.get_ref().page_num;

        let paginator = t_ds_access_token::Entity::find()
            .order_by_asc(t_ds_access_token::Column::Id)
            .paginate(&self.conn, page_size);

        let num_pages = paginator.num_pages().await.map_err(|_| {
            tonic::Status::from_error(Box::<DolphinErrorInfo>::new(
                AppStatus::InternalServerErrorArgs.into(),
            ))
        })? as i32;
        // Fetch paginated AccessToken
        let res: (Vec<t_ds_access_token::Model>, i32) = paginator
            .fetch_page(page_num - 1)
            .await
            .map(|p| (p, num_pages))
            .map_err(|_| {
                tonic::Status::from_error(Box::<DolphinErrorInfo>::new(
                    AppStatus::InternalServerErrorArgs.into(),
                ))
            })?;

        let ss: Vec<DsAccessTokenBean> = res.0.into_iter().map(|v| v.into()).collect();
        Ok(tonic::Response::new(ListDsAccessTokenBeansResponse {
            ds_access_token_beans: ss,
            next_page_token: todo!(),
        }))
    }

    async fn get_ds_access_token_bean(
        &self,
        _req: GrpcRequest<GetDsAccessTokenBeanRequest>,
    ) -> GrpcResponse<DsAccessTokenBean> {
        todo!()
    }

    async fn create_ds_access_token_bean(
        &self,
        _req: GrpcRequest<CreateDsAccessTokenBeanRequest>,
    ) -> GrpcResponse<DsAccessTokenBean> {
        todo!()
    }

    async fn update_ds_access_token_bean(
        &self,
        _req: GrpcRequest<UpdateDsAccessTokenBeanRequest>,
    ) -> GrpcResponse<DsAccessTokenBean> {
        todo!()
    }

    async fn delete_ds_access_token_bean(
        &self,
        _req: GrpcRequest<DeleteDsAccessTokenBeanRequest>,
    ) -> GrpcResponse<()> {
        todo!()
    }
}
