#[derive(Debug)]
enum Cereals {
    Barley, Millet, Rice, Rye, Spelt, Wheat
}

fn main() {
    let mut grains: Vec<Cereals> = vec![];
    grains.push(Cereals::Rye);

    drop(grains);

    println!("grains: {:?}", grains); // borrow of moved value: `grains`
}
