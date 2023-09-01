fn main(){
    let mut arr:[i64;6]=[-1;6];
    let mut j=0;
    for i in 0..11{
        if i%2==0{
            arr[j]=i;
            j+=1;
        }
    }
    print!("{:?}",arr);
}