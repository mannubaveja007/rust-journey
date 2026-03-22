// use std::time::Duration;
// use tokio::time::sleep;
// use tokio::sync::mpsc; // multiple producer , single consumer




// async fn Name(){
//     sleep(Duration::from_secs(10)).await;  
//     println!("Mannu Baveja");
// }


// async fn GreetFromName(){
//     sleep(Duration::from_secs(15)).await; // greetings and all get the shit name from Name() func call
//     while let Some(msg) = rx.recv().await {
//         println!("Welcome, {} to tokio",msg);
//     }
// }



// #[tokio::main]
// async fn main() {
//     let (tx,mut rx) = mpsc::channel(32); // 32 = buffer size

//     tokio::spawn(async move{
//         // tx ix now inside this task of this particular block

//         for(i) in 1..=5{
//             tx.send(i).await.unwrap();
//             sleep(Duration::from_secs(1)).await;
//         }
//     });

//     while let Some(msg) = rx.recv().await {
//         println!("Welcome, {} to tokio",msg);
//     }

//     println!("Done!");
// }

// #[tokio::main]
// async fn main(){
//     let mut count = 0;
//     tokio::spawn(async move { count +=1; });
//     tokio::spawn(async move {count += 1; });

//     println!("{}",count);   
// }

// deadlock occured in this as both of those child process are trying to update the 
// same count variable which voilates the rust ownership thing totally



// use std::sync::Mutex;
// use std::sync::Arc;
// #[tokio::main]
// async fn main(){
//     // we will see in this the locked box The MUTEX only 
//     // one person can open it at a time everyone else waits for it
//     let count = Arc::new(Mutex::new(0)); 
//     // zero isn ot locked inside this locked box
//     // which only one can access it at the time
//     let shared = Arc::clone(&count);
//     let shared2 = Arc::clone(&count);
//     let mut n = count.lock().unwrap(); // grab the key
//     *n +=1;


// }


// use std::sync::Mutex;
// use std::sync::Arc;
// #[tokio::main]
// async fn main(){
//     let counter = Arc::new(Mutex::new(0));
//     let mut handlees = vec![];

//     for i in 0..5{
//         // println!("{}",i);
//         let c = Arc::clone(&counter); // give each task it's own link
//         let h = tokio::spawn(async move{
//             let mut num = c.lock().unwrap(); // grab the key
//             *num +=1; // the real increment happens here
//             println!("Task {} set Counter to {}",i , *num);
//         });
//         handlees.push(h);
//     }

//     for h in handlees{ h.await.unwrap();} // wait for all
//     println!("Final : {}",counter.lock().unwrap());

// }


// use tokio::time::{sleep,Duration};
// select waut for whoever finishes first

// use std::sync::Mutex;
// use std::sync::Arc;
// #[tokio::main]
// async fn main(){
//     tokio::select! {
//         _ = sleep(Duration::from_secs(5)) => println!("5s timer won"),
//         _ = sleep(Duration::from_secs(2)) => println!("2s timer won"),
//     }
// }

// use std::sync::Mutex;
// use std::sync::Arc;



// async fn server_1(){
//     sleep(Duration::from_secs(3)).await;
//     // println!("Response from 1");
// }


// async fn server_2(){
//     sleep(Duration::from_secs(1)).await;
//     // println!("Response from 2");
// }

// async fn server_3(){
//     sleep(Duration::from_secs(2)).await;
//     // println!("Response from 3");
// }


// #[tokio::main]
// async fn main(){
//     // inner stuff
//     tokio::select! {
//         result1 = server_1() => println!("Response from 1"),    
//         result2 = server_2() => println!("Response from 2"),
//         result3 = server_3() => println!("Response from 3"),
//     }
// }

// async fn server_1(){
//     sleep(Duration::from_secs(3)).await;
//     // println!("Response from 1");
// }

// use std::sync::Mutex;
// use std::sync::Arc;
// use tokio::time::{timeout};

// #[tokio::main]
// async fn main(){
//     let result = timeout(Duration::from_secs(2),server_1()).await;
//     match result{
//         Ok(val) => println!("Got it : {:?}",val),
//         Err(_) => println!("timed out! Gave up!"),
//     }
// }


// use tokio::sync::Semaphore;
// use std::sync::Arc;

// #[tokio::main]
// async fn main(){
//     let sem = Arc::new(Semaphore::new(2)); // only 2 tickets are allowed
    
//     for i in 1..5{
//         let permit = sem.acquire().await.unwrap();
//         sleep(Duration::from_secs(1)).await;
//         println!("Task {} done", i);
//         drop(permit);
//     }
//     println!("Permit Acquired!");
// }


// use std::time::Duration;
// use tokio::time::sleep;
// use tokio::sync::{mpsc, Semaphore};
// use std::sync::Arc;

// struct Job{
//     id : u32,
//     duration_secs : u64,
// }

// #[tokio::main]
// async fn main() {
//     let (tx,mut rx) = mpsc::channel::<Job>(32); // an channel for jobs to share the data
//     // use mutex to share the data


//     // 1. producer : spawn a background task to generate jobs
   
//     tokio::spawn(async move{
//         for i in 1..=8{
//             let job = Job {
//                 id : i,
//                 duration_secs : (i*3) as u64 +1, // 1 , 2 ,3 seconds
//             };
//         println!("Queueing Job {}" , job.id);
//         tx.send(job).await.unwrap();
//         }
//     });

//     // this was the producer part for producing the thing

//     // 2. Consumer & rate limiter 

//     let sem = Arc::new(Semaphore::new(3)); // only 3 jobs at a time

//     while let Some(job) = rx.recv().await {
//         let permit = Arc::clone(&sem).acquire_owned().await.unwrap();
        
//         tokio::spawn(async move{
//             println!("Working on job {} for {}",
//             job.id,
//             job.duration_secs);
//             sleep(Duration::from_secs(job.duration_secs)).await;
//             println!("Job {} Done",job.id);

//             drop(permit)
//         });
//     }
    
    
//     sleep(Duration::from_secs(4)).await;
//     println!("All jobs are processed!");
// }


// use std::time::Duration;
// use tokio::time::sleep;
// use tokio::sync::{mpsc, Semaphore};
// use std::sync::Arc;

// struct Job {
//     id : u64,
//     duration_secs : u64,
// }


// #[tokio::main]
// async fn main(){
//     // setting up the stage

//     let (tx , mut rx) = mpsc::channel::<Job>(32);
//     let sem = Arc::new(Semaphore::new(3)); // permit and drop shit to make

//     // spawning the generator

//     tokio::spawn(async move{
//         for i in 1..=8{
//             tx.send(Job { id : i , duration_secs : (i%3) as u64 +1}).await.unwrap(); // put jobs in channel
//         }
//     });

//     // pull jobs from channel
//     while let Some(job) = rx.recv().await {
//         // 1. We got a job! Now we wait for a free permit (machine)
//         let permit = Arc::clone(&sem).acquire_owned().await.unwrap();

//         // 2. We have a permit! Spawn a worker to do the job.
//         tokio::spawn(async move {
//             println!("Working on job {} (takes {}s)", job.id, job.duration_secs);
            
//             // Do the actual work
//             sleep(Duration::from_secs(job.duration_secs)).await;
            
//             println!("Job {} is done!", job.id);

//             // 3. Return the permit so the next job can start
//             drop(permit);
//         });
//     }

//     // Wait 4 seconds at the very end to let the last workers finish printing
//     sleep(Duration::from_secs(4)).await;
//     println!("All jobs processed!");
// }


