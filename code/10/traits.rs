trait Monster {
    fn attack(&self);
    fn new() -> Self;
}

struct IndustrialRaverMonkey {
    life: int,
    strength: int,
    charisma: int,
    weapon: int,
}

struct DwarvenAngel {
    life: int,
    strength: int,
    charisma: int,
    weapon: int,
}

struct AssistantViceTentacleAndOmbudsman {
    life: int,
    strength: int,
    charisma: int,
    weapon: int,
}

struct TeethDeer {
    life: int,
    strength: int,
    charisma: int,
    weapon: int,
}

struct IntrepidDecomposedCyclist {
    life: int,
    strength: int,
    charisma: int,
    weapon: int,
}

struct Dragon {
    life: int,
    strength: int,
    charisma: int,
    weapon: int,
}

impl Monster for IndustrialRaverMonkey {
    fn attack(&self) {
        println!("The monkey attacks for {:d}.", self.strength)
    }

    fn new() -> IndustrialRaverMonkey {
        IndustrialRaverMonkey { life: 46, strength: 35, charisma: 91, weapon: 2 }
    }
}

impl Monster for DwarvenAngel {
    fn attack(&self) {
        println!("The angel attacks for {:d}.", self.strength)
    }
    fn new() -> DwarvenAngel {
        DwarvenAngel { life: 540, strength: 6, charisma: 144, weapon: 50 }
    }
}

impl Monster for AssistantViceTentacleAndOmbudsman {
    fn attack(&self) {
        println!("The tentacle attacks for {:d}.", self.strength)
    }
    fn new() -> AssistantViceTentacleAndOmbudsman {
        AssistantViceTentacleAndOmbudsman { life: 320, strength: 6, charisma: 144, weapon: 50 }
    }
}

impl Monster for TeethDeer {
    fn attack(&self) {
        println!("The deer attacks for {:d}.", self.strength)
    }
    fn new() -> TeethDeer {
        TeethDeer { life: 655, strength: 192, charisma: 19, weapon: 109 }
    }
}

impl Monster for IntrepidDecomposedCyclist {
    fn attack(&self) {
        println!("The cyclist attacks for {:d}.", self.strength)
    }
    fn new() -> IntrepidDecomposedCyclist {
        IntrepidDecomposedCyclist { life: 901, strength: 560, charisma: 422, weapon: 105 }
    }
}

impl Monster for Dragon {
    fn attack(&self) {
        println!("The dragon attacks for {:d}.", self.strength)
    }
    fn new() -> Dragon {
        Dragon { life: 1340, strength: 451, charisma: 1020, weapon: 939 }
    }
}

fn monsters_attack(monsters: &[&Monster]) {
    for monster in monsters.iter() {
        monster.attack();
    }
}

fn main() {
    let monkey: &IndustrialRaverMonkey               = &Monster::new();
    let angel: &DwarvenAngel                         = &Monster::new();
    let tentacle: &AssistantViceTentacleAndOmbudsman = &Monster::new();
    let deer: &TeethDeer                             = &Monster::new();
    let cyclist: &IntrepidDecomposedCyclist          = &Monster::new();
    let dragon: &Dragon                              = &Monster::new();

    let dwemthys_vector: &[&Monster] = [monkey as &Monster, angel as &Monster, tentacle as &Monster, deer as &Monster, cyclist as &Monster, dragon as &Monster];

    monsters_attack(dwemthys_vector);
}
