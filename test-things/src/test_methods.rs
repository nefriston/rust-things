
pub fn main() {
    let x = 10;
    let y = math_expression(x);
    println!("The value of y is {}", y);
    
    let strg = if x < 0 {
        "x is negative"
    } else {
        ""
    };
    print!("{}", strg)
;}
fn math_expression(x:i32)-> i32{
    let y={
        if x<0 {
            return 0;
        }
        x*x
    };
    return y;
}