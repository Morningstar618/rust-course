fn main() {
    let mut x = 15;
    let ref1: &mut i32 = &mut x;
    *ref1 = 13;

    println!("{}", x);
}
