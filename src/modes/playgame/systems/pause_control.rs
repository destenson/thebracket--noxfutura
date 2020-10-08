use super::super::GameStateResource;
use super::super::RunState;
use crate::modes::playgame::{DesignMode, MiningMode};
use bengine::VirtualKeyCode;
use legion::*;

#[system]
pub fn pause_control(
    #[resource] state: &mut GameStateResource,
    #[resource] run_state: &mut RunState,
) {
    if let Some(key) = state.keycode {
        match key {
            VirtualKeyCode::Grave => *run_state = RunState::Paused,
            VirtualKeyCode::Key1 => *run_state = RunState::SlowMo,
            VirtualKeyCode::Key2 => *run_state = RunState::Running,
            VirtualKeyCode::Key3 => *run_state = RunState::FullSpeed,
            VirtualKeyCode::T => {
                *run_state = RunState::Design {
                    mode: DesignMode::Lumberjack,
                }
            }
            VirtualKeyCode::B => {
                *run_state = RunState::Design {
                    mode: DesignMode::Buildings { bidx: 0, vox: None },
                };
            }
            VirtualKeyCode::D => {
                *run_state = RunState::Design {
                    mode: DesignMode::Mining { mode: MiningMode::Dig },
                };
            }
            _ => {}
        }
    }
}
