use btree_fraction::{UFrac16, UFrac8};

fn main() {
    let pi8 = UFrac8::try_from(std::f32::consts::PI).unwrap();
    println!("{pi8} = {pi8:?}");
    let pi16 = UFrac16::try_from(std::f32::consts::PI).unwrap();
    println!("{pi16} = {pi16:?} = {}", UFrac16::PI);
}
