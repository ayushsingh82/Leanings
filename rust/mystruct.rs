#[allow(dead_code)]

pub struct Person{
    pub first_name:String,
    pub last_name:String,
    pub birth_year:u16,
    pub birth_month:u8,
}

pub fn new_person()->Person{
    let p1=Person{first_name:"Ayush".to_string(),last_name:"Singh".to_string(),
birth_year:2003,birth_month:7};
    return p1;
}