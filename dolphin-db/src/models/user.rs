use rbatis::{crud, executor::Executor, RBatis};
use rbdc::datetime::DateTime;
use rbs::to_value;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub user_name: String,
    pub user_password: String,
    pub user_type: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tenant_id: i64,
    pub create_time: DateTime,
    pub update_time: DateTime,
    pub queue: Option<String>,
    pub state: i32,
    pub time_zone: Option<String>,
}
crud!(User {}, "t_ds_user");

impl User {
    pub async fn query_all_general_user(rb: RBatis) -> Vec<User> {
        rb.query_decode("select * from t_ds_user where user_type=?", vec![
            to_value!(1),
        ])
        .await
        .unwrap()
    }

    pub async fn query_all_general_user1(
        rb: &mut dyn Executor,
    ) -> std::result::Result<Vec<User>, rbatis::rbdc::Error> {
        let v = rb
            .query("select * from t_ds_user where user_type=?", vec![
                to_value!(1),
            ])
            .await?;

        rbatis::decode::decode(v)
    }

    // pub async fn select_by_column<V: serde::Serialize>(
    //     executor: &mut dyn rbatis::executor::Executor,
    //     table_column: &str,
    //     table_name: &str,
    //     column: &str,
    //     column_value: V,
    // ) -> std::result::Result<Vec<User>, rbatis::rbdc::Error> {
    //     let mut rb_arg_map = rbs::value::map::ValueMap::new();
    //     rb_arg_map.insert(
    //         "table_column".to_string().into(),
    //         rbs::to_value(table_column)?,
    //     );
    //     rb_arg_map.insert("table_name".to_string().into(), rbs::to_value(table_name)?);
    //     rb_arg_map.insert("column".to_string().into(), rbs::to_value(column)?);
    //     rb_arg_map.insert(
    //         "column_value".to_string().into(),
    //         rbs::to_value(column_value)?,
    //     );
    //     {}
    //     use rbatis::executor::RBatisRef;
    //     let driver_type = executor.rbatis_ref().driver_type()?;
    //     use rbatis::rbatis_codegen;
    //     pub fn do_py_sql(arg: &rbs::Value, _tag: char) -> (String, Vec<rbs::Value>) {
    //         use rbatis_codegen::ops::*;
    //         let mut sql = String::with_capacity(80usize);
    //         let mut args = Vec::with_capacity(20);
    //         args.push(rbs::to_value({ &arg["column_value"] }).unwrap_or_default());
    //         sql.push_str(
    //             "select ${table_column} from ${table_name}  where ${column} = ?"
    //                 .replacen("${table_column}", &{ &arg["table_column"] }.as_sql(), 1)
    //                 .replacen("${table_name}", &{ &arg["table_name"] }.as_sql(), 1)
    //                 .replacen("${column}", &{ &arg["column"] }.as_sql(), 1)
    //                 .as_str(),
    //         );
    //         return (sql, args);
    //     }
    //     let (mut sql, rb_args) = do_py_sql(&rbs::Value::Map(rb_arg_map), '?');
    //     use rbatis::executor::Executor;
    //     let r = executor.query(&sql, rb_args).await?;
    //     rbatis::decode::decode(r)
    // }
}
