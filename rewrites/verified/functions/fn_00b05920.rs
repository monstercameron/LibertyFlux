// original: 0x00b05920 sort_input_spans
/// Sort every input record span of the state object.
///
/// thiscall `(this)`: each span is a base pointer plus a 16-bit count
/// (end = base + count*8, UNSIGNED, wrapping) sorted by helper 1, cdecl
/// `(base, end, keyfn)`. The span at `+0x20/+0x24` always runs with the
/// first key function; the span at `+0x48/+0x4c` runs when the global
/// flag byte is set; the spans at `+0x10/+0x14` and `+0x18/+0x1c` run
/// when the other global flag byte is set; the spans at `+0x38/+0x3c`,
/// `+0x30/+0x34` and `+0x40/+0x44` always run with the second key
/// function. Returns the last helper answer.
export!(thiscall, rw_00b05920(this: u32) -> u32 {
    // File addresses; the worker relocates the original's immediates, so
    // the rewrite must relocate them too.
    const KEY_A_FILE: u32 = 0x00B0_58E0;
    const KEY_B_FILE: u32 = 0x00B0_5900;
    const FLAG_A: u32 = 0x0103_F71B;
    const FLAG_B: u32 = 0x0103_F71A;
    unsafe fn sort(base: u32, n: u32, f: u32) -> u32 {
        callee_cdecl!(1, u32, base, base.wrapping_add(n.wrapping_mul(8)), f)
    }
    unsafe {
        let rd = |off: u32| ((this + off) as *const u32).read_unaligned();
        let cnt = |off: u32| ((this + off) as *const u16).read_unaligned() as u32;
        let key_a = relocated(KEY_A_FILE);
        let key_b = relocated(KEY_B_FILE);
        let mut r = sort(rd(0x20), cnt(0x24), key_a);
        if *global::<u8>(FLAG_A) != 0 {
            r = sort(rd(0x48), cnt(0x4C), key_a);
        }
        if *global::<u8>(FLAG_B) != 0 {
            r = sort(rd(0x10), cnt(0x14), key_a);
            r = sort(rd(0x18), cnt(0x1C), key_a);
        }
        r = sort(rd(0x38), cnt(0x3C), key_b);
        r = sort(rd(0x30), cnt(0x34), key_b);
        r = sort(rd(0x40), cnt(0x44), key_b);
        r
    }
});
