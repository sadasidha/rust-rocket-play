use chrono::{Datelike, NaiveDate};


pub fn last_date_of_month(date: String) -> String {
    let nd = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
    if nd.month() == 12 {
        NaiveDate::from_ymd_opt(nd.year() + 1, 1, 1).unwrap().pred_opt().unwrap().format("%Y-%m-%d").to_string()
    } else {
        NaiveDate::from_ymd_opt(nd.year(), nd.month()+1, 1).unwrap().pred_opt().unwrap().format("%Y-%m-%d").to_string()
    }

}

pub fn first_date_of_month(date: String) -> String {
    let nd = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
    NaiveDate::from_ymd_opt(nd.year(), nd.month(), 1).unwrap().pred_opt().unwrap().format("%Y-%m-%d").to_string()
}

pub fn is_valid_date(date: &String) -> bool {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(_) => { true}
        Err(_) => { false }
    }
}


pub fn extract_date(date: String) -> (i32, i32, i32) {
    let r = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d");
    let d = r.unwrap();

    (d.year(),d.month().try_into().unwrap(),d.day().try_into().unwrap())

}

#[cfg(test)]
mod test_dates {
    use crate::utils::dates::{is_valid_date, last_date_of_month};

    #[test]
    fn test_last_date_of_month() {
        assert_eq!(last_date_of_month(String::from("2024-12-31")), "2024-12-31".to_string());
        assert_eq!(last_date_of_month(String::from("2024-01-01")), "2024-01-31".to_string());
        assert_eq!(last_date_of_month(String::from("2019-10-10")), "2019-10-31".to_string());
        assert_eq!(last_date_of_month(String::from("2019-09-10")), "2019-09-30".to_string());
        assert_eq!(last_date_of_month(String::from("2019-02-01")), "2019-02-28".to_string());
        assert_eq!(last_date_of_month(String::from("2024-02-01")), "2024-02-29".to_string());
    }

    #[test]
    fn test_is_valid_date() {
        assert_eq!(true, is_valid_date(&String::from("2024-01-01")));
        assert_eq!(false, is_valid_date(&String::from("2024-01")));
        assert_eq!(false, is_valid_date(&String::from("01-2021")));
    }
}