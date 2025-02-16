use crate::task::Task;
use crate::recurring_task::{RecurringTask, Frequency};

pub struct TaskBook {
    uuid: usize,
    recurring: Vec<RecurringTask>,
    one_offs: Vec<Task>, 
}


impl TaskBook {
    pub fn new() -> Self {
        let recurring_log = Vec::<RecurringTask>::new();
        let one_offs_log = Vec::<Task>::new();
        Self {
            uuid: 0,
            recurring: recurring_log,
            one_offs: one_offs_log,
        }
    }

    pub fn add_task(self, ){
        //TODO
    }
    pub fn add_recurring_task(self, ){
        //TODO
    }
    pub fn get_tasks_between(&self, start_time: f64, end_time: f64) -> Vec<Task> {
        let out = Vec::<Task>::new();
        //TODO
        // add a task for each one off and each instance of the recurring task in the give time window
        out
    }
}