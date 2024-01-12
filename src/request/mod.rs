pub mod dm;
pub mod stopfinder;

pub use dm::*;
pub use stopfinder::*;

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

    fn get(self) -> impl Future<Output = Result<Self::Response, reqwest::Error>>;
}

pub mod types {
    use std::fmt::Display;

    pub type NameDM = i32;
   
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

