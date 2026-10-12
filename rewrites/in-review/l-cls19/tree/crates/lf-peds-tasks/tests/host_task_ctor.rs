//! Host edge tests for the lifted task constructors.
//!
//! One case per question a reader of the code would ask: the tag each
//! kind stamps, the flag folds and packing, the branch arms, and the
//! call shapes. Bit-exactness against the verified rewrites is proven
//! by the 32-bit differential crate, not here.

use lf_peds_tasks::task_ctor::registry::{self, State};
use lf_peds_tasks::task_ctor::{PARAM_LEN, TaskInit, TaskParams};

/// Recording collaborator: logs every call with its arguments.
#[derive(Default)]
struct Fake {
    log: Vec<(&'static str, Vec<u32>)>,
}

impl TaskInit for Fake {
    fn base(&mut self, kind: u32) {
        self.log.push(("base", vec![kind]));
    }
    fn main(&mut self, kind: u32, g: u32, a1: u32, a0: u32, flag: u32) {
        self.log.push(("main", vec![kind, g, a1, a0, flag]));
    }
    fn block_2f(&mut self, arg: u32) {
        self.log.push(("block_2f", vec![arg]));
    }
    fn quant_word(&mut self, arg: u32) {
        self.log.push(("quant_word", vec![arg]));
    }
    fn chain_43_first(&mut self, arg: u32) {
        self.log.push(("chain_43_first", vec![arg]));
    }
    fn chain_43_second(&mut self, arg: u32) {
        self.log.push(("chain_43_second", vec![arg]));
    }
    fn copy_block(&mut self, arg: u32) {
        self.log.push(("copy_block", vec![arg]));
    }
    fn quant_vec(&mut self, arg: u32) {
        self.log.push(("quant_vec", vec![arg]));
    }
    fn quant_byte(&mut self, arg: u32) {
        self.log.push(("quant_byte", vec![arg]));
    }
    fn pair_first(&mut self, arg: u32) {
        self.log.push(("pair_first", vec![arg]));
    }
    fn pair_second(&mut self, arg: u32) {
        self.log.push(("pair_second", vec![arg]));
    }
    fn block_aux(&mut self, arg: u32) {
        self.log.push(("block_aux", vec![arg]));
    }
    fn sub_3e(&mut self, a0: u32, a1: u32, a_last: u32) {
        self.log.push(("sub_3e", vec![a0, a1, a_last]));
    }
    fn block_3a(&mut self, arg: u32) {
        self.log.push(("block_3a", vec![arg]));
    }
}

#[test]
fn registry_counts_eighteen_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 18);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 14);
    assert_eq!(registry::ROWS.len(), 32);
    for row in registry::ROWS {
        if row.state == State::Proven {
            assert_eq!(row.shape, "ParamBlock");
            assert!(!row.narrows.is_empty());
        }
    }
}

#[test]
fn block_is_64_bytes_and_blank_is_zero() {
    assert_eq!(PARAM_LEN, 0x40);
    let p = TaskParams::blank();
    assert_eq!(p.bytes(), &[0u8; 0x40]);
    assert_eq!((p.kind(), p.tag(), p.mode()), (0, 0, 0));
}

#[test]
fn from_bytes_round_trips() {
    let mut raw = [0u8; PARAM_LEN];
    for (i, b) in raw.iter_mut().enumerate() {
        *b = u8::try_from(i).unwrap().wrapping_mul(7);
    }
    let p = TaskParams::from_bytes(raw);
    assert_eq!(p.bytes(), &raw);
}

#[test]
fn kind_3e_stores_word_mode_and_tag() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_3e(&mut init, 0x1234, 0xaaaa, 0xbbbb, 0xdead_beef);
    assert_eq!(p.tag(), 4);
    assert_eq!(p.mode(), 1);
    assert_eq!(&p.bytes()[0x18..0x1c], &0xdead_beefu32.to_le_bytes());
    assert_eq!(
        init.log,
        [
            ("base", vec![0x3e]),
            ("main", vec![0x3e, 0x1234, 0xbbbb, 0xaaaa, 1]),
        ]
    );
}

#[test]
fn kind_36_mode_adds_eleven_wrapping() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_36(&mut init, 0, 0, 0, 0x11, 0x2233_4455, 0xfa, 0x77);
    assert_eq!(p.tag(), 1);
    assert_eq!(p.mode(), 0xfau8.wrapping_add(11));
    assert_eq!(p.bytes()[0x1c], 0x77);
    assert_eq!(p.bytes()[0x1d], 0x11);
    assert_eq!(p.bytes()[0x1e], 0xfa);
    assert_eq!(&p.bytes()[0x18..0x1c], &0x2233_4455u32.to_le_bytes());
}

