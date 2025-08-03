mod macros;
mod task;
mod task_formatter;
mod task_parsing;

use core::str;
use itertools::Itertools;
use std::process::Command;

use serde_json::json;

use crate::{task::Task, task_formatter::TaskFormatter, task_parsing::TaskParsing};

fn main() {
    let output = Command::new("task")
        .arg("export")
        //.arg("task export")
        .output()
        .expect("failed to execute process");

    let output = match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("Could not decode task export output"),
    };

    let mut tasks = serde_json::from_str::<Vec<TaskParsing>>(output)
        .expect("Could not convert output string to json")
        .into_iter()
        .map(TryInto::try_into)
        .process_results(|e| {
            e.filter(|entry: &Task| !entry.status.hidden())
                .sorted_by(|a, b| b.urgency.total_cmp(&a.urgency))
        })
        .expect("Found invalid task"); // reversed ordering: big first

    let most_urgent = tasks.next();
    let (running_tasks, non_running_tasks): (Vec<_>, Vec<_>) =
        tasks.partition(|elem| elem.start.is_some());

    let mut tooltip = "".to_string();
    if !running_tasks.is_empty() {
        let fmt = TaskFormatter::new(false)
            .with_description()
            .with_project()
            .with_due()
            .with_running()
            .with_scheduled();

        tooltip += "Running Tasks: \n";
        tooltip += &running_tasks
            .iter()
            .map(|t| format!("+{}", fmt.format(t)))
            .join("\n");
        tooltip += "\n \n";
    }
    if !non_running_tasks.is_empty() {
        let fmt = TaskFormatter::new(false)
            .with_description()
            .with_due()
            .with_project()
            .with_running()
            .with_scheduled();
        tooltip += "Non Running Tasks: \n";
        tooltip += &non_running_tasks
            .iter()
            .map(|t| format!("-{}", fmt.format(t)))
            .join("\n");
    }

    let preview = match most_urgent {
        Some(most_urgent) => TaskFormatter::new(true).without_id().format(&most_urgent),
        None => "No task found".to_string(),
    };

    let output = json!({
        "text": preview,
        "tooltip": tooltip,
    })
    .to_string();

    println!("{output}");
}
