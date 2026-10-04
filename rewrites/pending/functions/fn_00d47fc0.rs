// original: 0x00d47fc0 set_fixture_kind_by_name
/// Set the kind field of one fixture object by matching a name string.
///
/// Compares the input name against five built-in names in order
/// ("COUCH", "CHAIR", "STOOL", "STEP", "WALL"). On the first match with
/// 1-based position `n`, loads the object pointer `this[1][index]` and
/// stores `n` at offset 0xC of that object, returning the object pointer.
/// When nothing matches, returns the sign (-1 or +1) of the last
/// comparison, like the C strcmp the original inlines.
export!(thiscall, rw_00d47fc0(this: u32, index: u32, name: u32) -> u32 {
    unsafe {
        const LITS: [u32; 5] = [0xEE4108, 0xEE4110, 0xEE4118, 0xEE4120, 0xEE4128];
        const ARRAY_SLOT: usize = 1;
        const KIND_OFF: u32 = 0xC;
        let cmp_name = |input: u32, lit: u32| -> i32 {
            let mut off = 0u32;
            loop {
                let a = *((input.wrapping_add(off)) as *const u8);
                let b = *((lit.wrapping_add(off)) as *const u8);
                if a != b {
                    return if (a as u32) < (b as u32) { -1 } else { 1 };
                }
                if a == 0 {
                    return 0;
                }
                off = off.wrapping_add(1);
            }
        };
        let mut last = 1i32;
        for (i, lit) in LITS.iter().enumerate() {
            let r = cmp_name(name, relocated(*lit));
            if r == 0 {
                let arr = *((this as *const u32).add(ARRAY_SLOT));
                let obj = *((arr as *const u32).add(index as usize));
                *((obj.wrapping_add(KIND_OFF)) as *mut u32) = (i as u32) + 1;
                return obj;
            }
            last = r;
        }
        last as u32
    }
});
