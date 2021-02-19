use crate::data_generator::DataGenerator;
use crate::task_builder::TaskBuilder;

pub struct Mode {
    builder: TaskBuilder,
    mode: u8,
}

impl Mode {
    pub fn new(mode: u8) -> Self {
        Self {
            builder: TaskBuilder::new(mode),
            mode,
        }
    }

    pub fn make(self) {
        self.builder.preface();
        self.builder.table(&DataGenerator::mode_1());
        self.builder.export();
    }
}
