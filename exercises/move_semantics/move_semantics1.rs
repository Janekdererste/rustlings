// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut result = fill_vec( vec0);

    println!("{} has length {} content `{:?}`", "vec1", result.len(), result);

    result.push(88);

    println!("{} has length {} content `{:?}`", "vec1", result.len(), result);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {

    let mut mutable_vector = vec;

    mutable_vector.push(22);
    mutable_vector.push(44);
    mutable_vector.push(66);

    mutable_vector
}
