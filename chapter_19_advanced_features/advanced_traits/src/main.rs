use std::ops::Add;

#[derive(Debug, Clone)]
struct Millimeters(u32);
#[derive(Debug, Clone)]
struct Centimeters(u32);
#[derive(Debug, Clone)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Centimeters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Centimeters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 10))
    }
}

fn main() {
    let milli_size: Millimeters = Millimeters(300);

    let centi_size: Centimeters = Centimeters(15);

    let meters: Meters = Meters(3);

    println!(
        "{:?} milllimeters + {:?} meters = {:?}\n",
        milli_size.clone(),
        meters.clone(),
        milli_size.clone().add(meters)
    );
    println!(
        "{:?} milllimeters + {:?} centimeters = {:?}\n",
        milli_size.clone(),
        centi_size.clone(),
        milli_size.add(centi_size)
    );
}
