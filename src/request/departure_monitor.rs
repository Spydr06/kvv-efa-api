use super::{Request, API_ENDPOINT, types};
use crate::response::DepartureMonitorResponseData;

#[derive(Clone, Debug)]
pub struct DepartureMonitorRequest(String);

impl Request for DepartureMonitorRequest {
    type Builder = DepartureMonitorRequestBuilder;
    type Response = DepartureMonitorResponseData;
    const REQUEST_TYPE: &'static str = "XSLT_DM_REQUEST";

    fn url(&self) -> &String {
        &self.0
    }

    fn into_url(self) -> String {
        self.0
    }
    
    async fn get(self) -> Result<Self::Response, reqwest::Error> {
        let response = reqwest::get(self.url()).await?;
        response.json().await
    }
}

pub struct DepartureMonitorRequestBuilder {
    name: types::StationId,
    typ: types::Type,
    use_realtime: bool,
    limit: usize
}

impl Default for DepartureMonitorRequestBuilder {
    fn default() -> Self {
        Self {
            name: 0,
            typ: types::Type::Stop,
            use_realtime: true,
            limit: 10
        }
    }
}

impl DepartureMonitorRequestBuilder {
    const DEFAULT_OPTIONS: &'static str = "&coordOutputFormat=WGS84[dd.ddddd]&depType=stopEvents&locationServerActive=1&mode=direct&useOnlyStops=1";

    pub fn build(self) -> DepartureMonitorRequest {
        let mut url = format!("{API_ENDPOINT}/{}?outputFormat=JSON{}", DepartureMonitorRequest::REQUEST_TYPE, Self::DEFAULT_OPTIONS);
        url.push_str(&format!("&name_dm={}", self.name));
        url.push_str(&format!("&type_dm={}", self.typ));
        url.push_str(&format!("&useRealtime={}", self.use_realtime as i32));
        url.push_str(&format!("&limit={}", self.limit));

        DepartureMonitorRequest(url)
    }

    pub fn name(mut self, name: types::StationId) -> Self {
        self.name = name;
        self
    }
    
    pub fn typ(mut self, typ: types::Type) -> Self {
        self.typ = typ;
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.use_realtime = realtime;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

