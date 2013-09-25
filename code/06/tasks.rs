extern mod extra;
use extra::comm::DuplexStream;

fn plus_one(channel: &DuplexStream<int, int>) {
    let mut value: int;
    loop {
        value = channel.recv();
        channel.send(value + 1);
        if value == 0 { break; }
    }
}

fn main() {
    let (from_child, to_child) = DuplexStream();

    do spawn {
        plus_one(&to_child);
    }

    from_child.try_send(22);
    from_child.try_send(23);
    from_child.send(24);
    from_child.send(25);
    from_child.send(0);

    do 4.times {
        let answer = from_child.recv();
        println(answer.to_str());
    }
}
