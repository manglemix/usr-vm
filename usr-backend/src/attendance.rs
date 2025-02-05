use axum::{
    extract::State, http::StatusCode, routing::post, Form, Router
};
use sea_orm::{
    sea_query::Table, sqlx::types::chrono::Local, ActiveModelTrait, ActiveValue,
    ConnectionTrait, DatabaseConnection, Schema,
};
use serde::Deserialize;
use tracing::error;

use crate::{backup::backup_db, UsrState};

mod attendance;

#[derive(Deserialize)]
struct CheckIn {
    uid: String,
}

#[axum::debug_handler]
async fn add_attendance(
    State(state): State<&'static UsrState>,
    Form(CheckIn { uid }): Form<CheckIn>,
) -> (StatusCode, &'static str) {
    let Some(uid) = uid.strip_prefix('u').or_else(|| uid.strip_prefix('U')) else {
        return (StatusCode::BAD_REQUEST, "");
    };
    let Ok(uid) = uid.parse::<u32>() else {
        return (StatusCode::BAD_REQUEST, "");
    };
    let active_model = attendance::ActiveModel {
        uid: ActiveValue::Set(uid),
        date: ActiveValue::Set(Local::now().naive_local()),
    };

    match active_model.insert(&state.db).await {
        Ok(_) => {
            backup_db(state);
            (StatusCode::OK, "")
        }
        Err(e) => {
            error!("Failed to add attendance: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "")
        }
    }
}

pub fn router() -> Router<&'static UsrState> {
    Router::new()
        .route("/add/attendance", post(add_attendance))
}

pub async fn reset_tables(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(Table::drop().table(attendance::Entity).if_exists()))
        .await?;
    db.execute(builder.build(&schema.create_table_from_entity(attendance::Entity)))
        .await?;

    Ok(())
}
