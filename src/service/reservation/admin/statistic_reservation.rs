use core::option::Option::Some;
use std::sync::Arc;

use axum::{extract::State, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::model::{database::TypeRoom, error::AppError, response::GeneralResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonthlyTypeRoomStatistic {
    month_name: String,
    month_number: u32,
    type_room: Vec<TypeRoomStatistic>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarterlyTypeRoomStatistic {
    quarter: String,
    type_room: Vec<TypeRoomStatistic>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeRoomStatistic {
    type_room_id: u64,
    type_room_title: Option<String>,
    total_booked: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatisticInput {
    x_axis: StatisticXAxis,
    y_axis: StatisticYAxis,
    type_room_ids: Vec<u64>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum StatisticXAxis {
    Month = 1,
    Quarter = 2,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum StatisticYAxis {
    Reservation = 1,
    Revenue = 2,
}

pub async fn statistic_reservation(
    State(db): State<Arc<Postgrest>>,
    Json(input): Json<StatisticInput>,
) -> Result<GeneralResponse, AppError> {
    if input.y_axis == StatisticYAxis::Reservation {
        let type_room_ids_str: Vec<String> =
            input.type_room_ids.iter().map(|x| x.to_string()).collect();
        let type_rooms_template: Vec<TypeRoomStatistic> = db
            .from("type_room")
            .select("*")
            .in_("id", type_room_ids_str)
            .execute()
            .await?
            .json::<Vec<TypeRoom>>()
            .await?
            .into_iter()
            .map(|type_room| TypeRoomStatistic {
                type_room_id: type_room.id.unwrap_or_default(),
                type_room_title: type_room.title,
                total_booked: 0,
            })
            .collect();
        let mut template = Vec::new();
        for id in input.type_room_ids {
            let type_room = type_rooms_template.iter().find(|x| x.type_room_id == id);
            if let Some(tr) = type_room {
                template.push(tr.clone());
            }
        }

        if input.x_axis == StatisticXAxis::Month {
            let result = monthly_type_room_statistic(&db, template).await?;
            GeneralResponse::ok_with_result(result)
        } else {
            let result = quarterly_type_room_statistic(&db, template).await?;
            GeneralResponse::ok_with_result(result)
        }
    } else {
        if input.x_axis == StatisticXAxis::Month {
            let result: Value = db
                .rpc("get_monthly_revenue", "{}")
                .execute()
                .await?
                .json()
                .await?;
            GeneralResponse::ok_with_result(result)
        } else {
            let result: Value = db
                .rpc("get_quarterly_revenue", "{}")
                .execute()
                .await?
                .json()
                .await?;
            GeneralResponse::ok_with_result(result)
        }
    }
}

async fn monthly_type_room_statistic(
    db: &Arc<Postgrest>,
    type_rooms_template: Vec<TypeRoomStatistic>,
) -> Result<Vec<MonthlyTypeRoomStatistic>, AppError> {
    let query = db
        .rpc("get_monthly_type_room_statistic", "{}")
        .execute()
        .await?;
    let mut monthly_type_room_statistic: Vec<MonthlyTypeRoomStatistic> = query.json().await?;

    for statistic in monthly_type_room_statistic.iter_mut() {
        let mut type_rooms = type_rooms_template.clone();
        type_rooms.iter_mut().for_each(|type_room| {
            let find_type_room = statistic
                .type_room
                .iter()
                .find(|x| x.type_room_id == type_room.type_room_id);
            if let Some(founded_type_room) = find_type_room {
                type_room.total_booked = founded_type_room.total_booked;
            }
        });
        statistic.type_room = type_rooms;
    }
    Ok(monthly_type_room_statistic)
}

async fn quarterly_type_room_statistic(
    db: &Arc<Postgrest>,
    type_rooms_template: Vec<TypeRoomStatistic>,
) -> Result<Vec<QuarterlyTypeRoomStatistic>, AppError> {
    let query = db
        .rpc("get_quarterly_type_room_statistic", "{}")
        .execute()
        .await?;
    let mut quarterly_statistic: Vec<QuarterlyTypeRoomStatistic> = query.json().await?;
    for statistic in quarterly_statistic.iter_mut() {
        let mut type_rooms = type_rooms_template.clone();
        type_rooms.iter_mut().for_each(|type_room| {
            let find_type_room = statistic
                .type_room
                .iter()
                .find(|x| x.type_room_id == type_room.type_room_id);
            if let Some(founded_type_room) = find_type_room {
                type_room.total_booked = founded_type_room.total_booked;
            }
        });
        statistic.type_room = type_rooms;
    }
    Ok(quarterly_statistic)
}
