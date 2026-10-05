// original: 0x00a93db0 stream_advance_clock (proposed)

/// Advance the stream clock to stamp `dt` and release newly due ranges.
///
/// When `dt` exceeds the last stamp by more than the step constant (an
/// SSE `subss`/`comiss` pair against a read-only float) and the release
/// counter is below its limit, the release callee runs for the current
/// counter, which then increments and the stamp becomes `dt`. Then, while
/// the notify counter is below its limit, the current range's ids are
/// probed: a zero probe answer falls through to the retry callee, whose
/// non-zero answer breaks out of the range early; when the range drains
/// exactly, the notify callee runs for the counter and the counter
/// increments.
/// Floating point is one subtraction and one ordered comparison, pinned to
/// the original's operand order. The returned `eax` is whatever the last
/// executed block left there: the limit on early exit, the range table
/// base for a skipped range, the last probe answer after the loop, the
/// extra-plus-release sum when the notify is skipped, or the notify
/// answer.
///
/// Original: thiscall, one stack argument (float bits). Four callees
/// (stdcall 1 / cdecl 2 / cdecl 2 / stdcall 1 args).
lf_checker_rt::export!(thiscall, rw_00a93db0(this: u32, dt_bits: u32) -> u32 {
    unsafe {
        const STAMP_G: u32 = 0x012fb39c;
        const NOTIFY_COUNT_G: u32 = 0x012fb3a0;
        const RELEASE_COUNT_G: u32 = 0x012fb3a4;
        const LIMIT_G: u32 = 0x012fb394;
        const RANGE_TABLE_G: u32 = 0x012fb378;
        const ID_TABLE_G: u32 = 0x012fb388;
        const HANDLE_G: u32 = 0x012b4138;
        const EXTRA_G: u32 = 0x0103e8c8;
        const STEP_CONST: u32 = 0x00fe8c58;
        const RELEASE: u32 = 0;
        const PROBE: u32 = 1;
        const RETRY: u32 = 2;
        const NOTIFY: u32 = 3;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let dt = f32::from_bits(dt_bits);
        let last = f32::from_bits(rd32(lf_checker_rt::relocated(STAMP_G)));
        let step = f32::from_bits(rd32(lf_checker_rt::relocated(STEP_CONST)));
        let rel = rd32(lf_checker_rt::relocated(RELEASE_COUNT_G)) as i32;
        if sub(dt, last) > step {
            let lim = (rd16(lf_checker_rt::relocated(LIMIT_G)) as i32).wrapping_sub(1);
            if rel < lim {
                let _: u32 = lf_checker_rt::callee_stdcall!(RELEASE, u32, rel as u32);
                wr32(lf_checker_rt::relocated(RELEASE_COUNT_G),
                    (rel as u32).wrapping_add(1));
                wr32(lf_checker_rt::relocated(STAMP_G), dt_bits);
            }
        }
        let lim = (rd16(lf_checker_rt::relocated(LIMIT_G)) as i32).wrapping_sub(1);
        let note = rd32(lf_checker_rt::relocated(NOTIFY_COUNT_G)) as i32;
        if note >= lim {
            return lim as u32;
        }
        let ranges = rd32(lf_checker_rt::relocated(RANGE_TABLE_G));
        let ids = rd32(lf_checker_rt::relocated(ID_TABLE_G));
        let handle = rd32(lf_checker_rt::relocated(HANDLE_G));
        let mut k = rd32(ranges.wrapping_add((note as u32).wrapping_mul(4))) as i32;
        let hi = rd32(ranges.wrapping_add((note as u32).wrapping_mul(4)).wrapping_add(4)) as i32;
        let mut seen = ranges;
        while k < hi {
            let v = rd32(ids.wrapping_add((k as u32).wrapping_mul(4)));
            let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, v, handle);
            if probe & 0xff == 0 {
                seen = lf_checker_rt::callee_cdecl!(RETRY, u32, v, handle);
                if seen & 0xff != 0 {
                    break;
                }
            } else {
                seen = probe;
            }
            k = k.wrapping_add(1);
        }
        if k != hi {
            return seen;
        }
        let rel2 = rd32(lf_checker_rt::relocated(RELEASE_COUNT_G));
        let sum = rd32(lf_checker_rt::relocated(EXTRA_G)).wrapping_add(rel2);
        if note >= sum as i32 {
            return sum;
        }
        let answer: u32 = lf_checker_rt::callee_stdcall!(NOTIFY, u32, note as u32);
        wr32(lf_checker_rt::relocated(NOTIFY_COUNT_G), (note as u32).wrapping_add(1));
        let _ = this;
        answer
    }
});
