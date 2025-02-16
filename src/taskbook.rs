use todo_rs::task::Task;
use todo_rs::recurringtask::RecurringTask;

pub struct TaskBook {
    uuid: usize,
    owner: String,
    recurring: Vec<RecurringTask>,
    one_offs: Vec<Task>, 
}


impl TaskBook {
    pub fn new() -> Self {
        let recurring_log = Vec<RecurringTask>::new();
        let one_offs_log = Vec<Task>::new();
        Self {
            uuid: 0,
            owner: "".to_string(),
            completed: completed_log,
            recurring: recurring_log,
            one_offs: one_offs_log,
        }
    }
}