#[test]
fn kind_36_mode_wraps_past_ff() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_36(&mut init, 0, 0, 0, 0, 0, 0xff, 0);
    assert_eq!(p.mode(), 10);
}

#[test]
fn kind_34_folds_twice_arg_minus_one_into_bits_1_to_3() {
    // Prior flag byte 0xf1, argument 1: twice zero xors nothing.
    let mut raw = [0u8; PARAM_LEN];
    raw[0x24] = 0xf1;
    let mut p = TaskParams::from_bytes(raw);
    let mut init = Fake::default();
    p.init_34(&mut init, 0, 0, 0, 1, 0x1111_1111, 0x2222_2222);
    assert_eq!(p.tag(), 1);
    assert_eq!(p.mode(), 4);
    assert_eq!(p.bytes()[0x24], 0xf1);
    assert_eq!(&p.bytes()[0x18..0x1c], &0x1111_1111u32.to_le_bytes());
    assert_eq!(&p.bytes()[0x1c..0x20], &0x2222_2222u32.to_le_bytes());
    // Argument 3 sets bits 1..3 of a zero byte to 0b100.
    let mut p = TaskParams::blank();
    p.init_34(&mut init, 0, 0, 0, 3, 0, 0);
    assert_eq!(p.bytes()[0x24], 0x04);
}

#[test]
fn kind_3f_toggles_bit_1_from_bit_0_of_arg() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_3f(&mut init, 0, 0, 0, 1);
    assert_eq!(p.tag(), 9);
    assert_eq!(p.bytes()[0x03], 2);
    // High bits of the argument are ignored: bit 0 clear leaves bit 1 clear.
    let mut p = TaskParams::blank();
    p.init_3f(&mut init, 0, 0, 0, 0xfe);
    assert_eq!(p.bytes()[0x03], 0);
    // Other flag bits survive.
    let mut raw = [0u8; PARAM_LEN];
    raw[0x03] = 0xfd;
    let mut p = TaskParams::from_bytes(raw);
    p.init_3f(&mut init, 0, 0, 0, 0);
    assert_eq!(p.bytes()[0x03], 0xfd);
}

#[test]
fn kind_40_toggles_bit_0_and_runs_quantiser_after_stores() {
    let mut raw = [0u8; PARAM_LEN];
    raw[0x22] = 0xaa;
    let mut p = TaskParams::from_bytes(raw);
    let mut init = Fake::default();
    p.init_40(&mut init, 9, 1, 2, 0x5555_6666, 0x7777, 1);
    assert_eq!(p.tag(), 8);
    assert_eq!(p.bytes()[0x22], 0xab);
    assert_eq!(&p.bytes()[0x18..0x1c], &0x5555_6666u32.to_le_bytes());
    assert_eq!(
        init.log,
        [
            ("base", vec![0x40]),
            ("main", vec![0x40, 9, 2, 1, 1]),
            ("quant_word", vec![0x7777]),
        ]
    );
}

#[test]
fn kind_2f_packs_five_words_and_flag_bits() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    // All packing arguments zero: packed byte 0, mode 0.
    p.init_2f(&mut init, 0, 0, 0, 0x99, 0, 1, 2, 3, 4, 5, 0, 0, 0, 0);
    assert_eq!(p.tag(), 2);
    assert_eq!(p.bytes()[0x3c], 0);
    assert_eq!(p.mode(), 0);
    for (i, w) in [1u32, 2, 3, 4, 5].iter().enumerate() {
        let off = 0x18 + i * 4;
        assert_eq!(&p.bytes()[off..off + 4], &w.to_le_bytes());
    }
    assert_eq!(init.log[2], ("block_2f", vec![0x99]));
    // Bit 3 of the packed byte adds 2 to the mode.
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_2f(&mut init, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 3, 1, 1, 0);
    // packed = ((((1 | 0) << 2 | 3) * 2 + 1) << 3) | 7 = 0x7f; mode = 7 + 2.
    assert_eq!(p.bytes()[0x3c], 0x7f);
    assert_eq!(p.mode(), 9);
}

#[test]
fn kind_43_runs_four_calls_then_tag() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_43(&mut init, 5, 6, 7, 8, 9);
    assert_eq!(p.tag(), 7);
    assert_eq!(
        init.log,
        [
            ("base", vec![0x43]),
            ("main", vec![0x43, 5, 7, 6, 1]),
            ("chain_43_first", vec![8]),
            ("chain_43_second", vec![9]),
        ]
    );
    // Nothing else in the block moves.
    let mut expect = [0u8; PARAM_LEN];
    expect[0x02] = 7;
    assert_eq!(p.bytes(), &expect);
}

