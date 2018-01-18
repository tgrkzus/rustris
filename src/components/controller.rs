use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use specs::DenseVecStorage;
use specs::Component;

use components::task::Task;

/// Information about how this entity should be controlled (type)
/// Includes tasks this entity is performing and should be perform
pub struct WorkerControlComponent {
    task_id: Option<i64>,
}

impl Component for WorkerControlComponent {
    type Storage = DenseVecStorage<Self>;
}

impl WorkerControlComponent {
    pub fn new() -> Self {
        Self {
            task_id: None,
        }
    }

    pub fn clear_task(&mut self) {
        self.task_id = None;
    }

    pub fn set_task(&mut self, task: &Task) {
        self.task_id = Some(task.id);
    }

    pub fn has_task(&self) -> bool {
        match self.task_id {
            Some(_) => {
                return true;
            },
            _ => return false
        }
    }

    pub fn get_task_id(&self) -> Option<i64> {
        return self.task_id;
    }
}