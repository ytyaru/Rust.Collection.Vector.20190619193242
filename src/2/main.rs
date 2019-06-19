/*
 * RustのコレクションVector。
 * CreatedAt: 2019-06-19
 */
fn main() {
    let v: Vec<i32> = vec![100, 32, 57];
    for item in &v {
        println!("{}", item);
    }
    println!("{:?}", v);

    let mut v: Vec<i32> = vec![100, 32, 57];
    for item in &mut v {
        *item += 50;
        println!("{}", item);
    }
    println!("{:?}", v);
}

