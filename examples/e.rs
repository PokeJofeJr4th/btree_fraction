use btree_fraction::{UFrac16, UFrac32, UFrac8};

fn main() {
    let e = UFrac8::try_from(std::f64::consts::E).unwrap();
    println!("{e} = {e:?} = {}", UFrac8::E);
    let e = UFrac16::try_from(std::f64::consts::E).unwrap();
    println!("{e} = {e:?} = {}", UFrac16::E);
    let e = UFrac32::try_from(std::f64::consts::E).unwrap();
    println!("{e} = {e:?} = {}", UFrac32::E);
}
