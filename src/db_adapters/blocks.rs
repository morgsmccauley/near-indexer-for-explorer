use actix_diesel::dsl::AsyncRunQueryDsl;

use diesel::PgConnection;

use crate::models;
use crate::schema;

/// Saves block to database
pub(crate) async fn store_block(
    pool: &actix_diesel::Database<PgConnection>,
    block: &near_lake_framework::near_indexer_primitives::views::BlockView,
) -> anyhow::Result<()> {
    let block_model = models::blocks::Block::from(block);

    crate::await_retry_or_panic!(
        diesel::insert_into(schema::blocks::table)
            .values(block_model.clone())
            .on_conflict_do_nothing()
            .execute_async(pool),
        10,
        "Block was stored to database".to_string(),
        &block_model
    );
    Ok(())
}
