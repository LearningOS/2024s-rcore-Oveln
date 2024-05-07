//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use lazy_static::*;

struct TaskInQueue {
    task: Arc<TaskControlBlock>,
}

impl PartialEq for TaskInQueue {
    fn eq(&self, other: &Self) -> bool {
        self.task.get_stride() == other.task.get_stride()
    }
}

impl Eq for TaskInQueue {}

impl PartialOrd for TaskInQueue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.task.get_stride().cmp(&other.task.get_stride()).reverse())
    }
}

impl Ord for TaskInQueue {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.task.get_stride().cmp(&other.task.get_stride()).reverse()
    }
}

impl TaskInQueue {
    fn new(task: Arc<TaskControlBlock>) -> Self {
        Self { task }
    }
}

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    // ready_queue: VecDeque<Arc<TaskControlBlock>>,
    ready_queue: BinaryHeap<TaskInQueue>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            // ready_queue: VecDeque::new(),
            ready_queue: BinaryHeap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(TaskInQueue::new(task));
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue
            .pop()
            .map(|task_in_queue| task_in_queue.task)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
