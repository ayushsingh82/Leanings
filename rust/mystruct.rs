use std::cell::Cell;
//importing Cell type from cell module

#[derive(Debug)]
#[allow(dead_code)]

enum VehicleColor{
    Silver,
    Blue,
    Red,
    Green,
    White
}

#[derive(Debug)]
struct VehicleTuple(String,String,u16);

#[derive(Debug)]
struct Vehicle{
    manufacture:String,
    model:String,
    year:u16,
    color:VehicleColor,
}

impl Vehicle{
     fn  paint(&mut self,new_color:VehicleColor){
        self.color=new_color;
     }

     fn create_vehicle()->Vehicle{
        let new_vehicle=Vehicle{manufacture:"default".to_string(),model:"default".to_string(),year:1990,color:VehicleColor::Blue};
        return new_vehicle;
     }
}

 struct Person<'p>{
     first_name:Cell<&'p str>,
     last_name:String,
     birth_year:u16,
     birth_month:u8,
     meters_walked:u32,
}



pub fn create_vehicle(){
    // let myvehicle=new_vehicle();
    let mut myvehicle=Vehicle::create_vehicle();
    myvehicle.paint(VehicleColor::Red);
    println!("{:?}",myvehicle)
}

pub fn create_vehicleTuple(){
    println!("{:?}",new_vehicletuple());
    let myvehicletuple=new_vehicletuple();
    println!("Manufacturer:{0},model:{1}",myvehicletuple.0,myvehicletuple.1);
}

fn new_vehicletuple()->VehicleTuple{
  return VehicleTuple("Hyundai".to_string(),"Elantra".to_string(),2015);
}

fn new_vehicle()->Vehicle{
    let mut v1=Vehicle{manufacture:"Porsche".to_string(),model:"Cyane".to_string(),year:1993,color:VehicleColor::Red};
    v1.paint(VehicleColor::White);
    return v1;
}

fn new_person()->Person<'static>{
    let  p1=Person{first_name:Cell::from("Ayush"),last_name:"Singh".to_string(),
birth_year:2003,birth_month:7,meters_walked:0};
p1.first_name.set("Shannon");
    return p1;
}

pub fn test_create_person(){
    let myperson=new_person();
    println!("Fist name:{0} , last name:{1},birth month :{2},
    birth_year:{3},meters_walked:{4}",myperson.first_name.get(),myperson.last_name,myperson.birth_month,myperson.birth_year,myperson.meters_walked);
}