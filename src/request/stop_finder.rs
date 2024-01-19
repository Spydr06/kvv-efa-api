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
    name: &'a str,
    typ: types::Type,
    limit: usize
}

impl<'a> Default for StopFinderRequestBuilder<'a> {
    fn default() -> Self {
        Self {
            name: "",
            typ: types::Type::Any,
            limit: 10
        }
    }
}

impl<'a> StopFinderRequestBuilder<'a> {
    const DEFAULT_OPTIONS: &'static str = "&coordOutputFormat=WGS84[dd.ddddd]";

    pub fn build(self) -> StopFinderRequest {
        let mut url = format!("{API_ENDPOINT}/{}?outputFormat=JSON{}", StopFinderRequest::REQUEST_TYPE, Self::DEFAULT_OPTIONS);
        url.push_str(&format!("&name_sf={}", self.name));
        url.push_str(&format!("&type_sf={}", self.typ));
        url.push_str(&format!("&limit={}", self.limit));

        StopFinderRequest(url)
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    pub fn typ(mut self, typ: types::Type) -> Self {
        self.typ = typ;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}
