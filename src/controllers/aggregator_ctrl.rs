use crate::db::models::daily_rows_count;
use crate::db::models::daily_rows_count::DailyRowsCount;
use crate::utils::dates;
use crate::utils::error_message::ErrorMessage;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessData {
    processed: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    #[serde(rename = "type")]
    pub agg_type: String,
    pub start: String,
    pub end: String,
    pub parent_id_pre: String,
    pub grouping: String,
    // HashMap<parent_id, HashMap<date, HashMap<api_type, processed>>>
    pub data: HashMap<String, HashMap<String, HashMap<String, ProcessData>>>,
}

#[get("/aggregate?<start>&<end>&<agg_type>&<group_id>&<parent_id_pre>")]
pub async fn aggregator(
    start: String,
    end: String,
    agg_type: String,
    group_id: i64,
    parent_id_pre: String,
) -> Result<Json<UserData>, ErrorMessage> {
    rocket::debug!(
        "start: {}, end: {}, agg_type: {}, group_id: {}, parent_id_pre: {}",
        start.clone(),
        end.clone(),
        agg_type.clone(),
        group_id.clone(),
        parent_id_pre.clone()
    );

    if !dates::is_valid_date(&start) || !dates::is_valid_date(&end) {
        println!("Invalid date range {} to {}", start, end);
        return Err(ErrorMessage::BadRequest(
            Status::BadRequest,
            "Invalid date".to_string(),
        ));
    }

    let rows: Vec<DailyRowsCount> = daily_rows_count::select_by_date(
        start.clone(),
        end.clone(),
        group_id.clone(),
        agg_type.clone(),
        parent_id_pre.clone(),
    )
    .await;
    Ok(Json(UserData {
        agg_type,
        start,
        end,
        parent_id_pre,
        grouping: "none".to_string(),
        data: aggregate(rows),
    }))
}

fn aggregate(
    rows: Vec<DailyRowsCount>,
) -> HashMap<String, HashMap<String, HashMap<String, ProcessData>>> {
    let mut output: HashMap<String, HashMap<String, HashMap<String, ProcessData>>> = HashMap::new();
    for row in rows {
        output
            .entry(row.parent_id)
            .or_default()
            .entry(row.date)
            .or_default()
            .entry(row.api_type)
            .or_insert(ProcessData { processed: 0i64 })
            .processed += row.rows_count;
    }
    output
}
