pub mod departure_monitor;
pub mod stop_finder;
pub mod generic;

pub use departure_monitor::{DepartureMonitorRequest, DepartureMonitorRequestBuilder};
pub use stop_finder::{StopFinderRequest, StopFinderRequestBuilder};
//pub use generic::*;

#[cfg(feature = "reqwest")]
use std::future::Future;

const API_ENDPOINT: &'static str = "https://projekte.kvv-efa.de/sl3-alone/";

pub trait Request: Sized + Clone {
    type Builder: Default;
    type Response;

    const REQUEST_TYPE: &'static str;

    fn builder() -> Self::Builder {
        Self::Builder::default()
    }

    fn url(&self) -> &String;
    fn into_url(self) -> String;

    #[cfg(feature = "reqwest")]
    fn get(self) -> impl Future<Output = Result<Self::Response, reqwest::Error>>;
}

pub mod types {
    use std::fmt::Display;

    pub type StationId = i32;
   
    #[derive(Clone, Copy, Debug)]
    pub enum Type {
        Any,
        Stop,
        // TODO: find out other options
    }

    impl Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Stop => write!(f, "stop"),
                Self::Any => write!(f, "any")
            }
        }
    }
}

