use std::thread;
use std::time::Duration;

use futures::executor::ThreadPool;
use std::io::Read;

fn foo(id: i32) {
    for i in 0..10 {
        println!("thread #{}  count{}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn sample_18_2_1() {
    println!("sample_18_2_1 start");
    /*
    let  h1 = thread::spawn( || {foo(10);});
    let  h2 = thread::spawn( || {foo(20);});
    let  h3 = thread::spawn( || {foo(30);});
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    */
    thread::spawn(|| {
        foo(10);
    })
    .join()
    .unwrap();
    thread::spawn(|| {
        foo(20);
    })
    .join()
    .unwrap();
    thread::spawn(|| {
        foo(30);
    })
    .join()
    .unwrap();

    println!("sample_18_2_1 end");
}
async fn foo2(id: i32) {
    for i in 0..10 {
        println!("thread #{}  count{}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

async fn sub() {
    foo2(10).await;
    foo2(20).await;
    foo2(30).await;
}

fn sample_18_2_2() {
    println!("sample_18_2_2 start");
    println!("program start.");
    futures::executor::block_on(sub());
    println!("program end");
    println!("sample_18_2_2 end");
}

/* */
fn thread_closure() {
    println!("thread closure");
    let task = || {
        for i in 0..10 {
            println!("thread #1 count{}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    };
    let handle1 = thread::spawn(task);
    handle1.join().unwrap();
    println!("thread closure end");
}
fn thread_sample() {
    println!("multiple thread");
    let handle1 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count{}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #2 count{}.", i);
            thread::sleep(Duration::from_millis(1500));
        }
    });
    let handle3 = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #3 count{}.", i);
            thread::sleep(Duration::from_millis(2000));
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}

fn sample_18_3() {
    println!("[START] sample_18_3");
    let pool = ThreadPool::new().unwrap();
    let task = async {
        let mut id = 0;
        for j in 1..6 {
            id = id + 10;
            //let id = j * 10;
            pool.spawn_ok(async move {
                for i in 1..10 {
                    println!("thread #{} count{}", id, i);
                    id = id + 1;
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            thread::sleep(Duration::from_millis(500));
        }
    };
    println!("program start");
    futures::executor::block_on(task);
    std::io::stdin().read(&mut [0]);
    println!("program end");
    println!("[END] sample_18_3");
}

//
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count{}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("press enter key");
    //std::io::stdin().read(&mut [0]);
    handle.join().unwrap();

    thread_sample();
    thread_closure();
    sample_18_2_1();
    sample_18_2_2();
    sample_18_3();
    println!("program end");
}
