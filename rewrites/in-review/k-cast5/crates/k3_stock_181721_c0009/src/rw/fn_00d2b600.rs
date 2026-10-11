// original: 0x00d2b600 task_range_clamp (proposed)
/// Clamp a range estimate into [0x1f40, 0xffff]: with `s` the signed byte at
/// `+0x98` and `arr` the table at `+0x70`, return 0x1f40 at once when
/// `s >= [arr]`; otherwise take the entry at `(s+1)*2`, measure its
/// distance to `[p+0x20]` (`p` the argument), scale by the two global
/// factors, truncate, cap at 0xffff, and return and store at `+0xd2` the
/// maximum of that and 0x1f40. `+0xd0` is always cleared.
///
/// Thiscall, one stack word (pointer). Float order matches the original.
lf_checker_rt::export!(thiscall, rw_00d2b600(this: u32, p: u32) -> u32 {
    unsafe {
        const K1_GLOB: u32 = 0x00fe8ab8;
        const K2_GLOB: u32 = 0x00fe8c58;
        const LO: u32 = 0x1f40;
        const HI: i32 = 0xffff;
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        fn cvtt(x: f32) -> i32 {
            if x >= -2147483648.0 && x < 2147483648.0 {
                core::hint::black_box(x) as i32
            } else {
                i32::MIN
            }
        }
        let s = ((this + 0x98) as *const i8).read_unaligned() as i32;
        let arr = ((this + 0x70) as *const u32).read_unaligned();
        ((this + 0xd0) as *mut u16).write_unaligned(0);
        if s >= (arr as *const u32).read_unaligned() as i32 {
            ((this + 0xd2) as *mut u16).write_unaligned(LO as u16);
            return LO;
        }
        let q = ((p + 0x20) as *const u32).read_unaligned();
        let e = (s + 1) * 2;
        let base = (arr as u32).wrapping_add((e as u32).wrapping_mul(8));
        let dy = sub((base.wrapping_add(4) as *const f32).read_unaligned(), ((q + 0x34) as *const f32).read_unaligned());
        let dx = sub((base as *const f32).read_unaligned(), ((q + 0x30) as *const f32).read_unaligned());
        let dz = sub((base.wrapping_add(8) as *const f32).read_unaligned(), ((q + 0x38) as *const f32).read_unaligned());
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let k1 = lf_checker_rt::global::<f32>(K1_GLOB).read_unaligned();
        let k2 = lf_checker_rt::global::<f32>(K2_GLOB).read_unaligned();
        let mut n = cvtt(mul(mul(d2.sqrt(), k1), k2));
        if n > HI {
            n = HI;
        }
        let v = n as u16;
        if v as u32 > LO {
            ((this + 0xd2) as *mut u16).write_unaligned(v);
            v as u32
        } else {
            ((this + 0xd2) as *mut u16).write_unaligned(LO as u16);
            LO
        }
    }
});