#[test]
fn kinds_do_not_touch_each_others_bytes() {
    // A garbage block through kind 0x3e keeps every byte but mode,
    // the word slot and the tag.
    let mut raw = [0xccu8; PARAM_LEN];
    raw[0x03] = 0x81;
    let mut p = TaskParams::from_bytes(raw);
    let mut init = Fake::default();
    p.init_3e(&mut init, 0, 0, 0, 0);
    for (i, b) in p.bytes().iter().enumerate() {
        let expect = match i {
            0x14 => 1,
            0x18..=0x1b => 0,
            0x02 => 4,
            0x03 => 0x81,
            _ => 0xcc,
        };
        assert_eq!(*b, expect, "byte {i:#x}");
    }
}

#[test]
fn kind_3d_runs_main_only_with_flag_zero() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_3d(&mut init, 0x77, 1, 2);
    assert_eq!(p.tag(), 6);
    assert_eq!(init.log, [("main", vec![0x3d, 0x77, 2, 1, 0])]);
    let mut expect = [0u8; PARAM_LEN];
    expect[0x02] = 6;
    assert_eq!(p.bytes(), &expect);
}

#[test]
fn kind_42_runs_block_aux_then_tag() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_42(&mut init, 0, 0, 0, 0xabc);
    assert_eq!(p.tag(), 7);
    assert_eq!(init.log[1], ("block_aux", vec![0xabc]));
}

#[test]
fn kind_46_stores_word_at_1c() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_46(&mut init, 0, 0, 0, 0x55, 0x0102_0304);
    assert_eq!(p.tag(), 7);
    assert_eq!(&p.bytes()[0x1c..0x20], &0x0102_0304u32.to_le_bytes());
    assert_eq!(init.log[1], ("copy_block", vec![0x55]));
}

#[test]
fn kind_35_packs_five_args_over_old_flag_byte() {
    // a6=1, a5=0, a4=1, a3=2 over a zero byte: 1,2,5,41,82.
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_35(&mut init, 0, 0, 0, 0, 2, 1, 0, 1);
    assert_eq!(p.tag(), 1);
    assert_eq!(p.bytes()[0x1c], 82);
    // Old flag byte contributes bits 7 and 0 only.
    let mut raw = [0u8; PARAM_LEN];
    raw[0x1c] = 0xff;
    let mut p = TaskParams::from_bytes(raw);
    p.init_35(&mut init, 0, 0, 0, 0, 1, 0, 0, 0);
    assert_eq!(p.bytes()[0x1c], 0x81);
}

#[test]
fn kind_33_folds_into_1c_after_two_tail_calls() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_33(&mut init, 0, 0, 0, 0x11, 0x22, 3);
    assert_eq!(p.tag(), 1);
    assert_eq!(p.bytes()[0x1c], 0x04);
    assert_eq!(init.log[1], ("copy_block", vec![0x11]));
    assert_eq!(init.log[2], ("quant_vec", vec![0x22]));
}

#[test]
fn kind_41_packs_over_old_byte_and_stores_word() {
    let mut raw = [0u8; PARAM_LEN];
    raw[0x24] = 0xff;
    let mut p = TaskParams::from_bytes(raw);
    let mut init = Fake::default();
    p.init_41(&mut init, 0, 0, 0, 0, 0, 1, 0x99, 1, 0, 3);
    assert_eq!(p.tag(), 7);
    assert_eq!(p.bytes()[0x24], 0xfd);
    assert_eq!(&p.bytes()[0x20..0x24], &0x99u32.to_le_bytes());
    assert_eq!(init.log[2], ("quant_byte", vec![0]));
}

#[test]
fn kind_44_truncates_float_toward_zero() {
    let mut init = Fake::default();
    // 1.5 truncates to 1, doubled, plus the flag bit.
    let mut p = TaskParams::blank();
    p.init_44(&mut init, 0, 0, 0, 0, 0, 0x3fc0_0000, 1);
    assert_eq!(p.bytes()[0x20], 3);
    // NaN answers the most negative int: low byte zero.
    let mut p = TaskParams::blank();
    p.init_44(&mut init, 0, 0, 0, 0, 0, 0x7fc0_0000, 0);
    assert_eq!(p.bytes()[0x20], 0);
    // -1.5 truncates to -1: low byte doubled wraps.
    let mut p = TaskParams::blank();
    p.init_44(&mut init, 0, 0, 0, 0, 0, 0xbfc0_0000, 0);
    assert_eq!(p.bytes()[0x20], 0xfe);
    // 256.0 truncates to 256: low byte zero, flag bit survives.
    let mut p = TaskParams::blank();
    p.init_44(&mut init, 0, 0, 0, 0, 0, 0x4380_0000, 1);
    assert_eq!(p.bytes()[0x20], 1);
    assert_eq!(p.tag(), 7);
}

