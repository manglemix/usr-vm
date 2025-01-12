use std::
    sync::Arc
;

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use discord_webhook2::message::Message;
use sea_orm::{
    prelude::Decimal,
    sea_query::Table,
    sqlx::types::chrono::Local,
    ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, Schema,
};
use serde::Deserialize;
use tracing::error;

use crate::{scheduler, UsrState};

mod order;

#[derive(Deserialize)]
pub struct PendingOrder {
    pub name: String,
    pub count: u32,
    pub unit_cost: Decimal,
    pub store_in: Option<String>,
    pub team: scheduler::Team,
    pub reason: String
}

#[axum::debug_handler]
async fn new_order(
    State(state): State<Arc<UsrState>>,
    Json(pending_order): Json<PendingOrder>,
) -> (StatusCode, &'static str) {
    let webhook_msg = format!(
        ">>> **New Order!**\n**Name:** {}\n**Count:** {}\n**Unit Cost:** ${}\n**Subtotal:** ${}\n**Team:** {}\n**Reason:** {}",
        pending_order.name,
        pending_order.count,
        pending_order.unit_cost,
        Decimal::from(pending_order.count) * pending_order.unit_cost,
        pending_order.team,
        pending_order.reason
    );
    let active_model = order::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(pending_order.name),
        date: ActiveValue::Set(Local::now().naive_local()),
        status: ActiveValue::Set(order::Status::New),
        count: ActiveValue::Set(pending_order.count),
        unit_cost: ActiveValue::Set(pending_order.unit_cost),
        store_in: ActiveValue::Set(pending_order.store_in),
        team: ActiveValue::Set(pending_order.team),
        reason: ActiveValue::Set(pending_order.reason),
    };
    if let Err(e) = active_model.insert(&state.db).await {
        error!("Failed to create new order: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "")
    } else {
        tokio::spawn(async move {
            if let Err(e) = state
                .new_orders_webhook
                .send(&Message::new(|message| message.content(webhook_msg)))
                .await
            {
                error!("Failed to trigger new-order webhook: {e}");
            }
        });
        (StatusCode::OK, "")
    }
}

pub fn router() -> Router<Arc<UsrState>> {
    Router::new().route("/new/order", post(new_order))
}

pub async fn reset_tables(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(Table::drop().table(order::Entity).if_exists()))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(order::Entity)))
        .await?;

    Ok(())
}
