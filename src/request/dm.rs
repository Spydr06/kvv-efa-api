use super::{Request, API_ENDPOINT, types};
use crate::response::DMResponseData;

#[derive(Clone, Debug)]
pub struct DMRequest(String);

impl Request for DMRequest {
    type Builder = DMRequestBuilder;
    type Response = DMResponseData;
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

pub struct DMRequestBuilder {
    name_dm: types::NameDM,
    type_dm: types::Type,
    use_realtime: bool,
    limit: usize
}

impl Default for DMRequestBuilder {
    fn default() -> Self {
        Self {
            name_dm: 0,
            type_dm: types::Type::Stop,
            use_realtime: true,
            limit: 10
        }
    }
}

impl DMRequestBuilder {
    const DEFAULT_OPTIONS: &'static str = "&coordOutputFormat=WGS84[dd.ddddd]&depType=stopEvents&locationServerActive=1&mode=direct&useOnlyStops=1";

    pub fn build(self) -> DMRequest {
        let mut url = format!("{API_ENDPOINT}/{}?outputFormat=JSON{}", DMRequest::REQUEST_TYPE, Self::DEFAULT_OPTIONS);
        url.push_str(&format!("&name_dm={}", self.name_dm));
        url.push_str(&format!("&type_dm={}", self.type_dm));
        url.push_str(&format!("&useRealtime={}", self.use_realtime as i32));
        url.push_str(&format!("&limit={}", self.limit));

        DMRequest(url)
    }

    pub fn name_dm(mut self, name_dm: types::NameDM) -> Self {
        self.name_dm = name_dm;
        self
    }
    
    pub fn type_dm(mut self, type_dm: types::Type) -> Self {
        self.type_dm = type_dm;
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

