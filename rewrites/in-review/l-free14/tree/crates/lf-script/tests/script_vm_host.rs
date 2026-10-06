//! Host tests for the lifted script-VM group: edge cases a reader of
//! the code would ask about, run on the 64-bit host without rewrites.

use lf_script::script_vm::{
    AltitudeGate, AreaProbe, ExtentRoutine, PointSkip, SkipFlags, EXTENT_ARG_A, EXTENT_ARG_B,
    registry,
};

// --- The pack-and-skip routine ---

#[test]
fn skip_forwards_point_key_and_flags() {
    let proto = PointSkip;
    let mut seen = Vec::new();
    proto.emit(
        &mut |point: [u32; 3], key: u32, flags: SkipFlags| {
            seen.push((point, key, flags));
        },
        11,
        22,
        33,
        44,
        SkipFlags::FIRST_SET,
    );
    assert_eq!(seen, [([11, 22, 33], 44, SkipFlags::FIRST_SET)]);
}

#[test]
fn skip_flag_consts_are_distinct() {
    assert_ne!(SkipFlags::NONE, SkipFlags::FIRST_SET);
    assert_ne!(SkipFlags::NONE, SkipFlags::TRAIL_SET);
    assert_ne!(SkipFlags::FIRST_SET, SkipFlags::TRAIL_SET);
    assert_eq!(SkipFlags::NONE, SkipFlags(0, 0, 0));
    assert_eq!(SkipFlags::FIRST_SET, SkipFlags(1, 0, 0));
    assert_eq!(SkipFlags::TRAIL_SET, SkipFlags(0, 0, 1));
}

#[test]
fn skip_words_pass_through_verbatim() {
    // Opaque words, however extreme, reach the sink unchanged.
    let proto = PointSkip;
    for words in [
        [0u32, 0, 0],
        [0xFFFF_FFFF, 0x8000_0000, 0x7FFF_FFFF],
        [0x7FC0_0000, 0x7F80_0001, 0xFF80_0000],
    ] {
        let mut seen = None;
        proto.emit(
            &mut |point: [u32; 3], key: u32, flags: SkipFlags| {
                seen = Some((point, key, flags));
            },
            words[0],
            words[1],
            words[2],
            0xDEAD_BEEF,
            SkipFlags::TRAIL_SET,
        );
        assert_eq!(seen, Some((words, 0xDEAD_BEEF, SkipFlags::TRAIL_SET)));
    }
}

// --- The altitude gate ---

const NEG100: u32 = 0xC2C8_0000; // -100.0 as bits

#[test]
fn gate_above_threshold_keeps_z_without_query() {
    let gate = AltitudeGate::new(NEG100);
    let mut queried = false;
    let mut got = None;
    gate.register_point(
        &mut |_, _, _| {
            queried = true;
            0
        },
        &mut |point: [u32; 3], w: u32| got = Some((point, w)),
        1,
        2,
        0x42C8_0000, // 100.0: above the threshold
        7,
    );
    assert!(!queried, "no query above the threshold");
    assert_eq!(got, Some(([1, 2, 0x42C8_0000], 7)));
}

#[test]
fn gate_at_threshold_converts() {
    // Ordered equality converts: the threshold is the lowest kept altitude.
    let gate = AltitudeGate::new(NEG100);
    let mut mode = None;
    let mut got = None;
    gate.register_point(
        &mut |x: u32, y: u32, m: u32| {
            mode = Some((x, y, m));
            0x4120_0000 // 10.0
        },
        &mut |point: [u32; 3], w: u32| got = Some((point, w)),
        5,
        6,
        NEG100,
        7,
    );
    assert_eq!(mode, Some((5, 6, 4)), "query runs with (x, y, mode 4)");
    assert_eq!(got, Some(([5, 6, 0x4120_0000], 7)));
}

#[test]
fn gate_nan_altitude_keeps_z() {
    // Unordered comparisons never convert.
    let gate = AltitudeGate::new(NEG100);
    for z in [0x7FC0_0000, 0xFFC0_0001, 0x7F80_0001, 0x7F80_0000, 0xFF80_0000] {
        let mut queried = false;
        let mut point = [0u32; 3];
        gate.register_point(
            &mut |_, _, _| {
                queried = true;
                0
            },
            &mut |p: [u32; 3], _| point = p,
            0,
            0,
            z,
            0,
        );
        // Infinities are ordered: +inf keeps, -inf converts.
        let converts = z == 0xFF80_0000;
        assert_eq!(queried, converts, "z={z:#x}");
        assert_eq!(point[2], if converts { 0 } else { z }, "z={z:#x}");
    }
}

