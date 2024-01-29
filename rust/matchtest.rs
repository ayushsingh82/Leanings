pub fn test_match_string()->u32 {
        let car_manufacture:&str="Porche";

        match car_manufacture{
            "Hyundai"=>30000,
            "Posrche"=>90000,
            _ => 0
        }
}

 pub fn test_match_array(){
    let prices:[u32;4]=[3000,5000,9000,12000];

    match prices[0..=1]{
        [3000,5000]=>println!("You have some reasonably priced cars"),
        [3000,5000,..]=>println!("you have variety"),
        _=>println!("You don't have any priced car")
    }
 }


pub fn test_match_int(){

    // let myage:u16=35;


    // match myage{
    //    35 =>{
    //     println!("your age is 35");
    //    }
    //    156 =>{
    //     println!("your age is 156");
    //    }
    //    _ =>{
    //     println!("your age is not 35");
    //    }
    // }


    // match myage{
    //     1..=35 => println!("your age is 35"),
    //     150.. =>  println!("your age is above 150"),
    //     _ =>{
    //      println!("your age is something else");
    //     }
    //  }



    let  myage:u16=1;

    let y:u8=7;

    // match myage{
      
    //     1..=35 =>{
    //      println!("your age is 35");
    //     }
    //     0 | 5=> {
    //         println!("you are new ")
    //     }
    //     150.. =>{
    //      println!("your age is above 150");
    //     }
    //     _ =>{
    //      println!("your age is something else");
    //     }
    //  }

    match myage{
      
        1..=35 if y==5 => println!("your age is 35 and y is 5"),
        1..=35 if y!=5 => println!("you are upto 35 and y  is not 5 "),
        1..=35=>println!("you are upto 35 and y is not defined"),
        0=>println!("new born"),
        36..=149  => println!("you are above 35"),
        150.. =>{
         println!("your age is above 150");
        }
      
     }

}