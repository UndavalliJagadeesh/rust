fn main(){
    let a=10;
    let b=20;
    let op='*';
    match op{
        '+' => print!("{} + {} is {}",a,b,a+b),
        '-' => print!("{} - {} is {}",a,b,a-b),
        '*' => print!("{} * {} is {}",a,b,a*b),
        '/' => print!("{} / {} is {}",a,b,a/b),
        '%' => print!("{} % {} is {}",a,b,a%b),
        _=>print!("Invalid"),
    }
}