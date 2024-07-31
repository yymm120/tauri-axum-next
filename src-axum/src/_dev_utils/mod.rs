mod dev_db;
use crate::model::ModelManager;

use tokio::sync::OnceCell;

/// Initialize environment for local development.
#[allow(unused)]
pub async fn init_dev() {
	static INIT: OnceCell<()> = OnceCell::const_new();
	INIT.get_or_init(|| async {
		dev_db::init_dev_db().await.unwrap();
	})
	.await;
}


/// Initialize test environment. (now same with dev)
#[allow(unused)]
pub async fn init_test() -> ModelManager {
	static INIT: OnceCell<ModelManager> = OnceCell::const_new();
	let mm = INIT
		.get_or_init(|| async {
			init_dev().await;
			ModelManager::new_postgres().await.unwrap()
		})
		.await;
	mm.clone()
}