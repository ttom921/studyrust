//
//
//
//
//

//#region 寫函式庫給自己跟別人用
//#endregion 寫函式庫給自己跟別人用
//建立函式庫
// ```
// cargo new calculator --lib
// ```
//專案裡開一個 tests/bmi_test.rs和src同一層的資料夾
// ```
// extern crate calculator;//任選一個
// use calculator;//任選一個

// #[test]
// fn bmi_test() {
//     let result = calculator::bmi(180, 65);
//     assert_eq!(result, Ok(20.1))
// }

// #[test]
// fn bmi_test_invalid() {
//     let result = calculator::bmi(0, 65);
//     assert!(result.is_err());
// }
// ```
//實作補上，我直接寫在 src/lib.rs 裡：
// ```
// pub fn bmi<T, U>(height: T, weight: U) -> Result<f64, String>
// where
//     T: Into<f64>,
//     U: Into<f64>,
// {
//     let h = height.into();
//     let w = weight.into();

//     if h <= 0.0 || w <= 0.0 {
//         return Err("輸入值有誤".to_string());
//     }

//     let bmi = w / (h / 100.0 * h / 100.0);
//     Ok((bmi * 10.0).round() / 10.0)
// }
// ```

//不過如果你還沒打算上傳到 crates.io 的話，可以先在自己本機試玩看看。我們先開一個新的專案：
// ```
// cargo new hello-bmi
// ```
//接著打開 Cargo.toml，在 [dependencies] 段落底下加上套件的名稱以及它的路徑：
// ```
// [dependencies]
// # 可以是本機
// # calculator = {path = "../calculator"}
// # 可以是github
// # calculator = { git = "https://github.com/kaochenlong/bmi-calculator-lib.git", branch = "main" }

// # 可以改名
// bmi_calc = {path = "../calculator", package = "calculator"}
// ```

//接著編輯 src/main.rs 檔案：
// ```
// fn main() {
//     //let result = calculator::bmi(170, 65);
//     let result = bmi_calc::bmi(170, 65);
//     match result {
//         Ok(bmi) => {
//             println!("BMI:{}", bmi);
//         }
//         Err(message) => {
//             println!("{message}")
//         }
//     }
// }
// ```

//是說，如果有個套件名稱你不喜歡或是覺得可讀性不太好，你沒辦法幫它改名字，但可以在引入使用的時候幫它改一下：
// ```
// [dependencies]
// bmi_calc = { path = "/tmp/calculator", package = "calculator" }
// ```
//#region 把函數丟來丟去的高階函數

// fn operation<F: Fn(i32, i32) -> i32>(n: i32, m: i32, op: F) -> i32 {
//     op(n, m)
// }
//如果覺得有點囉嗦可用前面章節學過的 where 關鍵字來改寫：
// fn operation<F>(n: i32, m: i32, op: F) -> i32
// where
//     F: Fn(i32, i32) -> i32,
// {
//     op(n, m)
// }

// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }
// fn main() {
//     let minus = |a, b| a - b;
//     let resutl1 = operation(10, 20, add);
//     let result2 = operation(10, 20, minus);
//     let result3 = operation(10, 20, |x, y| x * y);

//     println!("{}", resutl1);
//     println!("{}", result2);
//     println!("{}", result3);
// }
//#endregion 把函數丟來丟去的高階函數
//#region 閉包（Closure）
// fn main() {
//     //捕獲附近 Scope 的值
//     let n = 100;
//     let add_one = || n + 1;
//     println!("結果:{}", add_one());
//     // let add_numbers = |x, y| {
//     //     println!("Hello Rust!");
//     //     println!("Hello Again!");
//     //     //其它實作程式碼
//     //     x + y
//     // };
//     // println!("計算結果:{}", add_numbers(4, 5));
//     // //但接著故意傳浮點數給它的時候就會出錯了
//     // //println!("計算結果:{}", add_numbers(0.1, 0.3));//這個有問題，

//     // let add_number = |x, y| x + y;
//     // println!("計算結果：{}", add_number(3, 2));
//     // let say_hello = || println!("Hello Rust");
//     // say_hello();
// }
//#endregion 閉包（Closure）
//#region 把東西印出來！

// #[derive(Debug)]
// struct Cat {
//     name: String,
//     age: u8,
// }