#[test]
fn gate_nan_threshold_keeps_z() {
    // An unordered threshold converts nothing either.
    for thresh in [0x7FC0_0000, 0x7F80_0001] {
        let gate = AltitudeGate::new(thresh);
        let mut queried = false;
        gate.register_restart(
            &mut |_, _, _| {
                queried = true;
                0
            },
            &mut |_, _, _| {},
            1,
            2,
            0xC2C8_0000, // -100.0, below any ordered threshold
            3,
            4,
        );
        assert!(!queried, "thresh={thresh:#x}");
    }
}

#[test]
fn gate_signed_zeros_compare_ordered() {
    // -0.0 and +0.0 are equal under >=, so both convert at a zero threshold.
    let gate = AltitudeGate::new(0x0000_0000);
    for z in [0x0000_0000, 0x8000_0000] {
        let mut queried = false;
        gate.register_point(
            &mut |_, _, _| {
                queried = true;
                0x3F80_0000
            },
            &mut |_, _| {},
            0,
            0,
            z,
            0,
        );
        assert!(queried, "z={z:#x} converts at a zero threshold");
    }
}

#[test]
fn gate_quiets_signalling_nan_answers() {
    // The x87 return path sets the quiet bit; the lift does it explicitly.
    let gate = AltitudeGate::new(NEG100);
    let mut point = [0u32; 3];
    gate.register_point(
        &mut |_, _, _| 0x7F80_0001,
        &mut |p: [u32; 3], _| point = p,
        0,
        0,
        0xC2C8_0001, // just below -100.0: converts
        0,
    );
    assert_eq!(point[2], 0x7FD0_0001);
}

#[test]
fn gate_preserves_quiet_nan_payloads() {
    // Quiet NaN bits, payload included, pass through untouched.
    let gate = AltitudeGate::new(NEG100);
    let mut point = [0u32; 3];
    gate.register_point(
        &mut |_, _, _| 0xFFC0_BEEF,
        &mut |p: [u32; 3], _| point = p,
        0,
        0,
        0xC2C8_0001,
        0,
    );
    assert_eq!(point[2], 0xFFC0_BEEF);
}

#[test]
fn gate_threshold_round_trips() {
    assert_eq!(AltitudeGate::new(0x1234_5678).threshold(), 0x1234_5678);
}

#[test]
fn gate_tails_forward_their_trailing_words() {
    let gate = AltitudeGate::new(0x7F80_0000); // +inf: everything converts
    let mut restart = None;
    gate.register_restart(
        &mut |_, _, _| 9,
        &mut |p: [u32; 3], w: u32, e: u32| restart = Some((p, w, e)),
        1,
        2,
        3,
        4,
        5,
    );
    assert_eq!(restart, Some(([1, 2, 9], 4, 5)));

    let mut full = None;
    gate.clear_area(
        &mut |_, _, _| 9,
        &mut |p: [u32; 3], w: u32, e: u32, z0: u32, z1: u32, z2: u32| {
            full = Some((p, w, e, z0, z1, z2));
        },
        1,
        2,
        3,
        4,
        5,
    );
    assert_eq!(full, Some(([1, 2, 9], 4, 5, 0, 0, 0)));

    let mut cars = None;
    gate.clear_area_cars(
        &mut |_, _, _| 9,
        &mut |p: [u32; 3], w: u32, a: u32, b: u32, c: u32| cars = Some((p, w, a, b, c)),
        1,
        2,
        3,
        4,
    );
    assert_eq!(cars, Some(([1, 2, 9], 4, 0, 1, 0)));
}

#[test]
fn gate_double_tail_runs_in_order() {
    use std::cell::RefCell;
    let gate = AltitudeGate::new(0xFF80_0000); // -inf: nothing converts
    // One log shared by both tails through a cell: order is what is pinned.
    let order = RefCell::new(Vec::new());
    gate.clear_objects(
        &mut |_, _, _| panic!("must not query above the threshold"),
        &mut |p: [u32; 3], w: u32| order.borrow_mut().push(("first", p, vec![w])),
        &mut |p: [u32; 3], w: u32, a: u32, b: u32, c: u32| {
            order.borrow_mut().push(("second", p, vec![w, a, b, c]));
        },
        1,
        2,
        0x42C8_0000, // 100.0
        4,
    );
    assert_eq!(
        *order.borrow(),
        [
            ("first", [1, 2, 0x42C8_0000], vec![4]),
            ("second", [1, 2, 0x42C8_0000], vec![4, 0, 0, 0]),
        ]
    );
}

