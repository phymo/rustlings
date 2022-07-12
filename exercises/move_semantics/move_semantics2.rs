// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);
    let mut vec1 = vec0.clone();
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) {
    let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
