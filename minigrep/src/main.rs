use std::{env, process,thread};
use std::io::SeekFrom::Start;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

// use minigrep::Config;
// use minigrep::run;

fn main() {
    // let args: Vec<String> = env::args().collect();
    //
    // let config = Config::build(&args).unwrap_or_else(|err|{
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    //
    // if let Err(e) = run(config){
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();


    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    thread::spawn(move ||{
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx{
        println!("Got: {}",received);
    }
}

fn multiple_thread(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter=Arc::clone(&counter);
        let handle = thread::spawn(move ||{
           let mut num = counter.lock().unwrap();

            *num +=1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result:: {}",*counter.lock().unwrap());
}