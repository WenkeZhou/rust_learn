/*
创建 数组的三个方法
*/

fn main() {
    println!("创建数组的三个方法");
    let a_1 = [1, 2, 3, 4, 5, 6];
    let a_2 :[&str; 3] = ["a", "b", "c"];
    let a_3 = ["hi"; 5];

    print!("a_1: {:?}\n", a_1);
    print!("a_2: {:?}\n", a_2);
    print!("a_3: {:?}\n", a_3);
    print!("hihihii a_1: {}\n", a_1[0]);

    let mut  a_1_mut = [1, 2, 3, 4, 5, 6];
    println!("a_1_mut: {:?}", a_1_mut);
    a_1_mut[0] = 99;
    println!("a_1_mut: {:?}", a_1_mut);

    println!("遍历方式一：通过长度来遍历\n");

    for index in 0..a_1.len() {
        println!("index is {}, item is {}", index, a_1[index])
    }

    println!("遍历方式二：通过迭代来遍历");
    for item in a_1.iter(){
        println!("item is {}", item)
    }

    println!("before update in main a_1： {:?}", a_1);
    update_arr(a_1);
    println!("after update in main a_1： {:?}", a_1);

    let mut a_1_refer = [1, 2, 3, 4, 5, 6];

    println!("before update in main a_1_refer： {:?}", a_1_refer);
    update_refer_arr(&mut a_1_refer);
    println!("after update in main a_1_refer： {:?}", a_1_refer);

}


fn update_arr(mut arr:[i32;6]) {
    for i in 0..6{
        arr[i] += 10;
    }
    println!("update in update_arr func a_1： {:?}", arr);
}


fn update_refer_arr(arr:&mut [i32;6]) {
    for i in 0..6{
        arr[i] += 10;
    }
    println!("update in update_refer_arr func a_1： {:?}", arr);
}
