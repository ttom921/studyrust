fn main() {
    println!("Hello, world!");
    println!("this is a test");
    //oao();
    //funcscope();
    return_many();
}
//函數相關
fn into(op: fn(i8, i8) -> i8, a: i8, b: i8) -> i8 {
    op(a, b)
}
fn add(a: i8, b: i8) -> i8 {
    a + b
}
fn product(a: i8, b: i8) -> i8 {
    a * b
}
pub fn oao() {
    let a = 5;
    let b = 4;
    println!("add={}", into(add, a, b));
    println!("product={}", into(product, a, b));
}
//隱藏函數
pub fn owo() {
    println!("1");
}
pub fn funcscope() {
    owo();
    {
        owo();
        fn owo() {
            println!("2");
        }
    }
}
//回傳多項
pub fn retmany() -> (i32, i32) {
    (5, 8)
}
pub fn return_many() {
    let (x, y) = retmany();
    println!("{} {}", x, y);
}
