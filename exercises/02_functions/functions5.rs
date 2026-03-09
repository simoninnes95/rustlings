// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num // Rust distinguishes between expressions and statements. Having the ; at the end of the line makes this return () which is a void in c/c++
}



fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
