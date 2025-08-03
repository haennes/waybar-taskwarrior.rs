use std::str::FromStr;

use chrono::{DateTime, Local};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Status {
    Pending,
    Deleted,
    Completed,
    Waiting,
    Recurring,
}
impl Status {
    pub fn hidden(&self) -> bool {
        matches!(self, Status::Deleted | Status::Completed)
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub(crate) struct Task {
    pub(crate) id: i32,
    pub(crate) description: String,
    pub(crate) end: Option<DateTime<Local>>,
    pub(crate) entry: Option<String>,
    pub(crate) modified: Option<String>,
    pub(crate) priority: Option<String>,
    pub(crate) status: Status,
    pub(crate) uuid: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) urgency: f32,
    pub(crate) due: Option<DateTime<Local>>,
    pub(crate) project: Option<String>,
    pub(crate) scheduled: Option<DateTime<Local>>,
    pub(crate) start: Option<DateTime<Local>>,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(Status::Pending),
            "deleted" => Ok(Status::Deleted),
            "completed" => Ok(Status::Completed),
            "waiting" => Ok(Status::Waiting),
            "recurring" => Ok(Status::Recurring),

            _ => Err(()),
        }
    }
}
