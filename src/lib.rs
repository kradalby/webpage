use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct Salary {
    pub company: String,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub salary: String,
    pub how_i_left: String,
    pub note: String,
}
