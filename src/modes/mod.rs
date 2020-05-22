use crate::opengl::*;
mod loader;
mod resources;
mod helpers;
mod main_menu;
mod planetgen;
mod planetgen2;
mod play;
use bracket_geometry::prelude::Point;

#[derive(Copy, Clone)]
pub enum ProgramMode {
    Loader,
    MainMenu,
    PlanetGen,
    PlanetGen2,
    Resume,
    PlayGame,
    Quit,
}

pub struct Program {
    mode: ProgramMode,
    resources: resources::SharedResources,
    loader : loader::Loader,
    mainmenu : main_menu::MainMenu,
    planetgen : planetgen::PlanetGen,
    planetgen2 : planetgen2::PlanetGen2,
    play : Option<play::PlayGame>
}

impl Program {
    pub fn new() -> Self {
        Self {
            mode: ProgramMode::Loader,
            resources: resources::SharedResources::new(),
            loader: loader::Loader::new(),
            mainmenu : main_menu::MainMenu::new(),
            planetgen: planetgen::PlanetGen::new(),
            planetgen2 : planetgen2::PlanetGen2::new(),
            play: None
        }
    }

    pub fn init(&mut self, gl: &Gl, ctx: &EngineContext) {
        self.resources.init(gl);
        self.planetgen2.init(gl, ctx);
    }

    pub fn tick(
        &mut self,
        ui: &imgui::Ui,
        gl: &Gl,
        ctx: &EngineContext
    ) -> bool {
        self.mode = match self.mode {
            ProgramMode::Loader => self.loader.tick(ui, gl, &self.resources),
            ProgramMode::MainMenu => self.mainmenu.tick(gl, &self.resources, ui, ctx),
            ProgramMode::PlanetGen => self.planetgen.tick(gl, &self.resources, ui),
            ProgramMode::PlanetGen2 => self.planetgen2.tick(gl, &self.resources, ui, ctx),
            ProgramMode::Resume => {
                if self.play.is_none() {
                    self.play = Some(play::PlayGame::new(self.load_game(), gl, ctx));
                }
                ProgramMode::PlayGame
            }
            ProgramMode::PlayGame => {
                self.play.as_mut().unwrap().tick(gl, ui, ctx)
            }
            _ => self.mode
        };
        true
    }

    pub fn on_resize(&mut self, _gl: &Gl, _new_size: Point) {

    }

    fn load_game(&self) -> crate::planet::SavedGame {
        use std::path::Path;
        use std::fs::File;
        use std::io::Read;
        let savepath = Path::new("world.dat");
        if !savepath.exists() {
            panic!("Saved game doesn't exist");
        }

        let f = File::open(&savepath).expect("Unable to open file");
        let mut d = flate2::read::ZlibDecoder::new(f);
        let mut s = String::new();
        d.read_to_string(&mut s).unwrap();

        let saved : crate::planet::SavedGame = ron::from_str(&s).unwrap();
        saved
    }
}
