mod parser;
mod risk;
use parser::InputData;
use risk::Risk;

fn makeRiskInstance(inputData: InputData) -> Risk {
    let risk = Risk::new(inputData.troops, inputData.enemyTroops);
    risk
}

fn run(inputData: InputData) {
    println!("Calculating...");
    let mut risk = makeRiskInstance(inputData);
    risk.printStats();
}

fn main() {
    println!("---Risk calculator---");
    match parser::readInput() {
        Ok(inputData) => run(inputData),
        Err(e) => println!("{}", e),
    }
}
