// original: 0x00CC6FA0 euphoria_contains_match (proposed)

/// Report whether any enumerated object carries a wanted tag.
///
/// Resolves the wanted-tag array for `kind` through the table callee (which
/// also reports how many tags there are), then walks every object the
/// enumerator callees yield: the object's tag (`+0xc`) is compared against
/// each wanted tag in turn. Returns 1 on the first match, 0 when the walk
/// ends or starts empty. A non-positive tag count skips the inner scan but
/// still walks the objects.
///
/// Original: 0x00CC6FA0 (stdcall, two stack words; 1-byte result).
lf_checker_rt::export!(stdcall, rw_00cc6fa0(obj: u32, kind: u32) -> u32 {
    unsafe {
        const TABLE_CALLEE: u32 = 1;
        const FIRST_CALLEE: u32 = 2;
        const NEXT_CALLEE: u32 = 3;
        const ITER_AT: u32 = 0x78;
        const TAG_AT: u32 = 0x0c;
        let mut tags = 0u32;
        let count = lf_checker_rt::callee_stdcall!(
            TABLE_CALLEE,
            u32,
            kind,
            &mut tags as *mut u32 as u32
        );
        let iter = (obj.wrapping_add(ITER_AT) as *const u32).read_unaligned();
        let mut cand = lf_checker_rt::callee_thiscall!(FIRST_CALLEE, u32, iter);
        if cand == 0 {
            return 0;
        }
        loop {
            if (count as i32) > 0 {
                let tag = (cand.wrapping_add(TAG_AT) as *const u32).read_unaligned();
                let mut i = 0u32;
                loop {
                    let want =
                        (tags.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
                    if tag == want {
                        return 1;
                    }
                    i = i.wrapping_add(1);
                    if !((i as i32) < (count as i32)) {
                        break;
                    }
                }
            }
            cand = lf_checker_rt::callee_thiscall!(NEXT_CALLEE, u32, iter);
            if cand == 0 {
                return 0;
            }
        }
    }
});
