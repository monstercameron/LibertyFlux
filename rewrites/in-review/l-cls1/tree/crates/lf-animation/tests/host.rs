//! Host edge tests for the lifted animation channels.
//!
//! One case per question a reader of the code would ask: empty channels,
//! single keys, last-key indices, boundary frames, NaN payloads, and the
//! domains the lift narrows (which panic). Bit-exactness against the
//! verified rewrites is proven by the 32-bit differential crate, not here.

use lf_animation::channel::frame::{
    AnimChannel, ONE, ROUND_HALF, ROUND_MAGIC, SNAP_HI, SNAP_LO, clamp_index, key_below,
    truncate_to_i32,
};
use lf_animation::channel::registry::{self, State};
use lf_animation::channel::{
    QuantizeFloat, RawBool, RawFloat, RawInt, RawVec3, StaticFloat, StaticInt, StaticQuat,
    StaticVec3,
};
use lf_math::{Quat, Vec3, Vec4};

fn q(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::from_xyzw(x, y, z, w)
}

fn v4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::from_array([x, y, z, w])
}

// Shared frame arithmetic.

#[test]
fn shared_constants_have_measured_bits() {
    assert_eq!(ROUND_HALF.to_bits(), 0x3F00_0000);
    assert_eq!(ROUND_MAGIC.to_bits(), 0x4B00_0000);
    assert_eq!(ONE.to_bits(), 0x3F80_0000);
    assert_eq!(SNAP_LO.to_bits(), 0x3A83_126F);
    assert_eq!(SNAP_HI.to_bits(), 0x3F7F_BE77);
}

#[test]
fn truncate_matches_conversion_at_boundaries() {
    assert_eq!(truncate_to_i32(0.0), 0);
    assert_eq!(truncate_to_i32(-0.0), 0);
    assert_eq!(truncate_to_i32(1.9), 1);
    assert_eq!(truncate_to_i32(-1.9), -1);
    assert_eq!(truncate_to_i32(2_147_483_648.0), i32::MIN); // 2^31: indefinite
    assert_eq!(truncate_to_i32(-2_147_483_648.0), i32::MIN); // converts, same value
    assert_eq!(truncate_to_i32(2_147_483_648.0 * 2.0), i32::MIN);
    assert_eq!(truncate_to_i32(f32::INFINITY), i32::MIN);
    assert_eq!(truncate_to_i32(f32::NEG_INFINITY), i32::MIN);
    assert_eq!(truncate_to_i32(f32::NAN), i32::MIN);
    assert_eq!(truncate_to_i32(2_147_483_520.0), 2_147_483_520);
}

#[test]
fn key_below_splits_frame_and_fraction() {
    // Exact frames yield the key strictly below with fraction 1: the
    // sampler's snap-up path then takes the exact key.
    assert_eq!(key_below(0.0), (-1, 1.0));
    assert_eq!(key_below(2.0), (1, 1.0));
    let (i, f) = key_below(2.5);
    assert_eq!(i, 2);
    assert!((f - 0.5).abs() < 1e-6);
    // A negative frame still yields the key at or below it.
    let (i, f) = key_below(-0.25);
    assert_eq!(i, -1);
    assert!((f - 0.75).abs() < 1e-6);
    // Huge frames lose their fraction to rounding but stay ordered.
    let (i, _) = key_below(100_000_000.0);
    assert_eq!(i, 100_000_000);
}

#[test]
fn clamp_orders_below_before_above() {
    assert_eq!(clamp_index(-5, 3), 0);
    assert_eq!(clamp_index(9, 3), 3);
    assert_eq!(clamp_index(2, 3), 2);
}

// Static float.

#[test]
fn static_float_ignores_everything_but_value() {
    let c = StaticFloat::new(1.5);
    assert_eq!(c.eval().to_bits(), 1.5f32.to_bits());
    assert_eq!(c.copy_key().to_bits(), 1.5f32.to_bits());
    assert_eq!(c.sample(f32::NAN).to_bits(), 1.5f32.to_bits());
    assert_eq!(c.key_count(), 1);
    // NaN payloads survive untouched.
    let nan = StaticFloat::new(f32::from_bits(0x7FC0_1234));
    assert_eq!(nan.eval().to_bits(), 0x7FC0_1234);
}

