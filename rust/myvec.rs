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

pub fn test_vec_string(){
    let first_names:Vec<&str>=vec!["Trevour","Nancy","Shanon","Billy"];

    for first_name in first_names.clone() {
        println!("Processing {}...",first_name);
    }

    println!("{:?}",first_names);
}

#[derive(Debug)]
struct Car{
    manufacturer:String,
    model:String
}

pub fn test_vec_car(){
    let mut car_list:Vec<Car>=vec![];

   for _ in 1..10u8{
    car_list.push(Car{manufacturer:"Porshe".to_string(),model:"Cyane".to_string()});
   }

    println!("{:?}",car_list);
    println!("{:?}",car_list.len());
    println!("{:?}",car_list.capacity());
}