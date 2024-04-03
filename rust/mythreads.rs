use std::thread::spawn;

pub fn test_threads(){
    let mut  x=0u128;
    for i in 1..500_000_000{
        x+=i;
    }
}

pub fn spawn_thread(){
     let thread_fn=  || {
        let mut  x=0u128;
        for i in 1..500_000_000{
            x+=i;
        }
        println!("\x1b[38;2;100;100;255mMain thread finished a little bit og work ..  let go check on worker threads.\x1b[0m] ");
     };

      println!("starting new worker thread...");
    let handle=spawn(thread_fn);
    let handle2=spawn(thread_fn);
    println!("worker thread completed...");

    // test_threads();

    // handle.join();
    // handle2.join();

    loop{
        test_threads();
        if handle.is_finished()  && handle2.is_finished(){
            println!("All the worker are done , lets get out of here!");
            break;
        }
     }
}