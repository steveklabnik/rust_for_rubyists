fn main() {
    do 10.times {
        do spawn {
            let greeting_message = "Hello?";
            println(greeting_message);
        }
    }
}

