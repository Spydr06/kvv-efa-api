pub mod response;
pub mod request;

use std::{ops::{Deref, DerefMut}, fmt::Debug, marker::PhantomData};

use serde::{Deserialize, de::Visitor};

#[derive(Default, Clone)]
pub struct ApiVec<T>(Vec<T>);

impl<T> ApiVec<T> {
    pub fn take(self) -> Vec<T> {
        self.0
    }
}

impl<T> Deref for ApiVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ApiVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Debug for ApiVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for ApiVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_any(ApiVecVisitor(PhantomData))
    }
}

struct ApiVecVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for ApiVecVisitor<T> {
    type Value = ApiVec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected sequence")        
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((_, value)) = access.next_entry::<&'de str, T>()? {
            vec.push(value);
        } 

        Ok(ApiVec(vec))
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {
        let mut vec = Vec::with_capacity(access.size_hint().unwrap_or(0));

        while let Some(value) = access.next_element()? {
            vec.push(value);
        }

        Ok(ApiVec(vec))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(ApiVec(vec![]))
    }
}

