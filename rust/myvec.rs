pub fn test_vec_int(){
    let mut my_ints:Vec<i32>=Vec::new();

    my_ints.push(30);
    my_ints.push(40);
    my_ints.push(50);
    my_ints.push(60);
    my_ints.push(70);
    my_ints.push(80);
   

    println!("{:?}",my_ints);
    println!("size of Vec:{:?}", my_ints.len());
    println!("capacity of Vec:{:?}", my_ints.capacity());


    println!("first item in vec is :{}",my_ints[0]);
    println!("first item in vec is :{:?}",&(&my_ints).as_slice()[0..5]);
    println!("first item in vec is :{:?}",&(&my_ints).as_slice()[1..=5]);

    println!("element at index 10 is {:?}",my_ints.get(10));
}