use std::{collections::{hash_map::Entry, HashMap, HashSet}, sync::Arc};

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, post}, Json, Router};
use sea_orm::{sea_query::Table, ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, Schema, TransactionTrait};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::UsrState;

mod availability;
mod team;

pub use team::Team;

#[derive(Deserialize)]
struct PendingSchedule {
    name: String,
    times: Box<[u16]>,
}

#[axum::debug_handler]
async fn add_schedule(State(state): State<Arc<UsrState>>, Json(pending_schedule): Json<PendingSchedule>) -> (StatusCode, &'static str) {
    let result = state.db.transaction(|tx| Box::pin(async move {
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
async fn del_schedule(State(state): State<Arc<UsrState>>, Json(pending_schedule): Json<PendingSchedule>) -> (StatusCode, &'static str) {
    let result = state.db.transaction(|tx| Box::pin(async move {
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
async fn set_teams(State(state): State<Arc<UsrState>>, Json(set_team): Json<SetTeam>) -> (StatusCode, &'static str) {
    let result = state.db.transaction(|tx| Box::pin(async move {
        team::Entity::delete_many().filter(team::Column::Name.eq(set_team.name.clone())).exec(tx).await?;
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
        error!("Failed to set teams: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "")
    } else {
        (StatusCode::OK, "")
    }
}

// #[axum::debug_handler]
// async fn get_teams(State(db): State<Arc<DatabaseConnection>>, Path(name): Path<String>) -> Response {
//     let result = team::Entity::find().filter(team::Column::Name.eq(name)).all(&*db).await;
    
//     match result {
//         Ok(models) => {
//             Json(models.into_iter().map(|model| model.team).collect::<Box<[team::Team]>>()).into_response()
//         }
//         Err(e) => {
//             error!("Failed to get teams: {e}");
//             (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
//         }
//     }
// }

#[derive(Serialize)]
struct Schedule {
    availabilities: Box<[Vec<String>]>,
    teams: HashMap<team::Team, Vec<String>>
}

#[axum::debug_handler]
async fn get_schedule(State(state): State<Arc<UsrState>>) -> Response {
    let (availabilities, teams) = tokio::join!(
        availability::Entity::find().all(&state.db),
        team::Entity::find().all(&state.db),
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
            let mut out: Box<[Vec<String>]> = std::iter::from_fn(|| Some(Vec::default())).take(7 * 8 * 4).collect();
            for model in availabilities {
                let hour_of_day = model.time % 24 * 4;
                if hour_of_day < 9 * 4 || hour_of_day >= 17 * 4 {
                    continue;
                }
                out[model.time as usize - 9 * 4].push(model.name);
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

pub fn router() -> Router<Arc<UsrState>> {
    Router::new()
    .route("/add/schedule", post(add_schedule))
    .route("/del/schedule", delete(del_schedule))
    .route("/get/schedule", get(get_schedule))
    .route("/set/team", post(set_teams))
    // .route("/get/team/:name", get(get_teams))
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