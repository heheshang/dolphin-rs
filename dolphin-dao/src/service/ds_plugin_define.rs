use super::service::DolphinRpcServer;
use proto::ds_plugin_define::ds_plugin_define_bean_service_server::DsPluginDefineBeanService;

#[tonic::async_trait]
impl DsPluginDefineBeanService for DolphinRpcServer {
    async fn list_ds_plugin_define_beans(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::ListDsPluginDefineBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_plugin_define::ListDsPluginDefineBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_plugin_define_bean(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::GetDsPluginDefineBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_plugin_define::DsPluginDefineBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_plugin_define_bean(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::CreateDsPluginDefineBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_plugin_define::DsPluginDefineBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_plugin_define_bean(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::UpdateDsPluginDefineBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_plugin_define::DsPluginDefineBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_plugin_define_bean(
        &self,
        _req: tonic::Request<proto::ds_plugin_define::DeleteDsPluginDefineBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
