//

// region: 資料型態（原始型別 - 數字篇）

// fn main() {
//     // let age: u8 = 1000;
//     // println!("{}", age);

//     // let age: u8 = 255;
//     // let new_age: u8 = age + 1;
//     // println!("{}", age);
//     // println!("{}", new_age);

//     // let result = 0.1 + 0.2;
//     // println!("{}", result);

//     // let name = "Hello Kitty";
//     // let age = 20;
//     // println!("hi,my name is {} ,and I am {} years old", name, age);

//     let mut age = 20;
//     age = 20.5;
//     println!("age is {}", age);
// }

// endregion: 資料型態（原始型別 - 數字篇）

// region: 資料型態（原始型別 - 字元、布林值）
// fn main() {
//     // let message='world';
//     // println!("hello {}",message);

//     // let cc: char = 'a';
//     // let huh = '蛤';
//     // let cat = '🐈';
//     // println!("{} say {}", cat, huh);

//     // let cats: u8 = 100;

//     // if cats > 1 {
//     //     println!("好多貓");
//     // } else {
//     //     println!("一隻貓");
//     // }

//     let has_cat = 0;
//     if has_cat {
//         println!("有貓")
//     } else {
//         println!("沒有貓")
//     }
// }

// endregion: 資料型態（原始型別 - 字元、布林值）

// region: 資料型態（原始型別 - 原始型別 - 陣列、元組）

// fn main() {
//     // let list: [u8; 3] = [1, 2, 3];
//     // println!("{:?}", list);

//     // let list: [u8; 3] = ['a', 2, 3];
//     // println!("{:?}", list);

//     //let list = [1, 2, 3];

//     // let list = [1450, 9527, 5566];
//     // println!("{}", list.len()); //印出3
//     // println!("{}", list[1]); //印出9527

//     // let list = [1450, 9527, 5566];
//     // for item in list.iter() {
//     //     println!("{}", item);
//     // }

//     // let list = [1450, 9527, 5566];
//     // let [_, b, c] = list;
//     // println!("{}", b);
//     // println!("{}", c);

//     //let point:(i32,i32,i32)=(100,200,300);

//     // let answer: (char, bool) = ('🐈', false);
//     // let pet = ('🐈', false);

//     // let pet = ('🐈', false, 18);
//     // println!("{} {} {}", pet.0, pet.1, pet.2);

//     let point = (100, 200, 300);
//     let (x, y, z) = point;
//     println!("{} {} {}", x, y, z);
// }
// endregion: 資料型態（原始型別 - 原始型別 - 陣列、元組）

// region: 變數與常數
// fn main() {
//     // let age: u8 = 20;

//     // let age = 20;
//     // println!("{}", age); //印出20

//     // let age: u8;
//     // println!("{}", age);

//     // let age = 20;
//     // println!("{}", age);

//     // age = 18; //改成18
//     // println!("{}", age);

//     // let mut age = 20; //加入 mut 修飾
//     // println!("{}", age);

//     // age = 18; //改成18
//     // println!("{}", age);

//     // let mut age = 20; //加入 mut 修飾
//     // println!("{}", age);

//     // let a = 10;

//     // if true {
//     //     println!("{}", a); //這個block裡面沒有變數a
//     // }
//     // println!("{}", a);

//     // let a = 10;
//     // if true {
//     //     let a = 20;
//     //     println!("{}", a); //這個block 裡面有變數a,所以印出 20
//     // }
//     // println!("{}", a); //不會受 if 裡的宣告所影響

//     // if true {
//     //     let a = 20;
//     // }

//     // println!("{}", a);

//     // const a=10;

//     const my_age: u8 = 10;
//     println!("{}", my_age);
// }
// endregion: 變數與常數

// region: 函數

// fn say_hello() {
//     println!("Hello,Rust");
// }
// fn main() {
//     // say_hello();
//     //print_number(5566);
//     // let result = add(1, 2);
//     // print!("{}", result);
//     // let tudata = get_data();
//     // print!("{} {} {}", tudata.0, tudata.1, tudata.2);

