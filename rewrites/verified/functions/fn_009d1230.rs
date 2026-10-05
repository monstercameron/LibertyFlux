// original: 0x009D1230 strsafe_length_cch (proposed)
//
/// Bounded string length with validation, HRESULT-style status.
///
/// `count` must be nonzero and at most `max` (both compared UNSIGNED);
/// otherwise the status is `E_INVALIDARG` (`0x80070057`) and `*out_len` is
/// cleared. Else the first `count` bytes at `src` are scanned for a NUL:
/// none found is also `E_INVALIDARG`. On success `*out_len` gets the length
/// in bytes not counting the NUL; on the NUL-missing failure it is cleared.
/// When `out_len` is null nothing is stored but the status is still
/// computed. Stdcall, four stack words `(src, count, out_len, max)`.
lf_checker_rt::export!(stdcall, rw_009D1230(src: u32, count: u32, out_len: u32, max: u32) -> u32 {
    unsafe {
        const E_INVALIDARG: u32 = 0x80070057;
        if count == 0 || count > max {
            if out_len != 0 {
                (out_len as *mut u32).write_unaligned(0);
            } else {
                // Original still writes through the null pointer: a fault.
                core::ptr::write_volatile(0 as *mut u32, 0);
            }
            return E_INVALIDARG;
        }
        let mut rest = count;
        let mut p = src;
        let mut status = 0u32;
        loop {
            if (p as *const u8).read() == 0 {
                break;
            }
            p = p.wrapping_add(1);
            rest -= 1;
            if rest == 0 {
                status = E_INVALIDARG;
                break;
            }
        }
        if out_len != 0 {
            if (status as i32) < 0 {
                (out_len as *mut u32).write_unaligned(0);
            } else {
                (out_len as *mut u32).write_unaligned(count.wrapping_sub(rest));
            }
        }
        status
    }
});
