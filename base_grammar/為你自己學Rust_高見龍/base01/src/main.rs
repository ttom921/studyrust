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
fn main() {
    // let age: u8 = 20;

    // let age = 20;
    // println!("{}", age); //印出20

    // let age: u8;
    // println!("{}", age);

    // let age = 20;
    // println!("{}", age);

    // age = 18; //改成18
    // println!("{}", age);

    // let mut age = 20; //加入 mut 修飾
    // println!("{}", age);

    // age = 18; //改成18
    // println!("{}", age);

    // let mut age = 20; //加入 mut 修飾
    // println!("{}", age);

    // let a = 10;

    // if true {
    //     println!("{}", a); //這個block裡面沒有變數a
    // }
    // println!("{}", a);

    // let a = 10;
    // if true {
    //     let a = 20;
    //     println!("{}", a); //這個block 裡面有變數a,所以印出 20
    // }
    // println!("{}", a); //不會受 if 裡的宣告所影響

    // if true {
    //     let a = 20;
    // }

    // println!("{}", a);

    // const a=10;

    const my_age: u8 = 10;
    println!("{}", my_age);
}
// endregion: 變數與常數