//     let age = 20;
//     let message = if age < 8 {
//         "小朋友"
//     } else if age >= 8 && age < 18 {
//         "年輕人"
//     } else {
//         "成年人"
//     };
//     print!("{}", message);
// }
// fn print_number(n: i32) {
//     println!("{}", n);
// }
// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }
// fn get_data() -> (char, i32, bool) {
//     return ('a', 30, true);
// }
// endregion: 函數

// region: Stack與Heap
// fn main() {
//     //draw(9527);

//     // let mut list = vec![1, 2, 3];
//     // list.push(4);

//     // println!("{:?}", list);

//     // let mut numbers = vec![1, 2, 3];
//     // println!("{},{}", numbers.len(), numbers.capacity());

//     // numbers.push(1);
//     // println!("{},{}", numbers.len(), numbers.capacity());

//     // numbers.push(1);
//     // numbers.push(1);
//     // println!("{},{}", numbers.len(), numbers.capacity());

//     // numbers.push(1);
//     // println!("{},{}", numbers.len(), numbers.capacity());

//     let rooms: Vec<u8> = Vec::with_capacity(20);
//     println!("{},{}", rooms.len(), rooms.capacity());
// }

// fn show_lotteries(n1: i32, n2: i32, n3: i32) {
//     println!("the lottery numbers are {} {} {}", n1, n2, n3);
// }
// fn draw(num: i32) {
//     show_lotteries(num + 1, num + 2, num + 3);
// }
// endregion: Stack與Heap

// region: 所有權(Ownership)
// fn main() {
//     // let total = calc_score();
//     // println!("{}", total);

//     // let scores = get_scroes();
//     // println!("{:?}", scores);

//     // let scores = get_scroes();
//     // let total_score = calc_score(&scores);
//     // println!("{:?}", total_score);
//     // println!("{:?}", scores);

//     // let mut scores = vec![1456, 9527, 5566];
//     // let total_score = calc_score(&scores);

//     let mut book = String::from("為你自已學Rust");
//     let b1 = &book;
//     let b2 = &book;
//     let b3 = &book;

//     println!("{:?},{:?},{:?}", b1, b2, b3);
// }
// fn calc_score() -> i32 {
//     let scores = vec![1450, 9527, 5566];
//     let mut total = 0;

//     for score in scores.iter() {
//         total += score;
//     }
//     return total;
// }
// fn calc_score(scores: &Vec<i32>) -> i32 {
//     let mut total = 0;
//     for score in scores.iter() {
//         total += score;
//     }
//     return total;
// }
// fn get_scroes() -> Vec<i32> {
//     let scores = vec![1450, 9527, 5566];
//     return scores; //自動䆁放佔用的記憶體
// }
// fn calc_score(scores: &mut Vec<i32>) -> i32 {
//     scores.push(123); //加料
//     let mut total = 0;
//     for score in scores.iter() {
//         total += score;
//     }
//     return total;
// }
// endregion: 所有權(Ownership)

// region: 切片(Slice)
// fn main() {
//     // let lost_numbers = [4, 8, 15, 16, 23, 42];
//     // let slice1 = &lost_numbers[0..3];
//     // let slice2 = &lost_numbers[4..6];

//     // println!("{:?}", slice1); //印出[4,8,15]
//     // println!("{:?}", slice2); //印出[23,42]

//     // let lost_numbers = [4, 8, 15, 16, 23, 42];
//     // let first_two_nums = &lost_numbers[..2]; //前2個
//     // let last_tree_nums = &lost_numbers[lost_numbers.len() - 3..]; //後3個

//     // println!("{:?}", first_two_nums); //印出[4,8]
//     // println!("{:?}", last_tree_nums); //印出[16,23,42]

//     // let numbers = &lost_numbers[..];
//     // println!("{:?}", numbers);

//     // let mut lost_numbers = vec![4, 8, 15, 16, 23, 42];
//     // let nums = &lost_numbers[0..3];
//     // nums[0] = 123;
//     // println!("{:?}", nums);//有error

//     // let mut lost_numbers = vec![4, 8, 15, 16, 23, 42];
//     // let nums = &mut lost_numbers[0..3];
//     // nums[0] = 5566;
//     // println!("{:?}", lost_numbers);

