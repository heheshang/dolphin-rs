use crate::{core_results::results::ApiResult, core_status::app_status::AppStatus};
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

pub struct AppError(pub AppStatus);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self.0 {
            AppStatus::SUCCESS => Json(ApiResult::new(Some(()))).into_response(),
            _ => Json(ApiResult::new_with_err_status(Some(()), self.0)).into_response(),
        }
    }
}


impl<E> From<E> for AppError
where E: Into<AppStatus>
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DolphinErrorInfo {
    pub code: i32,
    pub en_msg: String,
    pub cn_msg: String,
}

impl DolphinErrorInfo {
    pub fn new(code: i32, en_msg: String, cn_msg: String) -> DolphinErrorInfo {
        DolphinErrorInfo {
            code,
            en_msg,
            cn_msg,
        }
    }
}
impl std::error::Error for DolphinErrorInfo {}
impl core::fmt::Display for DolphinErrorInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "code:{},en_msg:{},cn_msg:{}",
            self.code, self.en_msg, self.cn_msg
        )
    }
}


impl Default for DolphinErrorInfo {
    fn default() -> Self {
        Self {
            code: 0,
            en_msg: "success".to_string(),
            cn_msg: "成功".to_string(),
        }
    }
}

impl From<AppStatus> for DolphinErrorInfo {
    fn from(status: AppStatus) -> Self {
        match status {
            AppStatus::SUCCESS =>
                DolphinErrorInfo::new(0, "success".to_string(), "成功".to_string()),
            AppStatus::InternalServerErrorArgs => DolphinErrorInfo::new(
                10000,
                "internal server error please check the log".to_string(),
                "内部服务错误，请查看日志".to_string(),
            ),
            AppStatus::RequestParamsNotValidError => DolphinErrorInfo::new(
                10001,
                "request parameter {0} is not valid".to_string(),
                "请求参数[{0}]无效".to_string(),
            ),
            AppStatus::TaskTimeoutParamsError => DolphinErrorInfo::new(
                10002,
                "task timeout parameter is not valid".to_string(),
                "任务超时参数无效".to_string(),
            ),
            AppStatus::UserNameExist => DolphinErrorInfo::new(
                10003,
                "user name already exists".to_string(),
                "用户名已存在".to_string(),
            ),
            AppStatus::UserNameNull => DolphinErrorInfo::new(
                10004,
                "user name is null".to_string(),
                "用户名不能为空".to_string(),
            ),
            AppStatus::HdfsOperationError => DolphinErrorInfo::new(
                10006,
                "hdfs operation error".to_string(),
                "hdfs操作错误".to_string(),
            ),
            AppStatus::TaskInstanceNotFound => DolphinErrorInfo::new(
                10008,
                "task instance not found".to_string(),
                "任务实例不存在".to_string(),
            ),
            AppStatus::OsTenantCodeExist => DolphinErrorInfo::new(
                10009,
                "os tenant code {0} already exists".to_string(),
                "操作系统租户[{0}]已存在".to_string(),
            ),
            AppStatus::UserNotExist => DolphinErrorInfo::new(
                10010,
                "user {0} not exists".to_string(),
                "用户[{0}]不存在".to_string(),
            ),
            AppStatus::AlertGroupNotExist => DolphinErrorInfo::new(
                10011,
                "alarm group not found".to_string(),
                "告警组不存在".to_string(),
            ),
            AppStatus::AlertGroupExist => DolphinErrorInfo::new(
                10012,
                "alarm group already exists".to_string(),
                "告警组名称已存在".to_string(),
            ),
            AppStatus::UserNamePasswdError => DolphinErrorInfo::new(
                10013,
                "user name or password error".to_string(),
                "用户名或密码错误".to_string(),
            ),
            AppStatus::LoginSessionFailed => DolphinErrorInfo::new(
                10014,
                "create session failed!".to_string(),
                "创建session失败".to_string(),
            ),
            AppStatus::DatasourceExist => DolphinErrorInfo::new(
                10015,
                "data source name already exists".to_string(),
                "数据源名称已存在".to_string(),
            ),
            AppStatus::DatasourceConnectFailed => DolphinErrorInfo::new(
                10016,
                "data source connection failed".to_string(),
                "建立数据源连接失败".to_string(),
            ),
            AppStatus::TenantNotExist => DolphinErrorInfo::new(
                10017,
                "tenant not exists".to_string(),
                "租户不存在".to_string(),
            ),
            AppStatus::ProjectNotFound => DolphinErrorInfo::new(
                10018,
                "project {0} not found ".to_string(),
                "项目[{0}]不存在".to_string(),
            ),
            AppStatus::ProjectAlreadyExists => DolphinErrorInfo::new(
                10019,
                "project {0} already exists".to_string(),
                "项目名称[{0}]已存在".to_string(),
            ),
            AppStatus::TaskInstanceNotExists => DolphinErrorInfo::new(
                10020,
                "task instance {0} does not exist".to_string(),
                "任务实例[{0}]不存在".to_string(),
            ),
            AppStatus::TaskInstanceNotSubWorkflowInstance => DolphinErrorInfo::new(
                10021,
                "task instance {0} is not sub process instance".to_string(),
                "任务实例[{0}]不是子流程实例".to_string(),
            ),
            AppStatus::ScheduleCronNotExists => DolphinErrorInfo::new(
                10022,
                "scheduler crontab {0} does not exist".to_string(),
                "调度配置定时表达式[{0}]不存在".to_string(),
            ),
            AppStatus::ScheduleCronOnlineForbidUpdate => DolphinErrorInfo::new(
                10023,
                "online status does not allow update operations".to_string(),
                "调度配置上线状态不允许修改".to_string(),
            ),
            AppStatus::ScheduleCronCheckFailed => DolphinErrorInfo::new(
                10024,
                "scheduler crontab expression validation failure: {0}".to_string(),
                "调度配置定时表达式验证失败: {0}".to_string(),
            ),
            AppStatus::MasterNotExists => DolphinErrorInfo::new(
                10025,
                "master does not exist".to_string(),
                "无可用master节点".to_string(),
            ),
            AppStatus::ScheduleStatusUnknown => DolphinErrorInfo::new(
                10026,
                "unknown status: {0}".to_string(),
                "未知状态: {0}".to_string(),
            ),
            AppStatus::CreateAlertGroupError => DolphinErrorInfo::new(
                10027,
                "create alert group error".to_string(),
                "创建告警组错误".to_string(),
            ),
            AppStatus::QueryAllAlertgroupError => DolphinErrorInfo::new(
                10028,
                "query all alertgroup error".to_string(),
                "查询告警组错误".to_string(),
            ),
            AppStatus::ListPagingAlertGroupError => DolphinErrorInfo::new(
                10029,
                "list paging alert group error".to_string(),
                "分页查询告警组错误".to_string(),
            ),
            AppStatus::UpdateAlertGroupError => DolphinErrorInfo::new(
                10030,
                "update alert group error".to_string(),
                "更新告警组错误".to_string(),
            ),
            AppStatus::DeleteAlertGroupError => DolphinErrorInfo::new(
                10031,
                "delete alert group error".to_string(),
                "删除告警组错误".to_string(),
            ),
            AppStatus::AlertGroupGrantUserError => DolphinErrorInfo::new(
                10032,
                "alert group grant user error".to_string(),
                "告警组授权用户错误".to_string(),
            ),
            AppStatus::CreateDatasourceError => DolphinErrorInfo::new(
                10033,
                "create datasource error".to_string(),
                "创建数据源错误".to_string(),
            ),
            AppStatus::UpdateDatasourceError => DolphinErrorInfo::new(
                10034,
                "update datasource error".to_string(),
                "更新数据源错误".to_string(),
            ),
            AppStatus::QueryDatasourceError => DolphinErrorInfo::new(
                10035,
                "query datasource error".to_string(),
                "查询数据源错误".to_string(),
            ),
            AppStatus::ConnectDatasourceFailure => DolphinErrorInfo::new(
                10036,
                "connect datasource failure".to_string(),
                "建立数据源连接失败".to_string(),
            ),
            AppStatus::ConnectionTestFailure => DolphinErrorInfo::new(
                10037,
                "connection test failure".to_string(),
                "测试数据源连接失败".to_string(),
            ),
            AppStatus::DeleteDataSourceFailure => DolphinErrorInfo::new(
                10038,
                "delete data source failure".to_string(),
                "删除数据源失败".to_string(),
            ),
            AppStatus::VerifyDatasourceNameFailure => DolphinErrorInfo::new(
                10039,
                "verify datasource name failure".to_string(),
                "验证数据源名称失败".to_string(),
            ),
            AppStatus::UnauthorizedDatasource => DolphinErrorInfo::new(
                10040,
                "unauthorized datasource".to_string(),
                "未经授权的数据源".to_string(),
            ),
            AppStatus::AuthorizedDataSource => DolphinErrorInfo::new(
                10041,
                "authorized data source".to_string(),
                "授权数据源失败".to_string(),
            ),
            AppStatus::LoginSuccess =>
                DolphinErrorInfo::new(10042, "login success".to_string(), "登录成功".to_string()),
            AppStatus::UserLoginFailure => DolphinErrorInfo::new(
                10043,
                "user login failure".to_string(),
                "用户登录失败".to_string(),
            ),
            AppStatus::ListWorkersError => DolphinErrorInfo::new(
                10044,
                "list workers error".to_string(),
                "查询worker列表错误".to_string(),
            ),
            AppStatus::ListMastersError => DolphinErrorInfo::new(
                10045,
                "list masters error".to_string(),
                "查询master列表错误".to_string(),
            ),
            AppStatus::UpdateProjectError => DolphinErrorInfo::new(
                10046,
                "update project error".to_string(),
                "更新项目信息错误".to_string(),
            ),
            AppStatus::QueryProjectDetailsByCodeError => DolphinErrorInfo::new(
                10047,
                "query project details by code error".to_string(),
                "查询项目详细信息错误".to_string(),
            ),
            AppStatus::CreateProjectError => DolphinErrorInfo::new(
                10048,
                "create project error".to_string(),
                "创建项目错误".to_string(),
            ),
            AppStatus::LoginUserQueryProjectListPagingError => DolphinErrorInfo::new(
                10049,
                "login user query project list paging error".to_string(),
                "分页查询项目列表错误".to_string(),
            ),
            AppStatus::DeleteProjectError => DolphinErrorInfo::new(
                10050,
                "delete project error".to_string(),
                "删除项目错误".to_string(),
            ),
            AppStatus::QueryUnauthorizedProjectError => DolphinErrorInfo::new(
                10051,
                "query unauthorized project error".to_string(),
                "查询未授权项目错误".to_string(),
            ),
            AppStatus::QueryAuthorizedProject => DolphinErrorInfo::new(
                10052,
                "query authorized project".to_string(),
                "查询授权项目错误".to_string(),
            ),
            AppStatus::QueryQueueListError => DolphinErrorInfo::new(
                10053,
                "query queue list error".to_string(),
                "查询队列列表错误".to_string(),
            ),
            AppStatus::CreateResourceError => DolphinErrorInfo::new(
                10054,
                "create resource error".to_string(),
                "创建资源错误".to_string(),
            ),
            AppStatus::UpdateResourceError => DolphinErrorInfo::new(
                10055,
                "update resource error".to_string(),
                "更新资源错误".to_string(),
            ),
            AppStatus::QueryResourcesListError => DolphinErrorInfo::new(
                10056,
                "query resources list error".to_string(),
                "查询资源列表错误".to_string(),
            ),
            AppStatus::QueryResourcesListPaging => DolphinErrorInfo::new(
                10057,
                "query resources list paging".to_string(),
                "分页查询资源列表错误".to_string(),
            ),
            AppStatus::DeleteResourceError => DolphinErrorInfo::new(
                10058,
                "delete resource error".to_string(),
                "删除资源错误".to_string(),
            ),
            AppStatus::VerifyResourceByNameAndTypeError => DolphinErrorInfo::new(
                10059,
                "verify resource by name and type error".to_string(),
                "资源名称或类型验证错误".to_string(),
            ),
            AppStatus::ViewResourceFileOnLineError => DolphinErrorInfo::new(
                10060,
                "view resource file online error".to_string(),
                "查看资源文件错误".to_string(),
            ),
            AppStatus::CreateResourceFileOnLineError => DolphinErrorInfo::new(
                10061,
                "create resource file online error".to_string(),
                "创建资源文件错误".to_string(),
            ),
            AppStatus::ResourceFileIsEmpty => DolphinErrorInfo::new(
                10062,
                "resource file is empty".to_string(),
                "资源文件内容不能为空".to_string(),
            ),
            AppStatus::EditResourceFileOnLineError => DolphinErrorInfo::new(
                10063,
                "edit resource file online error".to_string(),
                "更新资源文件错误".to_string(),
            ),
            AppStatus::DownloadResourceFileError => DolphinErrorInfo::new(
                10064,
                "download resource file error".to_string(),
                "下载资源文件错误".to_string(),
            ),
            AppStatus::CreateUdfFunctionError => DolphinErrorInfo::new(
                10065,
                "create udf function error".to_string(),
                "创建UDF函数错误".to_string(),
            ),
            AppStatus::ViewUdfFunctionError => DolphinErrorInfo::new(
                10066,
                "view udf function error".to_string(),
                "查询UDF函数错误".to_string(),
            ),
            AppStatus::UpdateUdfFunctionError => DolphinErrorInfo::new(
                10067,
                "update udf function error".to_string(),
                "更新UDF函数错误".to_string(),
            ),
            AppStatus::QueryUdfFunctionListPagingError => DolphinErrorInfo::new(
                10068,
                "query udf function list paging error".to_string(),
                "分页查询UDF函数列表错误".to_string(),
            ),
            AppStatus::QueryDatasourceByTypeError => DolphinErrorInfo::new(
                10069,
                "query datasource by type error".to_string(),
                "查询数据源信息错误".to_string(),
            ),
            AppStatus::VerifyUdfFunctionNameError => DolphinErrorInfo::new(
                10070,
                "verify udf function name error".to_string(),
                "UDF函数名称验证错误".to_string(),
            ),
            AppStatus::DeleteUdfFunctionError => DolphinErrorInfo::new(
                10071,
                "delete udf function error".to_string(),
                "删除UDF函数错误".to_string(),
            ),
            AppStatus::AuthorizedFileResourceError => DolphinErrorInfo::new(
                10072,
                "authorized file resource error".to_string(),
                "授权资源文件错误".to_string(),
            ),
            AppStatus::AuthorizeResourceTree => DolphinErrorInfo::new(
                10073,
                "authorize resource tree display error".to_string(),
                "授权资源目录树错误".to_string(),
            ),
            AppStatus::UnauthorizedUdfFunctionError => DolphinErrorInfo::new(
                10074,
                "unauthorized udf function error".to_string(),
                "查询未授权UDF函数错误".to_string(),
            ),
            AppStatus::AuthorizedUdfFunctionError => DolphinErrorInfo::new(
                10075,
                "authorized udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            ),
            AppStatus::CreateScheduleError => DolphinErrorInfo::new(
                10076,
                "create schedule error".to_string(),
                "创建调度配置错误".to_string(),
            ),
            AppStatus::UpdateScheduleError => DolphinErrorInfo::new(
                10077,
                "update schedule error".to_string(),
                "更新调度配置错误".to_string(),
            ),
            AppStatus::PublishScheduleOnlineError => DolphinErrorInfo::new(
                10078,
                "publish schedule online error".to_string(),
                "上线调度配置错误".to_string(),
            ),
            AppStatus::OfflineScheduleError => DolphinErrorInfo::new(
                10079,
                "offline schedule error".to_string(),
                "下线调度配置错误".to_string(),
            ),
            AppStatus::QueryScheduleListPagingError => DolphinErrorInfo::new(
                10080,
                "query schedule list paging error".to_string(),
                "分页查询调度配置列表错误".to_string(),
            ),
            AppStatus::QueryScheduleListError => DolphinErrorInfo::new(
                10081,
                "query schedule list error".to_string(),
                "查询调度配置列表错误".to_string(),
            ),
            AppStatus::QueryTaskListPagingError => DolphinErrorInfo::new(
                10082,
                "query task list paging error".to_string(),
                "分页查询任务列表错误".to_string(),
            ),
            AppStatus::QueryTaskRecordListPagingError => DolphinErrorInfo::new(
                10083,
                "query task record list paging error".to_string(),
                "分页查询任务记录错误".to_string(),
            ),
            AppStatus::CreateTenantError => DolphinErrorInfo::new(
                10084,
                "create tenant error".to_string(),
                "创建租户错误".to_string(),
            ),
            AppStatus::QueryTenantListPagingError => DolphinErrorInfo::new(
                10085,
                "query tenant list paging error".to_string(),
                "分页查询租户列表错误".to_string(),
            ),
            AppStatus::QueryTenantListError => DolphinErrorInfo::new(
                10086,
                "query tenant list error".to_string(),
                "查询租户列表错误".to_string(),
            ),
            AppStatus::UpdateTenantError => DolphinErrorInfo::new(
                10087,
                "update tenant error".to_string(),
                "更新租户错误".to_string(),
            ),
            AppStatus::DeleteTenantByIdError => DolphinErrorInfo::new(
                10088,
                "delete tenant by id error".to_string(),
                "删除租户错误".to_string(),
            ),
            AppStatus::VerifyOsTenantCodeError => DolphinErrorInfo::new(
                10089,
                "verify os tenant code error".to_string(),
                "操作系统租户验证错误".to_string(),
            ),
            AppStatus::CreateUserError => DolphinErrorInfo::new(
                10090,
                "create user error".to_string(),
                "创建用户错误".to_string(),
            ),
            AppStatus::QueryUserListPagingError => DolphinErrorInfo::new(
                10091,
                "query user list paging error".to_string(),
                "分页查询用户列表错误".to_string(),
            ),
            AppStatus::UpdateUserError => DolphinErrorInfo::new(
                10092,
                "update user error".to_string(),
                "更新用户错误".to_string(),
            ),
            AppStatus::DeleteUserByIdError => DolphinErrorInfo::new(
                10093,
                "delete user by id error".to_string(),
                "删除用户错误".to_string(),
            ),
            AppStatus::GrantProjectError => DolphinErrorInfo::new(
                10094,
                "grant project error".to_string(),
                "授权项目错误".to_string(),
            ),
            AppStatus::GrantResourceError => DolphinErrorInfo::new(
                10095,
                "grant resource error".to_string(),
                "授权资源错误".to_string(),
            ),
            AppStatus::GrantUdfFunctionError => DolphinErrorInfo::new(
                10096,
                "grant udf function error".to_string(),
                "授权UDF函数错误".to_string(),
            ),
            AppStatus::GrantDatasourceError => DolphinErrorInfo::new(
                10097,
                "grant datasource error".to_string(),
                "授权数据源错误".to_string(),
            ),
            AppStatus::GetUserInfoError => DolphinErrorInfo::new(
                10098,
                "get user info error".to_string(),
                "获取用户信息错误".to_string(),
            ),
            AppStatus::UserListError => DolphinErrorInfo::new(
                10099,
                "user list error".to_string(),
                "查询用户列表错误".to_string(),
            ),
            AppStatus::VerifyUsernameError => DolphinErrorInfo::new(
                10100,
                "verify username error".to_string(),
                "用户名验证错误".to_string(),
            ),
            AppStatus::UnauthorizedUserError => DolphinErrorInfo::new(
                10101,
                "unauthorized user error".to_string(),
                "查询未授权用户错误".to_string(),
            ),
            AppStatus::AuthorizedUserError => DolphinErrorInfo::new(
                10102,
                "authorized user error".to_string(),
                "查询授权用户错误".to_string(),
            ),
            AppStatus::QueryTaskInstanceLogError => DolphinErrorInfo::new(
                10103,
                "view task instance log error".to_string(),
                "查询任务实例日志错误".to_string(),
            ),
            AppStatus::DownloadTaskInstanceLogFileError => DolphinErrorInfo::new(
                10104,
                "download task instance log file error".to_string(),
                "下载任务日志文件错误".to_string(),
            ),
            AppStatus::CreateProcessDefinitionError => DolphinErrorInfo::new(
                10105,
                "create process definition error".to_string(),
                "创建工作流错误".to_string(),
            ),
            AppStatus::VerifyProcessDefinitionNameUniqueError => DolphinErrorInfo::new(
                10106,
                "verify process definition name unique error".to_string(),
                "工作流定义名称验证错误".to_string(),
            ),
            AppStatus::UpdateProcessDefinitionError => DolphinErrorInfo::new(
                10107,
                "update process definition error".to_string(),
                "更新工作流定义错误".to_string(),
            ),
            AppStatus::ReleaseProcessDefinitionError => DolphinErrorInfo::new(
                10108,
                "release process definition error".to_string(),
                "上线工作流错误".to_string(),
            ),
            AppStatus::QueryDetailOfProcessDefinitionError => DolphinErrorInfo::new(
                10109,
                "query detail of process definition error".to_string(),
                "查询工作流详细信息错误".to_string(),
            ),
            AppStatus::QueryProcessDefinitionList => DolphinErrorInfo::new(
                10110,
                "query process definition list".to_string(),
                "查询工作流列表错误".to_string(),
            ),
            AppStatus::EncapsulationTreeviewStructureError => DolphinErrorInfo::new(
                10111,
                "encapsulation treeview structure error".to_string(),
                "查询工作流树形图数据错误".to_string(),
            ),
            AppStatus::GetTasksListByProcessDefinitionIdError => DolphinErrorInfo::new(
                10112,
                "get tasks list by process definition id error".to_string(),
                "查询工作流定义节点信息错误".to_string(),
            ),
            AppStatus::QueryProcessInstanceListPagingError => DolphinErrorInfo::new(
                10113,
                "query process instance list paging error".to_string(),
                "分页查询工作流实例列表错误".to_string(),
            ),
            AppStatus::QueryTaskListByProcessInstanceIdError => DolphinErrorInfo::new(
                10114,
                "query task list by process instance id error".to_string(),
                "查询任务实例列表错误".to_string(),
            ),
            AppStatus::UpdateProcessInstanceError => DolphinErrorInfo::new(
                10115,
                "update process instance error".to_string(),
                "更新工作流实例错误".to_string(),
            ),
            AppStatus::QueryProcessInstanceByIdError => DolphinErrorInfo::new(
                10116,
                "query process instance by id error".to_string(),
                "查询工作流实例错误".to_string(),
            ),
            AppStatus::DeleteProcessInstanceByIdError => DolphinErrorInfo::new(
                10117,
                "delete process instance by id error".to_string(),
                "删除工作流实例错误".to_string(),
            ),
            AppStatus::QuerySubProcessInstanceDetailInfoByTaskIdError => DolphinErrorInfo::new(
                10118,
                "query sub process instance detail info by task id error".to_string(),
                "查询子流程任务实例错误".to_string(),
            ),
            AppStatus::QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError =>
                DolphinErrorInfo::new(
                    10119,
                    "query parent process instance detail info by sub process instance id error"
                        .to_string(),
                    "查询子流程该工作流实例错误".to_string(),
                ),
            AppStatus::QueryProcessInstanceAllVariablesError => DolphinErrorInfo::new(
                10120,
                "query process instance all variables error".to_string(),
                "查询工作流自定义变量信息错误".to_string(),
            ),
            AppStatus::EncapsulationProcessInstanceGanttStructureError => DolphinErrorInfo::new(
                10121,
                "encapsulation process instance gantt structure error".to_string(),
                "查询工作流实例甘特图数据错误".to_string(),
            ),
            AppStatus::QueryProcessDefinitionListPagingError => DolphinErrorInfo::new(
                10122,
                "query process definition list paging error".to_string(),
                "分页查询工作流定义列表错误".to_string(),
            ),
            AppStatus::SignOutError =>
                DolphinErrorInfo::new(10123, "sign out error".to_string(), "退出错误".to_string()),
            AppStatus::OsTenantCodeHasAlreadyExists => DolphinErrorInfo::new(
                10124,
                "os tenant code has already exists".to_string(),
                "操作系统租户已存在".to_string(),
            ),
            AppStatus::IpIsEmpty => DolphinErrorInfo::new(
                10125,
                "ip is empty".to_string(),
                "IP地址不能为空".to_string(),
            ),
            AppStatus::ScheduleCronReleaseNeedNotChange => DolphinErrorInfo::new(
                10126,
                "schedule release is already {0}".to_string(),
                "调度配置上线错误[{0}]".to_string(),
            ),
            AppStatus::CreateQueueError => DolphinErrorInfo::new(
                10127,
                "create queue error".to_string(),
                "创建队列错误".to_string(),
            ),
            AppStatus::QueueNotExist => DolphinErrorInfo::new(
                10128,
                "queue {0} not exists".to_string(),
                "队列ID[{0}]不存在".to_string(),
            ),
            AppStatus::QueueValueExist => DolphinErrorInfo::new(
                10129,
                "queue value {0} already exists".to_string(),
                "队列值[{0}]已存在".to_string(),
            ),
            AppStatus::QueueNameExist => DolphinErrorInfo::new(
                10130,
                "queue name {0} already exists".to_string(),
                "队列名称[{0}]已存在".to_string(),
            ),
            AppStatus::UpdateQueueError => DolphinErrorInfo::new(
                10131,
                "update queue error".to_string(),
                "更新队列信息错误".to_string(),
            ),
            AppStatus::NeedNotUpdateQueue => DolphinErrorInfo::new(
                10132,
                "need not update queue".to_string(),
                "无需更新队列信息".to_string(),
            ),
            AppStatus::VerifyQueueError => DolphinErrorInfo::new(
                10133,
                "verify queue error".to_string(),
                "验证队列信息错误".to_string(),
            ),
            AppStatus::NameNull => DolphinErrorInfo::new(
                10134,
                "name must be not null".to_string(),
                "名称不能为空".to_string(),
            ),
            AppStatus::NameExist => DolphinErrorInfo::new(
                10135,
                "name {0} already exists".to_string(),
                "名称[{0}]已存在".to_string(),
            ),
            AppStatus::SaveError =>
                DolphinErrorInfo::new(10136, "save error".to_string(), "保存错误".to_string()),
            AppStatus::DeleteProjectErrorDefinesNotNull => DolphinErrorInfo::new(
                10137,
                "please delete the process definitions in project first!".to_string(),
                "请先删除全部工作流定义".to_string(),
            ),
            AppStatus::BatchDeleteProcessInstanceByIdsError => DolphinErrorInfo::new(
                10117,
                "batch delete process instance by ids {0} error".to_string(),
                "批量删除工作流实例错误: {0}".to_string(),
            ),
            AppStatus::PreviewScheduleError => DolphinErrorInfo::new(
                10139,
                "preview schedule error".to_string(),
                "预览调度配置错误".to_string(),
            ),
            AppStatus::ParseToCronExpressionError => DolphinErrorInfo::new(
                10140,
                "parse cron to cron expression error".to_string(),
                "解析调度表达式错误".to_string(),
            ),
            AppStatus::ScheduleStartTimeEndTimeSame => DolphinErrorInfo::new(
                10141,
                "The start time must not be the same as the end".to_string(),
                "开始时间不能和结束时间一样".to_string(),
            ),
            AppStatus::DeleteTenantByIdFail => DolphinErrorInfo::new(
                10142,
                "delete tenant by id fail:for there are {0} process instances in executing using \
                 it"
                .to_string(),
                "删除租户失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ),
            AppStatus::DeleteTenantByIdFailDefines => DolphinErrorInfo::new(
                10143,
                "delete tenant by id fail:for there are {0} process definitions using it"
                    .to_string(),
                "删除租户失败，有[{0}]个工作流定义正在使用".to_string(),
            ),
            AppStatus::DeleteTenantByIdFailUsers => DolphinErrorInfo::new(
                10144,
                "delete tenant by id fail: for there are {0} users using it".to_string(),
                "删除租户失败，有[{0}]个用户正在使用".to_string(),
            ),
            AppStatus::DeleteWorkerGroupByIdFail => DolphinErrorInfo::new(
                10145,
                "delete worker group by id failfor there are {0} process instances in executing \
                 using it"
                    .to_string(),
                "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ),
            AppStatus::QueryWorkerGroupFail => DolphinErrorInfo::new(
                10146,
                "query worker group fail ".to_string(),
                "查询worker分组失败".to_string(),
            ),
            AppStatus::DeleteWorkerGroupFail => DolphinErrorInfo::new(
                10147,
                "delete worker group fail ".to_string(),
                "删除worker分组失败".to_string(),
            ),
            AppStatus::UserDisabled => DolphinErrorInfo::new(
                10148,
                "The current user is disabled".to_string(),
                "当前用户已停用".to_string(),
            ),
            AppStatus::CopyProcessDefinitionError => DolphinErrorInfo::new(
                10149,
                "copy process definition from {0} to {1} error : {2}".to_string(),
                "从{0}复制工作流到{1}错误 : {2}".to_string(),
            ),
            AppStatus::MoveProcessDefinitionError => DolphinErrorInfo::new(
                10150,
                "move process definition from {0} to {1} error : {2}".to_string(),
                "从{0}移动工作流到{1}错误 : {2}".to_string(),
            ),
            AppStatus::SwitchProcessDefinitionVersionError => DolphinErrorInfo::new(
                10151,
                "Switch process definition version error".to_string(),
                "切换工作流版本出错".to_string(),
            ),
            AppStatus::SwitchProcessDefinitionVersionNotExistProcessDefinitionError =>
                DolphinErrorInfo::new(
                    10152,
                    "Switch process definition version error: not exists process definition: \
                     [process definition id {0}]"
                        .to_string(),
                    "切换工作流版本出错：工作流不存在，[工作流id {0}]".to_string(),
                ),
            AppStatus::SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError =>
                DolphinErrorInfo::new(
                    10153,
                    "Switch process defi:nition version error: not exists process definition \
                     version: [process definition id {0}] [version number {1}]"
                        .to_string(),
                    "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"
                        .to_string(),
                ),
            AppStatus::QueryProcessDefinitionVersionsError => DolphinErrorInfo::new(
                10154,
                "query process definition versions error".to_string(),
                "查询工作流历史版本信息出错".to_string(),
            ),
            AppStatus::DeleteProcessDefinitionVersionError => DolphinErrorInfo::new(
                10156,
                "delete process definition version error".to_string(),
                "删除工作流历史版本出错".to_string(),
            ),
            AppStatus::QueryUserCreatedProjectError => DolphinErrorInfo::new(
                10157,
                "query user created project error error".to_string(),
                "查询用户创建的项目错误".to_string(),
            ),
            AppStatus::ProcessDefinitionCodesIsEmpty => DolphinErrorInfo::new(
                10158,
                "process definition codes is empty".to_string(),
                "工作流CODES不能为空".to_string(),
            ),
            AppStatus::BatchCopyProcessDefinitionError => DolphinErrorInfo::new(
                10159,
                "batch copy process definition error".to_string(),
                "复制工作流错误".to_string(),
            ),
            AppStatus::BatchMoveProcessDefinitionError => DolphinErrorInfo::new(
                10160,
                "batch move process definition error".to_string(),
                "移动工作流错误".to_string(),
            ),
            AppStatus::QueryWorkflowLineageError => DolphinErrorInfo::new(
                10161,
                "query workflow lineage error".to_string(),
                "查询血缘失败".to_string(),
            ),
            AppStatus::QueryAuthorizedAndUserCreatedProjectError => DolphinErrorInfo::new(
                10162,
                "query authorized and user created project error error".to_string(),
                "查询授权的和用户创建的项目错误".to_string(),
            ),
            AppStatus::DeleteProcessDefinitionByCodeFail => DolphinErrorInfo::new(
                10163,
                "delete process definition by code fail.to_string(), for there are {0} process \
                 instances in executing using it"
                    .to_string(),
                "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用".to_string(),
            ),
            AppStatus::CheckOsTenantCodeError => DolphinErrorInfo::new(
                10164,
                "Tenant code invalid.to_string(), should follow linux's users naming conventions"
                    .to_string(),
                "非法的租户名，需要遵守 Linux 用户命名规范".to_string(),
            ),
            AppStatus::ForceTaskSuccessError => DolphinErrorInfo::new(
                10165,
                "force task success error".to_string(),
                "强制成功任务实例错误".to_string(),
            ),
            AppStatus::TaskInstanceStateOperationError => DolphinErrorInfo::new(
                10166,
                "the status of task instance {0} is {1}.to_string(),Cannot perform force success \
                 operation"
                    .to_string(),
                "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作".to_string(),
            ),
            AppStatus::DatasourceTypeNotExist => DolphinErrorInfo::new(
                10167,
                "data source type not exist".to_string(),
                "数据源类型不存在".to_string(),
            ),
            AppStatus::ProcessDefinitionNameExist => DolphinErrorInfo::new(
                10168,
                "process definition name {0} already exists".to_string(),
                "工作流定义名称[{0}]已存在".to_string(),
            ),
            AppStatus::DatasourceDbTypeIllegal => DolphinErrorInfo::new(
                10169,
                "datasource type illegal".to_string(),
                "数据源类型参数不合法".to_string(),
            ),
            AppStatus::DatasourcePortIllegal => DolphinErrorInfo::new(
                10170,
                "datasource port illegal".to_string(),
                "数据源端口参数不合法".to_string(),
            ),
            AppStatus::DatasourceOtherParamsIllegal => DolphinErrorInfo::new(
                10171,
                "datasource other params illegal".to_string(),
                "数据源其他参数不合法".to_string(),
            ),
            AppStatus::DatasourceNameIllegal => DolphinErrorInfo::new(
                10172,
                "datasource name illegal".to_string(),
                "数据源名称不合法".to_string(),
            ),
            AppStatus::DatasourceHostIllegal => DolphinErrorInfo::new(
                10173,
                "datasource host illegal".to_string(),
                "数据源HOST不合法".to_string(),
            ),
            AppStatus::DeleteWorkerGroupNotExist => DolphinErrorInfo::new(
                10174,
                "delete worker group not exist ".to_string(),
                "删除worker分组不存在".to_string(),
            ),
            AppStatus::CreateWorkerGroupForbiddenInDocker => DolphinErrorInfo::new(
                10175,
                "create worker group forbidden in docker ".to_string(),
                "创建worker分组在docker中禁止".to_string(),
            ),
            AppStatus::DeleteWorkerGroupForbiddenInDocker => DolphinErrorInfo::new(
                10176,
                "delete worker group forbidden in docker ".to_string(),
                "删除worker分组在docker中禁止".to_string(),
            ),
            AppStatus::WorkerAddressInvalid => DolphinErrorInfo::new(
                10177,
                "worker address {0} invalid".to_string(),
                "worker地址[{0}]无效".to_string(),
            ),
            AppStatus::QueryWorkerAddressListFail => DolphinErrorInfo::new(
                10178,
                "query worker address list fail ".to_string(),
                "查询worker地址列表失败".to_string(),
            ),
            AppStatus::TransformProjectOwnership => DolphinErrorInfo::new(
                10179,
                "Please transform project ownership [{0}]".to_string(),
                "请先转移项目所有权[{0}]".to_string(),
            ),
            AppStatus::QueryAlertGroupError => DolphinErrorInfo::new(
                10180,
                "query alert group error".to_string(),
                "查询告警组错误".to_string(),
            ),
            AppStatus::CurrentLoginUserTenantNotExist => DolphinErrorInfo::new(
                10181,
                "the tenant of the currently login user is not specified".to_string(),
                "未指定当前登录用户的租户".to_string(),
            ),
            AppStatus::RevokeProjectError => DolphinErrorInfo::new(
                10182,
                "revoke project error".to_string(),
                "撤销项目授权错误".to_string(),
            ),
            AppStatus::QueryAuthorizedUser => DolphinErrorInfo::new(
                10183,
                "query authorized user error".to_string(),
                "查询拥有项目权限的用户错误".to_string(),
            ),
            AppStatus::ProjectNotExist => DolphinErrorInfo::new(
                10190,
                "This project was not found. Please refresh page.".to_string(),
                "该项目不存在".to_string(),
            ),

            AppStatus::TaskInstanceHostIsNull => DolphinErrorInfo::new(
                10191,
                "task instance host is null ".to_string(),
                "任务实例host为空".to_string(),
            ),
            AppStatus::QueryExecutingWorkflowError => DolphinErrorInfo::new(
                10192,
                "query executing workflow error".to_string(),
                "查询运行的工作流实例错误".to_string(),
            ),
            AppStatus::UdfFunctionNotExist => DolphinErrorInfo::new(
                20001,
                "UDF function not found".to_string(),
                "UDF函数不存在".to_string(),
            ),
            AppStatus::UdfFunctionExists => DolphinErrorInfo::new(
                20002,
                "UDF function already exists".to_string(),
                "UDF函数已存在".to_string(),
            ),
            AppStatus::ResourceNotExist => DolphinErrorInfo::new(
                20004,
                "resource not exist".to_string(),
                "资源不存在".to_string(),
            ),
            AppStatus::ResourceExist => DolphinErrorInfo::new(
                20005,
                "resource already exists".to_string(),
                "资源已存在".to_string(),
            ),
            AppStatus::ResourceSuffixNotSupportView => DolphinErrorInfo::new(
                20006,
                "resource suffix do not support online viewing".to_string(),
                "资源文件后缀不支持查看".to_string(),
            ),
            AppStatus::ResourceSizeExceedLimit => DolphinErrorInfo::new(
                20007,
                "upload resource file size exceeds limit".to_string(),
                "上传资源文件大小超过限制".to_string(),
            ),
            AppStatus::ResourceSuffixForbidChange => DolphinErrorInfo::new(
                20008,
                "resource suffix not allowed to be modified".to_string(),
                "资源文件后缀不支持修改".to_string(),
            ),
            AppStatus::UdfResourceSuffixNotJar => DolphinErrorInfo::new(
                20009,
                "UDF resource suffix name must be jar".to_string(),
                "UDF资源文件后缀名只支持[jar]".to_string(),
            ),
            AppStatus::HdfsCopyFail => DolphinErrorInfo::new(
                20010,
                "hdfs copy {0} -> {1} fail".to_string(),
                "hdfs复制失败：[{0}] -> [{1}]".to_string(),
            ),
            AppStatus::ResourceFileExist => DolphinErrorInfo::new(
                20011,
                "resource file {0} already exists !".to_string(),
                "资源文件[{0}]已存在".to_string(),
            ),
            AppStatus::ResourceFileNotExist => DolphinErrorInfo::new(
                20012,
                "resource file {0} not exists !".to_string(),
                "资源文件[{0}]不存在".to_string(),
            ),
            AppStatus::UdfResourceIsBound => DolphinErrorInfo::new(
                20013,
                "udf resource file is bound by UDF functions:{0}".to_string(),
                "udf函数绑定了资源文件[{0}]".to_string(),
            ),
            AppStatus::ResourceIsUsed => DolphinErrorInfo::new(
                20014,
                "resource file is used by process definition".to_string(),
                "资源文件被上线的流程定义使用了".to_string(),
            ),
            AppStatus::ParentResourceNotExist => DolphinErrorInfo::new(
                20015,
                "parent resource not exist".to_string(),
                "父资源文件不存在".to_string(),
            ),
            AppStatus::ResourceNotExistOrNoPermission => DolphinErrorInfo::new(
                20016,
                "resource not exist or no permission:please view the task node and remove error \
                 resource"
                    .to_string(),
                "请检查任务节点并移除无权限或者已删除的资源".to_string(),
            ),
            AppStatus::ResourceIsAuthorized => DolphinErrorInfo::new(
                20017,
                "resource is authorized to user {0}:suffix not allowed to be modified".to_string(),
                "资源文件已授权其他用户[{0}]".to_string(),
            ),
            AppStatus::UserNoOperationPerm => DolphinErrorInfo::new(
                30001,
                "user has no operation privilege".to_string(),
                "当前用户没有操作权限".to_string(),
            ),
            AppStatus::UserNoOperationProjectPerm => DolphinErrorInfo::new(
                30002,
                "user {0} is not has project {1} permission".to_string(),
                "当前用户[{0}]没有[{1}]项目的操作权限".to_string(),
            ),
            AppStatus::ProcessInstanceNotExist => DolphinErrorInfo::new(
                50001,
                "process instance {0} does not exist".to_string(),
                "工作流实例[{0}]不存在".to_string(),
            ),
            AppStatus::ProcessInstanceExist => DolphinErrorInfo::new(
                50002,
                "process instance {0} already exists".to_string(),
                "工作流实例[{0}]已存在".to_string(),
            ),
            AppStatus::ProcessDefineNotExist => DolphinErrorInfo::new(
                50003,
                "process definition {0} does not exist".to_string(),
                "工作流定义[{0}]不存在".to_string(),
            ),
            AppStatus::ProcessDefineNotRelease => DolphinErrorInfo::new(
                50004,
                "process definition {0} process version {1} not online".to_string(),
                "工作流定义[{0}] 工作流版本[{1}]不是上线状态".to_string(),
            ),
            AppStatus::SubProcessDefineNotRelease => DolphinErrorInfo::new(
                50004,
                "exist sub process definition not online".to_string(),
                "存在子工作流定义不是上线状态".to_string(),
            ),
            AppStatus::ProcessInstanceAlreadyChanged => DolphinErrorInfo::new(
                50005,
                "the status of process instance {0} is already {1}".to_string(),
                "工作流实例[{0}]的状态已经是[{1}]".to_string(),
            ),
            AppStatus::ProcessInstanceStateOperationError => DolphinErrorInfo::new(
                50006,
                "the status of process instance {0} is {1}.to_string(),Cannot perform the \
                 operation"
                    .to_string(),
                "工作流实例[{0}]的状态是[{1}]，无法执行该操作".to_string(),
            ),
            AppStatus::SubProcessInstanceNotExist => DolphinErrorInfo::new(
                50007,
                "the task belong to process instance does not exist".to_string(),
                "子工作流实例不存在".to_string(),
            ),
            AppStatus::ProcessDefineNotAllowedEdit => DolphinErrorInfo::new(
                50008,
                "process definition {0} does not allow edit".to_string(),
                "工作流定义[{0}]不允许修改".to_string(),
            ),
            AppStatus::ProcessInstanceExecutingCommand => DolphinErrorInfo::new(
                50009,
                "process instance {0} is executing command".to_string(),
                "工作流实例[{0}]正在执行命令".to_string(),
            ),
            AppStatus::ProcessInstanceNotSubProcessInstance => DolphinErrorInfo::new(
                50010,
                "process instance {0} is not sub process instance".to_string(),
                "工作流实例[{0}]不是子工作流实例".to_string(),
            ),
            AppStatus::TaskInstanceStateCountError => DolphinErrorInfo::new(
                50011,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            ),
            AppStatus::CountProcessInstanceStateError => DolphinErrorInfo::new(
                50012,
                "count process instance state error".to_string(),
                "查询各状态流程实例数错误".to_string(),
            ),
            AppStatus::CountProcessDefinitionUserError => DolphinErrorInfo::new(
                50013,
                "count process definition user error".to_string(),
                "查询各用户流程定义数错误".to_string(),
            ),
            AppStatus::StartProcessInstanceError => DolphinErrorInfo::new(
                50014,
                "start process instance error".to_string(),
                "运行工作流实例错误".to_string(),
            ),
            AppStatus::BatchStartProcessInstanceError => DolphinErrorInfo::new(
                50014,
                "batch start process instance error: {0}".to_string(),
                "批量运行工作流实例错误: {0}".to_string(),
            ),
            AppStatus::ProcessInstanceError => DolphinErrorInfo::new(
                50014,
                "process instance delete error: {0}".to_string(),
                "工作流实例删除[{0}]错误".to_string(),
            ),
            AppStatus::ExecuteProcessInstanceError => DolphinErrorInfo::new(
                50015,
                "execute process instance error".to_string(),
                "操作工作流实例错误".to_string(),
            ),
            AppStatus::CheckProcessDefinitionError => DolphinErrorInfo::new(
                50016,
                "check process definition error".to_string(),
                "工作流定义错误".to_string(),
            ),
            AppStatus::QueryRecipientsAndCopyersByProcessDefinitionError => DolphinErrorInfo::new(
                50017,
                "query recipients and copyers by process definition error".to_string(),
                "查询收件人和抄送人错误".to_string(),
            ),
            AppStatus::DataIsNotValid => DolphinErrorInfo::new(
                50017,
                "data {0} not valid".to_string(),
                "数据[{0}]无效".to_string(),
            ),
            AppStatus::DataIsNull => DolphinErrorInfo::new(
                50018,
                "data {0} is null".to_string(),
                "数据[{0}]不能为空".to_string(),
            ),
            AppStatus::ProcessNodeHasCycle => DolphinErrorInfo::new(
                50019,
                "process node has cycle".to_string(),
                "流程节点间存在循环依赖".to_string(),
            ),
            AppStatus::ProcessNodeSParameterInvalid => DolphinErrorInfo::new(
                50020,
                "process node {0} parameter invalid".to_string(),
                "流程节点[{0}]参数无效".to_string(),
            ),
            AppStatus::ProcessDefineStateOnline => DolphinErrorInfo::new(
                50021,
                "process definition [{0}] is already online".to_string(),
                "工作流定义[{0}]已上线".to_string(),
            ),
            AppStatus::DeleteProcessDefineByCodeError => DolphinErrorInfo::new(
                50022,
                "delete process definition by code error".to_string(),
                "删除工作流定义错误".to_string(),
            ),
            AppStatus::ScheduleCronStateOnline => DolphinErrorInfo::new(
                50023,
                "the status of schedule {0} is already online".to_string(),
                "调度配置[{0}]已上线".to_string(),
            ),
            AppStatus::DeleteScheduleCronByIdError => DolphinErrorInfo::new(
                50024,
                "delete schedule by id error".to_string(),
                "删除调度配置错误".to_string(),
            ),
            AppStatus::BatchDeleteProcessDefineError => DolphinErrorInfo::new(
                50025,
                "batch delete process definition error".to_string(),
                "批量删除工作流定义错误".to_string(),
            ),
            AppStatus::BatchDeleteProcessDefineByCodesError => DolphinErrorInfo::new(
                50026,
                "batch delete process definition by codes {0} error".to_string(),
                "批量删除工作流定义[{0}]错误".to_string(),
            ),
            AppStatus::DeleteProcessDefineByCodesError => DolphinErrorInfo::new(
                50026,
                "delete process definition by codes {0} error".to_string(),
                "删除工作流定义[{0}]错误".to_string(),
            ),
            AppStatus::TenantNotSuitable => DolphinErrorInfo::new(
                50027,
                "there is not any tenant suitable: please choose a tenant available.".to_string(),
                "没有合适的租户，请选择可用的租户".to_string(),
            ),
            AppStatus::ExportProcessDefineByIdError => DolphinErrorInfo::new(
                50028,
                "export process definition by id error".to_string(),
                "导出工作流定义错误".to_string(),
            ),
            AppStatus::BatchExportProcessDefineByIdsError => DolphinErrorInfo::new(
                50028,
                "batch export process definition by ids error".to_string(),
                "批量导出工作流定义错误".to_string(),
            ),
            AppStatus::ImportProcessDefineError => DolphinErrorInfo::new(
                50029,
                "import process definition error".to_string(),
                "导入工作流定义错误".to_string(),
            ),
            AppStatus::TaskDefineNotExist => DolphinErrorInfo::new(
                50030,
                "task definition [{0}] does not exist".to_string(),
                "任务定义[{0}]不存在".to_string(),
            ),
            AppStatus::CreateProcessTaskRelationError => DolphinErrorInfo::new(
                50032,
                "create process task relation error".to_string(),
                "创建工作流任务关系错误".to_string(),
            ),
            AppStatus::ProcessTaskRelationNotExist => DolphinErrorInfo::new(
                50033,
                "process task relation [{0}] does not exist".to_string(),
                "工作流任务关系[{0}]不存在".to_string(),
            ),
            AppStatus::ProcessTaskRelationExist => DolphinErrorInfo::new(
                50034,
                "process task relation is already exist  processCode:[{0}]".to_string(),
                "工作流任务关系已存在".to_string(),
            ),
            AppStatus::ProcessDagIsEmpty => DolphinErrorInfo::new(
                50035,
                "process dag is empty".to_string(),
                "工作流dag是空".to_string(),
            ),
            AppStatus::CheckProcessTaskRelationError => DolphinErrorInfo::new(
                50036,
                "check process task relation error".to_string(),
                "工作流任务关系参数错误".to_string(),
            ),
            AppStatus::CreateTaskDefinitionError => DolphinErrorInfo::new(
                50037,
                "create task definition error".to_string(),
                "创建任务错误".to_string(),
            ),
            AppStatus::UpdateTaskDefinitionError => DolphinErrorInfo::new(
                50038,
                "update task definition error".to_string(),
                "更新任务定义错误".to_string(),
            ),
            AppStatus::QueryTaskDefinitionVersionsError => DolphinErrorInfo::new(
                50039,
                "query task definition versions error".to_string(),
                "查询任务历史版本信息出错".to_string(),
            ),
            AppStatus::SwitchTaskDefinitionVersionError => DolphinErrorInfo::new(
                50040,
                "Switch task definition version error".to_string(),
                "切换任务版本出错".to_string(),
            ),
            AppStatus::DeleteTaskDefinitionVersionError => DolphinErrorInfo::new(
                50041,
                "delete task definition version error".to_string(),
                "删除任务历史版本出错".to_string(),
            ),
            AppStatus::DeleteTaskDefineByCodeError => DolphinErrorInfo::new(
                50042,
                "delete task definition by code error".to_string(),
                "删除任务定义错误".to_string(),
            ),
            AppStatus::QueryDetailOfTaskDefinitionError => DolphinErrorInfo::new(
                50043,
                "query detail of task definition error".to_string(),
                "查询任务详细信息错误".to_string(),
            ),
            AppStatus::QueryTaskDefinitionListPagingError => DolphinErrorInfo::new(
                50044,
                "query task definition list paging error".to_string(),
                "分页查询任务定义列表错误".to_string(),
            ),
            AppStatus::TaskDefinitionNameExisted => DolphinErrorInfo::new(
                50045,
                "task definition name [{0}] already exists".to_string(),
                "任务定义名称[{0}]已经存在".to_string(),
            ),
            AppStatus::ReleaseTaskDefinitionError => DolphinErrorInfo::new(
                50046,
                "release task definition error".to_string(),
                "上线任务错误".to_string(),
            ),
            AppStatus::MoveProcessTaskRelationError => DolphinErrorInfo::new(
                50047,
                "move process task relation error".to_string(),
                "移动任务到其他工作流错误".to_string(),
            ),
            AppStatus::DeleteTaskProcessRelationError => DolphinErrorInfo::new(
                50048,
                "delete process task relation error".to_string(),
                "删除工作流任务关系错误".to_string(),
            ),
            AppStatus::QueryTaskProcessRelationError => DolphinErrorInfo::new(
                50049,
                "query process task relation error".to_string(),
                "查询工作流任务关系错误".to_string(),
            ),
            AppStatus::TaskDefineStateOnline => DolphinErrorInfo::new(
                50050,
                "task definition [{0}] is already online".to_string(),
                "任务定义[{0}]已上线".to_string(),
            ),
            AppStatus::TaskHasDownstream => DolphinErrorInfo::new(
                50051,
                "Task exists downstream [{0}] dependence".to_string(),
                "任务存在下游[{0}]依赖".to_string(),
            ),
            AppStatus::TaskHasUpstream => DolphinErrorInfo::new(
                50052,
                "Task [{0}] exists upstream dependence".to_string(),
                "任务[{0}]存在上游依赖".to_string(),
            ),
            AppStatus::MainTableUsingVersion => DolphinErrorInfo::new(
                50053,
                "the version that the master table is using".to_string(),
                "主表正在使用该版本".to_string(),
            ),
            AppStatus::ProjectProcessNotMatch => DolphinErrorInfo::new(
                50054,
                "the project and the process is not match".to_string(),
                "项目和工作流不匹配".to_string(),
            ),
            AppStatus::DeleteEdgeError => DolphinErrorInfo::new(
                50055,
                "delete edge error".to_string(),
                "删除工作流任务连接线错误".to_string(),
            ),
            AppStatus::NotSupportUpdateTaskDefinition => DolphinErrorInfo::new(
                50056,
                "task state does not support modification".to_string(),
                "当前任务不支持修改".to_string(),
            ),
            AppStatus::NotSupportCopyTaskType => DolphinErrorInfo::new(
                50057,
                "task type [{0}] does not support copy".to_string(),
                "不支持复制的任务类型[{0}]".to_string(),
            ),
            AppStatus::HdfsNotStartup => DolphinErrorInfo::new(
                60001,
                "hdfs not startup".to_string(),
                "hdfs未启用".to_string(),
            ),
            AppStatus::StorageNotStartup => DolphinErrorInfo::new(
                60002,
                "storage not startup".to_string(),
                "存储未启用".to_string(),
            ),
            AppStatus::S3CannotRename => DolphinErrorInfo::new(
                60003,
                "directory cannot be renamed".to_string(),
                "S3无法重命名文件夹".to_string(),
            ),
            AppStatus::QueryDatabaseStateError => DolphinErrorInfo::new(
                70001,
                "query database state error".to_string(),
                "查询数据库状态错误".to_string(),
            ),
            AppStatus::CreateAccessTokenError => DolphinErrorInfo::new(
                70010,
                "create access token error".to_string(),
                "创建访问token错误".to_string(),
            ),
            AppStatus::GenerateTokenError => DolphinErrorInfo::new(
                70011,
                "generate token error".to_string(),
                "生成token错误".to_string(),
            ),
            AppStatus::QueryAccesstokenListPagingError => DolphinErrorInfo::new(
                70012,
                "query access token list paging error".to_string(),
                "分页查询访问token列表错误".to_string(),
            ),
            AppStatus::UpdateAccessTokenError => DolphinErrorInfo::new(
                70013,
                "update access token error".to_string(),
                "更新访问token错误".to_string(),
            ),
            AppStatus::DeleteAccessTokenError => DolphinErrorInfo::new(
                70014,
                "delete access token error".to_string(),
                "删除访问token错误".to_string(),
            ),
            AppStatus::AccessTokenNotExist => DolphinErrorInfo::new(
                70015,
                "access token not exist".to_string(),
                "访问token不存在".to_string(),
            ),
            AppStatus::QueryAccesstokenByUserError => DolphinErrorInfo::new(
                70016,
                "query access token by user error".to_string(),
                "查询访问指定用户的token错误".to_string(),
            ),
            AppStatus::CommandStateCountError => DolphinErrorInfo::new(
                80001,
                "task instance state count error".to_string(),
                "查询各状态任务实例数错误".to_string(),
            ),
            AppStatus::NegativeSizeNumberError => DolphinErrorInfo::new(
                80002,
                "query size number error".to_string(),
                "查询size错误".to_string(),
            ),
            AppStatus::StartTimeBiggerThanEndTimeError => DolphinErrorInfo::new(
                80003,
                "start time bigger than end time error".to_string(),
                "开始时间在结束时间之后错误".to_string(),
            ),
            AppStatus::QueueCountError => DolphinErrorInfo::new(
                90001,
                "queue count error".to_string(),
                "查询队列数据错误".to_string(),
            ),
            AppStatus::KerberosStartupState => DolphinErrorInfo::new(
                100001,
                "get kerberos startup state error".to_string(),
                "获取kerberos启动状态错误".to_string(),
            ),
            AppStatus::QueryAuditLogListPaging => DolphinErrorInfo::new(
                10057,
                "query resources list paging".to_string(),
                "分页查询资源列表错误".to_string(),
            ),
            AppStatus::PluginNotAUiComponent => DolphinErrorInfo::new(
                110001,
                "query plugin error: this plugin has no UI component".to_string(),
                "查询插件错误，此插件无UI组件".to_string(),
            ),
            AppStatus::QueryPluginsResultIsNull => DolphinErrorInfo::new(
                110002,
                "query alarm plugins result is empty:please check the startup status of the alarm \
                 component and confirm that the relevant alarm plugin is successfully registered"
                    .to_string(),
                "查询告警插件为空".to_string(),
            ),
            AppStatus::QueryPluginsError => DolphinErrorInfo::new(
                110003,
                "query plugins error".to_string(),
                "查询插件错误".to_string(),
            ),
            AppStatus::QueryPluginDetailResultIsNull => DolphinErrorInfo::new(
                110004,
                "query plugin detail result is null".to_string(),
                "查询插件详情结果为空".to_string(),
            ),
            AppStatus::UpdateAlertPluginInstanceError => DolphinErrorInfo::new(
                110005,
                "update alert plugin instance error".to_string(),
                "更新告警组和告警组插件实例错误".to_string(),
            ),
            AppStatus::DeleteAlertPluginInstanceError => DolphinErrorInfo::new(
                110006,
                "delete alert plugin instance error".to_string(),
                "删除告警组和告警组插件实例错误".to_string(),
            ),
            AppStatus::GetAlertPluginInstanceError => DolphinErrorInfo::new(
                110007,
                "get alert plugin instance error".to_string(),
                "获取告警组和告警组插件实例错误".to_string(),
            ),
            AppStatus::CreateAlertPluginInstanceError => DolphinErrorInfo::new(
                110008,
                "create alert plugin instance error".to_string(),
                "创建告警组和告警组插件实例错误".to_string(),
            ),
            AppStatus::QueryAllAlertPluginInstanceError => DolphinErrorInfo::new(
                110009,
                "query all alert plugin instance error".to_string(),
                "查询所有告警实例失败".to_string(),
            ),
            AppStatus::PluginInstanceAlreadyExit => DolphinErrorInfo::new(
                110010,
                "plugin instance already exit".to_string(),
                "该告警插件实例已存在".to_string(),
            ),
            AppStatus::ListPagingAlertPluginInstanceError => DolphinErrorInfo::new(
                110011,
                "query plugin instance page error".to_string(),
                "分页查询告警实例失败".to_string(),
            ),
            AppStatus::DeleteAlertPluginInstanceErrorHasAlertGroupAssociated =>
                DolphinErrorInfo::new(
                    110012,
                    "failed to delete the alert instance there is an alarm group associated with \
                     this alert instance"
                        .to_string(),
                    "删除告警实例失败，存在与此告警实例关联的警报组".to_string(),
                ),
            AppStatus::ProcessDefinitionVersionIsUsed => DolphinErrorInfo::new(
                110013,
                "this process definition version is used".to_string(),
                "此工作流定义版本被使用".to_string(),
            ),
            AppStatus::CreateEnvironmentError => DolphinErrorInfo::new(
                120001,
                "create environment error".to_string(),
                "创建环境失败".to_string(),
            ),
            AppStatus::EnvironmentNameExists => DolphinErrorInfo::new(
                120002,
                "this environment name [{0}] already exists".to_string(),
                "环境名称[{0}]已经存在".to_string(),
            ),
            AppStatus::EnvironmentNameIsNull => DolphinErrorInfo::new(
                120003,
                "this environment name shouldn't be empty.".to_string(),
                "环境名称不能为空".to_string(),
            ),
            AppStatus::EnvironmentConfigIsNull => DolphinErrorInfo::new(
                120004,
                "this environment config shouldn't be empty.".to_string(),
                "环境配置信息不能为空".to_string(),
            ),
            AppStatus::UpdateEnvironmentError => DolphinErrorInfo::new(
                120005,
                "update environment [{0}] info error".to_string(),
                "更新环境[{0}]信息失败".to_string(),
            ),
            AppStatus::DeleteEnvironmentError => DolphinErrorInfo::new(
                120006,
                "delete environment error".to_string(),
                "删除环境信息失败".to_string(),
            ),
            AppStatus::DeleteEnvironmentRelatedTaskExists => DolphinErrorInfo::new(
                120007,
                "delete environment error, related task exists".to_string(),
                "删除环境信息失败，存在关联任务".to_string(),
            ),
            AppStatus::QueryEnvironmentByNameError => DolphinErrorInfo::new(
                1200008,
                "not found environment [{0}] ".to_string(),
                "查询环境名称[{0}]信息不存在".to_string(),
            ),
            AppStatus::QueryEnvironmentByCodeError => DolphinErrorInfo::new(
                1200009,
                "not found environment [{0}] ".to_string(),
                "查询环境编码[{0}]不存在".to_string(),
            ),
            AppStatus::QueryEnvironmentError => DolphinErrorInfo::new(
                1200010,
                "login user query environment error".to_string(),
                "分页查询环境列表错误".to_string(),
            ),
            AppStatus::VerifyEnvironmentError => DolphinErrorInfo::new(
                1200011,
                "verify environment error".to_string(),
                "验证环境信息错误".to_string(),
            ),
            AppStatus::GetRuleFormCreateJsonError => DolphinErrorInfo::new(
                1200012,
                "get rule form create json error".to_string(),
                "获取规则 FROM-CREATE-JSON 错误".to_string(),
            ),
            AppStatus::QueryRuleListPagingError => DolphinErrorInfo::new(
                1200013,
                "query rule list paging error".to_string(),
                "获取规则分页列表错误".to_string(),
            ),
            AppStatus::QueryRuleListError => DolphinErrorInfo::new(
                1200014,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            ),
            AppStatus::QueryRuleInputEntryListError => DolphinErrorInfo::new(
                1200015,
                "query rule list error".to_string(),
                "获取规则列表错误".to_string(),
            ),
            AppStatus::QueryExecuteResultListPagingError => DolphinErrorInfo::new(
                1200016,
                "query execute result list paging error".to_string(),
                "获取数据质量任务结果分页错误".to_string(),
            ),
            AppStatus::GetDatasourceOptionsError => DolphinErrorInfo::new(
                1200017,
                "get datasource options error".to_string(),
                "获取数据源Options错误".to_string(),
            ),
            AppStatus::GetDatasourceTablesError => DolphinErrorInfo::new(
                1200018,
                "get datasource tables error".to_string(),
                "获取数据源表列表错误".to_string(),
            ),
            AppStatus::GetDatasourceTableColumnsError => DolphinErrorInfo::new(
                1200019,
                "get datasource table columns error".to_string(),
                "获取数据源表列名错误".to_string(),
            ),
            AppStatus::TaskGroupNameExist => DolphinErrorInfo::new(
                130001,
                "this task group name is repeated in a project".to_string(),
                "该任务组名称在一个项目中已经使用".to_string(),
            ),
            AppStatus::TaskGroupSizeError => DolphinErrorInfo::new(
                130002,
                "task group size error".to_string(),
                "任务组大小应该为大于1的整数".to_string(),
            ),
            AppStatus::TaskGroupStatusError => DolphinErrorInfo::new(
                130003,
                "task group status error".to_string(),
                "任务组已经被关闭".to_string(),
            ),
            AppStatus::TaskGroupFull => DolphinErrorInfo::new(
                130004,
                "task group is full".to_string(),
                "任务组已经满了".to_string(),
            ),
            AppStatus::TaskGroupUsedSizeError => DolphinErrorInfo::new(
                130005,
                "the used size number of task group is dirty".to_string(),
                "任务组使用的容量发生了变化".to_string(),
            ),
            AppStatus::TaskGroupQueueReleaseError => DolphinErrorInfo::new(
                130006,
                "failed to release task group queue".to_string(),
                "任务组资源释放时出现了错误".to_string(),
            ),
            AppStatus::TaskGroupQueueAwakeError => DolphinErrorInfo::new(
                130007,
                "awake waiting task failed".to_string(),
                "任务组使唤醒等待任务时发生了错误".to_string(),
            ),
            AppStatus::CreateTaskGroupError => DolphinErrorInfo::new(
                130008,
                "create task group error".to_string(),
                "创建任务组错误".to_string(),
            ),
            AppStatus::UpdateTaskGroupError => DolphinErrorInfo::new(
                130009,
                "update task group list error".to_string(),
                "更新任务组错误".to_string(),
            ),
            AppStatus::QueryTaskGroupListError => DolphinErrorInfo::new(
                130010,
                "query task group list error".to_string(),
                "查询任务组列表错误".to_string(),
            ),
            AppStatus::CloseTaskGroupError => DolphinErrorInfo::new(
                130011,
                "close task group error".to_string(),
                "关闭任务组错误".to_string(),
            ),
            AppStatus::StartTaskGroupError => DolphinErrorInfo::new(
                130012,
                "start task group error".to_string(),
                "启动任务组错误".to_string(),
            ),
            AppStatus::QueryTaskGroupQueueListError => DolphinErrorInfo::new(
                130013,
                "query task group queue list error".to_string(),
                "查询任务组队列列表错误".to_string(),
            ),
            AppStatus::TaskGroupCacheStartFailed => DolphinErrorInfo::new(
                130014,
                "cache start failed".to_string(),
                "任务组相关的缓存启动失败".to_string(),
            ),
            AppStatus::EnvironmentWorkerGroupsIsInvalid => DolphinErrorInfo::new(
                130015,
                "environment worker groups is invalid format".to_string(),
                "环境关联的工作组参数解析错误".to_string(),
            ),
            AppStatus::UpdateEnvironmentWorkerGroupRelationError => DolphinErrorInfo::new(
                130016,
                "update environment worker group relation error".to_string(),
                "更新环境关联的工作组错误".to_string(),
            ),
            AppStatus::TaskGroupQueueAlreadyStart => DolphinErrorInfo::new(
                130017,
                "task group queue already start".to_string(),
                "节点已经获取任务组资源".to_string(),
            ),
            AppStatus::TaskGroupStatusClosed => DolphinErrorInfo::new(
                130018,
                "The task group has been closed.".to_string(),
                "任务组已经被关闭".to_string(),
            ),
            AppStatus::TaskGroupStatusOpened => DolphinErrorInfo::new(
                130019,
                "The task group has been opened.".to_string(),
                "任务组已经被开启".to_string(),
            ),
            AppStatus::NotAllowToDisableOwnAccount => DolphinErrorInfo::new(
                130020,
                "Not allow to disable your own account".to_string(),
                "不能停用自己的账号".to_string(),
            ),
            AppStatus::NotAllowToDeleteDefaultAlarmGroup => DolphinErrorInfo::new(
                130030,
                "Not allow to delete the default alarm group ".to_string(),
                "不能删除默认告警组".to_string(),
            ),
            AppStatus::TimeZoneIllegal => DolphinErrorInfo::new(
                130031,
                "time zone [{0}] is illegal".to_string(),
                "时区参数 [{0}] 不合法".to_string(),
            ),
            AppStatus::QueryK8sNamespaceListPagingError => DolphinErrorInfo::new(
                1300001,
                "login user query k8s namespace list paging error".to_string(),
                "分页查询k8s名称空间列表错误".to_string(),
            ),
            AppStatus::K8sNamespaceExist => DolphinErrorInfo::new(
                1300002,
                "k8s namespace {0} already exists".to_string(),
                "k8s命名空间[{0}]已存在".to_string(),
            ),
            AppStatus::CreateK8sNamespaceError => DolphinErrorInfo::new(
                1300003,
                "create k8s namespace error".to_string(),
                "创建k8s命名空间错误".to_string(),
            ),
            AppStatus::UpdateK8sNamespaceError => DolphinErrorInfo::new(
                1300004,
                "update k8s namespace error".to_string(),
                "更新k8s命名空间信息错误".to_string(),
            ),
            AppStatus::K8sNamespaceNotExist => DolphinErrorInfo::new(
                1300005,
                "k8s namespace {0} not exists".to_string(),
                "命名空间ID[{0}]不存在".to_string(),
            ),
            AppStatus::K8sClientOpsError => DolphinErrorInfo::new(
                1300006,
                "k8s error with exception {0}".to_string(),
                "k8s操作报错[{0}]".to_string(),
            ),
            AppStatus::VerifyK8sNamespaceError => DolphinErrorInfo::new(
                1300007,
                "verify k8s and namespace error".to_string(),
                "验证k8s命名空间信息错误".to_string(),
            ),
            AppStatus::DeleteK8sNamespaceByIdError => DolphinErrorInfo::new(
                1300008,
                "delete k8s namespace by id error".to_string(),
                "删除命名空间错误".to_string(),
            ),
            AppStatus::VerifyParameterNameFailed => DolphinErrorInfo::new(
                1300009,
                "The file name verify failed".to_string(),
                "文件命名校验失败".to_string(),
            ),
            AppStatus::StoreOperateCreateError => DolphinErrorInfo::new(
                1300010,
                "create the resource failed".to_string(),
                "存储操作失败".to_string(),
            ),
            AppStatus::GrantK8sNamespaceError => DolphinErrorInfo::new(
                1300011,
                "grant namespace error".to_string(),
                "授权资源错误".to_string(),
            ),
            AppStatus::QueryUnauthorizedNamespaceError => DolphinErrorInfo::new(
                1300012,
                "query unauthorized namespace error".to_string(),
                "查询未授权命名空间错误".to_string(),
            ),
            AppStatus::QueryAuthorizedNamespaceError => DolphinErrorInfo::new(
                1300013,
                "query authorized namespace error".to_string(),
                "查询授权命名空间错误".to_string(),
            ),
            AppStatus::QueryCanUseK8sClusterError => DolphinErrorInfo::new(
                1300014,
                "login user query can used k8s cluster list error".to_string(),
                "查询可用k8s集群错误".to_string(),
            ),
            AppStatus::ResourceFullNameTooLongError => DolphinErrorInfo::new(
                1300015,
                "resource's fullname is too long error".to_string(),
                "资源文件名过长".to_string(),
            ),
            AppStatus::TenantFullNameTooLongError => DolphinErrorInfo::new(
                1300016,
                "tenant's fullname is too long error".to_string(),
                "租户名过长".to_string(),
            ),
        }
    }
}
