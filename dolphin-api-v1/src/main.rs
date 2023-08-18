// use dao::entity::t_ds_user::Entity as User;

fn main() {
    // let db = DbConn::new();

    // let mut users = User::find().all().await.unwrap();
    // log4rs::init_file("./log4rs.yaml", Default::default()).expect("init log4rs.yaml failed");

    api::main();
}