#[test]
fn static_float_adopt_checks_strided_samples() {
    let mut c = StaticFloat::new(0.0);
    // Stride 1: every other float is a sample.
    let samples = [10.0, 999.0, 10.05, 999.0, 9.95, 999.0];
    assert!(c.adopt_if_uniform(&samples, 3, 1, 0.1));
    assert_eq!(c.get().to_bits(), 10.0f32.to_bits());
    // The outlier fails even though it adopted the first sample.
    let mut c = StaticFloat::new(0.0);
    let samples = [10.0, 11.0];
    assert!(!c.adopt_if_uniform(&samples, 2, 0, 0.1));
    assert_eq!(c.get().to_bits(), 10.0f32.to_bits());
    // Count 1 (and below) always pass after adopting.
    let mut c = StaticFloat::new(0.0);
    assert!(c.adopt_if_uniform(&[7.0], 0, 0, 0.0));
    assert_eq!(c.get().to_bits(), 7.0f32.to_bits());
}

#[test]
#[should_panic]
fn static_float_adopt_empty_panics() {
    StaticFloat::new(0.0).adopt_if_uniform(&[], 0, 0, 0.0);
}

// Static int.

#[test]
fn static_int_adopt_compares_neighbours() {
    let mut c = StaticInt::new(0);
    assert!(c.adopt_if_uniform(&[5, 5, 5], 3));
    assert_eq!(c.get(), 5);
    // A mismatch leaves the old value alone.
    let mut c = StaticInt::new(9);
    assert!(!c.adopt_if_uniform(&[5, 5, 6], 3));
    assert_eq!(c.get(), 9);
    // Short counts adopt without comparing.
    let mut c = StaticInt::new(0);
    assert!(c.adopt_if_uniform(&[42], -3));
    assert_eq!(c.get(), 42);
}

#[test]
#[should_panic]
fn static_int_adopt_empty_panics() {
    StaticInt::new(0).adopt_if_uniform(&[], 1);
}

// Raw float.

#[test]
fn raw_float_lerp_uses_difference_times_t_plus_a() {
    let c = RawFloat::new(vec![1.0, 3.0]);
    assert_eq!(c.lerp_at(0, 0.25).to_bits(), 1.5f32.to_bits());
    assert_eq!(c.lerp_at_f64(0, 0.25).to_bits(), 1.5f64.to_bits());
    // The order matters for infinities: (inf - 1) * 0 + 1 is NaN, not 1.
    let c = RawFloat::new(vec![1.0, f32::INFINITY]);
    assert!(c.lerp_at(0, 0.0).is_nan());
}

#[test]
#[should_panic]
fn raw_float_lerp_past_last_key_panics() {
    let _ = RawFloat::new(vec![1.0, 2.0]).lerp_at(1, 0.5);
}

#[test]
fn raw_float_sample_snaps_and_clamps() {
    let c = RawFloat::new(vec![10.0, 20.0, 30.0]);
    // Exact keys, far-outside frames, and the midpoint.
    assert_eq!(c.sample(0.0).to_bits(), 10.0f32.to_bits());
    assert_eq!(c.sample(2.0).to_bits(), 30.0f32.to_bits());
    assert_eq!(c.sample(-100.0).to_bits(), 10.0f32.to_bits());
    assert_eq!(c.sample(100.0).to_bits(), 30.0f32.to_bits());
    assert_eq!(c.sample(0.5).to_bits(), 15.0f32.to_bits());
    // Within a thousandth above a key snaps down to it.
    assert_eq!(c.sample(1.0005).to_bits(), 20.0f32.to_bits());
    // Within a thousandth below the next key snaps up to it.
    assert_eq!(c.sample(0.9995).to_bits(), 20.0f32.to_bits());
    assert_eq!(c.key_count(), 3);
}