//     // let book = "為你自己學 Rust";
//     // publish_book(book);

//     // let scores = [88, 12, 39, 15, 10, 28, 92];
//     // let group1 = &scores[0..2]; //[88,12]
//     // let group2 = &scores[2..]; //[39,15,10,28,92]
//     // println!("{}", calc_score(group1));
//     // println!("{}", calc_score(group2));

//     let book: String = String::from("為你自己學 Rust");
//     println!("{}", book);
// }

// fn publish_book(book: &str) {
//     println!("{:?} 要上市囉！", book);
// }

// fn calc_score(scores: &[u16]) -> u16 {
//     scores.iter().sum()
// }
// endregion: 切片(Slice)

// region: 結構（Struct）
// struct Cat {
//     name: String,
//     age: u8,
//     is_sleeping: bool,
// }
// impl Cat {
//     fn greeting(&self) {
//         println!("Hello,my name is {}", self.name);
//     }
//     fn set_age(&mut self, age: u8) {
//         self.age = age;
//     }
//     fn run() {
//         println!("Go Go Power Rangers");
//     }
//     fn count(list: &[u8]) -> u8 {
//         list.iter().sum()
//     }
// }
// struct dog {
//     NAME: String,
//     Age: u8,
// }
// fn main() {
//     // let kitty = Cat {
//     //     name: String::from("Kitty"),
//     //     age: 12,
//     //     is_sleeping: true,
//     // };
//     // let name = String::from("kitty");
//     // let age = 12;
//     // let is_sleeping = true;
//     // let kitty = Cat {
//     //     name,
//     //     age,
//     //     is_sleeping,
//     // };
//     // println!("{}", kitty.name);
//     // println!("{}", kitty.age);
//     //println!("{}", kitty.is_sleeping);

//     //kitty.age = 20;

//     // let name = String::from("kitty");
//     // let age = 12;
//     // let is_sleeping = true;
//     // let mut kitty = Cat {
//     //     name,
//     //     age,
//     //     is_sleeping,
//     // };
//     // kitty.age = 20;

//     // println!("{}", kitty.name);
//     // println!("{}", kitty.age);
//     // println!("{}", kitty.is_sleeping);

//     let mut kitty = Cat {
//         name: String::from("Kitty"),
//         age: 12,
//         is_sleeping: true,
//     };

//     kitty.greeting(); // 印出 Hello, my name is Kitty
//     kitty.set_age(21);

//     println!("{}", kitty.name);
//     println!("{}", kitty.age);
//     println!("{}", kitty.is_sleeping);
//     Cat::run();

//     let result = Cat::count(&[10, 20, 30]);
//     println!("{}", result); //印出60
// }
// endregion: 結構（Struct）

// region: 生命週期（Lifetime）

// struct Cat {
//     name: String,
//     age: u8,
// }

// fn main() {
//     //print_age();

//     let kitty = Cat {
//         name: "kitty".to_string(),
//         age: 12,
//     };
//     let nancy = Cat {
//         name: "Nancy".to_string(),
//         age: 16,
//     };

//     let boss = boss_cat(&kitty, &nancy);
//     println!("{}", boss.name);
// }

// fn boss_cat(c1: &Cat, c2: &Cat) -> &Cat {
//     if c1.age > c2.age {
//         c1
//     } else {
//         c2
//     }
// }

// fn boss_cat<'a>(c1: &'a Cat, c2: &'a Cat) -> &'a Cat {
//     if c1.age > c2.age {
//         c1
//     } else {
//         c2
//     }
// }
// fn print_age() {
//     let age = 12; ////////-----------------+- 'age
//                   /////////////                 |
//     let my_age = &age; //-----+- 'my_age  |
//                        ////////     |           |
//     println!("{}", my_age); ///     |           |
//                             ///-----+           |
// } /////////////////////////////-----------------+

