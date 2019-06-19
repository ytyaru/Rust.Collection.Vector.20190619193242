/*
 * RustのコレクションVector。
 * CreatedAt: 2019-06-19
 */
fn main() {
    {
        let v = vec![1,2,3];
    } // vの各要素も解放される。もし要素が参照なら…

    // 参照
    let v: Vec<i32> = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("{:?}", third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    // 実行時エラー
//    let not_exist = &v[100]; // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    let not_exist = v.get(100); // Option::None
    println!("{:?}", not_exist);

    /*
    // 可変と不変の参照は同一スコープ内に存在できない（矛盾するから）
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6); // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    */
}

