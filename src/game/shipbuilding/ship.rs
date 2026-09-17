#[derive(Clone, Debug)]
pub struct Ship {
    name: String,
    engine: String,
}

impl Ship {
    pub fn new(engine: String) -> Self {
        Self { name: format!("{engine} I"), engine }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_engine(&self) -> String {
        self.engine.clone()
    }
}
