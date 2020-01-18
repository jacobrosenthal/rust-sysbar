use std::process;
use std::sync::mpsc::channel;
use std::{thread, time};

enum Status {
    Quit,
}

fn main() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        //ctrlc appears to be killing child process fine as well
        let mut handle = process::Command::new("caffeinate")
            .arg("-d")
            .spawn()
            .expect("failed to execute process");

        loop {
            thread::sleep(time::Duration::from_millis(1000));

            if rx.recv().is_ok() {
                handle.kill().unwrap();
                process::exit(0);
            }
        }
    });

    let mut menubar = sysbar::Sysbar::new("C");

    //built in quit terminates from objc, and doesnt clean up
    //so build our own
    menubar.add_item(
        "Quit",
        Box::new(move || {
            tx.send(Status::Quit).unwrap();
        }),
    );

    //menubar is blocking and needs to be on the main thread
    menubar.display();
}
