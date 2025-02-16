use crate::task::Task;

use std::time::{SystemTime, UNIX_EPOCH};

pub enum Frequency{
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub struct RecurringTask {
    pub created: f64,
    pub first_due: f64,
    pub frequency: Frequency,
    pub title: String,
    pub description: String,
    pub complete: bool,
}

impl RecurringTask {
    pub fn new_entry(first_due: f64, frequency: Frequency, title: String, description: String) -> Self {
        let now = SystemTime::now();
        let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
        Self {
            created: since_the_epoch.as_secs_f64(),
            first_due: first_due,
            frequency: frequency,
            title: title,
            description:description,
            complete: false
        }
    }

    pub fn get_tasks_between(&self, start_time: f64, end_time: f64) -> Vec<Task> {
        let out = Vec::<Task>::new();
        //TODO
        // add a task for each instance of the recurring task in the give time window
        out
    }
}