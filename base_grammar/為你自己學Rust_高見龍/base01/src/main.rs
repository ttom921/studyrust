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

// region: 生命週期（Lifetime）

struct Cat {
    name: String,
    age: u8,
}

fn main() {
    //print_age();

    let kitty = Cat {
        name: "kitty".to_string(),
        age: 12,
    };
    let nancy = Cat {
        name: "Nancy".to_string(),
        age: 16,
    };

    let boss = boss_cat(&kitty, &nancy);
    println!("{}", boss.name);
}

// fn boss_cat(c1: &Cat, c2: &Cat) -> &Cat {
//     if c1.age > c2.age {
//         c1
//     } else {
//         c2
//     }
// }

fn boss_cat<'a>(c1: &'a Cat, c2: &'a Cat) -> &'a Cat {
    if c1.age > c2.age {
        c1
    } else {
        c2
    }
}
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

// region: 生命週期（Lifetime）