// // struct Cat {
// //     name: String,
// //     age: u8,
// // }
// // use std::fmt::{Display, Formatter, Result};
// // impl Display for Cat {
// //     fn fmt(&self, f: &mut Formatter) -> Result {
// //         write!(f, "貓兒:{}", self.name)
// //     }
// // }
// fn main() {
//     let kitty = Cat {
//         name: String::from("Hellokitty"),
//         age: 12,
//     };
//     println!("{:?}", kitty);
//     //
//     println!("{:?}", 1);
//     println!("{:?}", false);
//     println!("{:?}", [1, 2, 3]);
//     println!("{:?}", vec![1, 2, 3]);
//     //
//     let message = format!("你好，我是 {}", "Hello Kitty");
//     println!("{}", message);
// }
//#endregion 把東西印出來！
// //#region 再看生命週期（Lifetime revisit）
// //Enum 裡的生命週期
// enum CatBreek<'a> {
//     Persian,           //波斯貓
//     AmericanShorthair, //美國短毛貓
//     Mix(&'a str, u8),  //米克斯
// }

// impl CatBreek<'_> {
//     fn say_something() {
//         println!("Hey!");
//     }
// }
// //泛型 + 生命週期
// struct Cat<'a, T> {
//     name: &'a str,
//     age: T,
// }
// impl<T> Cat<'_, T> {
//     fn say_hello(&self) {
//         println!("hello");
//     }
// }
// // struct Cat<'a> {
// //     name: &'a str,
// //     age: u8,
// // }

// // impl Cat<'_> {
// //     fn say_hello(&self) {
// //         println!("Hello");
// //     }
// // }
// // struct Cat {
// //     name: String,
// //     age: u8,
// // }
// fn main() {
//     let cat_name = "Kitty"; //------------+ 'cat_name
//                             /////////////                 ////////                |
//     let kitty = Cat {
//         //////////////////////////// /////////////--+ 'kitty  |
//         name: cat_name, //////////////////////  |         |
//         age: 12,        ///////////////////////  |         |
//     }; ///////////////////////////////////////  |         |
//        ///////////////////////////////////////  |         |
//     println!("{:?}", kitty.name); ////////////  |         |
//                                   ////////////--+         |
//                                   ////////////------------+
//                                   ///////////////////////////////////////
//                                   ///////////////////////////////////////
//                                   ///////////////////////////////////////

//     // let kitty = Cat {
//     //     name: String::from("Kitty"),
//     //     age: 12,
//     // };
// }
//#endregion 再看生命週期（Lifetime revisit）
//#region 套件（Crate）
// mod say_something;
// fn main() {
//     say_something::loudly("hello rust");
// }
//#endregion 套件（Crate）

//#region 模組（Module）
// mod greeting{
//     pub fn hi(){
//         println!("Hi, Rust");
//     }
//     fn hey(){
//         println!("Hey Rust");
//     }
// }
// mod greeting {
//     pub mod a {
//         pub mod b {
//             pub fn hi() {
//                 println!("Hi,a b Rust")
//             }
//         }
//     }
// }

// mod greeting {
//     pub fn hi() {
//         //super 是指目前這個模組的上一層。
//         //super::say_something::loudly("Hi,Rust super");
//         //crate 指的是當前專案的的最上層模組。
//         crate::say_something::loudly("Hi,Rust crate")
//     }
// }
// mod say_something {
//     pub fn loudly(message: &str) {
//         println!("{}!!!", message);
//     }
// }
// //use greeting::a::b::hi;
// fn main() {
//     //greeting::hi();
//     //greeting::a::b::hi();
//     //hi();
//     greeting::hi();
// }
//#endregion 模組（Module）
//#region 測試（Test）
// #[cfg(test)]
// mod bim {
//     use crate::bmi_calc;
//     #[test]
//     fn dummy() {
//         let result = 1 + 2;
//         assert_eq!(result, 3);
//     }
//     #[test]
//     fn test_calc() {
//         let result = bmi_calc(180, 65);
//         assert_eq!(result, 20.1);
//     }
// }
// fn bmi_calc<T, U>(height: T, weight: U) -> f64
// where
//     T: Into<f64>,
//     U: Into<f64>,
// {
//     let h = height.into() / 100.0;
//     let bmi = weight.into() / (h * h);
//     (bmi * 10.0).round() / 10.0
// }
// fn main() {}
//#endregion 測試（Test）