// --- The area-test protocol ---

#[test]
fn point_primes_only_on_the_low_byte() {
    let proto = AreaProbe;
    for (flag, primes) in [
        (0x0000_0000, false),
        (0x0000_0001, true),
        (0x0000_00FF, true),
        (0x0000_0100, false),
        (0xFFFF_FF00, false),
        (0xFFFF_FFFF, true),
    ] {
        let mut count = 0;
        let mut seen = None;
        proto.test_point(
            &mut || count += 1,
            &mut |p: [u32; 3], w: u32, z: u32| seen = Some((p, w, z)),
            1,
            2,
            3,
            4,
            flag,
        );
        assert_eq!(count, u32::from(primes), "flag={flag:#x}");
        assert_eq!(seen, Some(([1, 2, 3], 4, 0)), "flag={flag:#x}");
    }
}

#[test]
fn box_orders_each_axis() {
    let proto = AreaProbe;
    let mut seen = None;
    proto.test_box(
        &mut || {},
        &mut |mins: [u32; 3], maxs: [u32; 3], z: u32| seen = Some((mins, maxs, z)),
        0x4120_0000, // 10.0
        0xC120_0000, // -10.0
        0x3F80_0000, // 1.0
        0x3F80_0000, // 1.0
        0x4120_0000, // 10.0
        0xC120_0000, // -10.0
        0,
    );
    assert_eq!(
        seen,
        Some((
            [0x3F80_0000, 0xC120_0000, 0xC120_0000],
            [0x4120_0000, 0x4120_0000, 0x3F80_0000],
            0
        ))
    );
}

#[test]
fn box_nan_keeps_its_place() {
    // Unordered pairs never swap, whichever side the NaN is on.
    let proto = AreaProbe;
    for (x0, x1) in [
        (0x7FC0_0000, 0x3F80_0000),
        (0x3F80_0000, 0x7FC0_0000),
        (0x7FC0_0000, 0xFFC0_0001),
    ] {
        let mut seen = None;
        proto.test_box(
            &mut || {},
            &mut |mins: [u32; 3], maxs: [u32; 3], _: u32| seen = Some((mins, maxs)),
            x0,
            0,
            0,
            x1,
            0,
            0,
            0,
        );
        assert_eq!(seen, Some(([x0, 0, 0], [x1, 0, 0])), "x0={x0:#x} x1={x1:#x}");
    }
}

#[test]
fn box_equal_and_zero_pairs_do_not_swap() {
    let proto = AreaProbe;
    let mut seen = None;
    proto.test_box(
        &mut || {},
        &mut |mins: [u32; 3], maxs: [u32; 3], _: u32| seen = Some((mins, maxs)),
        0x4120_0000,
        0x0000_0000, // +0.0 first: not greater than -0.0, no swap
        0x8000_0000, // -0.0 first: not greater than +0.0, no swap
        0x4120_0000,
        0x8000_0000,
        0x0000_0000,
        0,
    );
    assert_eq!(
        seen,
        Some((
            [0x4120_0000, 0x0000_0000, 0x8000_0000],
            [0x4120_0000, 0x8000_0000, 0x0000_0000]
        ))
    );
}

#[test]
fn box_primes_before_testing() {
    use std::cell::RefCell;
    let proto = AreaProbe;
    let order = RefCell::new(Vec::new());
    proto.test_box(
        &mut || order.borrow_mut().push("prime"),
        &mut |_, _, _| order.borrow_mut().push("test"),
        0,
        0,
        0,
        0,
        0,
        0,
        1,
    );
    assert_eq!(*order.borrow(), ["prime", "test"]);
}

