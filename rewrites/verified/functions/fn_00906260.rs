// original: 0x00906260 input_find_and_drop (proposed)
/// Find a slot by its two keys and drop it through the remover.
///
/// Scans the handle table for the first index (skipping the static default
/// index) whose object is live, whose kind byte at `+8` is set, and whose
/// words at `+0x24` and `+0x48` equal `key_a` and `key_b`; on a match the
/// remover callee is tail-called with `(index, 0)` and its answer returned.
/// When nothing matches the loop counter `0x5DC` with its low byte cleared
/// (`0x500`, from `(an instruction of the original)`) is returned. Cdecl, two stack words.
export!(cdecl, rw_00906260(key_a: u32, key_b: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static default index, skipped by the scan (file VA).
        const DEFAULT: u32 = 0x01034494;
        /// Table length scanned.
        const LEN: u32 = 0x5DC;
        const KIND_OFF: u32 = 0x08;
        const KEY_A_OFF: u32 = 0x24;
        const KEY_B_OFF: u32 = 0x48;
        const REMOVE_ID: u32 = 1;
        let dflt = (global::<u32>(DEFAULT)).read_unaligned();
        let mut idx: u32 = 0;
        loop {
            if idx != dflt {
                let obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4)))
                    as *const u32)
                    .read_unaligned();
                if obj != 0
                    && ((obj.wrapping_add(KIND_OFF)) as *const u8).read() != 0
                    && ((obj.wrapping_add(KEY_A_OFF)) as *const u32).read_unaligned()
                        == key_a
                    && ((obj.wrapping_add(KEY_B_OFF)) as *const u32).read_unaligned()
                        == key_b
                {
                    return callee_cdecl!(REMOVE_ID, u32, idx, 0u32);
                }
            }
            idx = idx.wrapping_add(1);
            if idx >= LEN {
                break;
            }
        }
        LEN & 0xFFFFFF00
    }
});
