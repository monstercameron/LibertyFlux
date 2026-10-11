// original: 0x00BE80C0 anim_pick_or_sample (proposed)
// Specification of the float-to-integer conversion (lane a-S24). The original multiplies the count returned
// by the count callee (a signed 32-bit integer, converted to float) by a fraction in [0, 2) built from the low
// 16 bits of the random callee's word, then truncates the product to a 32-bit integer with cvttss2si. The
// withdrawn form used Rust's `as i32`, which saturates and maps NaN to 0, so a product at or above 2^31 stored
// 0x7fffffff where the processor stores 0x80000000. `cvtt_s32` returns the processor's value for NaN and for
// every product outside the signed 32-bit range, and otherwise truncates exactly as the instruction does.

/// Truncating float-to-integer conversion: the processor's cvttss2si r32 bit for bit. NaN and every value
/// outside the signed 32-bit range give the integer indefinite 0x80000000 (i32::MIN); inside the range `as`
/// truncates toward zero exactly as the instruction does.
#[inline(always)]
fn cvtt_s32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}
/// Resolve the blend handle for a slot, directly or by random sampling.
///
/// `obj` points to the task object and `target` to the subject. The
/// words at `+0x18`/`+0x1c` select the path: when the first is -1 the
/// stored handle at `+0x14` is reused as-is; otherwise, when the second
/// is set, the compute callee runs on the mover at `[target+0x78]` with
/// (first, second, the 1000.0 weight, -1). When the second is -1, the
/// row is sampled instead: the open callee (cdecl, first) gives a set,
/// the count callee (cdecl, set) its size, the random callee (cdecl)
/// supplies bits of which the low 16 become a unit fraction through the
/// 2^-15 constant, and the truncated product with the count picks the
/// index for the fetch callee (cdecl, set, index); the wide compute
/// callee then runs on the mover with (set, fetched, 0xd600, 3, the 8.0
/// weight). The handle is stored at `+0x14` on both compute paths, and
/// the attach callee always links this object (thiscall on the handle at
/// `+0x14`, with 1, the completion callback and this). Returns the
/// attach callee's answer.
///
/// Original: 0x00BE80C0 (thiscall, one stack word). The float product is
/// evaluated in the original's operand order; the count answers stay
/// small so the float-to-int conversion never overflows (where the
/// x86 instruction and Rust disagree).
lf_checker_rt::export!(thiscall, rw_00BE80C0(obj: u32, target: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x18;
        const SECOND: u32 = 0x1c;
        const HANDLE: u32 = 0x14;
        const MOVER: u32 = 0x78;
        const WEIGHT_DIRECT: u32 = 0x447a_0000;
        const WEIGHT_SAMPLE: u32 = 0x4100_0000;
        const UNIT_FRAC: f32 = f32::from_bits(0x3800_0000);
        const FETCH_KIND: u32 = 0xd600;
        const FETCH_FLAG: u32 = 3;
        const CALLBACK: u32 = 0x00BE4D80;
        const COMPUTE: u32 = 1;
        const OPEN: u32 = 2;
        const COUNT: u32 = 3;
        const RANDOM: u32 = 4;
        const FETCH: u32 = 5;
        const COMPUTE_WIDE: u32 = 6;
        const ATTACH: u32 = 7;
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let first = ((obj + FIRST) as *const u32).read_unaligned();
        if first != 0xffff_ffff {
            let second = ((obj + SECOND) as *const u32).read_unaligned();
            let h: u32;
            if second != 0xffff_ffff {
                let mover = ((target + MOVER) as *const u32).read_unaligned();
                h = lf_checker_rt::callee_thiscall!(
                    COMPUTE, u32, mover, first, second, WEIGHT_DIRECT, 0xffff_ffff
                );
            } else {
                let set: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, first);
                let n: u32 = lf_checker_rt::callee_cdecl!(COUNT, u32, set);
                let bits: u32 = lf_checker_rt::callee_cdecl!(RANDOM, u32,);
                let frac = fmul((bits & 0xffff) as f32, UNIT_FRAC);
                let idx = cvtt_s32(fmul((n as i32) as f32, frac)) as i32 as u32;
                let row: u32 = lf_checker_rt::callee_cdecl!(FETCH, u32, set, idx);
                let mover = ((target + MOVER) as *const u32).read_unaligned();
                h = lf_checker_rt::callee_thiscall!(
                    COMPUTE_WIDE, u32, mover, set, row, FETCH_KIND, FETCH_FLAG,
                    WEIGHT_SAMPLE
                );
            }
            ((obj + HANDLE) as *mut u32).write_unaligned(h);
        }
        let handle = ((obj + HANDLE) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            ATTACH, u32, handle, 1, lf_checker_rt::relocated(CALLBACK), obj
        )
    }
});