//#region 屬性（Attributes）
// #[derive(Debug)]
// struct Cat {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let kitty = Cat {
//         name: String::from("Kitty"),
//         age: 18,
//     };
//     println!("{:?}", kitty);
// }
//#endregion 屬性（Attributes）
//#region 錯誤處理（Error Handling）
// const BANK_BALANCE: u32 = 1000;
// fn withdraw(amount: u32) -> Result<u32, String> {
//     if amount > BANK_BALANCE {
//         return Err(String::from("餘額不足"));
//     }

//     Ok(amount)
// }
// fn main() {
//     match withdraw(10000) {
//         Ok(amount) => println!("提領金額 {} 元", amount),
//         Err(_) => panic!("💣💥"),
//     }
// }

// // fn main() {
// //     hello();
// // }

// // fn hello() {
// //     world();
// // }

// // fn world() {
// //     hey();
// // }

// // fn hey() {
// //     panic!("😱😱😱😱😱😱😱"); // 在這裡引爆
// // }

// // fn bmi_calculator<T, U>(height: T, weight: U) -> Result<f64, String>
// // where
// //     T: Into<f64>,
// //     U: Into<f64>,
// // {
// //     let w = weight.into();
// //     let h = height.into() / 100.0;

// //     if w <= 0.0 || h < 0.0 {
// //         return Err("輸入數值有誤".to_string());
// //     }
// //     Ok(w / (h * h))
// // }
// // fn main() {
// //     panic!("😱😱😱😱😱😱😱");
// //     // match bmi_calculator(170, 64.0) {
// //     //     Ok(result) => println!("{:.2}", result),
// //     //     Err(reason) => println!("{}", reason),
// //     // }
// // }

//#endregion 錯誤處理（Error Handling）
//#region 泛型（Generics）
//參數也能泛型
// fn add_number<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// use std::ops::Add;
// fn add_number<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn calc<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T>>(a: T, b: T, c: T) -> T {
//     a + b - c
// }
// use std::ops::{Add, Sub};
// // fn calc<T: Add<Output = T> + Sub<Output = T>>(a: T, b: T, c: T) -> T {
// //     a + b - c
// // }
// //另一種寫法
// fn calc<T>(a: T, b: T, c: T) -> T
// where
//     T: Add<Output = T> + Sub<Output = T>,
// {
//     a + b - c
// }
// fn main() {
//     // let res = add_number(1, 2);
//     // println!("{}", res);
//     // println!("{}", add_number(3.1, 9.8));
//     println!("{}", calc(1, 3, 1));
// }
// struct Rectangle<T> {
//     width: T,
//     height: T,
// }
// fn main() {
//     let rect_a = Rectangle {
//         width: 100,
//         height: 50,
//     };
//     let rect_b = Rectangle {
//         width: 38.5,
//         height: 19.5,
//     };
// }
//#endregion 泛型（Generics）

//#region列舉Option 不只是個選項
// const BANK_BALANCE: u32 = 1000;
// fn main() {
//     match withdraw(1000) {
//         Ok(amount) => println!("提領金頠 {}", amount),
//         Err(message) => println!("提領失敗 {}", message),
//     }
// }
// fn withdraw(amount: u32) -> Result<u32, String> {
//     if amount > BANK_BALANCE {
//         return Err(String::from("餘額不足"));
//     }

//     Ok(amount)
// }
//寫法2
// fn main() {
//     let friends = get_friends(true);
//     // println!("{:?}", friends);
//     //println!("{:?}", friends.unwrap());//所有權有轉移
//     // println!("{}", friends.is_some());
//     // println!("{}", friends.is_none());
//     println!("{:?}", friends.unwrap_or(vec![]));