#[test]
fn raw_float_single_key_answers_it_for_any_frame() {
    let c = RawFloat::new(vec![7.5]);
    for f in [-1e10, -0.5, 0.0, 0.3, 1.0, 1e10, f32::NAN] {
        assert_eq!(c.sample(f).to_bits(), 7.5f32.to_bits(), "frame {f}");
    }
}

#[test]
#[should_panic]
fn raw_float_sample_empty_panics() {
    RawFloat::new(vec![]).sample(0.0);
}

// Raw int.

#[test]
fn raw_int_rounds_half_up_and_clamps() {
    let c = RawInt::new(vec![100, 200, 300]);
    assert_eq!(c.key_at(2), 300);
    assert_eq!(c.sample(0.4), 100);
    assert_eq!(c.sample(0.5), 200);
    assert_eq!(c.sample(1.5), 300);
    assert_eq!(c.sample(-0.4), 100);
    assert_eq!(c.sample(-100.0), 100);
    assert_eq!(c.sample(100.0), 300);
    assert_eq!(c.sample(f32::NAN), 100); // indefinite index clamps low
    assert_eq!(c.key_count(), 3);
}

#[test]
fn raw_int_storage_size_counts_words_plus_header() {
    assert_eq!(RawInt::new(vec![]).storage_size(), 0x10);
    assert_eq!(RawInt::new(vec![1, 2, 3]).storage_size(), 0x10 + 12);
}

#[test]
#[should_panic]
fn raw_int_sample_empty_panics() {
    RawInt::new(vec![]).sample(0.0);
}

// Raw bool.

#[test]
fn raw_bool_decodes_masked_bits() {
    let c = RawBool::new(vec![0x00, 0b1011]);
    // Index 9 (byte 1) masks with 0b1001: low bits survive.
    assert_eq!(c.sample(9.0), 1);
    // Index 8 masks with 0b1000: nothing in the low three bits.
    assert_eq!(c.sample(8.0), 0);
    // Frame 0: index 0 masks everything away.
    assert_eq!(c.sample(0.0), 0);
    assert_eq!(c.key_count(), 2);
}

#[test]
fn raw_bool_storage_size_counts_bytes_plus_header() {
    assert_eq!(RawBool::new(vec![]).storage_size(), 0x10);
    assert_eq!(RawBool::new(vec![0, 0, 0]).storage_size(), 0x13);
}

#[test]
#[should_panic]
fn raw_bool_sample_past_last_byte_panics() {
    RawBool::new(vec![0xFF]).sample(100.0);
}

#[test]
#[should_panic]
fn raw_bool_sample_empty_panics() {
    RawBool::new(vec![]).sample(0.0);
}

// Raw vector.

#[test]
fn raw_vec3_lerp_blends_difference_times_t_plus_lo() {
    let c = RawVec3::new(vec![
        v4(0.0, 0.0, 0.0, 1.0),
        v4(4.0, 8.0, 12.0, 2.0),
    ]);
    let m = c.lerp_at(0, 0.5);
    assert_eq!(m, Vec3::from_array([2.0, 4.0, 6.0]));
    // Padding plays no part in the blend.
    assert_eq!(c.lerp_at(0, 0.0), Vec3::from_array([0.0, 0.0, 0.0]));
}

#[test]
fn raw_vec3_sample_snap_keeps_padding_lerp_leaves_it() {
    let c = RawVec3::new(vec![
        v4(1.0, 2.0, 3.0, 11.0),
        v4(5.0, 6.0, 7.0, 12.0),
    ]);
    // Snap path: the whole key, padding included.
    let mut out = Vec4::from_array([0.0, 0.0, 0.0, 0.0]);
    c.sample_into(0.0, &mut out);
    assert_eq!(out, v4(1.0, 2.0, 3.0, 11.0));
    // Lerp path: x, y, z blend, padding untouched.
    let mut out = Vec4::from_array([0.0, 0.0, 0.0, 99.0]);
    c.sample_into(0.5, &mut out);
    assert_eq!(out, v4(3.0, 4.0, 5.0, 99.0));
}

