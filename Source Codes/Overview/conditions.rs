fn main(){
    let mut x=0;
    let y=100;
    if x==y{
        println!("x and y are equal ({x}={y})");
    }
    else if x>y{
        println!("x is greater than y ({x}>{y})");
    }
    else{
        x=x+50;
        println!("The value of x is {x}\nThe value of y is {y}");
    }
}
