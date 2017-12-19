#[no_mangle]
pub extern fn fibonacci(num: i32) -> i32 {
    if num <= 1 {
        return 1;
    } else {
        return fibonacci(num - 1) + fibonacci(num - 2);
    }   
}

fn main() {
    for i in 1..41 {
        println!("{} - {}", i, fibonacci(i));  
    }  
}
