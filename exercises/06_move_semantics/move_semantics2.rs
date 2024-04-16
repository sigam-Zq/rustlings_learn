// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    // 这个地方处理冲突时候这两天冲突了- 最后都保留了
    let mut vec1 = fill_vec(vec0.clone());
    let vec1 = fill_vec(vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
