mod dice;
use dice::Dice;

#[derive(Debug, Copy, Clone)]
pub struct Risk {
    troops: i32,
    enemyTroops: i32,
}

impl Risk {
    pub fn new(troops: i32, enemyTroops: i32) -> Self {
        Risk {
            troops: troops,
            enemyTroops: enemyTroops,
        }
    }

    pub fn setTroops(&mut self, troops: i32) {
        self.troops = troops;
    }

    fn getAttackRolls(troops: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        if troops > 1 {
            vec.push(Dice::new().eyes);
        }
        if troops > 2 {
            vec.push(Dice::new().eyes);
        }
        if troops > 3 {
            vec.push(Dice::new().eyes);
        }
        vec.sort();
        vec.reverse();
        vec
    }

    fn getDefenceRolls(troops: i32) -> Vec<i32> {
        let mut vec = Vec::new();
        vec.push(Dice::new().eyes);
        if troops > 1 {
            vec.push(Dice::new().eyes);
        }
        vec.sort();
        vec.reverse();
        vec
    }

    fn simulateAttack(&mut self) {
        let attackRolls = Risk::getAttackRolls(self.troops);
        let defenceRolls = Risk::getDefenceRolls(self.enemyTroops);
        if (attackRolls[0] > defenceRolls[0]) {
            self.enemyTroops -= 1;
        } else {
            self.troops -= 1;
        }
        if defenceRolls.len() > 1 && attackRolls.len() > 1 {
            if (attackRolls[1] > defenceRolls[1]) {
                self.enemyTroops -= 1;
            } else {
                self.troops -= 1;
            }
        }
    }

    fn simulateBlitz(mut self) -> bool {
        while self.troops > 1 && self.enemyTroops >= 1 {
            self.simulateAttack();
        }
        self.troops > 1
    }

    fn getWinPercentage(self) -> f32 {
        let numBattles = 100000;
        let mut numWins = 0;
        for n in 0..numBattles {
            if Risk::simulateBlitz(self.clone()) {
                numWins += 1;
            }
        }
        (numWins as f32 / numBattles as f32) * 100.0
    }

    pub fn printStats(&mut self) {
        let winPercentage: f32 = Risk::getWinPercentage(self.clone());
        println!("Win chance: {:.2}", winPercentage);
    }
}
