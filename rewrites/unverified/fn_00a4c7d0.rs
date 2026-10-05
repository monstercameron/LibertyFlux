// original: 0x00A4C7D0 NativeImpl_REMOVE_CAR_WINDOW

/// Removes the indexed window when the slot table names it.
///
/// Returns at once unless the word at `this + KIND` (0x1304) is 0, 1 or 2.
/// Takes `obj` from `this + PREF` (0x0DC4), falling back to `this + ALT`
/// (0x38); a null object returns. Calls the resolving callee (`obj` in
/// `ecx`, no stack words) for a descriptor, scales the argument by 4, and
/// resolves `info = [TABLE_ENTRY(model) + INFO]` (0xCC) from the global
/// pointer table by the signed word at `this + MODEL` (0x2E). Returns when
/// the dword at `arg4 + info` is -1. Otherwise scans the descriptor's
/// `count = byte[desc + COUNT]` (0x1F3) entries (zero skips the scan):
/// entry `i` is the word at `[array + 4*i] + ITEM` (0x0E) with `array =
/// [desc + ARRAY]` (0xD4), sign-extended; on equality with the slot dword
/// the acting callee runs (`obj` in `ecx`, `i` on the stack). Returns nothing
/// defined. (The original spills `arg4` over its dead incoming-arg slot; a
/// Rust rewrite cannot address that slot, so the contract switches the stack
/// check off and the rewrite keeps the value in a variable.)
///
/// Original: 0x00A4C7D0 (thiscall, one stack word), two callees.
lf_checker_rt::export!(thiscall, rw_00A4C7D0(this: u32, arg: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x1304;
        const PREF: u32 = 0x0DC4;
        const ALT: u32 = 0x38;
        const MODEL: u32 = 0x2E;
        const TABLE: u32 = 0x01295CD8;
        const INFO: u32 = 0xCC;
        const COUNT: u32 = 0x1F3;
        const ARRAY: u32 = 0xD4;
        const ITEM: u32 = 0x0E;
        const NONE: u32 = 0xFFFF_FFFF;
        const RESOLVE_CALLEE: u32 = 1;
        const ACT_CALLEE: u32 = 2;
        let kind = ((this + KIND) as *const u32).read_unaligned();
        if kind != 0 && kind != 1 && kind != 2 {
            return 0;
        }
        let mut obj = ((this + PREF) as *const u32).read_unaligned();
        if obj == 0 {
            obj = ((this + ALT) as *const u32).read_unaligned();
            if obj == 0 {
                return 0;
            }
        }
        let desc: u32 = lf_checker_rt::callee_thiscall!(RESOLVE_CALLEE, u32, obj);
        let arg4 = arg.wrapping_mul(4);
        let model = ((this + MODEL) as *const i16).read_unaligned() as i32;
        let slot = lf_checker_rt::relocated(TABLE)
            .wrapping_add((model * 4) as u32);
        let entry = (slot as *const u32).read_unaligned();
        let info =
            ((entry.wrapping_add(INFO)) as *const u32).read_unaligned();
        let slot_val =
            ((info.wrapping_add(arg4)) as *const u32).read_unaligned();
        if slot_val == NONE {
            return 0;
        }
        let count = ((desc + COUNT) as *const u8).read() as u32;
        if count == 0 {
            return 0;
        }
        let array = ((desc + ARRAY) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let item = ((((array.wrapping_add(i * 4)) as *const u32)
                .read_unaligned()
                + ITEM) as *const i16)
                .read_unaligned() as i32 as u32;
            if item == slot_val {
                lf_checker_rt::callee_thiscall!(ACT_CALLEE, u32, obj, i);
            }
            i += 1;
        }
        0
    }
});
