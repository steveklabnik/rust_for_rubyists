use std::thread::Thread;

fn main() {
    for _ in range(0u, 10) {
        let _ = Thread::spawn(move ||) {
            let greeting_message = "Hello?";
            println!("{}", greeting_message);
        });
    }
}
