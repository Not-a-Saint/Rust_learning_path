use crate::triangle::Triangle;

mod triangle;

#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    age: i32,
    balance: f64,
}


fn main() {
    // let tr1 = Triangle {
    //     cat1: 6.0,
    //     cat2: 8.0,
    // };

    // let hyp = tr1.find_hyp();
    // println!("{hyp}");
    // let area = tr1.find_area();
    // println!("{area}");

    let isc_tr = Triangle::create_isc(5.0);
    println!("{:?}", isc_tr);

    let hyp = isc_tr.find_hyp();

    let area = isc_tr.find_area();

    println!("{hyp} {area}")
}