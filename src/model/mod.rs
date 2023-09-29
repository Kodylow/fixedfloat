// region:    --- Modules

mod base;
mod store;
pub mod task;
pub mod user;

use crate::model::store::{new_db_pool, Db};
use anyhow::Result;

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	/// Constructor
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;

		Ok(ModelManager { db })
	}

	/// Returns the sqlx db pool reference.
	/// (Only for the model layer)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
