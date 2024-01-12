use serde::Deserialize;

use crate::ApiVec;

use super::{Parameter, DateTime, Date, ServingLines, Departure, ResponseData};

response!(pub struct DMResponseData {
    parameters: Vec<Parameter>,
    dm: ResponseData,
    arr: ResponseData,   
    date_time: DateTime as "dateTime",
    date_range: Vec<Date> as "dateRange",
    serving_lines: ServingLines as "servingLines",
    departure_list: ApiVec<Departure> as "departureList",
});


