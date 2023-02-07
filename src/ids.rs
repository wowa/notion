use std::fmt::Display;
use std::fmt::Error;
use uuid::Uuid;

pub trait Identifier: Display {
    fn value(&self) -> String;
}
/// Meant to be a helpful trait allowing anything that can be
/// identified by the type specified in `ById`.
pub trait AsIdentifier<ById: Identifier> {
    fn as_id(&self) -> &ById;
}

impl<T> AsIdentifier<T> for T
where
    T: Identifier,
{
    fn as_id(&self) -> &T {
        self
    }
}

impl<T> AsIdentifier<T> for &T
where
    T: Identifier,
{
    fn as_id(&self) -> &T {
        self
    }
}
macro_rules! uuid_identifer {
    ($name:ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl Identifier for $name {
            fn value(&self) -> String {
                self.0.to_string()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(Uuid::parse_str(s).unwrap()))
            }
        }
    };
}

macro_rules! string_identifer {
    ($name:ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
        #[serde(transparent)]
        pub struct $name(String);

        impl Identifier for $name {
            fn value(&self) -> String {
                self.0.clone()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(String::from_str(s).unwrap()))
            }
        }
    };
}

// According to Notion API Reference id's are UUIDv4 (https://developers.notion.com/reference/intro#json-conventions)
// can be represented with or without dashes.
// Using uuid crate.
uuid_identifer!(DatabaseId);
uuid_identifer!(PageId);
uuid_identifer!(BlockId);
string_identifer!(UserId);
string_identifer!(PropertyId);

impl From<PageId> for BlockId {
    fn from(page_id: PageId) -> Self {
        BlockId(page_id.0)
    }
}
