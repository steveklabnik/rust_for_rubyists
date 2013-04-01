use task::spawn;

fn main() {

    for 10.times {
        do spawn {
            let greeting_message = "Hello?";
            io::println(greeting_message);
        }
    }
}
