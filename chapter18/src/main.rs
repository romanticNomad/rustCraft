#[allow(dead_code)]
mod average;
use average::AvgCollection;

#[allow(dead_code)]
mod gui;

fn main() {
    let mut data1 = AvgCollection::new(vec![5,78,1,2,9]);
    dbg!(&data1);
    data1.del();
    dbg!(&data1);
    data1.stock(15);
    dbg!(&data1);
}
