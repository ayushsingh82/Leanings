use closures::test_closures;
use crate::mystruct::{test_create_person,create_vehicle,create_vehicleTuple};

pub mod helper;
pub mod closures;
pub mod matchtest;
pub mod optiontest;
pub mod mystruct;
pub mod test_traits;
pub mod myvec;
pub mod myhashmap;
pub mod myhashset;
pub mod myiters;

fn main(){
    // println!("hello");

    // test_func();

    // let myresult:String=helper::namehelpers::get_full_name("Ayush","SINGH");
    // println!("Hello from {0}",myresult);

    // let new_age=helper::privatefns::get_age_plue_5(23);
    // println!("new age is {}",new_age);

    // test_if();

    // test_while();

    // test_loop();

    // test_for()

    // test_closures();

    // matchtest::test_match_int();
    // matchtest::test_match_string();
    // matchtest::test_match_array();


    
    // let result:Option<u8>=optiontest::test_option_types();
    // println!("{0}",result.unwrap());

    //   let strresult: Option<String>=optiontest::test_option_string();
    //   println!("Name is {}",strresult.unwrap());

    //   let charresult: Option<optiontest::CharacterType>=optiontest::test_option_chartype();

    //   if charresult.is_some(){
    //     println!("User has selected a chartype");
    //   }else{
    //     println!("Chartype is null");
    //   }


    //   println!("character type is{}",charresult.unwrap().to_string());
// mystruct::test_create_person();


// test_create_person();
// create_vehicle();
// create_vehicleTuple();

// test_traits::create_person();


// myvec::test_vec_int();
// myvec::test_vec_string();
// myvec::test_vec_car();

// myhashmap::test_hashmap_basic();
// myhashset::test_hashset_type();

myiters::test_rust_iterators();
     
    
}


fn test_if(){
    let age_to_drive=16u8;

    println!("Enter the age :");
    let myinput=&mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();
    //unwrap() is used to remove the worning of not using the fn coming back from read_line

    let age=myinput.replace("\n","").parse::<u8>().unwrap();
    if age > age_to_drive{
        print!("Issuing licese");
    }
    else if age==16 {
         println!("Wait one more year");
    }else{
        println!("Age less thn 16");
    }


    //short form is in below line
    let drivers_license:bool= if age>=16 {true} else {false};
    
}


fn test_loop(){
    let mut x=1;
    loop{
        println!("Hello from rust");

        if x>5 {
            break;
        }
        x+=1;
    }
}

fn test_for(){
    let ages:[i32;4]=[14,67,89,97];
    let age_to_drive=16i32;

    for value in ages {
        // println!("The current age is {0}",value );

        if value>= age_to_drive{
            println!("you can drive {0}",value);
        }else{
            println!("wait a little {0}",value);
        }
    }
}

#[allow(dead_code)]
fn test_while(){
  let age_to_drive=16u8;

  let mut current_age=0u8;
   
   while current_age < age_to_drive{
    println!("Waiting...");
    current_age+=1;

    if current_age==6{
        break;
    }
   }

}

#[allow(dead_code)]
fn test_func(){
    let x:f32=255.0;
    let y:i32= x as i32 -5;
    println!("{}",y);


    let mut iam:bool=true;
    iam=false;
    println!("{}",iam);

    let mystr:char='😊';
    println!("{}",mystr);

    let mut name:&str="ayush";
    name="singh";
    println!("{}",name);

    //in tuples u can have multiple data types
    let name1:(&str,&str,i32)=("ayush","singh",40);
   println!("{:?}",name1);

   // if default formatter error  {} --> {:?}

   let ages:[u16;7]=[40,30,50,60,70,80,90];
    println!("{:?}",ages);

    let new_ages:&[u16]=&ages[1..=4];
    println!("{:?}",new_ages);   
 
}




