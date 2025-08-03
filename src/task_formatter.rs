use crate::{Task, cond_opt_format};
use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
use paste::paste;

use crate::{cond_format, toggle};

pub(crate) struct TaskFormatter {
    urgency: bool,
    tags: bool,
    project: bool,
    description: bool,
    id: bool,
    due: bool,
    scheduled: bool,
    running: bool,
}

impl TaskFormatter {
    pub fn new(default: bool) -> TaskFormatter {
        TaskFormatter {
            urgency: default,
            tags: default,
            project: default,
            description: default,
            id: default,
            due: default,
            scheduled: default,
            running: default,
        }
    }
    toggle!(
        urgency,
        tags,
        project,
        description,
        id,
        due,
        scheduled,
        running
    );
    cond_format!(" ({:.2})", urgency);
    cond_format!(" '{}'", description);
    cond_format!(" (ID: {})", id);
    cond_opt_format!(" due {}", due, TaskFormatter::format_time);
    cond_opt_format!(" scheduled {}", scheduled);
    cond_opt_format!(" [{}]", project);

    fn format_time(time: &DateTime<Local>) -> String {
        HumanTime::from(*time).to_string()
    }

    pub fn format(&self, task: &Task) -> String {
        format!(
            "{urg}{proj}{tags}{desc}{id}{due}{scheduled}{start}",
            urg = self.format_urgency(task),
            proj = self.format_project(task),
            tags = {
                if self.tags && !task.tags.is_empty() {
                    TaskFormatter::format_array(&task.tags)
                } else {
                    "".to_string()
                }
            },
            desc = self.format_description(task),
            id = self.format_id(task),
            due = self.format_due(task),
            scheduled = self.format_scheduled(task),
            start = {
                if let Some(t) = task.start
                    && self.running
                {
                    let mut prefix = "🏃 since";

                    if t > Local::now() {
                        prefix = "🏃 in";
                    }

                    format!("| {} {}", prefix, Self::format_time(&t))
                } else {
                    "".to_string()
                }
            }
        )
    }

    fn format_array(vec: &[String]) -> String {
        let mut string = String::new();
        string.push('{');
        string.push_str(&vec.join(", ").to_string());
        string.push('}');

        string
    }
}
