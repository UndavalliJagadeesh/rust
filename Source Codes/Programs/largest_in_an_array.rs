fn main(){
    let arr = [1,42,53,21,25,21];
    let mut temp_value = 0;
    temp_value = arr[0];
    for i in &arr{
        if temp_value<*i{
            temp_value=*i;
        }
    }
    println!("The largest value in the array is {temp_value}");
}
