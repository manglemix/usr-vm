use std::{collections::{hash_map::Entry, HashMap, HashSet}, sync::Arc};

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post}, Json, Router};
use sea_orm::{sea_query::Table, ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect, Schema, TransactionTrait};
use serde::{Deserialize, Serialize};
use tracing::error;

mod availability;
mod team;

#[derive(Deserialize)]
struct PendingSchedule {
    name: String,
    times: Box<[u16]>,
}

#[axum::debug_handler]
async fn add_schedule(State(db): State<Arc<DatabaseConnection>>, Json(pending_schedule): Json<PendingSchedule>) -> (StatusCode, &'static str) {
    let result = db.transaction(|tx| Box::pin(async move {
        for time in pending_schedule.times {
            let active_model = availability::ActiveModel {
                name: ActiveValue::Set(pending_schedule.name.clone()),
                time: ActiveValue::Set(time),
            };
            active_model.insert(tx).await?;
        }
        Result::<_, sea_orm::DbErr>::Ok(())
    })).await;
    
    if let Err(e) = result {
        error!("Failed to insert schedule: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "")
    } else {
        (StatusCode::OK, "")
    }
}

#[axum::debug_handler]
async fn del_schedule(State(db): State<Arc<DatabaseConnection>>, Json(pending_schedule): Json<PendingSchedule>) -> (StatusCode, &'static str) {
    let result = db.transaction(|tx| Box::pin(async move {
        for time in pending_schedule.times {
            let active_model = availability::ActiveModel {
                name: ActiveValue::Set(pending_schedule.name.clone()),
                time: ActiveValue::Set(time),
            };
            active_model.delete(tx).await?;
        }
        Result::<_, sea_orm::DbErr>::Ok(())
    })).await;
    
    if let Err(e) = result {
        error!("Failed to delete schedule: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "")
    } else {
        (StatusCode::OK, "")
    }
}

#[derive(Deserialize)]
struct SetTeam {
    name: String,
    teams: HashSet<team::Team>,
}

#[axum::debug_handler]
async fn set_team(State(db): State<Arc<DatabaseConnection>>, Json(set_team): Json<SetTeam>) -> (StatusCode, &'static str) {
    // let result = team::Entity::find().filter(team::Column::Name.eq(set_team.name.clone())).column(team::Column::Team).all(&*db).await;
    // let to_delete = match result {
    //     Ok(x) => x,
    //     Err(e) => {
    //         error!("Failed to find schedule: {e}");
    //         return (StatusCode::INTERNAL_SERVER_ERROR, "");
    //     }
    // };
    
    let result = db.transaction(|tx| Box::pin(async move {
        team::Entity::delete_many().filter(team::Column::Name.eq(set_team.name.clone())).exec(tx).await?;
        // for model in to_delete {
        //     model.delete(tx).await?;
        // }
        for team in set_team.teams {
            let active_model = team::ActiveModel {
                name: ActiveValue::Set(set_team.name.clone()),
                team: ActiveValue::Set(team)
            };
            active_model.insert(tx).await?;
        }
        Result::<_, sea_orm::DbErr>::Ok(())
    })).await;
    
    if let Err(e) = result {
        error!("Failed to set team: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "")
    } else {
        (StatusCode::OK, "")
    }
}

#[derive(Serialize)]
struct Schedule {
    availabilities: Box<[Vec<String>]>,
    teams: HashMap<team::Team, Vec<String>>
}

#[axum::debug_handler]
async fn get_schedule(State(db): State<Arc<DatabaseConnection>>) -> Response {
    let (availabilities, teams) = tokio::join!(
        availability::Entity::find().all(&*db),
        team::Entity::find().all(&*db),
    );

    let availabilities = match availabilities {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to enumerate availabilities: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "").into_response();
        }
    };

    let teams = match teams {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to enumerate teams: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "").into_response();
        }
    };

    Json(Schedule {
        availabilities: {
            let mut out: Box<[Vec<String>]> = std::iter::from_fn(|| Some(Vec::default())).take(7 * 24 * 4).collect();
            for model in availabilities {
                out[model.time as usize].push(model.name);
            }
            out
        },
        teams: {
            let mut out = HashMap::<team::Team, Vec<String>>::new();
            for model in teams {
                match out.entry(model.team) {
                    Entry::Occupied(mut occupied_entry) => occupied_entry.get_mut().push(model.name),
                    Entry::Vacant(vacant_entry) => vacant_entry.insert(vec![]).push(model.name),
                }
            }
            out
        }
    }).into_response()
}

pub fn router() -> Router<Arc<DatabaseConnection>> {
    Router::new()
    .route("/add/schedule", post(add_schedule))
    .route("/del/schedule", delete(del_schedule))
    .route("/get/schedule", get(get_schedule))
    .route("/set/team", post(set_team))
}

pub async fn reset_tables(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(Table::drop().table(team::Entity).if_exists())).await?;
    db.execute(builder.build(Table::drop().table(availability::Entity).if_exists())).await?;
    db.execute(builder.build(&schema.create_table_from_entity(team::Entity))).await?;
    db.execute(builder.build(&schema.create_table_from_entity(availability::Entity))).await?;

    Ok(())
}