// fn print_age() {
//     let my_age;
//     {
//         let age = 12;
//         my_age = &age;
//     }
//     println!("{}", my_age);
// }
//
// fn print_age() {
//     let my_age; ///////---------------+-- 'my_age
//                 /////////////               |
//     {
//         /////////////////////               |
//         let age = 12; //------+ 'age   |
//         my_age = &age; //////      |        |
//     } ///////////////////////------+        |
//       ///////////////////////               |
//     println!("{}", my_age); //              |
// } ///////////////////////////---------------+

// endregion: 生命週期（Lifetime）

// region: 特徵（Trait）
// trait Flable {
//     // fn fly(&self);
//     fn fly(&self) {
//         println!("飛呀～飛呀～小飛俠");
//     }
// }
// trait Greeting {
//     // fn say_hello(&self) {
//     //     println!("你好，我是{}", self.name);
//     // }
//     fn say_hello(&self) {
//         println!("你好，我是{}", self.name());
//     }
//     fn name(&self) -> &str;
// }
// trait Animal {
//     fn sleep(&self);
// }
// // impl Flable for Cat {
// //     // 實作內容在這裡
// //     // fn fly(&self) {
// //     //     println!("嘿，我是{}, 你看我會飛，你不會", self.name);
// //     // }
// //     // fn hey(&self) {
// //     //     println!("How you doing");
// //     // }
// // }
// impl Flable for Cat {}
// //impl Flable for Dog {}

// impl Greeting for Cat {
//     fn name(&self) -> &str {
//         self.name.as_str()
//     }
// }
// impl Greeting for Dog {
//     fn name(&self) -> &str {
//         self.name.as_str()
//     }
// }
// impl Animal for Cat {
//     fn sleep(&self) {
//         println!("{} Zzzzzz", self.name);
//     }
// }
// struct Cat {
//     name: String,
//     age: u8,
// }
// struct Dog {
//     name: String,
//     age: u8,
// }

// fn bungee(someone: &dyn Flable) {
//     someone.fly();
// }
// fn main() {
//     // let kitty = Cat {
//     //     name: String::from("Kitty"),
//     //     age: 18,
//     // };
//     // kitty.fly(); // 印出 嘿，我是 Kitty，你看我會飛，你不會！
//     let kitty = Cat {
//         name: String::from("Kitty"),
//         age: 18,
//     };
//     let lucky = Dog {
//         name: String::from("lucky"),
//         age: 17,
//     };
//     // kitty.fly();
//     // lucky.fly();
//     //
//     // kitty.say_hello();
//     // lucky.say_hello();
//     //bungee(&kitty);
//     //bungee(&lucky);

//     kitty.sleep();
// }
// endregion: 特徵（Trait）
// region: 列舉（Enum）
// enum CatBreed {
//     Persian,           //波斯貓
//     AmericanShorthair, //美國短毛貓
//     Mix,               //米克斯
// }

// #[allow(dead_code)]
// enum CatBreed {
//     Persian,           //波斯貓
//     AmericanShorthair, //美國短毛貓
//     Mix(String, u8),   //米克斯
// }
// struct Skill {
//     action: String,
// }
// enum CatBreed {
//     Persian,              //波斯貓
//     AmericanShorthair,    //美國短毛貓
//     Mix(String, u8),      //米克斯
//     Other(Skill),         //其它
//     Alien { power: u32 }, //外星貓
// }
// impl CatBreed {
//     fn go(&self) {
//         println!("Go!");
//     }
// }
// fn main() {
//     //let breed = CatBreed::Persian;
//     // match breed {
//     //     CatBreed::Persian => {
//     //         println!("我是波斯貓");
//     //     }
//     //     CatBreed::AmericanShorthair => {
//     //         println!("我是美國短毛貓");
//     //     }
//     //     CatBreed::Mix => {
//     //         println!("我是米克斯");
//     //     }
//     // }
//     // match breed {
//     //     CatBreed::Persian => println!("我是波斯貓"),
//     //     CatBreed::AmericanShorthair => println!("我是美國短毛貓"),
//     //     CatBreed::Mix => println!("我是米克斯"),
//     // }

//     // match breed {
//     //     CatBreed::Persian => println!("我是波斯貓"),
//     //     CatBreed::AmericanShorthair => println!("我是美國短毛貓"),
//     // }

