use std::str::FromStr;

use crate::task::{Status, Task};
use chrono::{Local, ParseError};
use itertools::Either;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TaskParsing {
    id: i32,
    description: String,
    end: Option<String>,
    entry: Option<String>,
    modified: Option<String>,
    priority: Option<String>,
    status: String,
    uuid: Option<String>,
    tags: Option<Vec<String>>,
    urgency: f32,
    due: Option<String>,
    project: Option<String>,
    scheduled: Option<String>,
    start: Option<String>,
}

impl TryInto<Task> for TaskParsing {
    type Error = Either<(), ParseError>;
    fn try_into(self) -> Result<Task, Self::Error> {
        let TaskParsing {
            id,
            description,
            end,
            entry,
            modified,
            priority,
            status,
            uuid,
            tags,
            urgency,
            due,
            project,
            scheduled,
            start,
        } = self;
        let time = |time: Option<String>| -> Result<Option<chrono::DateTime<Local>>, ParseError> {
            let res = time.ok_or(Either::Left(())).and_then(|elem: String| {
                let elem = format!(
                    "{}-{}-{}T{}:{}:{}Z",
                    &elem[0..4],   // Year
                    &elem[4..6],   // Month
                    &elem[6..8],   // Day
                    &elem[9..11],  // Hour
                    &elem[11..13], // Minute
                    &elem[13..15]  // Second
                );

                chrono::DateTime::parse_from_rfc3339(&elem)
                    .map(|m| m.with_timezone(&chrono::Local))
                    .map_err(Either::Right)
            });
            match res {
                Ok(s) => Ok(Some(s)),
                Err(e) => match e {
                    Either::Left(_) => Ok(None),
                    Either::Right(e) => Err(e),
                },
            }
        };
        let status: Status = Status::from_str(&status).map_err(Either::Left)?;
        Ok(Task {
            id,
            description,
            end: time(end).map_err(Either::Right)?,
            entry,
            modified,
            priority,
            status,
            uuid,
            tags: tags.unwrap_or(Vec::new()),
            urgency,
            due: time(due).map_err(Either::Right)?,
            project,
            scheduled: time(scheduled).map_err(Either::Right)?,
            start: time(start).map_err(Either::Right)?,
        })
    }
}
