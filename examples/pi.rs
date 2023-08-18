use btree_fraction::UFrac8;

fn main() {
    let pi = UFrac8::try_from(std::f32::consts::PI).unwrap();
    println!("{pi} = {pi:?}")
}
