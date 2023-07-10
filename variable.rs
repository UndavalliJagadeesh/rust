fn main(){
    let mut x=10;
    let y=100;
    x=x+10;
    // y=y+10; ==> y is immutable. Can be changed as below
    let y = y+10;
    println!("The value of x is {x}");
    println!("The value of y is {y}");
}
