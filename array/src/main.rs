fn main() {
    let arr = [1,2,3,4,5]; 
    let n : usize = arr.len();
    println!("The number of elements in an array is : {}",n);
    // indexing in the array starts from 0 and till the n
    println!("The first element on the Array is {}",arr[0]);
    println!("The 2nd element on the Array is {}",arr[1]);
    println!("The 3rd element on the Array is {}",arr[2]);
    println!("The 4th element on the Array is {}",arr[3]);
    println!("The 5th element on the Array is {}",arr[4]);
    println!("So now instead of doing these things manually we can simply iterate it");

    // slicing in the array starts from 0 to till then n

    println!("The element from 0 to 3 are {:?}",&arr[0..3]);
    // like that we can do the slicing in the array !

    // let's do the iteration over the array it uses borrowing concept !

    for item in arr{
        println!("{}",item);
    }
    // println!("{:?}",arr); simple array print hogyi
// so it's like item jo arr ke ander hai sare print kar do or any other operation ok!


}

