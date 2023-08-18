use btree_fraction::UFrac8;

fn main() {
    let pi = UFrac8::try_from(1.618).unwrap();
    println!("{pi} = {pi:?}")
}
