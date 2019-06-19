/*
 * RustのコレクションVector。
 * CreatedAt: 2019-06-19
 */
fn main() {
    // 異なる型を保持する方法（enum）
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
