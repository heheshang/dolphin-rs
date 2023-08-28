use serde::{Deserialize, Serialize};

/**
 * status enum      // todo #4855 One category one interval
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppStatus {
    SUCCESS,
    InternalServerErrorArgs,
    RequestParamsNotValidError, //(10001, "request parameter {0} is not valid", "请求参数[{0}]无效"),
    TaskTimeoutParamsError, //(10002, "task timeout parameter is not valid", "任务超时参数无效"),
    UserNameExist,          //(10003, "user name already exists", "用户名已存在"),
    UserNameNull,           //(10004, "user name is null", "用户名不能为空"),
    HdfsOperationError,     //(10006, "hdfs operation error", "hdfs操作错误"),
    TaskInstanceNotFound,   //(10008, "task instance not found", "任务实例不存在"),
    OsTenantCodeExist, //(10009, "os tenant code {0} already exists", "操作系统租户[{0}]已存在"),
    UserNotExist,      //(10010, "user {0} not exists", "用户[{0}]不存在"),
    AlertGroupNotExist, //(10011, "alarm group not found", "告警组不存在"),
    AlertGroupExist,   //(10012, "alarm group already exists", "告警组名称已存在"),
    UserNamePasswdError, //(10013, "user name or password error", "用户名或密码错误"),
    LoginSessionFailed, //(10014, "create session failed!", "创建session失败"),
    DatasourceExist,   //(10015, "data source name already exists", "数据源名称已存在"),
    DatasourceConnectFailed, //(10016, "data source connection failed", "建立数据源连接失败"),
    TenantNotExist,    //(10017, "tenant not exists", "租户不存在"),
    ProjectNotFound,   //(10018, "project {0} not found ", "项目[{0}]不存在"),
    ProjectAlreadyExists, //(10019, "project {0} already exists", "项目名称[{0}]已存在"),
    TaskInstanceNotExists, //(10020, "task instance {0} does not exist", "任务实例[{0}]不存在"),
    TaskInstanceNotSubWorkflowInstance, //(10021, "task instance {0} is not sub process instance", "任务实例[{0}]不是子流程实例"),
    ScheduleCronNotExists, //(10022, "scheduler crontab {0} does not exist", "调度配置定时表达式[{0}]不存在"),
    ScheduleCronOnlineForbidUpdate, //(10023, "online status does not allow update operations", "调度配置上线状态不允许修改"),
    ScheduleCronCheckFailed, //(10024, "scheduler crontab expression validation failure: {0}", "调度配置定时表达式验证失败: {0}"),
    MasterNotExists,         //(10025, "master does not exist", "无可用master节点"),
    ScheduleStatusUnknown,   //(10026, "unknown status: {0}", "未知状态: {0}"),
    CreateAlertGroupError,   //(10027, "create alert group error", "创建告警组错误"),
    QueryAllAlertgroupError, //(10028, "query all alertgroup error", "查询告警组错误"),
    ListPagingAlertGroupError, //(10029, "list paging alert group error", "分页查询告警组错误"),
    UpdateAlertGroupError,   //(10030, "update alert group error", "更新告警组错误"),
    DeleteAlertGroupError,   //(10031, "delete alert group error", "删除告警组错误"),
    AlertGroupGrantUserError, //(10032, "alert group grant user error", "告警组授权用户错误"),
    CreateDatasourceError,   //(10033, "create datasource error", "创建数据源错误"),
    UpdateDatasourceError,   //(10034, "update datasource error", "更新数据源错误"),
    QueryDatasourceError,    //(10035, "query datasource error", "查询数据源错误"),
    ConnectDatasourceFailure, //(10036, "connect datasource failure", "建立数据源连接失败"),
    ConnectionTestFailure,   //(10037, "connection test failure", "测试数据源连接失败"),
    DeleteDataSourceFailure, //(10038, "delete data source failure", "删除数据源失败"),
    VerifyDatasourceNameFailure, //(10039, "verify datasource name failure", "验证数据源名称失败"),
    UnauthorizedDatasource,  //(10040, "unauthorized datasource", "未经授权的数据源"),
    AuthorizedDataSource,    //(10041, "authorized data source", "授权数据源失败"),
    LoginSuccess,            //(10042, "login success", "登录成功"),
    UserLoginFailure,        //(10043, "user login failure", "用户登录失败"),
    ListWorkersError,        //(10044, "list workers error", "查询worker列表错误"),
    ListMastersError,        //(10045, "list masters error", "查询master列表错误"),
    UpdateProjectError,      //(10046, "update project error", "更新项目信息错误"),
    QueryProjectDetailsByCodeError, //(10047, "query project details by code error", "查询项目详细信息错误"),
    CreateProjectError,             //(10048, "create project error", "创建项目错误"),
    LoginUserQueryProjectListPagingError, //(10049, "login user query project list paging error", "分页查询项目列表错误"),
    DeleteProjectError,                   //(10050, "delete project error", "删除项目错误"),
    QueryUnauthorizedProjectError, //(10051, "query unauthorized project error", "查询未授权项目错误"),
    QueryAuthorizedProject,        //(10052, "query authorized project", "查询授权项目错误"),
    QueryQueueListError,           //(10053, "query queue list error", "查询队列列表错误"),
    CreateResourceError,           //(10054, "create resource error", "创建资源错误"),
    UpdateResourceError,           //(10055, "update resource error", "更新资源错误"),
    QueryResourcesListError,       //(10056, "query resources list error", "查询资源列表错误"),
    QueryResourcesListPaging,      //(10057, "query resources list paging", "分页查询资源列表错误"),
    DeleteResourceError,           //(10058, "delete resource error", "删除资源错误"),
    VerifyResourceByNameAndTypeError, //(10059, "verify resource by name and type error", "资源名称或类型验证错误"),
    ViewResourceFileOnLineError, //(10060, "view resource file online error", "查看资源文件错误"),
    CreateResourceFileOnLineError, //(10061, "create resource file online error", "创建资源文件错误"),
    ResourceFileIsEmpty,           //(10062, "resource file is empty", "资源文件内容不能为空"),
    EditResourceFileOnLineError,   //(10063, "edit resource file online error", "更新资源文件错误"),
    DownloadResourceFileError,     //(10064, "download resource file error", "下载资源文件错误"),
    CreateUdfFunctionError,        //(10065, "create udf function error", "创建UDF函数错误"),
    ViewUdfFunctionError,          //(10066, "view udf function error", "查询UDF函数错误"),
    UpdateUdfFunctionError,        //(10067, "update udf function error", "更新UDF函数错误"),
    QueryUdfFunctionListPagingError, //(10068, "query udf function list paging error", "分页查询UDF函数列表错误"),
    QueryDatasourceByTypeError, //(10069, "query datasource by type error", "查询数据源信息错误"),
    VerifyUdfFunctionNameError, //(10070, "verify udf function name error", "UDF函数名称验证错误"),
    DeleteUdfFunctionError,     //(10071, "delete udf function error", "删除UDF函数错误"),
    AuthorizedFileResourceError, //(10072, "authorized file resource error", "授权资源文件错误"),
    AuthorizeResourceTree, //(10073, "authorize resource tree display error", "授权资源目录树错误"),
    UnauthorizedUdfFunctionError, //(10074, "unauthorized udf function error", "查询未授权UDF函数错误"),
    AuthorizedUdfFunctionError,   //(10075, "authorized udf function error", "授权UDF函数错误"),
    CreateScheduleError,          //(10076, "create schedule error", "创建调度配置错误"),
    UpdateScheduleError,          //(10077, "update schedule error", "更新调度配置错误"),
    PublishScheduleOnlineError,   //(10078, "publish schedule online error", "上线调度配置错误"),
    OfflineScheduleError,         //(10079, "offline schedule error", "下线调度配置错误"),
    QueryScheduleListPagingError, //(10080, "query schedule list paging error", "分页查询调度配置列表错误"),
    QueryScheduleListError,       //(10081, "query schedule list error", "查询调度配置列表错误"),
    QueryTaskListPagingError,     //(10082, "query task list paging error", "分页查询任务列表错误"),
    QueryTaskRecordListPagingError, //(10083, "query task record list paging error", "分页查询任务记录错误"),
    CreateTenantError,              //(10084, "create tenant error", "创建租户错误"),
    QueryTenantListPagingError, //(10085, "query tenant list paging error", "分页查询租户列表错误"),
    QueryTenantListError,       //(10086, "query tenant list error", "查询租户列表错误"),
    UpdateTenantError,          //(10087, "update tenant error", "更新租户错误"),
    DeleteTenantByIdError,      //(10088, "delete tenant by id error", "删除租户错误"),
    VerifyOsTenantCodeError,    //(10089, "verify os tenant code error", "操作系统租户验证错误"),
    CreateUserError,            //(10090, "create user error", "创建用户错误"),
    QueryUserListPagingError,   //(10091, "query user list paging error", "分页查询用户列表错误"),
    UpdateUserError,            //(10092, "update user error", "更新用户错误"),
    DeleteUserByIdError,        //(10093, "delete user by id error", "删除用户错误"),
    GrantProjectError,          //(10094, "grant project error", "授权项目错误"),
    GrantResourceError,         //(10095, "grant resource error", "授权资源错误"),
    GrantUdfFunctionError,      //(10096, "grant udf function error", "授权UDF函数错误"),
    GrantDatasourceError,       //(10097, "grant datasource error", "授权数据源错误"),
    GetUserInfoError,           //(10098, "get user info error", "获取用户信息错误"),
    UserListError,              //(10099, "user list error", "查询用户列表错误"),
    VerifyUsernameError,        //(10100, "verify username error", "用户名验证错误"),
    UnauthorizedUserError,      //(10101, "unauthorized user error", "查询未授权用户错误"),
    AuthorizedUserError,        //(10102, "authorized user error", "查询授权用户错误"),
    QueryTaskInstanceLogError,  //(10103, "view task instance log error", "查询任务实例日志错误"),
    DownloadTaskInstanceLogFileError, //(10104, "download task instance log file error", "下载任务日志文件错误"),
    CreateProcessDefinitionError, //(10105, "create process definition error", "创建工作流错误"),
    VerifyProcessDefinitionNameUniqueError, //(10106, "verify process definition name unique error", "工作流定义名称验证错误"),
    UpdateProcessDefinitionError, //(10107, "update process definition error", "更新工作流定义错误"),
    ReleaseProcessDefinitionError, //(10108, "release process definition error", "上线工作流错误"),
    QueryDetailOfProcessDefinitionError, //(10109, "query detail of process definition error", "查询工作流详细信息错误"),
    QueryProcessDefinitionList, //(10110, "query process definition list", "查询工作流列表错误"),
    EncapsulationTreeviewStructureError, //(10111, "encapsulation treeview structure error", "查询工作流树形图数据错误"),
    GetTasksListByProcessDefinitionIdError, //(10112, "get tasks list by process definition id error", "查询工作流定义节点信息错误"),
    QueryProcessInstanceListPagingError, //(10113, "query process instance list paging error", "分页查询工作流实例列表错误"),
    QueryTaskListByProcessInstanceIdError, //(10114, "query task list by process instance id error", "查询任务实例列表错误"),
    UpdateProcessInstanceError, //(10115, "update process instance error", "更新工作流实例错误"),
    QueryProcessInstanceByIdError, //(10116, "query process instance by id error", "查询工作流实例错误"),
    DeleteProcessInstanceByIdError, //(10117, "delete process instance by id error", "删除工作流实例错误"),
    QuerySubProcessInstanceDetailInfoByTaskIdError, //(10118, "query sub process instance detail info by task id error", "查询子流程任务实例错误"),
    QueryParentProcessInstanceDetailInfoBySubProcessInstanceIdError, //(10119, "query parent process instance detail info by sub process instance id error", "查询子流程该工作流实例错误"),
    QueryProcessInstanceAllVariablesError, //(10120, "query process instance all variables error", "查询工作流自定义变量信息错误"),
    EncapsulationProcessInstanceGanttStructureError, //(10121, "encapsulation process instance gantt structure error", "查询工作流实例甘特图数据错误"),
    QueryProcessDefinitionListPagingError, //(10122, "query process definition list paging error", "分页查询工作流定义列表错误"),
    SignOutError,                          //(10123, "sign out error", "退出错误"),
    OsTenantCodeHasAlreadyExists, //(10124, "os tenant code has already exists", "操作系统租户已存在"),
    IpIsEmpty,                    //(10125, "ip is empty", "IP地址不能为空"),
    ScheduleCronReleaseNeedNotChange, //(10126, "schedule release is already {0}", "调度配置上线错误[{0}]"),
    CreateQueueError,                 //(10127, "create queue error", "创建队列错误"),
    QueueNotExist,                    //(10128, "queue {0} not exists", "队列ID[{0}]不存在"),
    QueueValueExist, //(10129, "queue value {0} already exists", "队列值[{0}]已存在"),
    QueueNameExist,  //(10130, "queue name {0} already exists", "队列名称[{0}]已存在"),
    UpdateQueueError, //(10131, "update queue error", "更新队列信息错误"),
    NeedNotUpdateQueue, //(10132, "no content changes, no updates are required", "数据未变更，不需要更新队列信息"),
    VerifyQueueError,   //(10133, "verify queue error", "验证队列信息错误"),
    NameNull,           //(10134, "name must be not null", "名称不能为空"),
    NameExist,          //(10135, "name {0} already exists", "名称[{0}]已存在"),
    SaveError,          //(10136, "save error", "保存错误"),
    DeleteProjectErrorDefinesNotNull, //(10137, "please delete the process definitions in project first!", "请先删除全部工作流定义"),
    BatchDeleteProcessInstanceByIdsError, //(10117, "batch delete process instance by ids {0} error", "批量删除工作流实例错误: {0}"),
    PreviewScheduleError,                 //(10139, "preview schedule error", "预览调度配置错误"),
    ParseToCronExpressionError, //(10140, "parse cron to cron expression error", "解析调度表达式错误"),
    ScheduleStartTimeEndTimeSame, //(10141, "The start time must not be the same as the end", "开始时间不能和结束时间一样"),
    DeleteTenantByIdFail, //(10142, "delete tenant by id fail, for there are {0} process instances in executing using it", "删除租户失败，有[{0}]个运行中的工作流实例正在使用"),
    DeleteTenantByIdFailDefines, //(10143, "delete tenant by id fail, for there are {0} process definitions using it", "删除租户失败，有[{0}]个工作流定义正在使用"),
    DeleteTenantByIdFailUsers, //(10144, "delete tenant by id fail, for there are {0} users using it", "删除租户失败，有[{0}]个用户正在使用"),
    DeleteWorkerGroupByIdFail, //(10145, "delete worker group by id fail, for there are {0} process instances in executing using it", "删除Worker分组失败，有[{0}]个运行中的工作流实例正在使用"),
    QueryWorkerGroupFail,      //(10146, "query worker group fail ", "查询worker分组失败"),
    DeleteWorkerGroupFail,     //(10147, "delete worker group fail ", "删除worker分组失败"),
    UserDisabled,              //(10148, "The current user is disabled", "当前用户已停用"),
    CopyProcessDefinitionError, //(10149, "copy process definition from {0} to {1} error : {2}", "从{0}复制工作流到{1}错误 : {2}"),
    MoveProcessDefinitionError, //(10150, "move process definition from {0} to {1} error : {2}", "从{0}移动工作流到{1}错误 : {2}"),
    SwitchProcessDefinitionVersionError, //(10151, "Switch process definition version error", "切换工作流版本出错"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionError, //(10152  , "Switch process definition version error: not exists process definition, [process definition id {0}]", "切换工作流版本出错：工作流不存在，[工作流id {0}]"),
    SwitchProcessDefinitionVersionNotExistProcessDefinitionVersionError, //(10153 , "Switch process defi:nition version error: not exists process definition version, [process definition id {0}] [version number {1}]", "切换工作流版本出错：工作流版本信息不存在，[工作流id {0}] [版本号 {1}]"),
    QueryProcessDefinitionVersionsError, //(10154, "query process definition versions error", "查询工作流历史版本信息出错"),
    DeleteProcessDefinitionVersionError, //(10156, "delete process definition version error", "删除工作流历史版本出错"),

    QueryUserCreatedProjectError, //(10157, "query user created project error error", "查询用户创建的项目错误"),
    ProcessDefinitionCodesIsEmpty, //(10158, "process definition codes is empty", "工作流CODES不能为空"),
    BatchCopyProcessDefinitionError, //(10159, "batch copy process definition error", "复制工作流错误"),
    BatchMoveProcessDefinitionError, //(10160, "batch move process definition error", "移动工作流错误"),
    QueryWorkflowLineageError,       //(10161, "query workflow lineage error", "查询血缘失败"),
    QueryAuthorizedAndUserCreatedProjectError, //(10162, "query authorized and user created project error error", "查询授权的和用户创建的项目错误"),
    DeleteProcessDefinitionByCodeFail, //(10163, "delete process definition by code fail, for there are {0} process instances in executing using it", "删除工作流定义失败，有[{0}]个运行中的工作流实例正在使用"),
    CheckOsTenantCodeError, //(10164, "Tenant code invalid, should follow linux's users naming conventions", "非法的租户名，需要遵守 Linux 用户命名规范"),
    ForceTaskSuccessError,  //(10165, "force task success error", "强制成功任务实例错误"),
    TaskInstanceStateOperationError, //(10166, "the status of task instance {0} is {1},Cannot perform force success operation", "任务实例[{0}]的状态是[{1}]，无法执行强制成功操作"),
    DatasourceTypeNotExist,          //(10167, "data source type not exist", "数据源类型不存在"),
    ProcessDefinitionNameExist, //(10168, "process definition name {0} already exists", "工作流定义名称[{0}]已存在"),
    DatasourceDbTypeIllegal,    //(10169, "datasource type illegal", "数据源类型参数不合法"),
    DatasourcePortIllegal,      //(10170, "datasource port illegal", "数据源端口参数不合法"),
    DatasourceOtherParamsIllegal, //(10171, "datasource other params illegal", "数据源其他参数不合法"),
    DatasourceNameIllegal,        //(10172, "datasource name illegal", "数据源名称不合法"),
    DatasourceHostIllegal,        //(10173, "datasource host illegal", "数据源HOST不合法"),
    DeleteWorkerGroupNotExist, //(10174, "delete worker group not exist ", "删除worker分组不存在"),
    CreateWorkerGroupForbiddenInDocker, //(10175, "create worker group forbidden in docker ", "创建worker分组在docker中禁止"),
    DeleteWorkerGroupForbiddenInDocker, //(10176, "delete worker group forbidden in docker ", "删除worker分组在docker中禁止"),
    WorkerAddressInvalid, //(10177, "worker address {0} invalid", "worker地址[{0}]无效"),
    QueryWorkerAddressListFail, //(10178, "query worker address list fail ", "查询worker地址列表失败"),
    TransformProjectOwnership, //(10179, "Please transform project ownership [{0}]", "请先转移项目所有权[{0}]"),
    QueryAlertGroupError,      //(10180, "query alert group error", "查询告警组错误"),
    CurrentLoginUserTenantNotExist, //(10181, "the tenant of the currently login user is not specified", "未指定当前登录用户的租户"),
    RevokeProjectError,             //(10182, "revoke project error", "撤销项目授权错误"),
    QueryAuthorizedUser, //(10183, "query authorized user error", "查询拥有项目权限的用户错误"),
    ProjectNotExist, //(10190, "This project was not found. Please refresh page.", "该项目不存在,请刷新页面"),
    TaskInstanceHostIsNull, //(10191, "task instance host is null", "任务实例host为空"),
    QueryExecutingWorkflowError, //(10192, "query executing workflow error", "查询运行的工作流实例错误"),

    UdfFunctionNotExist, //(20001, "UDF function not found", "UDF函数不存在"),
    UdfFunctionExists,   //(20002, "UDF function already exists", "UDF函数已存在"),
    ResourceNotExist,    //(20004, "resource not exist", "资源不存在"),
    ResourceExist,       //(20005, "resource already exists", "资源已存在"),
    ResourceSuffixNotSupportView, //(20006, "resource suffix do not support online viewing", "资源文件后缀不支持查看"),
    ResourceSizeExceedLimit, //(20007, "upload resource file size exceeds limit", "上传资源文件大小超过限制"),
    ResourceSuffixForbidChange, //(20008, "resource suffix not allowed to be modified", "资源文件后缀不支持修改"),
    UdfResourceSuffixNotJar, //(20009, "UDF resource suffix name must be jar", "UDF资源文件后缀名只支持[jar]"),
    HdfsCopyFail,            //(20010, "hdfs copy {0} -> {1} fail", "hdfs复制失败：[{0}] -> [{1}]"),
    ResourceFileExist, //(20011, "resource file {0} already exists in hdfs,please delete it or change name!", "资源文件[{0}]在hdfs中已存在，请删除或修改资源名"),
    ResourceFileNotExist, //(20012, "resource file {0} not exists !", "资源文件[{0}]不存在"),
    UdfResourceIsBound, //(20013, "udf resource file is bound by UDF functions:{0}", "udf函数绑定了资源文件[{0}]"),
    ResourceIsUsed, //(20014, "resource file is used by process definition", "资源文件被上线的流程定义使用了"),
    ParentResourceNotExist, //(20015, "parent resource not exist", "父资源文件不存在"),
    ResourceNotExistOrNoPermission, //(20016, "resource not exist or no permission,please view the task node and remove error resource", "请检查任务节点并移除无权限或者已删除的资源"),
    ResourceIsAuthorized, //(20017, "resource is authorized to user {0},suffix not allowed to be modified", "资源文件已授权其他用户[{0}],后缀不允许修改"),

    UserNoOperationPerm, //(30001, "user has no operation privilege", "当前用户没有操作权限"),
    UserNoOperationProjectPerm, //(30002, "user {0} is not has project {1} permission", "当前用户[{0}]没有[{1}]项目的操作权限"),

    ProcessInstanceNotExist, //(50001, "process instance {0} does not exist", "工作流实例[{0}]不存在"),
    ProcessInstanceExist, //(50002, "process instance {0} already exists", "工作流实例[{0}]已存在"),
    ProcessDefineNotExist, //(50003, "process definition {0} does not exist", "工作流定义[{0}]不存在"),
    ProcessDefineNotRelease, //(50004, "process definition {0} process version {1} not online", "工作流定义[{0}] 工作流版本[{1}]不是上线状态"),
    SubProcessDefineNotRelease, //(50004, "exist sub process definition not online", "存在子工作流定义不是上线状态"),
    ProcessInstanceAlreadyChanged, //(50005, "the status of process instance {0} is already {1}", "工作流实例[{0}]的状态已经是[{1}]"),
    ProcessInstanceStateOperationError, //(50006, "the status of process instance {0} is {1},Cannot perform {2} operation", "工作流实例[{0}]的状态是[{1}]，无法执行[{2}]操作"),
    SubProcessInstanceNotExist, //(50007, "the task belong to process instance does not exist", "子工作流实例不存在"),
    ProcessDefineNotAllowedEdit, //(50008, "process definition {0} does not allow edit", "工作流定义[{0}]不允许修改"),
    ProcessInstanceExecutingCommand, //(50009, "process instance {0} is executing the command, please wait ...", "工作流实例[{0}]正在执行命令，请稍等..."),
    ProcessInstanceNotSubProcessInstance, //(50010, "process instance {0} is not sub process instance", "工作流实例[{0}]不是子工作流实例"),
    TaskInstanceStateCountError, //(50011, "task instance state count error", "查询各状态任务实例数错误"),
    CountProcessInstanceStateError, //(50012, "count process instance state error", "查询各状态流程实例数错误"),
    CountProcessDefinitionUserError, //(50013, "count process definition user error", "查询各用户流程定义数错误"),
    StartProcessInstanceError, //(50014, "start process instance error", "运行工作流实例错误"),
    BatchStartProcessInstanceError, //(50014, "batch start process instance error: {0}", "批量运行工作流实例错误: {0}"),
    ProcessInstanceError, //(50014, "process instance delete error: {0}", "工作流实例删除[{0}]错误"),
    ExecuteProcessInstanceError, //(50015, "execute process instance error", "操作工作流实例错误"),
    CheckProcessDefinitionError, //(50016, "check process definition error", "工作流定义错误"),
    QueryRecipientsAndCopyersByProcessDefinitionError, //(50017, "query recipients and copyers by process definition error", "查询收件人和抄送人错误"),
    DataIsNotValid,                 //(50017, "data {0} not valid", "数据[{0}]无效"),
    DataIsNull,                     //(50018, "data {0} is null", "数据[{0}]不能为空"),
    ProcessNodeHasCycle,            //(50019, "process node has cycle", "流程节点间存在循环依赖"),
    ProcessNodeSParameterInvalid, //(50020, "process node {0} parameter invalid", "流程节点[{0}]参数无效"),
    ProcessDefineStateOnline, //(50021, "process definition [{0}] is already online", "工作流定义[{0}]已上线"),
    DeleteProcessDefineByCodeError, //(50022, "delete process definition by code error", "删除工作流定义错误"),
    ScheduleCronStateOnline, //(50023, "the status of schedule {0} is already online", "调度配置[{0}]已上线"),
    DeleteScheduleCronByIdError, //(50024, "delete schedule by id error", "删除调度配置错误"),
    BatchDeleteProcessDefineError, //(50025, "batch delete process definition error", "批量删除工作流定义错误"),
    BatchDeleteProcessDefineByCodesError, //(50026, "batch delete process definition by codes {0} error", "批量删除工作流定义[{0}]错误"),
    DeleteProcessDefineByCodesError, //(50026, "delete process definition by codes {0} error", "删除工作流定义[{0}]错误"),
    TenantNotSuitable, //(50027, "there is not any tenant suitable, please choose a tenant available.", "没有合适的租户，请选择可用的租户"),
    ExportProcessDefineByIdError, //(50028, "export process definition by id error", "导出工作流定义错误"),
    BatchExportProcessDefineByIdsError, //(50028, "batch export process definition by ids error", "批量导出工作流定义错误"),
    ImportProcessDefineError, //(50029, "import process definition error", "导入工作流定义错误"),
    TaskDefineNotExist, //(50030, "task definition [{0}] does not exist", "任务定义[{0}]不存在"),
    CreateProcessTaskRelationError, //(50032, "create process task relation error", "创建工作流任务关系错误"),
    ProcessTaskRelationNotExist, //(50033, "process task relation [{0}] does not exist", "工作流任务关系[{0}]不存在"),
    ProcessTaskRelationExist, //(50034, "process task relation is already exist, processCode:[{0}]", "工作流任务关系已存在, processCode:[{0}]"),
    ProcessDagIsEmpty,        //(50035, "process dag is empty", "工作流dag是空"),
    CheckProcessTaskRelationError, //(50036, "check process task relation error", "工作流任务关系参数错误"),
    CreateTaskDefinitionError,     //(50037, "create task definition error", "创建任务错误"),
    UpdateTaskDefinitionError,     //(50038, "update task definition error", "更新任务定义错误"),
    QueryTaskDefinitionVersionsError, //(50039, "query task definition versions error", "查询任务历史版本信息出错"),
    SwitchTaskDefinitionVersionError, //(50040, "Switch task definition version error", "切换任务版本出错"),
    DeleteTaskDefinitionVersionError, //(50041, "delete task definition version error", "删除任务历史版本出错"),
    DeleteTaskDefineByCodeError, //(50042, "delete task definition by code error", "删除任务定义错误"),
    QueryDetailOfTaskDefinitionError, //(50043, "query detail of task definition error", "查询任务详细信息错误"),
    QueryTaskDefinitionListPagingError, //(50044, "query task definition list paging error", "分页查询任务定义列表错误"),
    TaskDefinitionNameExisted, //(50045, "task definition name [{0}] already exists", "任务定义名称[{0}]已经存在"),
    ReleaseTaskDefinitionError, //(50046, "release task definition error", "上线任务错误"),
    MoveProcessTaskRelationError, //(50047, "move process task relation error", "移动任务到其他工作流错误"),
    DeleteTaskProcessRelationError, //(50048, "delete process task relation error", "删除工作流任务关系错误"),
    QueryTaskProcessRelationError, //(50049, "query process task relation error", "查询工作流任务关系错误"),
    TaskDefineStateOnline, //(50050, "task definition [{0}] is already online", "任务定义[{0}]已上线"),
    TaskHasDownstream, //(50051, "Task exists downstream [{0}] dependence", "任务存在下游[{0}]依赖"),
    TaskHasUpstream,   //(50052, "Task [{0}] exists upstream dependence", "任务[{0}]存在上游依赖"),
    MainTableUsingVersion, //(50053, "the version that the master table is using", "主表正在使用该版本"),
    ProjectProcessNotMatch, //(50054, "the project and the process is not match", "项目和工作流不匹配"),
    DeleteEdgeError,        //(50055, "delete edge error", "删除工作流任务连接线错误"),
    NotSupportUpdateTaskDefinition, //(50056, "task state does not support modification", "当前任务不支持修改"),
    NotSupportCopyTaskType, //(50057, "task type [{0}] does not support copy", "不支持复制的任务类型[{0}]"),
    HdfsNotStartup,         //(60001, "hdfs not startup", "hdfs未启用"),
    StorageNotStartup,      //(60002, "storage not startup", "存储未启用"),
    S3CannotRename,         //(60003, "directory cannot be renamed", "S3无法重命名文件夹"),
    /**
     * for monitor
     */
    QueryDatabaseStateError, //(70001, "query database state error", "查询数据库状态错误"),

    CreateAccessTokenError, //(70010, "create access token error", "创建访问token错误"),
    GenerateTokenError,     //(70011, "generate token error", "生成token错误"),
    QueryAccesstokenListPagingError, //(70012, "query access token list paging error", "分页查询访问token列表错误"),
    UpdateAccessTokenError,          //(70013, "update access token error", "更新访问token错误"),
    DeleteAccessTokenError,          //(70014, "delete access token error", "删除访问token错误"),
    AccessTokenNotExist,             //(70015, "access token not exist", "访问token不存在"),
    QueryAccesstokenByUserError, //(70016, "query access token by user error", "查询访问指定用户的token错误"),

    CommandStateCountError, //(80001, "task instance state count error", "查询各状态任务实例数错误"),
    NegativeSizeNumberError, //(80002, "query size number error", "查询size错误"),
    StartTimeBiggerThanEndTimeError, //(80003, "start time bigger than end time error", "开始时间在结束时间之后错误"),
    QueueCountError,                 //(90001, "queue count error", "查询队列数据错误"),

    KerberosStartupState, //(100001, "get kerberos startup state error", "获取kerberos启动状态错误"),

    // audit log
    QueryAuditLogListPaging, //(10057, "query resources list paging", "分页查询资源列表错误"),

    //plugin
    PluginNotAUiComponent, //(110001, "query plugin error, this plugin has no UI component", "查询插件错误，此插件无UI组件"),
    QueryPluginsResultIsNull, //(110002, "query alarm plugins result is empty, please check the startup status of the alarm component and confirm that the relevant alarm plugin is successfully registered", "查询告警插件为空, 请检查告警组件启动状态并确认相关告警插件已注册成功"),
    QueryPluginsError,        //(110003, "query plugins error", "查询插件错误"),
    QueryPluginDetailResultIsNull, //(110004, "query plugin detail result is null", "查询插件详情结果为空"),

    UpdateAlertPluginInstanceError, //(110005, "update alert plugin instance error", "更新告警组和告警组插件实例错误"),
    DeleteAlertPluginInstanceError, //(110006, "delete alert plugin instance error", "删除告警组和告警组插件实例错误"),
    GetAlertPluginInstanceError, //(110007, "get alert plugin instance error", "获取告警组和告警组插件实例错误"),
    CreateAlertPluginInstanceError, //(110008, "create alert plugin instance error", "创建告警组和告警组插件实例错误"),
    QueryAllAlertPluginInstanceError, //(110009, "query all alert plugin instance error", "查询所有告警实例失败"),
    PluginInstanceAlreadyExit, //(110010, "plugin instance already exit", "该告警插件实例已存在"),
    ListPagingAlertPluginInstanceError, //(110011, "query plugin instance page error", "分页查询告警实例失败"),
    DeleteAlertPluginInstanceErrorHasAlertGroupAssociated, //(110012, "failed to delete the alert instance, there is an alarm group associated with this alert instance", "删除告警实例失败，存在与此告警实例关联的警报组"),
    ProcessDefinitionVersionIsUsed, //(110013, "this process definition version is used", "此工作流定义版本被使用"),

    CreateEnvironmentError, //(120001, "create environment error", "创建环境失败"),
    EnvironmentNameExists, //(120002, "this environment name [{0}] already exists", "环境名称[{0}]已经存在"),
    EnvironmentNameIsNull, //(120003, "this environment name shouldn't be empty.", "环境名称不能为空"),
    EnvironmentConfigIsNull, //(120004, "this environment config shouldn't be empty.", "环境配置信息不能为空"),
    UpdateEnvironmentError, //(120005, "update environment [{0}] info error", "更新环境[{0}]信息失败"),
    DeleteEnvironmentError, //(120006, "delete environment error", "删除环境信息失败"),
    DeleteEnvironmentRelatedTaskExists, //(120007, "this environment has been used in tasks,so you can't delete it.", "该环境已经被任务使用，所以不能删除该环境信息"),
    QueryEnvironmentByNameError, //(1200008, "not found environment [{0}] ", "查询环境名称[{0}]信息不存在"),
    QueryEnvironmentByCodeError, //(1200009, "not found environment [{0}] ", "查询环境编码[{0}]不存在"),
    QueryEnvironmentError, //(1200010, "login user query environment error", "分页查询环境列表错误"),
    VerifyEnvironmentError, //(1200011, "verify environment error", "验证环境信息错误"),
    GetRuleFormCreateJsonError, //(1200012, "get rule form create json error", "获取规则 FROM-CREATE-JSON 错误"),
    QueryRuleListPagingError,   //(1200013, "query rule list paging error", "获取规则分页列表错误"),
    QueryRuleListError,         //(1200014, "query rule list error", "获取规则列表错误"),
    QueryRuleInputEntryListError, //(1200015, "query rule list error", "获取规则列表错误"),
    QueryExecuteResultListPagingError, //(1200016, "query execute result list paging error", "获取数据质量任务结果分页错误"),
    GetDatasourceOptionsError, //(1200017, "get datasource options error", "获取数据源Options错误"),
    GetDatasourceTablesError,  //(1200018, "get datasource tables error", "获取数据源表列表错误"),
    GetDatasourceTableColumnsError, //(1200019, "get datasource table columns error", "获取数据源表列名错误"),
    TaskGroupNameExist, //(130001, "this task group name is repeated in a project", "该任务组名称在一个项目中已经使用"),
    TaskGroupSizeError, //(130002, "task group size error", "任务组大小应该为大于1的整数"),
    TaskGroupStatusError, //(130003, "task group status error", "任务组已经被关闭"),
    TaskGroupFull,      //(130004, "task group is full", "任务组已经满了"),
    TaskGroupUsedSizeError, //(130005, "the used size number of task group is dirty", "任务组使用的容量发生了变化"),
    TaskGroupQueueReleaseError, //(130006, "failed to release task group queue", "任务组资源释放时出现了错误"),
    TaskGroupQueueAwakeError, //(130007, "awake waiting task failed", "任务组使唤醒等待任务时发生了错误"),
    CreateTaskGroupError,     //(130008, "create task group error", "创建任务组错误"),
    UpdateTaskGroupError,     //(130009, "update task group list error", "更新任务组错误"),
    QueryTaskGroupListError,  //(130010, "query task group list error", "查询任务组列表错误"),
    CloseTaskGroupError,      //(130011, "close task group error", "关闭任务组错误"),
    StartTaskGroupError,      //(130012, "start task group error", "启动任务组错误"),
    QueryTaskGroupQueueListError, //(130013, "query task group queue list error", "查询任务组队列列表错误"),
    TaskGroupCacheStartFailed,    //(130014, "cache start failed", "任务组相关的缓存启动失败"),
    EnvironmentWorkerGroupsIsInvalid, //(130015, "environment worker groups is invalid format", "环境关联的工作组参数解析错误"),
    UpdateEnvironmentWorkerGroupRelationError, //(130016, "You can't modify the worker group, because the worker group [{0}] and this environment [{1}] already be used in the task [{2}]", "您不能修改工作组选项，因为该工作组 [{0}] 和 该环境 [{1}] 已经被用在任务 [{2}] 中"),
    TaskGroupQueueAlreadyStart, //(130017, "task group queue already start", "节点已经获取任务组资源"),
    TaskGroupStatusClosed,      //(130018, "The task group has been closed.", "任务组已经被关闭"),
    TaskGroupStatusOpened,      //(130019, "The task group has been opened.", "任务组已经被开启"),
    NotAllowToDisableOwnAccount, //(130020, "Not allow to disable your own account", "不能停用自己的账号"),
    NotAllowToDeleteDefaultAlarmGroup, //(130030, "Not allow to delete the default alarm group ", "不能删除默认告警组"),
    TimeZoneIllegal, //(130031, "time zone [{0}] is illegal", "时区参数 [{0}] 不合法"),

    QueryK8sNamespaceListPagingError, //(1300001, "login user query k8s namespace list paging error", "分页查询k8s名称空间列表错误"),
    K8sNamespaceExist, //(1300002, "k8s namespace {0} already exists", "k8s命名空间[{0}]已存在"),
    CreateK8sNamespaceError, //(1300003, "create k8s namespace error", "创建k8s命名空间错误"),
    UpdateK8sNamespaceError, //(1300004, "update k8s namespace error", "更新k8s命名空间信息错误"),
    K8sNamespaceNotExist, //(1300005, "k8s namespace {0} not exists", "命名空间ID[{0}]不存在"),
    K8sClientOpsError, //(1300006, "k8s error with exception {0}", "k8s操作报错[{0}]"),
    VerifyK8sNamespaceError, //(1300007, "verify k8s and namespace error", "验证k8s命名空间信息错误"),
    DeleteK8sNamespaceByIdError, //(1300008, "delete k8s namespace by id error", "删除命名空间错误"),
    VerifyParameterNameFailed,   //(1300009, "The file name verify failed", "文件命名校验失败"),
    StoreOperateCreateError,     //(1300010, "create the resource failed", "存储操作失败"),
    GrantK8sNamespaceError,      //(1300011, "grant namespace error", "授权资源错误"),
    QueryUnauthorizedNamespaceError, //(1300012, "query unauthorized namespace error", "查询未授权命名空间错误"),
    QueryAuthorizedNamespaceError, //(1300013, "query authorized namespace error", "查询授权命名空间错误"),
    QueryCanUseK8sClusterError, //(1300014, "login user query can used k8s cluster list error", "查询可用k8s集群错误"),
    ResourceFullNameTooLongError, //(1300015, "resource's fullname is too long error", "资源文件名过长"),
    TenantFullNameTooLongError,   //(1300016, "tenant's fullname is too long error", "租户名过长");
}


// pub fn serialize_status<S>(value: &AppStatus, serializer: S) -> Result<S::Ok, S::Error>
// where S: Serializer {
//     let s = match value {
//         AppStatus::SUCCESS => ErrorCode::new(0, "success".to_string(), "成功".to_string()),
//         AppStatus::InternalServerErrorArgs => ErrorCode {
//             code: 500,
//             en_msg: "ssss".to_string(),
//             cn_msg: "sssss".to_string(),
//         },
//         AppStatus::RequestParamsNotValidError => ErrorCode {
//             code: 500,
//             en_msg: "ssss".to_string(),
//             cn_msg: "sssss".to_string(),
//         },

//     };
//     serializer.serialize_newtype_struct("errcode", &s)
// }

impl Default for AppStatus {
    fn default() -> Self {
        Self::SUCCESS
    }
}
