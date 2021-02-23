//Rust力が足りなくて実装できない
// pub fn nloop(n: usize, f: Fn()) {
//     for i in 0..n {
//         f();
//     }
// }

pub trait U16Mixin {
    fn to_string_with_zero_padding(&self) -> String;
}

impl U16Mixin for u16 {
    fn to_string_with_zero_padding(&self) -> String {
        format!("{0:>04}", self)
    }
}
