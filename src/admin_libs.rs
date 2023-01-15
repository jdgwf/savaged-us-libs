use serde::{Serialize, Deserialize};

use crate::book::Book;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FetchAdminParameters {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub login_token: Option<String>,
    #[serde(default)]
    pub filter: Option<String>,

    #[serde(default)]
    pub filter_book: u32,

    #[serde(default)]
    pub number_per_page: u32,
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub sort_by: Option<String>,
    #[serde(default)]
    pub sort_by_ascending: bool,

    #[serde(default)]
    pub needs_book_list: bool,

}

pub fn new_fetch_admin_params() -> FetchAdminParameters {

    FetchAdminParameters {
        api_key: None,
        login_token: None,
        filter: None,
        filter_book: 0,
        number_per_page: 25,
        current_page: 0,
        sort_by: None,
        sort_by_ascending: true,
        needs_book_list: true,
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AdminPagingStatistics {
    pub non_filtered_count: u32,
    pub filtered_count: u32,
    pub book_list: Option<Vec<Book>>,
}

