enum Monster {
    ScubaArgentine(int, int, int, int),
    IndustrialRaverMonkey(int, int, int, int)
}


impl Monster {
    fn attack(&self) {
        match *self {
            ScubaArgentine(l, s, c, w) => println!("The monster attacks for {:d} damage.", w),
            IndustrialRaverMonkey(l, s, c, w) => println!("The monster attacks for {:d} damage.", w)
        }
    }
}

fn main() {
    let irm = IndustrialRaverMonkey(46, 35, 91, 2);
    irm.attack();
}

