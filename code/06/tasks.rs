use std::comm::{channel, Sender, Receiver};

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
    let mut value: int;
    loop {
        value = receiver.recv();
        sender.send(value + 1);
        if value == 0 { break; }
    }
}

fn main() {
    let (from_parent_sender, from_parent_receiver) = channel();
    let (from_child_sender, from_child_receiver) = channel();

    spawn(proc() {
        plus_one(&from_child_sender, &from_parent_receiver);
    });

    from_parent_sender.send(22);
    from_parent_sender.send(23);
    from_parent_sender.send(24);
    from_parent_sender.send(24);

    from_parent_sender.send(0);

    for _ in range(0i, 4) {
        let answer = from_child_receiver.recv();
        println!("{:s}", answer.to_string());
    }
}
