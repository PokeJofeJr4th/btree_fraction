use std::cmp::Ordering;

use crate::UFrac32;

#[test]
fn to_fraction() {
    assert_eq!(UFrac32::ONE.to_fraction(), (1, 1));
    assert_eq!(UFrac32::from_bits(0x4000_0000).to_fraction(), (1, 2));
    assert_eq!(UFrac32::from_bits(0xC000_0000).to_fraction(), (2, 1));
    assert_eq!(UFrac32::from_bits(0xfc00_0000).to_fraction(), (6, 1));
    assert_eq!(UFrac32::from_bits(0xfe00_0000).to_fraction(), (7, 1));
    assert_eq!(UFrac32::from_bits(0x0400_0000).to_fraction(), (1, 6));
    assert_eq!(UFrac32::from_bits(0x0200_0000).to_fraction(), (1, 7));
    assert_eq!(UFrac32::from_bits(0x4800_0000).to_fraction(), (4, 7));
    assert_eq!(UFrac32::from_bits(0xa800_0000).to_fraction(), (8, 5));
    assert_eq!(UFrac32::from_bits(0xb000_0000).to_fraction(), (5, 3));
}

#[test]
fn invert() {
    assert_eq!(UFrac32::ONE.invert().to_fraction(), (1, 1));
    assert_eq!(
        UFrac32::from_bits(0x4800_0000).invert().to_fraction(),
        (7, 4)
    );
    assert_eq!(UFrac32::MIN.invert(), UFrac32::MAX);
    assert_eq!(UFrac32::MAX.invert(), UFrac32::MIN);
}

#[test]
fn from_u32() {
    for i in 0..=32 {
        assert_eq!(UFrac32::try_from(i).unwrap().to_fraction(), (i, 1));
    }
}

#[test]
fn from_f64() {
    assert_eq!(UFrac32::try_from(1.0).unwrap().to_fraction(), (1, 1));
    assert_eq!(UFrac32::try_from(0.5).unwrap().to_fraction(), (1, 2));
    println!("{}", UFrac32::try_from(1.618).unwrap());
    assert_eq!(UFrac32::try_from(1.618).unwrap().to_fraction(), (809, 500));
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

#[test]
fn tree() {
    assert_eq!(UFrac32::ONE.left_child().unwrap(), UFrac32::from_bits(0x4000_0000));
    assert_eq!(UFrac32::ONE.right_child().unwrap(), UFrac32::from_bits(0xC000_0000));
    assert_eq!(UFrac32::ONE.parent(), None);

    assert_eq!(UFrac32::ZERO.left_child(), None);
    assert_eq!(UFrac32::ZERO.right_child(), None);
    assert_eq!(UFrac32::ZERO.parent(), None);
}


#[test]
fn is_leaf() {
    assert!(!UFrac32::ZERO.is_leaf());
    assert!(UFrac32::MIN.is_leaf());
    assert!(!UFrac32::ONE.is_leaf());
    assert!(!UFrac32::GOLDEN_RATIO.is_leaf());
    assert!(!UFrac32::E.is_leaf());
    assert!(UFrac32::PI.is_leaf());
    assert!(UFrac32::MAX.is_leaf());
}