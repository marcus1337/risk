use rand::Rng;

pub struct Dice{
    pub eyes : i32
}

impl Dice{

    pub fn new() -> Self {
        let mut dice = Dice{eyes : 1};
        dice.roll();
        dice
    }

    pub fn roll(&mut self){
        let eyes = rand::thread_rng().gen_range(1..7);
        self.eyes = eyes;
    }
}


