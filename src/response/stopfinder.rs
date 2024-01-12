use serde::Deserialize;

use super::{Parameter, ResponseData};

response!(pub struct StopFinderResponseData {
    parameters: Vec<Parameter>,
    stop_finder: ResponseData as "stopFinder"
});

