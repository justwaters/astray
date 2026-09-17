#[derive(Clone, Debug)]
pub struct Ship {
    name: String,
    engine: String,
    weapon: String,
}

impl Ship {
    pub fn new(engine: String, weapon: String) -> Self {
        Self { name: format!("{engine} I"), engine, weapon }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_engine(&self) -> String {
        self.engine.clone()
    }

    pub fn get_weapon(&self) -> String {
        self.weapon.clone()
    }
}
