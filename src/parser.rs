use std::env;
use std::fmt;

pub enum ReadError {
    TroopArgMissing,
    EnemyTroopArgMissing,
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReadError::TroopArgMissing => write!(f, "<TROOP> missing!"),
            ReadError::EnemyTroopArgMissing => write!(f, "<enemy_troop> missing!"),
        }
    }
}

#[derive(Default)]
pub struct InputData {
    pub troops: i32,
    pub enemyTroops: i32,
}

impl InputData {
    fn new() -> Self {
        Default::default()
    }
}

pub fn readInput() -> Result<InputData, ReadError> {
    let args: Vec<_> = env::args().collect();
    let troopsArg = args.get(1);
    let enemyTroopsArg = args.get(2);

    if troopsArg.is_none() {
        return Err(ReadError::TroopArgMissing);
    }
    if enemyTroopsArg.is_none() {
        return Err(ReadError::EnemyTroopArgMissing);
    }
    let mut inputData = InputData::new();
    inputData.troops = troopsArg.unwrap().parse::<i32>().unwrap_or(0);
    inputData.enemyTroops = enemyTroopsArg.unwrap().parse::<i32>().unwrap_or(0);

    Ok(inputData)
}
