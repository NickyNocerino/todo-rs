use crate::task::Task;

use std::time::{SystemTime, Duration, UNIX_EPOCH};

pub enum Frequency{
    Daily,
    Weekly,
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
        let mut out = Vec::<Task>::new();
        let mut instance = UNIX_EPOCH + Duration::from_secs_f64(self.first_due);
        while {
            instance
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64() < end_time
        }
            {
            if {
                instance
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs_f64() > start_time
            } {
                out.push(Task::new_backdated_entry(
                    self.created,
                    instance.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs_f64(),
                    self.title.clone(),
                    self.description.clone()
                ))
            }
            match self.frequency {
                Frequency::Daily => {
                    instance = instance +  Duration::from_secs(86400);
                }, 
                Frequency::Weekly => {
                    instance = instance +  Duration::from_secs(7*86400);
                }
            }
        }
        out
    }
}