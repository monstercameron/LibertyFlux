// original: 0x0091EF30 wstr_has_prefix
/// Test whether a wide string starts with a given wide prefix.
///
/// Returns 1 when every character of `prefix` matches the corresponding
/// character of `hay`, including when the prefix is empty; otherwise returns
/// 0 at the first mismatch. Only the low byte of `EAX` is meaningful (the
/// original leaves pointer residue in the upper bytes), so the return
/// channel compares `al` only.
export!(cdecl, rw_0091ef30(hay: u32, pre: u32) -> u32 {
    unsafe {
        let h = hay as *const u16;
        let p = pre as *const u16;
        if *p == 0 {
            return 1;
        }
        let mut i: usize = 0;
        loop {
            if *h.add(i) != *p.add(i) {
                return 0;
            }
            i += 1;
            if *p.add(i) == 0 {
                return 1;
            }
        }
    }
});
