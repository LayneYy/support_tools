use sqlx::mysql::MySqlQueryAs;

use support_tools::db;

#[tokio::main]
async fn main() {
    let users: Result<Vec<User>, _> = sqlx::query_as::<_, User>("select user_id,mobile,agent_level from user")
        .fetch_all(db::get_pool())
        .await;

    match users {
        Ok(users) => {
            users.iter().for_each(|u| {
                println!("{:?}", u);
            });
        }
        Err(why) => {
            println!("something wrong {:?}", why);
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
struct User {
    //用户ID
    user_id: i64,
    //手机号
    mobile: String,
    //用户级别
    agent_level: i32,
}