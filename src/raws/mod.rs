mod formats;
pub use formats::*;
use formats::{load_biomes, load_materials};
use parking_lot::RwLock;
mod material_map;

pub struct Raws {
    pub biomes: Biomes,
    pub materials: Materials,
    pub matmap : material_map::MaterialMap
}

impl Raws {
    fn new() -> Self {
        Self {
            biomes: Biomes::new(),
            materials: Materials::new(),
            matmap : material_map::MaterialMap::new()
        }
    }

    fn load(&mut self) {
        self.biomes = load_biomes();
        self.materials = load_materials();
    }
}

lazy_static! {
    pub static ref RAWS: RwLock<Raws> = RwLock::new(Raws::new());
}

pub fn load_raws() {
    RAWS.write().load();
}

pub fn get_material_by_tag(name: &str) -> Option<usize> {
    let lock = RAWS.read();
    let finder = lock.materials.materials
        .iter()
        .enumerate()
        .find(|(_,m)| m.name == name);
    if finder.is_some() {
        Some(finder.unwrap().0)
    } else {
        None
    }
}