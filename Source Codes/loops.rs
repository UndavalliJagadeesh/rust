fn main(){
    println!("Using LOOP");
    rust_loop();
    println!();
    println!("Using WHILE loop");
    while_loop();
    println!();
    println!("Using FOR loop");
    for_loop();
}

fn rust_loop(){
    let mut i=10;

    loop{
        if i==0{
            break;
        }
        print!("{}", i);
        i-=1;
    }
}

fn while_loop(){
    let mut i=0;

    while i<10{
        print!("{}", i);
        i+=1;
    }
}

fn for_loop(){
    for i in 1..10{
        print!("{}",i);
    }
}
