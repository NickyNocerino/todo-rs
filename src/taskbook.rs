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
}