#[test]
fn raw_vec3_storage_size_adds_one_extra_slot() {
    assert_eq!(RawVec3::new(vec![]).storage_size(), 0x10);
    let one = RawVec3::new(vec![v4(0.0, 0.0, 0.0, 0.0)]);
    assert_eq!(one.storage_size(), 0x20);
}

#[test]
#[should_panic]
fn raw_vec3_sample_empty_panics() {
    RawVec3::new(vec![]).sample(0.0);
}

// Static vector and quaternion.

#[test]
fn static_vec3_copy_reports_padding_in_value() {
    let c = StaticVec3::new(v4(1.0, 2.0, 3.0, 4.0));
    assert_eq!(c.get(), v4(1.0, 2.0, 3.0, 4.0));
}

#[test]
fn static_vec3_adopt_ignores_padding_compares_xyz() {
    let mut c = StaticVec3::new(Vec4::from_array([0.0, 0.0, 0.0, 0.0]));
    let records = [
        v4(1.0, 1.0, 1.0, 50.0),
        v4(1.0, 1.0, 1.0, 60.0), // padding differs: still uniform
    ];
    assert!(c.adopt_if_uniform(&records, 2, 0.01));
    assert_eq!(c.get(), v4(1.0, 1.0, 1.0, 50.0));
    // A z outlier beyond tolerance fails.
    let mut c = StaticVec3::new(Vec4::from_array([0.0, 0.0, 0.0, 0.0]));
    let records = [v4(1.0, 1.0, 1.0, 0.0), v4(1.0, 1.0, 2.0, 0.0)];
    assert!(!c.adopt_if_uniform(&records, 2, 0.5));
    // Count 1 adopts without comparing.
    let mut c = StaticVec3::new(Vec4::from_array([0.0, 0.0, 0.0, 0.0]));
    assert!(c.adopt_if_uniform(&records, 1, 0.0));
    assert_eq!(c.get(), v4(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn static_quat_holds_rotation() {
    let c = StaticQuat::new(q(0.0, 0.0, 0.0, 1.0));
    assert_eq!(c.get(), q(0.0, 0.0, 0.0, 1.0));
    assert_eq!(StaticQuat::default().get(), Quat::IDENTITY);
}

// Quantized float fragment.

#[test]
fn quantize_float_scales_biases_and_blends() {
    let c = QuantizeFloat::new(0.5, 1.0);
    // (10 * 0.5 + 1) = 6, (20 * 0.5 + 1) = 11, blend at 0.5 = 8.5.
    assert_eq!(c.eval_scaled(10, 20, 0.5).to_bits(), 8.5f64.to_bits());
    // The largest raw value rounds once, to nearest even.
    let c = QuantizeFloat::new(1.0, 0.0);
    assert_eq!(
        c.eval_scaled(u64::MAX, u64::MAX, 0.0).to_bits(),
        f64::from(u64::MAX as f32).to_bits()
    );
    // t = 0 takes the lower sample exactly, NaN scale propagates.
    assert_eq!(c.eval_scaled(4, 9, 0.0).to_bits(), 4.0f64.to_bits());
    let nan = QuantizeFloat::new(f32::NAN, 0.0);
    assert!(nan.eval_scaled(4, 9, 0.5).is_nan());
}

// Registry honesty.

#[test]
fn registry_counts_match_proof_scope() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 20, "proven methods");
    assert_eq!(lifted, 0, "everything lifted is proven");
    assert!(missing > 0, "missing methods are listed, not hidden");
    for row in registry::ROWS {
        if row.state == State::Proven {
            continue;
        }
        assert!(
            !row.narrows.is_empty(),
            "{}::{} is missing without a reason",
            row.channel,
            row.method
        );
    }
}
