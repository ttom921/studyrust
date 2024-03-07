//
//
//
//
//
//#region列舉Option 不只是個選項
const BANK_BALANCE: u32 = 1000;
fn main() {
    match withdraw(1000) {
        Ok(amount) => println!("提領金頠 {}", amount),
        Err(message) => println!("提領失敗 {}", message),
    }
}
fn withdraw(amount: u32) -> Result<u32, String> {
    if amount > BANK_BALANCE {
        return Err(String::from("餘額不足"));
    }

    Ok(amount)
}
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
