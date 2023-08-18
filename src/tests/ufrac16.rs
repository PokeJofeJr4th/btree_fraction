use std::cmp::Ordering;

use crate::UFrac16;

#[test]
fn to_fraction() {
    assert_eq!(UFrac16::INFINITY.to_fraction(), (1, 0));
    assert_eq!(UFrac16::ONE.to_fraction(), (1, 1));
    assert_eq!(UFrac16::from_bits(0x1000).to_fraction(), (1, 2));
    assert_eq!(UFrac16::from_bits(0x1001).to_fraction(), (2, 1));
    assert_eq!(UFrac16::from_bits(0x501f).to_fraction(), (6, 1));
    assert_eq!(UFrac16::from_bits(0x603f).to_fraction(), (7, 1));
    assert_eq!(UFrac16::from_bits(0x5000).to_fraction(), (1, 6));
    assert_eq!(UFrac16::from_bits(0x6000).to_fraction(), (1, 7));
    assert_eq!(UFrac16::from_bits(0x4002).to_fraction(), (4, 7));
    assert_eq!(UFrac16::from_bits(0x4005).to_fraction(), (8, 5));
    assert_eq!(UFrac16::from_bits(0x3005).to_fraction(), (5, 3));
}

#[test]
fn invert() {
    assert_eq!(UFrac16::ZERO.invert().to_fraction(), (1, 0));
    assert_eq!(UFrac16::ONE.invert().to_fraction(), (1, 1));
    assert_eq!(UFrac16::INFINITY.invert().to_fraction(), (0, 1));
    assert_eq!(UFrac16::from_bits(0x4002).invert().to_fraction(), (7, 4));
}

#[test]
fn from_u8() {
    for i in 0..=6 {
        assert_eq!(UFrac16::try_from(i).unwrap().to_fraction(), (i, 1));
    }
}

#[test]
fn from_f32() {
    assert_eq!(UFrac16::try_from(1.0).unwrap().to_fraction(), (1, 1));
    assert_eq!(UFrac16::try_from(0.5).unwrap().to_fraction(), (1, 2));
    println!("{}", UFrac16::try_from(1.618).unwrap());
    assert_eq!(UFrac16::try_from(1.618).unwrap().to_fraction(), (610, 377));
}

#[test]
fn ordering() {
    println!(
        "{}({0:?}) < {}({1:?})",
        UFrac16::try_from(2).unwrap(),
        UFrac16::try_from(3).unwrap()
    );
    assert_eq!(
        UFrac16::try_from(2)
            .unwrap()
            .cmp(&UFrac16::try_from(3).unwrap()),
        Ordering::Less
    );
    println!(
        "{} > {}",
        UFrac16::try_from(4).unwrap(),
        UFrac16::try_from(3).unwrap()
    );
    assert_eq!(
        UFrac16::try_from(4)
            .unwrap()
            .cmp(&UFrac16::try_from(3).unwrap()),
        Ordering::Greater
    );
    println!("{:?} = {0}", UFrac16::try_from(1.20001).unwrap());
    assert_eq!(
        UFrac16::try_from(1.2)
            .unwrap()
            .cmp(&UFrac16::try_from(1.20001).unwrap()),
        Ordering::Less
    );
    println!("{:?} = {0}", UFrac16::try_from(2).unwrap());
    println!("{:?} = {0}", UFrac16::try_from(1.9).unwrap());
    assert_eq!(
        UFrac16::try_from(2)
            .unwrap()
            .cmp(&UFrac16::try_from(1.9).unwrap()),
        Ordering::Greater
    );
}
