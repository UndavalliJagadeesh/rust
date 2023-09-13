fn fact(num:i32)->i32{
    if num<=1 {
        return 1;
    }
    return num*fact(num-1);
}

fn main(){
    let n = 5;
    print!("The factorial of {} is {}", n, fact(n));
}