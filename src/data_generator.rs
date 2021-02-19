pub type TaskDataRow = Vec<&'static str>;
pub type TaskDataTable = Vec<TaskDataRow>;
pub struct DataGenerator;

impl DataGenerator {
    pub fn mode_1() -> TaskDataTable {
        let mut vec = Vec::new();
        for _ in 0..5 {
            vec.push(vec!["名前", "人畜無害", "金鳳花"]);
        }
        vec
    }
}
