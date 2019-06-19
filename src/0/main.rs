/*
 * RustのコレクションVector。
 * CreatedAt: 2019-06-19
 */
fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    assert_eq!(v, Vec::new());

    let v: Vec<i32> = vec![1,2,3];
    println!("{:?}", v);
    assert_eq!(v, vec![1,2,3]);

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    println!("{:?}", v);
    assert_eq!(v, vec![5]);
}

