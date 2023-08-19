use std::cmp::Ordering;

use crate::UFrac16;

#[test]
fn to_fraction() {
    assert_eq!(UFrac16::ONE.to_fraction(), (1, 1));
    assert_eq!(UFrac16::from_bits(0x0002).to_fraction(), (1, 2));
    assert_eq!(UFrac16::from_bits(0x0003).to_fraction(), (2, 1));
    assert_eq!(UFrac16::from_bits(0x003f).to_fraction(), (6, 1));
    assert_eq!(UFrac16::from_bits(0x007f).to_fraction(), (7, 1));
    assert_eq!(UFrac16::from_bits(0x0020).to_fraction(), (1, 6));
    assert_eq!(UFrac16::from_bits(0x0040).to_fraction(), (1, 7));
    assert_eq!(UFrac16::from_bits(0x0012).to_fraction(), (4, 7));
    assert_eq!(UFrac16::from_bits(0x0015).to_fraction(), (8, 5));
    assert_eq!(UFrac16::from_bits(0x000d).to_fraction(), (5, 3));
}

#[test]
fn invert() {
    assert_eq!(UFrac16::ONE.invert().to_fraction(), (1, 1));
    assert_eq!(UFrac16::from_bits(0x0012).invert().to_fraction(), (7, 4));
}

#[test]
fn from_u16() {
    for i in 0..16 {
        assert_eq!(UFrac16::try_from(i).unwrap().to_fraction(), (i, 1));
    }
}

#[test]
fn from_f64() {
    assert_eq!(UFrac16::try_from(1.0).unwrap().to_fraction(), (1, 1));
    assert_eq!(UFrac16::try_from(0.5).unwrap().to_fraction(), (1, 2));
    println!("{}", UFrac16::try_from(1.618).unwrap());
    assert_eq!(UFrac16::try_from(1.618).unwrap().to_fraction(), (809, 500));
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
