use crate::response::StopFinderResponseData;

use super::{Request, types, API_ENDPOINT};

#[derive(Debug, Clone)]
pub struct StopFinderRequest(String);

impl Request for StopFinderRequest {
    type Builder = StopFinderRequestBuilder<'static>;
    type Response = StopFinderResponseData;

    const REQUEST_TYPE: &'static str = "XSLT_STOPFINDER_REQUEST";

    fn url(&self) -> &String {
        &self.0
    }

    fn into_url(self) -> String {
        self.0.into()
    }

    async fn get(self) -> Result<Self::Response, reqwest::Error> {
        let response = reqwest::get(self.url()).await?;
        response.json().await
    }
}

pub struct StopFinderRequestBuilder<'a> {
    name_sf: &'a str,
    type_sf: types::Type,
    limit: usize
}

impl<'a> Default for StopFinderRequestBuilder<'a> {
    fn default() -> Self {
        Self {
            name_sf: "",
            type_sf: types::Type::Any,
            limit: 10
        }
    }
}

impl<'a> StopFinderRequestBuilder<'a> {
    const DEFAULT_OPTIONS: &'static str = "&coordOutputFormat=WGS84[dd.ddddd]";

    pub fn build(self) -> StopFinderRequest {
        let mut url = format!("{API_ENDPOINT}/{}?outputFormat=JSON{}", StopFinderRequest::REQUEST_TYPE, Self::DEFAULT_OPTIONS);
        url.push_str(&format!("&name_sf={}", self.name_sf));
        url.push_str(&format!("&type_sf={}", self.type_sf));
        url.push_str(&format!("&limit={}", self.limit));

        StopFinderRequest(url)
    }

    pub fn name_sf(mut self, name: &'a str) -> Self {
        self.name_sf = name;
        self
    }

    pub fn type_sf(mut self, type_sf: types::Type) -> Self {
        self.type_sf = type_sf;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}
