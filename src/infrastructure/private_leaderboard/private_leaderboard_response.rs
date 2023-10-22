use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PrivateLeaderboardResponse {
    // event: String,
    // owner_id: u32,
    pub members: HashMap<String, MemberResults>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MemberResults {
    pub completion_day_level: HashMap<usize, DayResults>,
    pub name: String,
    pub local_score: usize,
    // last_star_ts: u32,
    // stars: usize,
    // global_score: usize,
    // id: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct DayResults {
    #[serde(rename = "1")]
    pub stage_one: Option<StageResults>,
    #[serde(rename = "2")]
    pub stage_two: Option<StageResults>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StageResults {
    // star_index: u32,
    // get_star_ts: u32,
}
