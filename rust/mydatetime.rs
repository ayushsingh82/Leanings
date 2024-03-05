// extern crate chrono;
use chrono::{Utc, Duration as ChronoDuration};

use std::{ops::Sub, time::{Duration,Instant}};

pub fn test_stdtime(){
     let dur1= Duration::from_secs(15);
     println!("{}",dur1.as_millis());

     let dur2=Duration::from_millis(55500);
    //  let dur3=dur1.sub(dur2);

    //  println!("{}",dur3.as_millis());

     let dur4=dur1.checked_sub(dur2);
     println!("{}",dur4.unwrap_or_default().as_millis());

     let now=Instant::now();

     std::thread::sleep(Duration::from_millis(200));

     println!("Elapsed time is {}",now.elapsed().as_micros());
 
}

pub fn test_chrono(){
      let utc_now=chrono::Utc::now();
      println!("{}",utc_now.format("%Z %Y %b %d %H"));

    let local_time=chrono::Local::now();
    println!("{}",local_time.format("%Z %Y %b %d %H"));
}