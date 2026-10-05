// original: 0x00c3dbf0 scaled_table_bracket (proposed)
/// Step a cursor until a value is bracketed by a scaled u16 table.
///
/// `ctr` points at the cursor word, `nxt` at the next-index word, `f2b`
/// and `f4b` are float bit patterns, `count` the table length and `tab`
/// the table, whose entries are u16 at byte `6 + 12*i`. Each pass reads
/// `c = [ctr]`, forms `x1 = table[c]/3`, writes `[nxt] = c+1`, then either
/// (when `c+1 >= count`) writes `[nxt] = 0` and takes `x0 = f4`, or takes
/// `x0 = table[c+1]/3`. When `x1 > f2` the cursor moves to
/// `(c-1+count)%count`, when `f2 <= x0` (or either is NaN) it stops,
/// else it moves to `(c+1)%count`. The redundant second `x1 > f2` test in
/// the original is dead (it repeats the first with unchanged operands) and
/// is omitted. The exit value in EAX (a division quotient or table word)
/// is incidental, so the function is void.
///
/// Original: 0x00c3dbf0 (cdecl, six stack words, no calls).
lf_checker_rt::export!(cdecl, rw_00c3dbf0(
    ctr: u32,
    nxt: u32,
    f2b: u32,
    count: u32,
    f4b: u32,
    tab: u32,
) -> u32 {
    unsafe {
        const K: u32 = 0x3eaa_aaab; // 1/3 (const, embedded)
        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(p: u32) -> u16 {
            unsafe { (p as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn w32(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let k = f32::from_bits(K);
        let f2 = f32::from_bits(f2b);
        let f4 = f32::from_bits(f4b);
        loop {
            let c = rd32(ctr);
            let x1 = fmul(rd16(tab + 12 * c + 6) as f32, k);
            w32(nxt, c + 1);
            let x0 = if c + 1 >= count {
                w32(nxt, 0);
                f4
            } else {
                fmul(rd16(tab + 12 * (c + 1) + 6) as f32, k)
            };
            if x1 > f2 {
                w32(ctr, c.wrapping_sub(1).wrapping_add(count) % count);
            } else if !(f2 > x0) {
                break;
            } else {
                w32(ctr, (c + 1) % count);
            }
        }
        0
    }
});
