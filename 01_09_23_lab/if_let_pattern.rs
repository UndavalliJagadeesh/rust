fn main(){
    let tup = ("CSE","IT","ME");
    if let ("CSE",x,y)=tup{
        print!("College has {} and {} also", x,y);
    }else{
        print!("College is not good.")
    }
}