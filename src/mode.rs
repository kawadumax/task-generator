use crate::data_generator::DataGenerator;
use crate::task_builder::TaskBuilder;

pub struct ModeController {
    mode: Box<dyn Mode>,
}

impl ModeController {
    pub fn new(mode: Box<dyn Mode>) -> Self {
        Self { mode }
    }

    pub fn make(self) {
        self.mode.make()
    }
}

pub trait Mode {
    fn make(self: Box<Self>);
}

pub struct Level1 {
    pub builder: TaskBuilder,
}

impl Mode for Level1 {
    fn make(self: Box<Self>) {
        self.builder.preface();
        self.builder.table(&DataGenerator::mode_1());
        self.builder.export();
    }
}

pub struct Level2 {
    pub builder: TaskBuilder,
}

impl Mode for Level2 {
    fn make(self: Box<Self>) {
        self.builder.preface();
        self.builder.table(&DataGenerator::mode_2());
        self.builder.export();
    }
}

pub enum ModeKind {
    Level1 = 1,
    Level2 = 2,
}

use std::convert::TryFrom;

impl TryFrom<usize> for ModeKind {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == ModeKind::Level1 as usize => Ok(ModeKind::Level1),
            x if x == ModeKind::Level2 as usize => Ok(ModeKind::Level2),
            _ => Err(()),
        }
    }
}
