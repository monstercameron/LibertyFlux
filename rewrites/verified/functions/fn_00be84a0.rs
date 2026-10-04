// original: 0x00BE84A0 timer_format_min_sec (proposed)
/// Format a millisecond count as "mm:ss" into the object's text buffer.
///
/// `obj` points to the object; the count is the dword behind the pointer
/// at `+0x00`. Three multiply-and-shift divisions split it into a seconds
/// part (mod 60) and a minutes part (mod 100), which are formatted with
/// "%02d:%02d" into the buffer at `+0x14` through the format callee
/// (cdecl, buffer, format, minutes, seconds). Returns the callee's
/// answer. The divisions are reproduced exactly as wrapping signed
/// multiply/shift sequences, so negative counts behave as the original.
///
/// Original: 0x00BE84A0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00BE84A0(obj: u32) -> u32 {
    unsafe {
        const COUNT_PTR: u32 = 0x00;
        const TEXT: u32 = 0x14;
        const FMT: u32 = 0x00EB9638;
        const FORMAT: u32 = 1;
        const DIV_A: i32 = 0x1062_4dd3u32 as i32;
        const DIV_B: i32 = 0x8888_8889u32 as i32;
        const DIV_C: i32 = 0x51eb_851fu32 as i32;
        let slot = ((obj + COUNT_PTR) as *const u32).read_unaligned();
        let count = (slot as *const i32).read_unaligned();
        let hi_a = ((count as i64).wrapping_mul(DIV_A as i64) >> 32) as i32;
        let t = hi_a >> 6;
        let mut secs = t.wrapping_add(((t as u32) >> 31) as i32);
        let hi_b = ((secs as i64).wrapping_mul(DIV_B as i64) >> 32) as i32;
        let d = hi_b.wrapping_add(secs) >> 5;
        let mut mins = d.wrapping_add(((d as u32) >> 31) as i32);
        let mut back = mins.wrapping_shl(4);
        back = back.wrapping_sub(mins);
        back = back.wrapping_shl(2);
        secs = secs.wrapping_sub(back);
        let hi_c = ((mins as i64).wrapping_mul(DIV_C as i64) >> 32) as i32;
        let d3 = hi_c >> 5;
        let q = d3.wrapping_add(((d3 as u32) >> 31) as i32);
        mins = mins.wrapping_sub(q.wrapping_mul(100));
        lf_checker_rt::callee_cdecl!(
            FORMAT,
            u32,
            obj.wrapping_add(TEXT),
            lf_checker_rt::relocated(FMT),
            mins as u32,
            secs as u32
        )
    }
});

