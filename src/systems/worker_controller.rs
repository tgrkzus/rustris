use specs::{System, SystemData, ReadStorage, WriteStorage, Fetch, FetchMut, Join};

use cgmath::Vector2;

use components::controller::WorkerControlComponent;
use components::team::{TeamComponent, Team};
use components::task::{TaskCreatorComponent, TaskManager, Task, TaskBuilder, TaskCategory, TaskType};
use components::transform::TransformComponent;
use render::camera::Camera;


use input::InputController;

/// The Brain of the worker
///
/// Assigns task based information for a worker.
/// Note: Only _asks_ doesn't do! (See WorkerProcessSystem for execution)
///
/// Find's tasks for workers and starts them
/// Creates new tasks for workers who are task creators
pub struct WorkerTaskSystem;

impl<'a> System<'a> for WorkerTaskSystem {
    type SystemData = (WriteStorage<'a, WorkerControlComponent>,
                       WriteStorage<'a, TaskCreatorComponent>,
                       ReadStorage<'a, TeamComponent>,
                       ReadStorage<'a, TransformComponent>,
                       FetchMut<'a, TaskManager>,
                       Fetch<'a, InputController>,
    );

    fn run(&mut self,
           (
               mut controls,
               mut task_creators,
               teams, transforms,
               mut task_manager, input_controller): Self::SystemData) {

        for (mut task_creator,) in (&mut task_creators,).join() {

            if let Some(id) = task_creator.task_id {
                if let Some(task) = task_manager.get_task_by_id(id) {
                    if task.is_finished() {
                        //println!("Task is finished");
                    }

                    if task.is_claimed() {
                        //println!("Task is claimed");
                    }
                }
            }

            if input_controller.is_left_clicked() {
                let task: Task = TaskBuilder::new(
                    TaskType::Move(input_controller.mouse.0, input_controller.mouse.1),
                    TaskCategory::General)
                    .build();

                task_creator.task_id = Some(task_manager.add_task(task));
            }
        }

        for (mut control,) in (&mut controls,).join() {
            if let Some(task) = task_manager.claim_task() {
                //println!("Got task {}", task.id);
                control.set_task(task);
            }
            else {
                //println!("No task!");
            }
        }
    }
}

/// The Body of the worker
///
/// Processes tasks for a worker. Ticks them appropriately and actually performs them
pub struct WorkerProcessSystem;

impl<'a> System<'a> for WorkerProcessSystem {
    type SystemData = (WriteStorage<'a, WorkerControlComponent>,
                       WriteStorage<'a, TransformComponent>,
                       WriteStorage<'a, TaskCreatorComponent>,
                       WriteStorage<'a, TeamComponent>,
                       Fetch<'a, Camera>,
                       FetchMut<'a, TaskManager>,
    );

    fn run(&mut self, (mut controls, mut transforms, task_creators, teams, camera, mut task_manager): Self::SystemData) {
        for (mut control, mut transform, ) in (&mut controls, &mut transforms, ).join() {

            if let Some(id) = control.get_task_id() {
                if let Some(mut task) = task_manager.get_task_by_id_mut(id) {

                    match task.task_type {
                        TaskType::Move(x, y) => {

                            let p = camera.screen_to_world(Vector2::new(x, y));
                            //println!("Moved to: {}, {}", p.x, p.y);
                            transform.set_position(p);

                            control.clear_task();

                            task.set_finished();
                        },
                        _ => {},
                    }

                }
            }
        }
    }
}


