extern crate task_generator;
use task_generator::*;
fn main() {
    let task_builder = TaskBuilder::new();
    task_builder.delete_pdf();
    task_builder.export();
}
