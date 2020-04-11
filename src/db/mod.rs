use futures::executor::block_on;
use sqlx::{MySqlConnection, Pool};
use sqlx::mysql::MySqlPool;

lazy_static! {
   pub static ref POOL : Pool<MySqlConnection> = init_pool();
}
///初始化链接
fn init_pool() -> Pool<MySqlConnection> {
    block_on(
        MySqlPool::builder()
            .max_size(5)
            .build("mysql://root:admin_123@localhost:3306/shuashua_dev")
    ).expect("can't connect to mysql")
}

///获取数据库连接
pub fn get_pool() -> &'static Pool<MySqlConnection> {
    &(*POOL)
}
