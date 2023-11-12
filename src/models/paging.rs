use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
#[serde(transparent)]
pub struct PagingCursor(String);

impl std::fmt::Display for PagingCursor {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Serialize, Debug, Eq, PartialEq, Default, Clone)]
pub struct Paging {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<PagingCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,
}

pub trait Pageable {
    fn start_from(
        self,
        starting_point: Option<PagingCursor>,
    ) -> Self;
}
