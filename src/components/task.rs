use std::collections::HashMap;

use specs::VecStorage;
use specs::Component;

/// Flags this entity as a task creator. These tasks when generated will be added to a task manager
/// which other entities with WorkerControlComponents will query for appropriate tasks
///
/// TaskCreators can then watch for these tasks to be updated etc.
#[derive(Debug)]
pub struct TaskCreatorComponent {
    pub task_id: Option<i64>,
}

impl Component for TaskCreatorComponent {
    type Storage = VecStorage<Self>;
}

#[derive(Clone)]
pub enum TaskType {
    Move(i32, i32),

    // Plan (TODO), Ticks required
    Build(i32, i32), // + requirements?
}

#[derive(Clone)]
/// Different categories of tasks. Allows tasks to be selected based on task consumers requirements
pub enum TaskCategory {
    General,
    Specialist,
}

#[derive(Clone)]
pub enum TaskState {
    Unclaimed,
    Claimed,
    Finished,
}

#[derive(Clone)]
/// A task is built from a task type which contains information relevant to the task
pub struct Task {
    pub id: i64,
    pub task_type: TaskType,
    pub category: TaskCategory,
    pub state: TaskState,
    pub priority: bool,
}

impl Task {
    pub fn set_finished(&mut self) {
        self.state = TaskState::Finished;
    }

    pub fn is_finished(&self) -> bool {
        if let TaskState::Finished = self.state {
            return true;
        }
        return false;
    }

    pub fn is_claimed(&self) -> bool {
        if let TaskState::Claimed = self.state {
            return true;
        }
        return false;
    }
}

/// Helper builder for creating complex tasks
pub struct TaskBuilder {
    task: Task,
}

impl TaskBuilder {
    pub fn new(task_type: TaskType, category: TaskCategory) -> Self {
        Self {
            task: Task {
                id: -1,
                task_type,
                category,
                state: TaskState::Unclaimed,
                priority: false,
            }
        }
    }

    pub fn set_priority(mut self) -> Self {
        self.task.priority = true;
        self
    }

    /// Add a requirement to be fulfilled before this task be executed
    pub fn add_requirement(mut self) -> Self {
        self
    }

    pub fn build(mut self) -> Task {
        self.task
    }
}

/// The task manager resource to be fetched from the world.
///
/// TODO more info
///
///
/// Tasks are stored by ID. They remain stored in the manager until the task is complete and
/// the the task creator has been informed of that. External resources only store the id of the
/// task and fetch it immutably or mutably depending on their requirements.
///
/// Events are not easy to implement so task creators can poll for the task state when they care
/// about it. The task should simply store it's progress and state. Paused, Started, Finished with a
/// value stored inside which represents the amount of progress (i.e. tick count?)
pub struct TaskManager {
    tasks: HashMap<i64, Task>,
    current_id: i64,
}

impl TaskManager {

    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            current_id: 0,
        }
    }

    /// Add's a task to the queue.
    ///
    /// Returns the id of the task
    pub fn add_task(&mut self, mut task: Task) -> i64 {
        task.id = self.current_id;
        self.tasks.insert(self.current_id, task);

        let prev_id = self.current_id;
        self.current_id += 1;

        prev_id
    }

    pub fn get_task_by_id(&self, id: i64) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn get_task_by_id_mut(&mut self, id: i64) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    /// TODO claimer requirements (i.e. priorities, restrictions etc.)
    pub fn claim_task(&mut self) -> Option<&mut Task> {
        let id = self.current_id - 1;
        let mut t = self.get_task_by_id_mut(id);

        if let Some(ref mut task) = t {

            if let TaskState::Finished = task.state {
                return None;
            }

            task.state = TaskState::Claimed;
        }

        t
    }

    pub fn finish_task(&mut self, id: i64) {
        let t = self.get_task_by_id_mut(id);

        if let Some(task) = t {
            task.state = TaskState::Finished;
        }
    }
}