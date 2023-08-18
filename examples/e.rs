use btree_fraction::UFrac8;

fn main() {
    let e = UFrac8::try_from(std::f32::consts::E).unwrap();
    println!("{e} = {e:?}")
}
