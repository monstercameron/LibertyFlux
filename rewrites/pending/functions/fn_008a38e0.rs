// original: 0x008a38e0 aud_weighted_pick_scan
/// Sum selected weights, scale the total, find the winning index.
///
/// Original 0x008A38E0 (`stdcall(hdr, n, elems)`): sums the weight word of
/// each of the `n` 8-byte elements unless the element index appears in the
/// exclusion list at `hdr+6` (active only when the count byte at `hdr+5`
/// is below `n`); passes `(0, sum)` to the scaler callee, which returns a
/// double; then subtracts weights in order from that total (as float) and
/// returns the first index where the remainder is ordered-less-or-equal to
/// zero, or 0 when all remainders stay positive (with `n`'s high bytes
/// preserved in the returned word, as the original only writes AL).
/// Scalar SSE intrinsics keep the operation order bit-exact.
export!(stdcall, rw_008a38e0(hdr: u32, n: u32, elems: u32) -> u32 {    use core::arch::x86::*;
    unsafe fn excluded(hdr: u32, gated: bool, count: u32, i: u32) -> bool {
        if !gated {
            return false;
        }
        let want = (i & 0xFF) as i32;
        let mut j = 0u32;
        while j < count {
            let b = ((hdr + 6 + j) as *const u8).read_unaligned() as i8 as i32;
            if b == want {
                return true;
            }
            j += 1;
        }
        false
    }
    unsafe {
        let count = ((hdr + 5) as *const u8).read_unaligned() as u32;
        let gated = count < n;
        let elem_weight = |i: u32| unsafe {
            _mm_load_ss((elems + 4 + i.wrapping_mul(8)) as *const f32)
        };
        let mut sum = _mm_setzero_ps();
        let mut i = 0u32;
        while i < n {
            if !excluded(hdr, gated, count, i) {
                sum = _mm_add_ss(sum, elem_weight(i));
            }
            i = i.wrapping_add(1);
        }
        let mut sum_bits = 0u32;
        _mm_store_ss(&mut sum_bits as *mut u32 as *mut f32, sum);
        let scaled: f64 = callee_cdecl!(1, f64, 0, sum_bits);
        let mut rest = scaled as f32;
        let _ = &mut rest;
        if n == 0 {
            return 0;
        }
        // Second pass in scalar SSE: x87-free, same order as the original.
        let mut rest_ss = _mm_set_ss(rest);
        let mut i = 0u32;
        loop {
            if !excluded(hdr, gated, count, i) {
                rest_ss = _mm_sub_ss(rest_ss, elem_weight(i));
                rest = _mm_cvtss_f32(rest_ss);
                if !(rest > 0.0 || rest.is_nan()) {
                    return (n & 0xFFFFFF00) | (i & 0xFF);
                }
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
        n & 0xFFFFFF00
    }
});
