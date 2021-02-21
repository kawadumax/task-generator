pub type TaskDataRow = Vec<&'static str>;
pub type TaskDataTable = Vec<TaskDataRow>;
pub struct DataGenerator;

impl DataGenerator {
    pub fn mode_1() -> TaskDataTable {
        let mut data = Vec::new();
        for _ in 0..5 {
            data.push(vec!["名前", "人畜無害", "金鳳花"]);
        }
        data
    }
}
