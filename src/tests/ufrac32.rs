use std::cmp::Ordering;

use crate::UFrac32;

#[test]
fn to_fraction() {
    assert_eq!(UFrac32::INFINITY.to_fraction(), (1, 0));
    assert_eq!(UFrac32::ONE.to_fraction(), (1, 1));
    assert_eq!(UFrac32::from_bits(0x0800_0000).to_fraction(), (1, 2));
    assert_eq!(UFrac32::from_bits(0x0800_0001).to_fraction(), (2, 1));
    assert_eq!(UFrac32::from_bits(0x2800_001f).to_fraction(), (6, 1));
    assert_eq!(UFrac32::from_bits(0x3000_003f).to_fraction(), (7, 1));
    assert_eq!(UFrac32::from_bits(0x2800_0000).to_fraction(), (1, 6));
    assert_eq!(UFrac32::from_bits(0x3000_0000).to_fraction(), (1, 7));
    assert_eq!(UFrac32::from_bits(0x2000_0002).to_fraction(), (4, 7));
    assert_eq!(UFrac32::from_bits(0x2000_0005).to_fraction(), (8, 5));
    assert_eq!(UFrac32::from_bits(0x1800_0005).to_fraction(), (5, 3));
}

#[test]
fn invert() {
    assert_eq!(UFrac32::ZERO.invert().to_fraction(), (1, 0));
    assert_eq!(UFrac32::ONE.invert().to_fraction(), (1, 1));
    assert_eq!(UFrac32::INFINITY.invert().to_fraction(), (0, 1));
    assert_eq!(
        UFrac32::from_bits(0x2000_0002).invert().to_fraction(),
        (7, 4)
    );
}

#[test]
fn from_u8() {
    for i in 0..=6 {
        assert_eq!(UFrac32::try_from(i).unwrap().to_fraction(), (i, 1));
    }
}

#[test]
fn from_f64() {
    assert_eq!(UFrac32::try_from(1.0).unwrap().to_fraction(), (1, 1));
    assert_eq!(UFrac32::try_from(0.5).unwrap().to_fraction(), (1, 2));
    println!("{}", UFrac32::try_from(1.618).unwrap());
    assert_eq!(UFrac32::try_from(1.618).unwrap().to_fraction(), (521, 322));
}

#[test]
fn ordering() {
    println!(
        "{}({0:?}) < {}({1:?})",
        UFrac32::try_from(2).unwrap(),
        UFrac32::try_from(3).unwrap()
    );
    assert_eq!(
        UFrac32::try_from(2)
            .unwrap()
            .cmp(&UFrac32::try_from(3).unwrap()),
        Ordering::Less
    );
    println!(
        "{} > {}",
        UFrac32::try_from(4).unwrap(),
        UFrac32::try_from(3).unwrap()
    );
    assert_eq!(
        UFrac32::try_from(4)
            .unwrap()
            .cmp(&UFrac32::try_from(3).unwrap()),
        Ordering::Greater
    );
    println!("{:?} = {0}", UFrac32::try_from(1.20001).unwrap());
    assert_eq!(
        UFrac32::try_from(1.2)
            .unwrap()
            .cmp(&UFrac32::try_from(1.20001).unwrap()),
        Ordering::Less
    );
    println!("{:?} = {0}", UFrac32::try_from(2).unwrap());
    println!("{:?} = {0}", UFrac32::try_from(1.9).unwrap());
    assert_eq!(
        UFrac32::try_from(2)
            .unwrap()
            .cmp(&UFrac32::try_from(1.9).unwrap()),
        Ordering::Greater
    );
}
