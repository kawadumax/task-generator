use crate::util::U16Mixin;
use std::u16;
// use fake::{faker::phone_number::en::PhoneNumber, Fake};
use gimei;
use rand::{prelude::ThreadRng, thread_rng, Rng};
pub type TaskDataRow = Vec<String>;
pub type TaskDataTable = Vec<TaskDataRow>;

pub struct DataGenerator;

impl DataGenerator {
    pub fn mode_1() -> TaskDataTable {
        let mut data = Vec::new();
        for _ in 0..38 {
            let name = gimei::name();
            let address = gimei::address();
            let mut pg = PhoneDataGenerator::new();
            data.push(vec![
                name.to_kanji(),
                name.to_hiragana(),
                name.gender.to_string(),
                address.to_string(),
                pg.phone(),
            ]);
        }
        data
    }
}

struct PhoneDataGenerator {
    rng: ThreadRng, // デフォルトの乱数生成器を初期化します
}

impl PhoneDataGenerator {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }

    pub fn phone(&mut self) -> String {
        let num_080_or_090 = if self.rng.gen() { "080" } else { "090" };
        let four_digit_first: u16 = self.rng.gen();
        let four_digit_latter: u16 = self.rng.gen();
        let eight_digit_string = (four_digit_first % 10_000).to_string_with_zero_padding()
            + "-"
            + &(four_digit_latter % 10_000).to_string_with_zero_padding();
        num_080_or_090.to_string() + "-" + &eight_digit_string
    }
}
