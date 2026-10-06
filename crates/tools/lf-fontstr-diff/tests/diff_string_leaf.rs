//! Differential cases: the string's leaf accessors.
//!
//! Each lifted method against its verified rewrite on the same generated
//! inputs, comparing the return value and every written byte. Each case
//! also runs a deliberately wrong lift, which must be caught at least
//! once. 32-bit target only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_fontstr_diff::rewrites::*;
    use lf_fontstr_diff::rt::{self};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        F20A, F20D, F32_EDGE, F209, Image, OBJ_SIZE, PUSH_A, PUSH_B, Rng, SIZE, STYLE, STYLED,
        U32_EDGE, assert_only_changed, check_numbered, check_virtual, lift_of,
    };

    /// One fixture: the 32-bit image with planted row texts so every
    /// row the lift reads holds a terminator.
    struct Fixture {
        obj: Image,
    }

    impl Fixture {
        fn build(rng: &mut Rng, nrows: usize) -> Self {
            let mut obj = Image::random(OBJ_SIZE, rng);
            obj.w32(0, 0x1111_1111);
            // Live text: terminated early so copies stay short.
            let live_len = 8 + (rng.u32() % 24) as usize;
            let live = rng.cstr(live_len);
            obj.wbytes(support::TEXT, &live);
            for s in 0..nrows {
                let len = (rng.u32() % 40) as usize;
                let t = rng.cstr(len);
                obj.wbytes(support::ROW_TEXT + s * 256, &t);
            }
            Self { obj }
        }

        fn this(&self) -> u32 {
            self.obj.addr()
        }
    }

    // Deliberately wrong lifts: each must be caught at least once.
    mod wrong {
        use lf_input_frontend::font_string::FontString;

        pub fn style_word_into(s: &FontString, out: &mut u32) {
            *out = s.style().wrapping_add(1);
        }

        pub fn set_style(s: &mut FontString, value: u32) -> u32 {
            s.set_style(value ^ 0xFFFF_FFFF)
        }

        pub fn set_size(s: &mut FontString, value: f32) {
            s.set_size(-value);
        }

        pub fn set_push_a(s: &mut FontString, value: f32) {
            s.set_push_a(f32::from_bits(value.to_bits().wrapping_add(1)));
        }

        pub fn set_push_b(s: &mut FontString, value: f32) {
            let _ = s;
            let _ = value;
        }

        pub fn set_styled(s: &mut FontString, value: u8) {
            s.set_styled(value.wrapping_add(1));
        }

        pub fn set_flag_209(s: &mut FontString, value: u8) {
            s.set_flag_209(value.wrapping_add(1));
        }

        pub fn set_flag_20a(s: &mut FontString, value: u8) {
            s.set_flag_20a(value.wrapping_add(1));
        }

        pub fn set_flag_20d(s: &mut FontString, value: u8) {
            s.set_flag_20d(value.wrapping_add(1));
        }
    }

    #[test]
    fn diff_style_word_into() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x6E33);
        let mut caught = 0;
        for i in 0..40u32 {
            let mut fx = Fixture::build(&mut rng, 3);
            let style = if i < 12 {
                U32_EDGE[i as usize]
            } else {
                rng.u32()
            };
            fx.obj.w32(STYLE, style);
            let s = lift_of(&fx.obj, 3);
            let before = fx.obj.buf.to_vec();
            let out = Box::new(0xDEAD_BEEFu32);
            let r = unsafe { fn_00db7050::rw_00db7050(fx.this(), addr_of_box(&out)) };
            assert_eq!(r, addr_of_box(&out), "answers out");
            assert_eq!(*out, style);
            let mut lout = 0xDEAD_BEEFu32;
            s.style_word_into(&mut lout);
            assert_eq!(lout, *out);
            assert_only_changed(&before, &fx.obj.buf, &[]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual(rt::take_virtual(), &[]);
            let mut wout = 0;
            wrong::style_word_into(&s, &mut wout);
            if wout != *out {
                caught += 1;
            }
        }
        assert!(caught > 0, "wrong style getter never caught");
    }

    fn addr_of_box(b: &Box<u32>) -> u32 {
        support::addr(&**b)
    }

    #[test]
    fn diff_set_style() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0x7130);
        let mut caught = 0;
        for i in 0..40u32 {
            let mut fx = Fixture::build(&mut rng, 3);
            let value = if i < 12 {
                U32_EDGE[i as usize]
            } else {
                rng.u32()
            };
            let src = Box::new(value);
            let mut s = lift_of(&fx.obj, 3);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db7320::rw_00db7320(fx.this(), support::addr(&*src)) };
            assert_eq!(r, value);
            assert_eq!(fx.obj.r32(STYLE), value);
            assert_eq!(s.set_style(value), value);
            assert_eq!(s.style(), fx.obj.r32(STYLE));
            assert_only_changed(&before, &fx.obj.buf, &[(STYLE, 4)]);
            check_numbered(rt::take_numbered(), &[]);
            check_virtual(rt::take_virtual(), &[]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 3);
            if wrong::set_style(&mut w, value) != value || w.style() != value {
                caught += 1;
            }
            let _ = src;
        }
        assert!(caught > 0, "wrong style setter never caught");
    }

    #[test]
    fn diff_float_setters() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xF107);
        let mut caught = [0, 0, 0];
        for i in 0..48u32 {
            let bits = if i < 14 {
                F32_EDGE[i as usize]
            } else {
                rng.u32()
            };
            // Size (vf121).
            let mut fx = Fixture::build(&mut rng, 3);
            let mut s = lift_of(&fx.obj, 3);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db7360::rw_00db7360(fx.this(), bits) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r32(SIZE), bits);
            s.set_size(f32::from_bits(bits));
            assert_eq!(s.size().to_bits(), bits);
            assert_only_changed(&before, &fx.obj.buf, &[(SIZE, 4)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 3);
            wrong::set_size(&mut w, f32::from_bits(bits));
            if w.size().to_bits() != bits {
                caught[0] += 1;
            }
            // First pushed float (vf123).
            let mut fx = Fixture::build(&mut rng, 3);
            let mut s = lift_of(&fx.obj, 3);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db7330::rw_00db7330(fx.this(), bits) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r32(PUSH_A), bits);
            s.set_push_a(f32::from_bits(bits));
            assert_eq!(s.pushed().0.to_bits(), bits);
            assert_only_changed(&before, &fx.obj.buf, &[(PUSH_A, 4)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 3);
            wrong::set_push_a(&mut w, f32::from_bits(bits));
            if w.pushed().0.to_bits() != bits {
                caught[1] += 1;
            }
            // Second pushed float (vf124).
            let mut fx = Fixture::build(&mut rng, 3);
            let mut s = lift_of(&fx.obj, 3);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db7390::rw_00db7390(fx.this(), bits) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r32(PUSH_B), bits);
            s.set_push_b(f32::from_bits(bits));
            assert_eq!(s.pushed().1.to_bits(), bits);
            assert_only_changed(&before, &fx.obj.buf, &[(PUSH_B, 4)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 3);
            wrong::set_push_b(&mut w, f32::from_bits(bits));
            if w.pushed().1.to_bits() != bits {
                caught[2] += 1;
            }
            check_numbered(rt::take_numbered(), &[]);
            check_virtual(rt::take_virtual(), &[]);
        }
        assert!(caught[0] > 0, "wrong size setter never caught");
        assert!(caught[1] > 0, "wrong push_a setter never caught");
        assert!(caught[2] > 0, "wrong push_b setter never caught");
    }

    #[test]
    fn diff_flag_setters() {
        let _guard = rt::script_lock();
        let mut rng = Rng(0xF7A6);
        let mut caught = [0, 0, 0, 0];
        // Every byte value with random upper residue in the argument.
        for v in 0..=255u32 {
            let arg = v | (rng.u32() & 0xFFFF_FF00);
            // Styled (vf127).
            let mut fx = Fixture::build(&mut rng, 1);
            let mut s = lift_of(&fx.obj, 1);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db7380::rw_00db7380(fx.this(), arg) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r8(STYLED), v as u8);
            s.set_styled(v as u8);
            assert_eq!(s.flags()[0], v as u8);
            assert_only_changed(&before, &fx.obj.buf, &[(STYLED, 1)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 1);
            wrong::set_styled(&mut w, v as u8);
            if w.flags()[0] != v as u8 {
                caught[0] += 1;
            }
            // Flag 209 (vf128): wrong lift stores the whole word's low
            // byte plus one, caught the same way.
            let mut fx = Fixture::build(&mut rng, 1);
            let mut s = lift_of(&fx.obj, 1);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db73b0::rw_00db73b0(fx.this(), arg) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r8(F209), v as u8);
            s.set_flag_209(v as u8);
            assert_eq!(s.flags()[1], v as u8);
            assert_only_changed(&before, &fx.obj.buf, &[(F209, 1)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 1);
            wrong::set_flag_209(&mut w, v as u8);
            if w.flags()[1] != v as u8 {
                caught[1] += 1;
            }
            // Flag 20A (vf129).
            let mut fx = Fixture::build(&mut rng, 1);
            let mut s = lift_of(&fx.obj, 1);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db73c0::rw_00db73c0(fx.this(), arg) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r8(F20A), v as u8);
            s.set_flag_20a(v as u8);
            assert_eq!(s.flags()[2], v as u8);
            assert_only_changed(&before, &fx.obj.buf, &[(F20A, 1)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 1);
            wrong::set_flag_20a(&mut w, v as u8);
            if w.flags()[2] != v as u8 {
                caught[2] += 1;
            }
            // Flag 20D (vf125).
            let mut fx = Fixture::build(&mut rng, 1);
            let mut s = lift_of(&fx.obj, 1);
            let before = fx.obj.buf.to_vec();
            let r = unsafe { fn_00db74b0::rw_00db74b0(fx.this(), arg) };
            assert_eq!(r, 0);
            assert_eq!(fx.obj.r8(F20D), v as u8);
            s.set_flag_20d(v as u8);
            assert_eq!(s.flags()[5], v as u8);
            assert_only_changed(&before, &fx.obj.buf, &[(F20D, 1)]);
            let mut w = lift_of(&Image::from_vec(before.clone()), 1);
            wrong::set_flag_20d(&mut w, v as u8);
            if w.flags()[5] != v as u8 {
                caught[3] += 1;
            }
            check_numbered(rt::take_numbered(), &[]);
            check_virtual(rt::take_virtual(), &[]);
        }
        assert!(caught.iter().all(|c| *c > 0), "a wrong flag lift escaped");
    }
}
