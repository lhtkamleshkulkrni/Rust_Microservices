use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Filter {
    pub filter_field: String,
    pub filter_key: String,
}