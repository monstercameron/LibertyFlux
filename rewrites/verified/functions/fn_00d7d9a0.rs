// original: 0x00D7D9A0 copy_links_range_fallback (proposed)

/// Copy two link words, refreshing the source first when out of range.
///
/// When bit 3 of the flag byte at `a0 + 0xf1e` is clear the source is
/// refreshed unconditionally; otherwise the distance from the signed
/// word pair at `+0xf3c`/`+0xf3e` to the float pair at `[a0+0x20]+0x30`
/// is formed and the source is refreshed only when the distance is
/// strictly above 5.0 (ordered compare, NaN skips). Then `*a1` and `*a2`
/// receive the words at `+0xf40`/`+0xf44`, and the return is bit 4 of
/// the flag byte with `a2`'s upper bytes. Cdecl, three stack words.
/// Float operation order is the original's.
use lf_checker_rt::{callee_cdecl, export};

const REFRESH: u32 = 1;

export!(cdecl, rw_00d7d9a0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const FLAG_OFF: u32 = 0xf1e;
        const STALE_BIT: u8 = 8;
        const RET_BIT: u32 = 4;
        const X_OFF: u32 = 0xf3e;
        const Y_OFF: u32 = 0xf3c;
        const BLK_OFF: u32 = 0x20;
        const RANGE: f32 = f32::from_bits(0x40a0_0000); // 5.0
        let flags = ((a0 + FLAG_OFF) as *const u8).read();
        if flags & STALE_BIT == 0 {
            callee_cdecl!(REFRESH, u32, a0);
        } else {
            let x = ((a0 + X_OFF) as *const i16).read_unaligned() as i32 as f32;
            let y = ((a0 + Y_OFF) as *const i16).read_unaligned() as i32 as f32;
            let q = ((a0 + BLK_OFF) as *const u32).read_unaligned();
            let qx = f32::from_bits(((q + 0x34) as *const u32).read_unaligned());
            let qy = f32::from_bits(((q + 0x30) as *const u32).read_unaligned());
            let d0 = sub(x, qx);
            let d1 = sub(y, qy);
            let r = add(mul(d0, d0), mul(d1, d1)).sqrt();
            if r > RANGE {
                callee_cdecl!(REFRESH, u32, a0);
            }
        }
        (a1 as *mut u32).write_unaligned(((a0 + 0xf40) as *const u32).read_unaligned());
        (a2 as *mut u32).write_unaligned(((a0 + 0xf44) as *const u32).read_unaligned());
        (a2 & 0xffff_ff00) | (((flags >> RET_BIT) & 1) as u32)
    }
});
