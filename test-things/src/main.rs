use std::{array, io, usize};
fn main() {
    let arr:[i32;5]= [1, 4, 7, 5, 12];
    for_and_range_test(&arr);
}
fn ownership_test() {

    {
        let s1 = String::from("hello1");
        let s2 = s1; // s1 is moved to s2, s1 is no longer valid
        // println!("{}", s1); // This line would cause a compile-time error
        println!("{}", s2); // This works fine too
    }
    // s2 goes out of scope here, and the memory is freed

    {
        let s1 = String::from("hello2");
        let s2 = s1.clone(); // s1 is cloned to s2, both are valid
        println!("{}", s1); // This works fine
        println!("{}", s2); // This works fine too
    }

    {
        let s1 =5;
        let s2 = s1; // s1 is copied to s2, both are valid
        println!("{}", s1); // This works fine
        println!("{}", s2); // Has its own copy because i32 implements Copy trait
    }

    {
        let arr1 = [1, 4, 7, 5, 12];  // i32 implements Copy
        let arr2 = arr1;  // arr1 is copied to arr2
        println!("arr1: {:?}", arr1);  // Still valid
        println!("arr2: {:?}", arr2);  // Has its own copy if add element without copy trait it wouldt work
    }
    {
        let s1 = String::from("hello3");
        let s2=10;
        take_ownership(s1); // s1 is moved to the function
        make_copy(s2); // s2 is copied to the function
    }
}
fn take_ownership(s: String) {
    println!("{}", s);
}
fn make_copy(s: i32) {
    println!("{}", s);
}


fn for_and_range_test(arr: &[i32]) {
    for elements in arr {
        println!("elements of array for in: {}", elements);
    }
    for i in  0..arr.len() {
        println!("elements of array for in range {}: {}", i, arr[i]);
    }
    let vec: Vec<usize> = (0..arr.len()).collect();
    for &i in &vec {
        println!("elements of array for in vec {}: {}", i , arr[i]);
    }
    println!("sum of array indexes range: {}", (0..arr.len()).sum::<usize>());
    println!("sum of array values arr.iter: {}", arr.iter().sum::<i32>());
}


fn loop_test_2(){
    let mut counter = 0;
    let mut sum=0;
    let square_number=loop {
        if counter == 3 {
            break sum;
        }
        sum += square_input();
        counter += 1;
    };
    println!("The square number is {}", square_number);
}

fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn square_input()-> u32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x = input.trim().parse().expect("Please type a number!");
    let y = math_expression(x);
    println!("The value of y is {}", y);
    
    let strg = if x < 0 {
        "x is negative"
    } else {
        ""
    };
    print!("{}", strg);
    return y as u32;
}
fn math_expression(x:i32)-> i32{
    let y={
        if x<0 {
            return 0;
        }
        x*x
    };
    return y;
}


fn test_array_staff(){
    let _tuple:(i8,&'static str,f32) = (2,"hello", 4.5);

    let array:[i32; 5] =[1,4,7,5,12];
    let mut index=String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index:usize = index.trim().parse().expect("Please type a number!");
    println!("The value at index {} is {}", index, array[index])
}
