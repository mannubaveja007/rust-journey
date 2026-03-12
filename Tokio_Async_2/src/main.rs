use tokio::time::{sleep, Duration};

async fn task1(){
    println!("Task 1 started");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 is completed after 2 seconds!");
}

async fn task2(){
    println!("Task 2 started");
    sleep(Duration::from_secs(10)).await;
    println!("task 2 is completed after 10 seconds")
}



async fn task3(){
    println!("Task 3 started");
    sleep(Duration::from_secs(0)).await;
    println!("Task 3 is completed hehe fast as you think!");
}
#[tokio::main]
async fn main(){
    let t1 = task1();
    let t2 = task2();
    let t3 = task3();
    tokio::join!(t1,t2,t3);
}


//  what it does it it starts both of the task at the same time and then whenever the task is done it just shows it up in the output and it can use struck like Result for better error handling i guess


