mod ui;
pub use ui::*;
use parking_lot::RwLock;
use std::thread;
use crate::modes::playgame::{BlockRenderPass, GBufferTestPass, VoxRenderPass};

lazy_static! {
    pub static ref LOADER: RwLock<LoaderState> = RwLock::new(LoaderState::new());
}

pub struct LoaderState {
    progress: f32,
    status: String,
    done: bool,

    pub rpass: Option<BlockRenderPass>,
    pub gpass: Option<GBufferTestPass>,
    pub vpass: Option<VoxRenderPass>
}

impl LoaderState {
    pub fn new() -> Self {
        Self{
            progress: 0.0,
            status: "Randomly Flipping Bits...".to_string(),
            done: false,
            rpass: None,
            gpass: None,
            vpass: None
        }
    }

    pub fn start_loading() {
        thread::spawn(|| {
            LOADER.write().update(0.01, "Starting to load things", false);
            crate::raws::load_raws();
            let rpass = BlockRenderPass::new();
            let gbuffer_pass = GBufferTestPass::new(
                &rpass.gbuffer,
            );
            let vox_pass = VoxRenderPass::new(
                &rpass.uniform_bind_group_layout,
            );

            let mut lock = LOADER.write();
            lock.rpass = Some(rpass);
            lock.gpass = Some(gbuffer_pass);
            lock.vpass = Some(vox_pass);
            std::mem::drop(lock);
            LOADER.write().update(1.00, "Built all the things", true);
        });
    }

    fn update(&mut self, progress: f32, status: &str, is_done: bool) {
        self.progress = progress;
        self.status = status.to_string();
        self.done = is_done;
    }
}

pub fn loader_progress(progress: f32, status: &str, is_done: bool) {
    LOADER.write().update(progress, status, is_done);
}