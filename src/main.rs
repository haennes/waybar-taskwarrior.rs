use core::str;
use std::process::Command;

use chrono_humanize::HumanTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: i32,
    description: String,
    end: Option<String>,
    entry: Option<String>,
    modified: Option<String>,
    priority: Option<String>,
    status: String,
    uuid: Option<String>,
    tags: Option<serde_json::Value>,
    urgency: f32,
    due: Option<String>,
    project: Option<String>,
    scheduled: Option<String>,
    start: Option<String>,
}

struct TaskFormatter {
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
    fn new(default: bool) -> TaskFormatter {
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

    fn format(self, task: &Task) -> String {
        let mut string_vec: Vec<String> = Vec::new();

        if self.urgency {
            string_vec.push(format!("(Prio: {})", task.urgency));
        }

        if self.tags && task.tags.is_some() {
            string_vec.push(format!("[{}]", task.tags.as_ref().unwrap()));
        }

        if self.project && task.project.is_some() {
            string_vec.push(format!("[{}]", task.project.as_ref().unwrap()));
        }

        if self.description {
            string_vec.push(format!("'{}'", task.description))
        }

        if self.id {
            string_vec.push(format!("(ID: {})", task.id));
        }

        if self.due && task.due.is_some() {
            string_vec.push(format!("| due {}", format_time(task.due.as_ref().unwrap())));
        }

        if self.scheduled && task.scheduled.is_some() {
            string_vec.push(format!("| scheduled {}", task.scheduled.as_ref().unwrap()));
        }

        if self.running && task.start.is_some() {
            string_vec.push(format!("| running since {}", format_time(task.start.as_ref().unwrap())));
        }

        string_vec.join(" ")
    }
}

fn format_time(input_time: &String) -> String {
    let formatted_input = format!(
        "{}-{}-{}T{}:{}:{}Z",
        &input_time[0..4],  // Year
        &input_time[4..6],  // Month
        &input_time[6..8],  // Day
        &input_time[9..11], // Hour
        &input_time[11..13], // Minute
        &input_time[13..15] // Second
    );

    let utc_time = chrono::DateTime::parse_from_rfc3339(&formatted_input).unwrap();

    let local_time = utc_time.with_timezone(&chrono::Local); 

    let human_time = HumanTime::from(local_time);

    human_time.to_string()
}

// If not modification of Vec<...> is done, use [...] (Slice) instead
/*
 * Modifies the tasks vec to remove running tasks and return the running tasks
 */
fn get_running_tasks(tasks: &mut Vec<Task>) -> Vec<Task> {
    let mut running_tasks: Vec<Task> = Vec::new();

    for task in tasks.into_iter() {
        if task.start.is_some() {
            running_tasks.push(task.clone());
        }
    }

    for running_task in running_tasks.iter() {
        let index = tasks.iter().position( |pos| pos.id == running_task.id );

        if index.is_some() {
            tasks.remove(index.unwrap());
        } 
    }

    running_tasks
}

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("task export")
        .output()
        .expect("failed to execute process");

    let output = match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("Could not decode task export output"),
    };

    let mut tasks: Vec<Task> = serde_json::from_str::<Vec<Task>>(output)
            .expect("Could not convert output string to json")
            .into_iter()
            .filter(|entry| entry.status != "deleted" && entry.status != "completed")
            .collect();


    let running_tasks = get_running_tasks(&mut tasks);

    let mut task_fmt = TaskFormatter::new(false);

    if running_tasks.len() > 0 {    
      task_fmt.description = true;
      task_fmt.id = true;
      task_fmt.running = true;
      
      let main_text = format!("Currently running task: {}", task_fmt.format(running_tasks.get(0).unwrap()));
      println!("{}", main_text);
    }

    println!("{:?}", tasks);
}