//     // let friends = get_friends(false);
//     //  match friends {
//     //     None => println!("我是邊緣人我驕傲！"),
//     //     Some(list) => println!("我有好多朋友{:?}", list),
//     // }
// }
// fn get_friends(has_money: bool) -> Option<Vec<u8>> {
//     if !has_money {
//         return None;
//     }
//     let friends: Vec<u8> = vec![1, 2, 3, 4, 5];
//     Some(friends)
// }
//寫法2
//寫法1
// fn main() {
//     let friends = get_friends(false);
//     if friends.len() == 0 {
//         println!("我是邊緣人我驕傲！");
//     } else {
//         println!("我有好多朋友{:?}", friends);
//     }
// }
// fn get_friends(has_money: bool) -> Vec<u8> {
//     if !has_money {
//         return vec![];
//     }
//     let friends: Vec<u8> = vec![1, 2, 3, 4, 5];
//     return friends;
// }
//寫法1
//#endregion列舉Option 不只是個選項
//#region列舉（Enum）
// struct Skill{
//     action:String,
// }
// enum CatBreed{
//     Persian,//波斯貓
//     AmericanShorthair,//美國短毛貓
//     Mix(String,u8),//米克斯
//     Other(Skill),//其它
//     Alien{power:u32},//外星貓
// }
// impl CatBreed{
//     fn go(&self){
//         print!("Go!");
//     }
// }
// fn main(){
//     let goku_cat=CatBreed::Other(Skill{action:"龜派氣功".to_string()});
//     let frieza_cat=CatBreed::Alien { power:530000 };

//     greeting(&goku_cat);
//     greeting(&frieza_cat);
//     frieza_cat.go();
// }
// fn greeting(cat:&CatBreed){
//     match cat{
//         CatBreed::Mix(name,age)=>println!("我是米克斯，我叫{} 我今年{}歲",name,age),
//         CatBreed::Other(skill)=>println!("使出絕招{}",skill.action),
//         CatBreed::Alien{power}=>println!("我的戰鬥力是{}",power),
//         _=>println!("我是品種貓")
//     }
// }
// enum CatBreed{
//     Persian,//波斯貓
//     AmericanShorthair,//美國短毛貓
//     Mix(String,u8),//米克斯
// }
// fn main(){
//     let kitty=CatBreed::Mix(String::from("Kitty"),8);
//     let nancy=CatBreed::Persian;

//     greeting(&kitty);
//     greeting(&nancy);
// }
// fn greeting(cat: &CatBreed){
//     match cat{
//         CatBreed::Mix(name,age)=>println!("我是米克斯，我叫{} 我今年{}歲",name,age),
//         _=>println!("我是品種貓")
//     }
// }
// enum CatBreed{
//     Persian,//波斯貓
//     AmericanShorthair,//美國短毛貓
//     Mix,//米克斯
// }
// fn main(){
//     let breed= CatBreed::Persian;

//     match breed{
//         CatBreed::Persian=>{
//             println!("我是波斯貓");
//         }
//         CatBreed::AmericanShorthair=>{
//             println!("我是美國短毛貓");
//         }
//         CatBreed::Mix=>{
//             println!("我是米克斯");
//         }
//         _=> println!("我是品種貓"),
//     }
// }
//#endregion列舉（Enum）
// //#region特徵（Trait）
// struct Cat {
//     name:String,
//     age:u8,
// }
// struct Dog{
//     name:String,
//     age:u8,
// }
// trait Flyable{
//     fn fly(&self){
//         println!("飛呀！飛呀 小飛俠")
//     }
// }
// impl Flyable for Dog{

// }
// impl Flyable for Cat{
//     //實作內容在這裡
//     fn fly(&self){
//         println!("嘿，我是{},你看我會飛，你不會",self.name);
//     }

// }
// trait Greeting{
//     fn say_hello(&self){
//         println!("你好，我是{}",self.name());
//     }
//     fn name(&self)-> &str;
// }
// impl Greeting for Cat{
//     fn name(&self)->&str{
//         self.name.as_str()
//     }
// }
// impl Greeting for Dog{
// fn name(&self)->&str{
//         self.name.as_str()
//     }
// }
// trait Animal{
//     fn sleep(&self);
// }
// impl Animal for Cat{
//     fn sleep(&self){
//         println!("cat {} ZZzzzz",self.name);
//     }
// }
// fn bungee(someone:&dyn Flyable){
//     someone.fly();
// }
// fn main(){
//     let kitty= Cat{name:String::from("Kitty"),age:18};
//     let lucky=Dog{name:String::from("Locky"),age:10};
//     kitty.fly();
//     lucky.fly();
//     //
//     kitty.say_hello();
//     lucky.say_hello();
//     //
//     bungee(&kitty);
//     bungee(&lucky);
//     //
//     kitty.sleep();
// }
// //#endregion特徵（Trait）
//#region生命週期（Lifetime）
// struct Cat {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let kitty = Cat {
//         name: "Kitty".to_string(),
//         age: 12,
//     };
//     let nancy = Cat {
//         name: "Nancy".to_string(),
//         age: 16,
//     };
//     let boss = boss_cat(&kitty, &nancy);
//     println!("{}", boss.name);
// }
// fn boss_cat<'a>(c1: &'a Cat, c2: &'a Cat) -> &'a Cat {
//     if c1.age > c2.age {
//         c1
//     } else {
//         c2
//     }
// }
// fn print_age(){
//     let my_age;
//     {
//         let age=12;
//         my_age=&age;
//     }
//     println!("{}",my_age);
// }
//#endregion生命週期（Lifetime）

