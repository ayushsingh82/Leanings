struct Person {
    first_name: String,
}

pub fn test_thread_variables() {
    let age = 34;
    let person01 = Person { first_name: String::from("Trevour") };

    let print_age =  || {
        println!("Your age is {}", age);
        println!("Your name is: {}", &person01.first_name);
    };

    let _result = std::thread::spawn(print_age).join();

    std::thread::scope(|scope: &Scope<'_, '_>| {
        scope.spawn(print_age);
    });

    println!("Your age is {}", age);
    println!("Your name is: {}", person01.first_name);

    println!("Finished printing age");
}
