use btree_fraction::{UFrac16, UFrac8};

fn main() {
    let e = UFrac8::try_from(std::f32::consts::E).unwrap();
    println!("{e} = {e:?}");
    let e = UFrac16::try_from(std::f32::consts::E).unwrap();
    println!("{e} = {e:?} = {}", UFrac16::E);
}
