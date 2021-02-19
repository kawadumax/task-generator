use crate::task_builder::TaskBuilder;

pub struct Mode {
    builder: TaskBuilder,
    mode: u8
}

impl Mode {
    pub fn new(mode: u8) -> Self{
        Self { 
            builder: TaskBuilder::new(mode),
            mode
        }
    }

    pub fn make(self){
        self.builder.delete_pdf();
        self.builder.export();
    }
}