//     // match breed {
//     //     CatBreed::Mix => println!("我是米克斯"),
//     //     _ => println!("我是品種貓"),
//     // }

//     // match breed {
//     //     _ => println!("我是品種貓"),
//     //     CatBreed::Mix => println!("我是米克斯"),
//     // }

//     // let kitty = CatBreed::Mix(String::from("kitty"), 8);
//     // let nancy = CatBreed::Persian;
//     // greeting(&kitty);
//     // greeting(&nancy);

//     let goku_cat = CatBreed::Other(Skill {
//         action: "龜派氣功".to_string(),
//     });
//     let frieza_cat = CatBreed::Alien { power: 530000 }; //戰鬥力53萬
//     greeting(&goku_cat);
//     greeting(&frieza_cat);
//     frieza_cat.go();
// }
// fn greeting(cat: &CatBreed) {
//     match cat {
//         CatBreed::Mix(name, age) => println!("我是米克斯, 我叫{} 我今年{} 歲", name, age),
//         _ => println!("我是品種貓"),
//     }
// }
// fn greeting(cat: &CatBreed) {
//     match cat {
//         CatBreed::Mix(name, age) => println!("我是米克斯, 我叫{} 我今年{} 歲", name, age),
//         CatBreed::Other(skill) => println!("使出絕招{}", skill.action),
//         CatBreed::Alien { power } => println!("我是戰鬥力是{}", power),
//         _ => println!("我是品種貓"),
//     }
// }
// endregion: 列舉（Enum）

// region: Option 不只是個選項
//fn get_friends() -> Option<Vec<u8>> {}
// fn get_friends(has_money: bool) -> Option<Vec<u8>> {
//     if !has_money {
//         return None;
//     }
//     let friends: Vec<u8> = vec![1, 2, 3, 4, 5];
//     Some(friends)
// }
// fn get_friends(has_money: bool) -> Vec<u8> {
//     if !has_money {
//         return vec![];
//     }
//     let friends: Vec<u8> = vec![1, 2, 3, 4, 5];
//     return friends;
// }

// fn withdraw(amount: u32) -> u32 {
//     // 判斷帳戶餘額
// }
// const BANK_BALANCE: u32 = 1000;
// fn withdraw(amount: u32) -> Result<u32, String> {
//     // 判斷帳戶餘額
//     if amount > BANK_BALANCE {
//         return Err(String::from("餘額不足"));
//     }
//     Ok(amount)
// }
// fn main() {
//     // let friends = get_friends(false);

//     // if friends.len() == 0 {
//     //     println!("我是邊緣人我驕傲！")
//     // } else {
//     //     println!("我有好多朋友 {:?}", friends)
//     // }

//     // let friends = get_friends(false);

//     // match friends {
//     //     None => println!("我是邊緣人我驕傲！"),
//     //     Some(list) => println!("我有好多朋友 {:?}", list),
//     // }
//     //let friends = get_friends(true);
//     // println!("{:?}", friends);
//     // println!("{:?}", friends.unwrap());

//     //
//     // println!("{}", friends.is_some());
//     // println!("{}", friends.is_none());
//     //
//     //println!("{:?}", friends.unwrap_or(vec![]));

//     match withdraw(100) {
//         Ok(amount) => println!("提領金額{}元", amount),
//         Err(message) => println!("提領失敗：{}", message),
//     }
// }
// endregion: Option 不只是個選項
// region: 泛型（Generics）
// struct  Rectangle{
//     width:u32,
//     height:u32,
// }

// struct RectangleU32 {
//     width: u32,
//     height: u32,
// }
// struct RectangleF32 {
//     width: f32,
//     height: f32,
// }

// struct Rectangle<T> {
//     width: T,
//     height: T,
// }

// fn add_number(a:i32,b:i32) -> i32{
//     a+b
// }
// fn add_number<T>(a: T, b: T) -> T {
//     a + b
// }
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
// fn calc<T: Add<Output = T> + Sub<Output = T>>(a: T, b: T, c: T) -> T {
//     a + b - c
// }

// use std::ops::{Add, Sub};

