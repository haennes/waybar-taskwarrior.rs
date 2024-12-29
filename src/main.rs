use core::str;
use std::{process::Command, str::FromStr};

use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
use prettytable::{Cell, Row, Table};
use serde::{Serialize, Deserialize};

use serde_json::json;

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

    fn format(&self, task: &Task) -> String {
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
            string_vec.push(format!("| scheduled {}", format_time(task.scheduled.as_ref().unwrap())));
        }

        if self.running && task.start.is_some() {
            let mut prefix = "running since";
            let date = convert_to_date(task.start.as_ref().unwrap());

            if date > Local::now() {
                prefix = "starting";
            }

            string_vec.push(format!("| {} {}", prefix, format_time(task.start.as_ref().unwrap())));
        }

        string_vec.join(" ")
    }

    fn push_empty(vec: &mut Vec<String>){
        vec.push(String::from_str("-").unwrap())
    }

    fn format_table(&self, tasks: &[Task]) -> String {
        let mut table = Table::new();

        let mut headers: Vec<&str> = Vec::new();

        if self.urgency {
            headers.push("Priority");
        }

        if self.tags {
            headers.push("Tags");
        }

        if self.project {
            headers.push("Project");
        }

        if self.description {
            headers.push("Description");
        }

        if self.id {
            headers.push("ID");
        }

        if self.due {
            headers.push("Due");
        }

        if self.scheduled {
            headers.push("Scheduled");
        }

        if self.running {
            headers.push("Running / Starting");
        }
        

        table.add_row(Row::new(headers.into_iter().map(|entry| Cell::new(entry)).collect()));

        for task in tasks {
            let mut data: Vec<String> = Vec::new();

            if self.urgency {
                data.push(format!("{}", task.urgency));
            }

            if self.tags {
                if task.tags.is_none() {
                    TaskFormatter::push_empty(&mut data);
                } else {
                    data.push(format!("[{}]", task.tags.as_ref().unwrap()));
                }
            }

            if self.project {
                if task.project.is_none() {
                    TaskFormatter::push_empty(&mut data);
                } else {
                    data.push(format!("[{}]", task.project.as_ref().unwrap()));
                }
            }

            if self.description {
                data.push(task.description.clone());
            }

            if self.id {
                data.push(task.id.to_string());
            }

            if self.due {
                if task.due.is_none() {
                    TaskFormatter::push_empty(&mut data);
                } else {
                    data.push(format_time(task.due.as_ref().unwrap()));
                }
            }

            if self.scheduled {
                if task.scheduled.is_none() {
                    TaskFormatter::push_empty(&mut data);
                } else {
                    data.push(format_time(task.scheduled.as_ref().unwrap()));
                }
            }

            if self.running {
                if task.start.is_none() {
                    TaskFormatter::push_empty(&mut data);
                } else {
                    
                    let mut prefix = "running since";
                    let date = convert_to_date(task.start.as_ref().unwrap());

                    if date > Local::now() {
                        prefix = "starting";
                    }

                    data.push(format!("| {} {}", prefix, format_time(task.start.as_ref().unwrap())));
                }
            }

            table.add_row(Row::new(data.into_iter().map(|entry| Cell::new(&entry)).collect()));                
        }

        table.to_string()
    }
}

fn convert_to_date(input_time: &String) -> DateTime<Local> {
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

    local_time
}

fn format_time(input_time: &String) -> String {

    let date = convert_to_date(input_time);

    let human_time = HumanTime::from(date);

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

fn format_hover(tasks: &[Task]) -> String {
    if tasks.len() == 0 {
        return String::from_str("No more tasks").unwrap();
    }
    
    let mut vec: Vec<String> = Vec::new();

    let task_fmt = TaskFormatter::new(true);
    
    for task in tasks.into_iter() {
        vec.push(format!("- {}", task_fmt.format(task)));
    }

    vec.join("\n")
}

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

    let mut tasks: Vec<Task> = serde_json::from_str::<Vec<Task>>(output)
            .expect("Could not convert output string to json")
            .into_iter()
            .filter(|entry| entry.status != "deleted" && entry.status != "completed")
            .collect();

    tasks.sort_by(|a, b| b.urgency.total_cmp(&a.urgency)); // reversed ordering: big first
    
    let running_tasks = get_running_tasks(&mut tasks);

    let mut task_fmt = TaskFormatter::new(false);

    let return_json: serde_json::Value; 

    if running_tasks.len() > 0 {    
      task_fmt.description = true;
      task_fmt.id = true;
      task_fmt.running = true;

      let main_text = format!("Current running task: {}", task_fmt.format(running_tasks.get(0).unwrap()));

      return_json = json!({
        "text": main_text,
        "tooltip": format_hover(&tasks),
      });

      println!("{}", return_json);
      return;
    }

    let mut string = String::from_str("No task found").unwrap();

    if tasks.len() > 0 {
        let most_urgent = tasks.remove(0);

        task_fmt = TaskFormatter::new(true);
        task_fmt.id = false;

        string = format!("Most urgent task: {}", task_fmt.format(&most_urgent));
    }

    task_fmt = TaskFormatter::new(true);
    // task_fmt.id = false;
    // task_fmt.running = false;

    let output = json!({
        "text": string,
        // "tooltip": task_fmt.format_table(&tasks),
        "tooltip": format_hover(&tasks),
    }).to_string();

    println!("{}", output);
}
