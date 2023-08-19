use std::cmp::Ordering;

use crate::UFrac8;

#[test]
fn to_fraction() {
    assert_eq!(UFrac8::ONE.to_fraction(), (1, 1));
    assert_eq!(UFrac8::from_bits(0b0000_0010).to_fraction(), (1, 2));
    assert_eq!(UFrac8::from_bits(0b0000_0011).to_fraction(), (2, 1));
    assert_eq!(UFrac8::from_bits(0b0011_1111).to_fraction(), (6, 1));
    assert_eq!(UFrac8::from_bits(0b0111_1111).to_fraction(), (7, 1));
    assert_eq!(UFrac8::from_bits(0b0010_0000).to_fraction(), (1, 6));
    assert_eq!(UFrac8::from_bits(0b0100_0000).to_fraction(), (1, 7));
    assert_eq!(UFrac8::from_bits(0b0001_0010).to_fraction(), (4, 7));
    assert_eq!(UFrac8::from_bits(0b0001_0101).to_fraction(), (8, 5));
    assert_eq!(UFrac8::from_bits(0b0000_1101).to_fraction(), (5, 3));
}

#[test]
fn invert() {
    assert_eq!(UFrac8::ZERO.invert().to_fraction(), (8, 1));
    assert_eq!(UFrac8::ONE.invert().to_fraction(), (1, 1));
    assert_eq!(
        UFrac8::from_bits(0b0001_0010).invert().to_fraction(),
        (7, 4)
    );
}

#[test]
fn from_u8() {
    for i in 0..8 {
        assert_eq!(UFrac8::try_from(i).unwrap().to_fraction(), (i, 1));
    }
}

#[test]
fn from_f64() {
    assert_eq!(UFrac8::try_from(1.0).unwrap().to_fraction(), (1, 1));
    assert_eq!(UFrac8::try_from(0.5).unwrap().to_fraction(), (1, 2));
    println!("{}", UFrac8::try_from(1.618).unwrap());
    assert_eq!(UFrac8::try_from(1.618).unwrap().to_fraction(), (21, 13));
}

#[test]
fn ordering() {
    println!(
        "{}({0:?}) < {}({1:?})",
        UFrac8::try_from(2).unwrap(),
        UFrac8::try_from(3).unwrap()
    );
    assert_eq!(
        UFrac8::try_from(2)
            .unwrap()
            .cmp(&UFrac8::try_from(3).unwrap()),
        Ordering::Less
    );
    println!(
        "{} > {}",
        UFrac8::try_from(4).unwrap(),
        UFrac8::try_from(3).unwrap()
    );
    assert_eq!(
        UFrac8::try_from(4)
            .unwrap()
            .cmp(&UFrac8::try_from(3).unwrap()),
        Ordering::Greater
    );
    println!("{:?} = {0}", UFrac8::try_from(1.9).unwrap());
    assert_eq!(
        UFrac8::try_from(2)
            .unwrap()
            .cmp(&UFrac8::try_from(1.9).unwrap()),
        Ordering::Greater
    );
}