// fn calc<T>(a: T, b: T, c: T) -> T
// where
//     T: Add<Output = T> + Sub<Output = T>,
// {
//     a + b - c
// }
// fn main() {
//     // let rect_a=Rectangle{width:100,height:50};//沒問題
//     // let rect_b=Rectangle{width:38.5,height:19.5};//不行

//     // let rect_a = RectanRectangleU32gle {
//     //     width: 100,
//     //     height: 50,
//     // };
//     // let rect_b = RectangleF32 {
//     //     width: 38.5,
//     //     height: 19.5,
//     // };

//     // let rect_a = Rectangle {
//     //     width: 100,
//     //     height: 50,
//     // };
//     // let rect_b = Rectangle {
//     //     width: 38.5,
//     //     height: 19.5,
//     // };

//     // println!("{}", add_number(1, 2));
//     // println!("{}", add_number(3.1, 9.8));
//     //println!("{}", add_number(true, true));
// }

// endregion: 泛型（Generics）

// region: 錯誤處理（Error Handling）
// fn bmi_calculator<T, U>(height: T, weight: U) -> Result<f64, String>
// where
//     T: Into<f64>,
//     U: Into<f64>,
// {
//     let w = weight.into();
//     let h = height.into() / 100.0;
//     if w <= 0.0 || h <= 0.0 {
//         return Err("輸入數值有誤".to_string());
//     }
//     Ok(w / (h * h))
// }

// const BANK_BALANCE: u32 = 1000;
// fn withdraw(amount: u32) -> Result<u32, String> {
//     // 判斷帳戶餘額
//     if amount > BANK_BALANCE {
//         return Err(String::from("餘額不足"));
//     }
//     Ok(amount)
// }
// fn main() {
//     // match bmi_calculator(170, 70.5) {
//     //     Ok(result) => print!("{:.2}", result), //印出 24.39
//     //     Err(reason) => println!("{}", reason),
//     // }
//     // panic!("😱😱😱😱😱😱😱");

//     // hello();

//     // match withdraw(1200) {
//     //     Ok(amount) => println!("提領金額 {} 元", amount),
//     //     Err(message) => println!("提領失敗：{}", message),
//     // }
//     match withdraw(1200) {
//         Ok(amount) => println!("提領金額 {} 元", amount),
//         Err(_) => panic!("💣💥"),
//     }
// }
// fn hello() {
//     world();
// }
// fn world() {
//     hey();
// }
// fn hey() {
//     panic!("😱😱😱😱😱😱😱");
// }
// endregion: 錯誤處理（Error Handling）

// region: 屬性（Attributes）

// #[allow(dead_code)]
// #[derive(Debug)]
// struct Cat {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let kitty = Cat {
//         name: String::from("kitty"),
//         age: 18,
//     };
//     println!("{:?}", kitty);
// }
// endregion: 屬性（Attributes）
// region: 測試（Test）

// #[cfg(test)]
// mod bmi {
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
// endregion: 測試（Test）

// region: 模組（Module）
// mod greeting {
//     pub fn hi() {
//         println!("Hi, Rust");
//     }
//     fn hey() {
//         println!("Hey Rust");
//     }
// }
// mod greeting {
//     mod a {
//         mod b {
//             pub fn hi() {
//                 println!("Hi, Rust");
//             }
//         }
//     }
// }

// mod greeting {
//     pub mod a {
//         pub mod b {
//             pub fn hi() {
//                 println!("Hi, Rust");
//             }
//             pub fn hey() {
//                 println!("Hey Rust");
//             }
//         }
//     }
// }

//use greeting::a::b::hi;
// use greeting::a::b::{hey, hi};

// mod greeting {
//     pub fn hi() {
//         // 想要在這裡呼叫 loudly 函數
//         //super::say_something::loudly("rust");
//         crate::say_something::loudly("crate rust")
//     }
// }
// mod say_something {
//     pub fn loudly(message: &str) {
//         println!("{}!!!!", message.to_uppercase());
//     }
// }
// fn main() {
//     //greeting::hi();

//     //greeting::a::b::hi();

//     //greeting::a::b::hi();

