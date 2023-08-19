use btree_fraction::{UFrac16, UFrac32, UFrac8};

fn main() {
    println!("{}", std::f64::consts::PI);
    let pi8 = UFrac8::try_from(std::f64::consts::PI).unwrap();
    println!("{pi8} = {pi8:?} = {}", UFrac8::PI);
    let pi16 = UFrac16::try_from(std::f64::consts::PI).unwrap();
    println!("{pi16} = {pi16:?} = {}", UFrac16::PI);
    let pi32 = UFrac32::try_from(std::f64::consts::PI).unwrap();
    println!("{pi32} = {pi32:?} = {}", UFrac32::PI);

    let pi_minus_3_32 = UFrac32::try_from(std::f64::consts::PI - 3.0).unwrap();
    println!("{pi_minus_3_32} = {pi_minus_3_32:?}");
}
