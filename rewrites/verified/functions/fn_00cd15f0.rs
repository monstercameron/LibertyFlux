// original: 0x00cd15f0 melee_pair_find
/// Search the object's id table for a key pair, returning 1 if present else 0.
///
/// Like `melee_id_find`, but each 8-byte entry holds two dwords starting at
/// `this+0x64` and both must equal (`key`, `val`). Unsigned-bound scan,
/// first match wins. Thiscall with two stack arguments; only AL is compared.
export!(thiscall, rw_00cd15f0(this: u32, key: u32, val: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x60;
        const ITEMS_OFF: u32 = 0x64;
        const STRIDE: u32 = 8;
        let count = (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let entry = this
                .wrapping_add(ITEMS_OFF)
                .wrapping_add(i.wrapping_mul(STRIDE));
            let first = (entry as *const u32).read_unaligned();
            if key == first {
                let second = (entry.wrapping_add(4) as *const u32).read_unaligned();
                if val == second {
                    return 1;
                }
            }
            i = i.wrapping_add(1);
        }
        0
    }
});
