//! Host tests on the 64-bit host: the edge cases a reader of the lifted
//! ratio routine would ask about. The differential cases against the 32-bit
//! rewrites live in `ratio_diff.rs`; these run everywhere.

use lf_vehicles::veh_ratio::RatioCell;

fn quotient(numer: f32, denom: f32) -> f32 {
    let mut cell = RatioCell::new(numer, denom, 1.5);
    cell.refresh();
    assert_eq!(cell.numer().to_bits(), numer.to_bits(), "numerator is untouched");
    assert_eq!(cell.denom().to_bits(), denom.to_bits(), "denominator is untouched");
    cell.value()
}

#[test]
fn ordinary_quotient_and_operand_order() {
    assert_eq!(quotient(2.0, 1.0), 2.0);
    assert_eq!(quotient(1.0, 2.0), 0.5);
    assert_eq!(quotient(-2.0, 2.0), -1.0);
}

#[test]
fn zero_denominator_gives_signed_infinity() {
    assert_eq!(quotient(1.0, 0.0).to_bits(), f32::INFINITY.to_bits());
    assert_eq!(quotient(1.0, -0.0).to_bits(), f32::NEG_INFINITY.to_bits());
    assert_eq!(quotient(-1.0, 0.0).to_bits(), f32::NEG_INFINITY.to_bits());
}

#[test]
fn zero_over_zero_is_not_a_number() {
    assert!(quotient(0.0, 0.0).is_nan());
    assert!(quotient(-0.0, 0.0).is_nan());
    assert!(quotient(-0.0, -0.0).is_nan());
}

#[test]
fn zero_numerator_keeps_its_sign() {
    assert_eq!(quotient(-0.0, 1.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(quotient(0.0, -1.0).to_bits(), (-0.0f32).to_bits());
}

#[test]
fn infinities_follow_the_hardware_rules() {
    assert_eq!(quotient(f32::INFINITY, 2.0).to_bits(), f32::INFINITY.to_bits());
    assert_eq!(quotient(2.0, f32::INFINITY).to_bits(), 0.0f32.to_bits());
    assert!(quotient(f32::INFINITY, f32::INFINITY).is_nan());
}

#[test]
fn a_subnormal_result_is_kept() {
    let q = quotient(1.0, f32::from_bits(0x7F7F_FFFF));
    assert_eq!(q.to_bits(), (1.0f32 / f32::from_bits(0x7F7F_FFFF)).to_bits());
    assert!(q > 0.0 && q < f32::MIN_POSITIVE);
}

#[test]
fn refresh_overwrites_a_previous_quotient() {
    let mut cell = RatioCell::new(9.0, 3.0, 99.0);
    cell.refresh();
    assert_eq!(cell.value(), 3.0);
    cell.set_sources(1.0, 4.0);
    cell.refresh();
    assert_eq!(cell.value(), 0.25);
}