#[test]
fn kind_45_takes_the_block_arm_on_nonzero() {
    let mut raw = [0u8; PARAM_LEN];
    raw[0x28] = 0xff;
    let mut p = TaskParams::from_bytes(raw);
    let mut init = Fake::default();
    // a7=9 keeps bits 0..1 and 4..7, then packs zeros below.
    p.init_45(&mut init, 0, 0, 5, 0x21, 0, 0, 0, 0, 9, 0xaaaa, 0xbbbb);
    assert_eq!(init.log[1], ("block_aux", vec![0x21]));
    assert_eq!(p.bytes()[0x28], 0xf0);
    assert_eq!(&p.bytes()[0x20..0x24], &0xbbbbu32.to_le_bytes());
    assert_eq!(&p.bytes()[0x24..0x28], &0xaaaau32.to_le_bytes());
}

#[test]
fn kind_45_takes_the_pair_arm_on_zero_and_folds_small_selectors() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    // a7=2 folds bits 2..3 to 0b10, then packs zeros below.
    p.init_45(&mut init, 0, 0, 0, 0, 0x31, 0x32, 0, 0, 2, 0, 0);
    assert_eq!(init.log[1], ("pair_first", vec![0x31]));
    assert_eq!(init.log[2], ("pair_second", vec![0x32]));
    assert_eq!(p.bytes()[0x28], 8);
    assert_eq!(p.tag(), 7);
}

#[test]
fn kind_3a_stamps_kind_and_packs_flags() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    // a6=1, a5=1 over a zero byte, a4=0: 3, 6, 6.
    p.init_3a(&mut init, 1, 2, 0x41, 3, 0, 1, 1);
    assert_eq!(p.kind(), 0x3a);
    assert_eq!(p.mode(), 2);
    assert_eq!(p.tag(), 4);
    assert_eq!(p.bytes()[0x2c], 6);
    assert_eq!(init.log[0], ("sub_3e", vec![1, 2, 3]));
    assert_eq!(init.log[1], ("block_3a", vec![0x41]));
}

#[test]
fn kind_3b_sets_bit_1_on_equal_consts_only() {
    let mut init = Fake::default();
    let mut p = TaskParams::blank();
    p.init_3b(
        &mut init,
        0,
        0,
        5,
        0,
        0x1234,
        7,
        8,
        9,
        0x3f80_0000,
        0x3f80_0000,
    );
    assert_eq!(p.kind(), 0x3b);
    assert_eq!(p.mode(), 7);
    assert_eq!(p.bytes()[0x03] & 2, 2);
    // Equal-magnitude zeros compare equal too.
    let mut p = TaskParams::blank();
    p.init_3b(&mut init, 0, 0, 0, 0, 0, 0, 0, 0, 0x0000_0000, 0x8000_0000);
    assert_eq!(p.bytes()[0x03] & 2, 2);
    // NaN never sets the bit, even against itself.
    let mut p = TaskParams::blank();
    p.init_3b(&mut init, 0, 0, 0, 0, 0, 0, 0, 0, 0x7fc0_0000, 0x7fc0_0000);
    assert_eq!(p.bytes()[0x03] & 2, 0);
    // Unequal constants leave the flag byte alone.
    let mut p = TaskParams::blank();
    p.init_3b(&mut init, 0, 0, 0, 0, 0, 0, 0, 0, 0x3f80_0000, 0x4000_0000);
    assert_eq!(p.bytes()[0x03] & 2, 0);
}

#[test]
fn kind_3c_copies_three_words_and_stamps_kind() {
    let mut p = TaskParams::blank();
    let mut init = Fake::default();
    p.init_3c(&mut init, 9, 8, 7, &[1, 2, 3]);
    assert_eq!(p.kind(), 0x3c);
    assert_eq!(p.mode(), 0);
    assert_eq!(p.tag(), 5);
    assert_eq!(&p.bytes()[0x1c..0x20], &1u32.to_le_bytes());
    assert_eq!(&p.bytes()[0x20..0x24], &2u32.to_le_bytes());
    assert_eq!(&p.bytes()[0x24..0x28], &3u32.to_le_bytes());
    assert_eq!(init.log[0], ("sub_3e", vec![9, 8, 7]));
}
