fn main(){
    let mut a = 10;
    let mut iterations = 0;
    // for i in 1..a+1{
    //     iterations += 1;
    //     println!("Iteration no - {}", iterations);
    // }
    while a>0 {
        iterations += 1;
        println!("Iteration no - {}", iterations);
        a -= 1;
    }
}