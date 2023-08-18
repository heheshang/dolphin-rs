use crate::entity::user::ActiveModel as UserActiveModel;


pub async fn user_list() -> std::io::Result<i32, futures::future::err> {
    let user_list = UserActiveModel::find().all().await?;
    Ok(user_list.len() as i32)
}
