use crate::systems::REGION;
use legion::systems::Schedulable;
use legion::*;
use nox_components::*;

pub fn build() -> impl Schedulable {
    let sb = SystemBuilder::new("work")
        .with_query(<(Write<MyTurn>, Read<Position>, Read<IdentityTag>)>::query())
        .build(|_, ecs, _, actors| {
            // Look for a job to do
            actors
                .iter_mut(ecs)
                .filter(|(turn, _, _)| {
                    turn.active && turn.shift == ScheduleTime::Work && turn.job == JobType::None
                })
                .for_each(|(mut turn, pos, id)| {
                    turn.order = WorkOrder::None;
                    // todo: include more attributes
                    if let Some(job) = REGION.write().jobs_board.evaluate_jobs(id.0, &*pos) {
                        turn.job = job;
                        println!("Assigned job: {:?}", turn.job);
                    } else {
                        turn.order = WorkOrder::MoveRandomly;
                    }
                });
        });
    sb
}
