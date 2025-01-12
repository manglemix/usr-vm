use std::
    sync::Arc
;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use discord_webhook2::message::Message;
use sea_orm::{
    prelude::Decimal, sea_query::Table, sqlx::types::chrono::Local, ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, EntityTrait, Schema
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

#[derive(Deserialize)]
pub struct ChangeOrder {
    pub id: u32,
    pub name: String,
    pub count: u32,
    pub unit_cost: Decimal,
    pub store_in: Option<String>,
    pub team: scheduler::Team,
    pub reason: String
}

#[axum::debug_handler]
async fn change_order(
    State(state): State<Arc<UsrState>>,
    Json(change_order): Json<ChangeOrder>,
) -> (StatusCode, &'static str) {
    match order::Entity::find_by_id(change_order.id).one(&state.db).await {
        Ok(Some(model)) => {
            if model.status != order::Status::New {
                return (StatusCode::BAD_REQUEST, "Order has already been processed");
            }
        }
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, "Order not found");
        }
        Err(e) => {
            error!("Failed to find order: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "");
        }
    }
    let webhook_msg = format!(
        ">>> ***Order Changed***\n**Name:** {}\n**Count:** {}\n**Unit Cost:** ${}\n**Subtotal:** ${}\n**Team:** {}\n**Reason:** {}",
        change_order.name,
        change_order.count,
        change_order.unit_cost,
        Decimal::from(change_order.count) * change_order.unit_cost,
        change_order.team,
        change_order.reason
    );
    let active_model = order::ActiveModel {
        id: ActiveValue::Unchanged(change_order.id),
        name: ActiveValue::Set(change_order.name),
        date: ActiveValue::NotSet,
        status: ActiveValue::NotSet,
        count: ActiveValue::Set(change_order.count),
        unit_cost: ActiveValue::Set(change_order.unit_cost),
        store_in: ActiveValue::Set(change_order.store_in),
        team: ActiveValue::Set(change_order.team),
        reason: ActiveValue::Set(change_order.reason),
    };
    if let Err(e) = active_model.update(&state.db).await {
        error!("Failed to change order: {e}");
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

#[derive(Deserialize)]
struct DeleteOrder {
    id: u32
}

#[axum::debug_handler]
async fn cancel_order(
    State(state): State<Arc<UsrState>>,
    Json(DeleteOrder { id }): Json<DeleteOrder>,
) -> (StatusCode, &'static str) {
    let webhook_msg;
    match order::Entity::find_by_id(id).one(&state.db).await {
        Ok(Some(model)) => {
            if model.status != order::Status::New {
                return (StatusCode::BAD_REQUEST, "Order has already been processed");
            }
            webhook_msg = format!(
                ">>> ***Order Cancelled***\n**Name:** {}\n**Count:** {}\n**Team:** {}",
                model.name,
                model.count,
                model.team,
            );
        }
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, "Order not found");
        }
        Err(e) => {
            error!("Failed to find order: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "");
        }
    }

    if let Err(e) = order::Entity::delete_by_id(id).exec(&state.db).await {
        error!("Failed to delete order: {e}");
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

#[derive(Deserialize)]
pub struct UpdateOrder {
    pub id: u32,
    pub status: order::Status
}

#[axum::debug_handler]
async fn update_order(
    State(state): State<Arc<UsrState>>,
    Json(update_order): Json<UpdateOrder>,
) -> (StatusCode, &'static str) {
    let webhook_msg;
    match order::Entity::find_by_id(update_order.id).one(&state.db).await {
        Ok(Some(model)) => {
            if model.status == order::Status::InStorage {
                return (StatusCode::BAD_REQUEST, "Order is already in storage");
            }
            if model.status == update_order.status {
                return (StatusCode::BAD_REQUEST, "Order is already in that state");
            }
            webhook_msg = format!(
                ">>> **Order Update!**\n**Name:** {}\n**Team:** {}\n**Status:** {}",
                model.name,
                model.team,
                update_order.status
            );
        }
        Ok(None) => {
            return (StatusCode::BAD_REQUEST, "Order not found");
        }
        Err(e) => {
            error!("Failed to find order: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "");
        }
    }
    
    let active_model = order::ActiveModel {
        id: ActiveValue::Unchanged(update_order.id),
        name: ActiveValue::NotSet,
        date: ActiveValue::NotSet,
        status: ActiveValue::Set(update_order.status),
        count: ActiveValue::NotSet,
        unit_cost: ActiveValue::NotSet,
        store_in: ActiveValue::NotSet,
        team: ActiveValue::NotSet,
        reason: ActiveValue::NotSet,
    };
    if let Err(e) = active_model.update(&state.db).await {
        error!("Failed to update order: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "")
    } else {
        tokio::spawn(async move {
            if let Err(e) = state
                .order_updates_webhook
                .send(&Message::new(|message| message.content(webhook_msg)))
                .await
            {
                error!("Failed to trigger order-updates webhook: {e}");
            }
        });
        (StatusCode::OK, "")
    }
}

pub fn router() -> Router<Arc<UsrState>> {
    Router::new()
        .route("/new/order", post(new_order))
        .route("/change/order", post(change_order))
        .route("/del/order", delete(cancel_order))
        .route("/update/order", post(update_order))
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
