use std::cmp::Ordering;

use crate::UFrac8;

#[test]
fn to_fraction() {
    assert_eq!(UFrac8::ONE.to_fraction(), (1, 1));
    assert_eq!(UFrac8::from_bits(0b0100_0000).to_fraction(), (1, 2));
    assert_eq!(UFrac8::from_bits(0b1100_0000).to_fraction(), (2, 1));
    assert_eq!(UFrac8::from_bits(0b1111_1100).to_fraction(), (6, 1));
    assert_eq!(UFrac8::from_bits(0b1111_1110).to_fraction(), (7, 1));
    assert_eq!(UFrac8::from_bits(0b0000_0100).to_fraction(), (1, 6));
    assert_eq!(UFrac8::from_bits(0b0000_0010).to_fraction(), (1, 7));
    assert_eq!(UFrac8::from_bits(0b0100_1000).to_fraction(), (4, 7));
    assert_eq!(UFrac8::from_bits(0b1010_1000).to_fraction(), (8, 5));
    assert_eq!(UFrac8::from_bits(0b1011_0000).to_fraction(), (5, 3));
}

#[test]
fn invert() {
    assert_eq!(UFrac8::ONE.invert(), UFrac8::ONE);
    assert_eq!(UFrac8::ONE.invert().to_fraction(), (1, 1));
    assert_eq!(
        UFrac8::from_bits(0b0100_1000).invert().to_fraction(),
        (7, 4)
    );
    assert_eq!(UFrac8::MIN.invert(), UFrac8::MAX);
    assert_eq!(UFrac8::MAX.invert(), UFrac8::MIN);
    assert_eq!(UFrac8::ZERO.invert_unchecked(), UFrac8::ZERO);
}

#[test]
fn from_u8() {
    for i in 0..=8 {
        assert_eq!(UFrac8::try_from(i).unwrap().to_fraction(), (i, 1));
    }
}

#[test]
fn children() {
    for i in 1..=255 {
        let frac = UFrac8::from_bits(i);
        if frac.is_leaf() {
            continue;
        }
        assert_eq!(
            frac.children().unwrap(),
            (frac.left_child().unwrap(), frac.right_child().unwrap())
        );
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

#[test]
fn tree() {
    assert_eq!(UFrac8::ONE.left_child().unwrap(), UFrac8::from_bits(0b0100_0000));
    assert_eq!(UFrac8::ONE.right_child().unwrap(), UFrac8::from_bits(0b1100_0000));
    assert_eq!(UFrac8::ONE.parent(), None);

    assert_eq!(UFrac8::ZERO.left_child(), None);
    assert_eq!(UFrac8::ZERO.right_child(), None);
    assert_eq!(UFrac8::ZERO.parent(), None);
}

#[test]
fn is_leaf() {
    assert!(!UFrac8::ZERO.is_leaf());
    assert!(UFrac8::MIN.is_leaf());
    assert!(!UFrac8::ONE.is_leaf());
    assert!(!UFrac8::GOLDEN_RATIO.is_leaf());
    assert!(!UFrac8::E.is_leaf());
    assert!(UFrac8::PI.is_leaf());
    assert!(UFrac8::MAX.is_leaf());
}
