use std::io;

fn main() {
    let tuple:(i8,&'static str,f32) = (2,"hello", 4.5);

    let array:[i32; 5] =[1,4,7,5,12];
    let mut index=String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index:usize = index.trim().parse().expect("Please type a number!");
    println!("The value at index {} is {}", index, array[index]);

}
