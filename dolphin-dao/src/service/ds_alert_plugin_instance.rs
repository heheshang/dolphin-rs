use proto::ds_alert_plugin_instance::ds_alert_plugin_instance_bean_service_server::DsAlertPluginInstanceBeanService;
// use sea_orm::entity::prelude::*;

use super::service::DolphinRpcServer;


#[tonic::async_trait]
impl DsAlertPluginInstanceBeanService for DolphinRpcServer {
    async fn list_ds_alert_plugin_instance_beans(
        &self,
        _request: tonic::Request<
            proto::ds_alert_plugin_instance::ListDsAlertPluginInstanceBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::ListDsAlertPluginInstanceBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_plugin_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_alert_plugin_instance::GetDsAlertPluginInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::DsAlertPluginInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_alert_plugin_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_alert_plugin_instance::CreateDsAlertPluginInstanceBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::DsAlertPluginInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_alert_plugin_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_alert_plugin_instance::UpdateDsAlertPluginInstanceBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_plugin_instance::DsAlertPluginInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_alert_plugin_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_alert_plugin_instance::DeleteDsAlertPluginInstanceBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
