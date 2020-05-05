use std::thread; // for spawing new threads
use std::sync::mpsc; // for "tributary channels": Multiple Producer Single Consumer


fn main() {
    println!("Hello, world!");

    let (tx, rx) = mpsc::channel();
    let ns: String = String::from("shibboleth!");

    let handle = thread::spawn(move || { // think of move as "take ownership of the things you use here"
        let msg: String = String::from("Helo from this spawned thread!");
        tx.send(msg).unwrap();
    });

    println!("The string {} was never moved so it is still in scope.", ns);



    let msg_rec = rx.recv().unwrap();
    println!("{}", msg_rec);

    // the program seems to execute okay most the time without the handle.join() below,
    //    however I want to ensure the code in the spawned thread is executed
    handle.join().unwrap(); // block until the spawned thread is done
}
