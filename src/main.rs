use std::{convert::TryInto, env};
use task_generator::{Level1, Level2, Mode, ModeController, ModeKind, TaskBuilder};
fn main() {
    let args: Vec<String> = env::args().collect();
    // 0番目はファイルのパス、1番目から引数
    let mode_args = match args[1].parse::<usize>() {
        Ok(args_num) => args_num,
        Err(_) => 1,
    };

    let mode: Box<dyn Mode> = select_mode(mode_args);
    ModeController::new(mode).make();
}

fn select_mode(mode_args: usize) -> Box<dyn Mode> {
    match mode_args.try_into() {
        Ok(ModeKind::Level1) => Box::new(Level1 {
            builder: TaskBuilder::new(),
        }),
        Ok(ModeKind::Level2) => Box::new(Level2 {
            builder: TaskBuilder::new(),
        }),
        Err(_) => {
            println!("用意されていないレベルです。レベル１を出力します。");
            Box::new(Level1 {
                builder: TaskBuilder::new(),
            })
        }
    }
}