//     //hi();
//     // hi();
//     // hey();

//     greeting::hi();
// }
// endregion: 模組（Module）

// region: 套件（Crate）
// mod say_something;
// fn main() {
//     say_something::loudly("hello rust");
// }
// endregion: 套件（Crate）
// region: 再看生命週期（Lifetime revisit）
// struct Cat {
//     name: String,
//     age: u8,
// }
// struct Cat {
//     name: &str,
//     age: u8,
// }
// #[derive(Debug)]
// struct Cat<'a> {
//     name: &'a str,
//     age: u8,
// }

// impl Cat {
//     fn sae_hello(&self) {
//         println!("Hello");
//     }
// }
// impl<'a> Cat<'a> {
//     fn sae_hello(&self) {
//         println!("Hello");
//     }
// }
// impl Cat<'_> {
//     fn sae_hello(&self) {
//         println!("Hello");
//     }
// }
// #[derive(Debug)]
// struct Cat<'a, T> {
//     name: &'a str,
//     age: T,
// }
// impl<T> Cat<'_, T> {
//     fn sae_hello(&self) {
//         println!("Hello");
//     }
// }

// enum CatBreed<'a> {
//     Persian,           // 波斯貓
//     AmericanShorthair, // 美國短毛貓
//     Mix(&'a str, u8),  // 米克斯
// }

// impl CatBreed<'_> {
//     fn say_something() {
//         println!("Hey!");
//     }
// }
// fn main() {
//     // let kitty = Cat {
//     //     name: String::from("Kitty"),
//     //     age: 12,
//     // };
//     // let kitty = Cat {
//     //     name: "Kitty",
//     //     age: 12,
//     // };
//     let cat_name = "Kitty"; //------------+ 'cat_name
//                             //            |
//     let kitty = Cat {
//         //--+ 'kitty  |
//         name: cat_name, //  |         |
//         age: 12,        //  |         |
//     }; //  |         |
//        //  |         |
//     println!("{:?}", kitty); //  |         |
//                              //--+         |
// } //------------+

// endregion: 再看生命週期（Lifetime revisit）

// region: 把東西印出來！

// struct Cat {
//     name: String,
// }
// use std::fmt::{Debug, Display, Formatter, Result};

// impl Display for Cat {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "貓兒:{}", self.name)
//     }
// }
// impl Debug for Cat {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "貓兒:{}", self.name)
//     }
// }

// #[derive(Debug)]
// struct Cat {
//     name: String,
// }

// struct Cat {
//     name: String,
// }
// use std::fmt::{Display, Formatter, Result};

// impl Display for Cat {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{}", self.name.to_uppercase())
//     }
// }
// fn main() {
//     // let kitty = Cat {
//     //     name: String::from("Kitty"),
//     // };
//     // println!("{}", kitty);
//     // println!("{:?}", kitty);

//     // let kitty = Cat {
//     //     name: String::from("Kitty"),
//     // };
//     // println!("{:?}", kitty);

//     // let message = format!("你好，我是 {}", "Hello Kitty");
//     // println!("{}", message);

//     let kitty = Cat {
//         name: String::from("kitty"),
//     };
//     println!("{}", kitty.to_string()); // 印出 KITTY
// }
// endregion: 把東西印出來！

// region: 閉包（Closure）
fn main() {
    // let say_hello = || println!("Hello Rust");
    // say_hello();

    // let add_numbers = |x, y| x + y;
    // println!("計算結果：{}", add_numbers(1, 2));

    // let add_numbers = |x, y| {
    //     println!("Hello Rust!");
    //     println!("Hello Again!");
    //     //其它實作程式碼
    //     x + y
    // };
    // println!("計算結果：{}", add_numbers(1, 2));

    // let add_numbers = |x, y| x + y;
    // println!("計算結果: {}", add_numbers(1, 2));
    // println!("計算結果: {}", add_numbers(0.1, 0.2));

    // let n = 100;
    // let add_one = || n + 1;
    // println!("結果：{}", add_one()); //印出101

    //let n = 100;
}
// fn add_one() {
//     return n + 1;
// }
// endregion: 閉包（Closure）