#[test]
fn expanded_centre_by_zero_is_the_centre_twice() {
    let proto = AreaProbe;
    let routine = ExtentRoutine::new(0x0E00_0001).unwrap();
    let mut seen = None;
    proto.test_expanded(
        &mut |row: [u32; 8], r: ExtentRoutine, view: [u32; 9], a: u32, b: u32| {
            seen = Some((row, r.get(), view, a, b));
        },
        routine,
        0x4120_0000, // 10.0
        0xC120_0000, // -10.0
        0x3F80_0000, // 1.0
        0,
        0,
        0,
    );
    let (row, tag, view, a, b) = seen.unwrap();
    assert_eq!(tag, 0x0E00_0001);
    assert_eq!((a, b), (EXTENT_ARG_A, EXTENT_ARG_B));
    assert_eq!(
        row,
        [
            0x4120_0000,
            0xC120_0000,
            0x3F80_0000,
            0,
            0x4120_0000,
            0xC120_0000,
            0x3F80_0000,
            0
        ]
    );
    assert_eq!(
        view,
        [
            0,
            0x4120_0000,
            0xC120_0000,
            0x3F80_0000,
            0,
            0x4120_0000,
            0xC120_0000,
            0x3F80_0000,
            0
        ]
    );
}

#[test]
fn expanded_unit_radius_rounds_like_single_precision() {
    // 1.0 - 1.0 is +0.0 and 1.0 + 1.0 is 2.0, exactly.
    let proto = AreaProbe;
    let routine = ExtentRoutine::new(7).unwrap();
    let mut row = [0u32; 8];
    proto.test_expanded(
        &mut |r: [u32; 8], _, _, _, _| row = r,
        routine,
        0x3F80_0000,
        0,
        0,
        0x3F80_0000,
        0,
        0,
    );
    assert_eq!(row[0], 0x0000_0000);
    assert_eq!(row[4], 0x4000_0000);
}

#[test]
fn expanded_negative_radius_normalises() {
    // centre - r exceeds centre + r when r is negative: the pair swaps.
    let proto = AreaProbe;
    let routine = ExtentRoutine::new(7).unwrap();
    let mut row = [0u32; 8];
    proto.test_expanded(
        &mut |r: [u32; 8], _, _, _, _| row = r,
        routine,
        0x3F80_0000, // 1.0
        0,
        0,
        0xBF80_0000, // -1.0
        0,
        0,
    );
    assert_eq!(row[0], 0x0000_0000); // min: 1.0 + -1.0
    assert_eq!(row[4], 0x4000_0000); // max: 1.0 - -1.0
}

#[test]
fn expanded_infinite_radius_saturates() {
    let proto = AreaProbe;
    let routine = ExtentRoutine::new(7).unwrap();
    let mut row = [0u32; 8];
    proto.test_expanded(
        &mut |r: [u32; 8], _, _, _, _| row = r,
        routine,
        0x3F80_0000,
        0,
        0,
        0x7F80_0000, // +inf
        0,
        0,
    );
    assert_eq!(row[0], 0xFF80_0000); // 1.0 - inf
    assert_eq!(row[4], 0x7F80_0000); // 1.0 + inf
}

#[test]
fn expanded_nan_radius_keeps_its_place() {
    // centre - NaN and centre + NaN are both NaN: no swap, NaN both sides.
    let proto = AreaProbe;
    let routine = ExtentRoutine::new(7).unwrap();
    let mut row = [0u32; 8];
    proto.test_expanded(
        &mut |r: [u32; 8], _, _, _, _| row = r,
        routine,
        0x3F80_0000,
        0,
        0,
        0x7FC0_0000,
        0,
        0,
    );
    assert!(f32::from_bits(row[0]).is_nan());
    assert!(f32::from_bits(row[4]).is_nan());
}

#[test]
fn extent_consts_match_the_call_shape() {
    assert_eq!(EXTENT_ARG_A, 0x1C);
    assert_eq!(EXTENT_ARG_B, 0x0D);
}

#[test]
fn extent_routine_rejects_zero() {
    assert!(ExtentRoutine::new(0).is_none());
    assert_eq!(ExtentRoutine::new(0x0E00_0001).unwrap().get(), 0x0E00_0001);
}

// --- The registry ---

#[test]
fn registry_covers_all_twenty_routines() {
    assert_eq!(registry::ROWS.len(), 20);
    assert_eq!(registry::counts(), (14, 6));
    for row in registry::ROWS {
        assert_ne!(
            row.state,
            registry::State::Lifted,
            "{} must be proven or missing",
            row.func
        );
        assert!(row.func.starts_with("script_vm_"), "{}", row.func);
    }
    let proven: Vec<_> = registry::ROWS
        .iter()
        .filter(|r| r.state == registry::State::Proven)
        .map(|r| r.func)
        .collect();
    assert_eq!(proven.len(), 14);
}
