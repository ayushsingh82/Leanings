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

    let mut car_lot2:Vec<Car>=vec![];

   for _ in 1..10u8{
    car_list.push(Car{manufacturer:"Porshe".to_string(),model:"Cyane".to_string()});
   }

   for _ in 1..10u8{
    car_lot2.push(Car{manufacturer:"Hyundai".to_string(),model:"Sonata".to_string()});
   }

   car_list.append(&mut car_lot2);

   car_list.insert(0,Car{manufacturer:"Lambo".to_string(),model:"Urus".to_string()});

   car_list.remove(0);

//    let keep=|e:&Car| {if e.manufacturer=="Porsche" {return true;} else {return  false;}};
//    car_list.retain(keep);

car_list.reserve(500);

    println!("{:?}",car_list);
    println!("{:?}",car_list.len());
    println!("{:?}",car_lot2.len());

     println!("{:?}",car_list.get(0).unwrap()); 

}