use mysql::{prelude::Queryable, Row};
use crate::db::db_conn::get_conn;
use crate::utils::dates;


pub struct DailyRowsCount {
    pub parent_id: String,
    pub date: String,
    pub api_type: String,
    pub rows_count: i64,
}

pub async fn select_by_date(start_date: String, end_date: String, group_id: i64, agg_type: String, parent_id_part: String) -> Vec<DailyRowsCount> {
    let query: &str;
    let s_date: String;
    let e_date: String;
    let e_parent_id: String = format!("{}%", parent_id_part);
    if agg_type == "daily" {
        query = "SELECT parent_id, `date`, api_type, SUM(rows_count) rows_count FROM (SELECT CONCAT(year, '-', month, '-', day) `date`, COALESCE(api_type, '') api_type, COALESCE(parent_id, '') parent_id, rows_count FROM daily_rows_count WHERE CONCAT(year, '-', month, '-', day) >= ? AND CONCAT(year, '-', month, '-', day) <= ? AND COALESCE(api_type, '') != '' AND group_id=?  AND parent_id LIKE ? ) a GROUP BY parent_id, date, api_type";
        s_date = start_date;
        e_date = end_date;
    } else {
        query = "\
        SELECT \
            parent_id, \
            `date`, \
            api_type, \
            SUM(rows_count) rows_count \
        FROM (\
            SELECT \
                CONCAT(year, '-', month) `date`, \
                COALESCE(api_type, '') api_type, \
                COALESCE(parent_id, '') parent_id, \
                rows_count \
            FROM \
                daily_rows_count \
            WHERE \
                    CONCAT(year, '-', month, '-', day) >= ? \
                AND CONCAT(year, '-', month, '-', day) <= ? \
                AND COALESCE(api_type, '') != '' \
                AND group_id=? \
                AND parent_id LIKE ? ) a GROUP BY parent_id, date, api_type";
        s_date = dates::first_date_of_month(start_date);
        e_date = dates::last_date_of_month(end_date);
    }

    let mut pool_conn = get_conn().await.get().unwrap().get_conn().unwrap();
    let rows: Vec<Row> = pool_conn.exec(query, (s_date, e_date, group_id, e_parent_id)).unwrap();
    let mut ret: Vec<DailyRowsCount> = Vec::new();
    for row in rows {
        ret.push(DailyRowsCount {
            parent_id: row.get("parent_id").unwrap(),
            date: row.get("date").unwrap(),
            api_type: row.get("api_type").unwrap(),
            rows_count: row.get("rows_count").unwrap(),
        });
    }
    ret
}