// //#region 結構（Struct）
// struct Cat {
//     name: String,
//     age: u8,
//     is_sleeping: bool,
// }
// impl Cat {
//     fn greeting(&self){
//         println!("Hello, my name is {} and age is {}",self.name,self.age);
//     }
//     fn set_age(&mut self,age:u8){
//         self.age=age;
//     }
//     fn count(list:&[u8])->u8{
//         list.iter().sum()
//     }
// }
// fn main() {
//     let mut kitty = Cat {
//         name: String::from("Kitty"),
//         age: 12,
//         is_sleeping: true,
//     };
//     kitty.set_age(21);
//     println!("{}", kitty.name);
//     kitty.greeting();
//     let result= Cat::count(&[10,20,30]);
//     println!("{}",result);
// }
// //#endregion 結構（Struct）
// //#region 切片
// fn main(){
//     // let lost_numbers =[4,5,15,16,23,42];
//     // let slice1=&lost_numbers[0..3];
//     // let slice2 =&lost_numbers[4..6];

//     // println!("{:?}",slice1);
//     // println!("{:?}",slice2);

//     // let lost_numbers=vec![4,6,15,16,23,42];
//     // let first_two_numbers= &lost_numbers[..2];//前2個
//     // let last_tree_nums=&lost_numbers[lost_numbers.len()-3..];//後3個
//     // let total_nums=&lost_numbers[..];
//     // println!("{:?}",first_two_numbers);
//     // println!("{:?}",last_tree_nums);
//     // println!("{:?}",total_nums);

//     // let mut lost_numbers=vec![4,8,15,16,23,42];
//     // let nums=&lost_numbers[0..3];
//     // nums[0]=123;
//     // println!("{:?}",lost_numbers);

//     // let mut lost_numbers=vec![4,8,15,16,23,42];
//     // let nums=&mut lost_numbers[0..3];
//     // nums[0]=5566;
//     // println!("{:?}",lost_numbers);

//     // let book="為你自己學 Rust";
//     // publish_book(book);

//     // let scores=[88,12,39,15,10,28,92];
//     // let group1=&scores[0..2];
//     // let group2=&scores[2..];

//     // println!("{}",calc_score(group1));
//     // println!("{}",calc_score(group2));

//     // let book:String=String::from("為你自己學 Rust");
//     // println!("{}",book);

//     let a=0x4000_0000 + 0xa2;

//     // use fo the bit shift "<<" operation;
//     let b=1<<5;
//     //{:X} will format values as hexadecimal
//     println!("{:X} {:X}",a,b);
//     println!("{:x} {:b}",a,b);

// }
// fn calc_score(scores:&[u16])->u16{
//     scores.iter().sum()
// }
// fn publish_book(book:&str){
//     println!("{:?} to market!",book );
// }
//#endregion 切片

//#region 所有權
// fn main() {
//     // let mut scores= get_score();
//     // let total_score=calc_score(&mut scores);

//     // println!("{:?}",total_score);
//     // println!("{:?}", scores);  // 印出 scores

//     // let book = String::from("為了自己學 rust");

//     // let b1= &book;
//     // let b2= &book;
//     // let b3= &book;
//     // println!("{:?},{:?},{:?}",b1,b2,b3);
// }
// // fn get_score()->Vec<i32>{
// //     let scores=vec![1450,9527,5566];
// //     return scores;
// // }
// // fn calc_score(scores:&mut Vec<i32>)->i32{
// //     scores.push(1234);
// //     let mut total=0;
// //     for score in scores.iter(){
// //         total+=score;
// //     }
// //     return total;
// // }
//#endregion 所有權
