/*
创建 数组的三个方法
*/

fn main() {
    println!("创建数组的三个方法");
    let a_1 = [1, 2, 3, 4, 5];
    let a_2 :[&str; 3] = ["a", "b", "c"];
    let a_3 = ["hi"; 5];

    print!("a_1: {:?}\n", a_1);
    print!("a_2: {:?}\n", a_2);
    print!("a_3: {:?}\n", a_3);
}
