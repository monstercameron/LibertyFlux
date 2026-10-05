// original: 0x00c48b10 ccam_dispatch_vt28 (proposed)
/// Dispatch on the kind reported by the child's vtable slot 0x28.
///
/// Calls the child `obj`'s kind hook (callee 1, through its vtable)
/// and dispatches on the result: kind `0x17` runs the append routine
/// (callee 6), kind `0x18` the reset routine (callee 5), anything else
/// except `0x24` returns the adjusted kind with a zero low byte. Kind
/// `0x24` consults the state byte at `obj + STATE`: when it is 0, or
/// when the mode byte fetched through `obj + MODE_BASE` is at least
/// `0x10`, the update routine runs with a null argument (callee 4);
/// otherwise the prepare routine runs first (callee 2) and its result
/// is passed to the update routine (callee 3). Returns whatever the
/// chosen routine returned.
///
/// Original: stdcall, one stack word (the entry ecx is overwritten
/// before any use), callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(stdcall, rw_00c48b10(obj: u32) -> u32 {
    const VT_SLOT_KIND: u32 = 0x28;
    const KIND_APPEND: u32 = 0x17;
    const KIND_RESET: u32 = 0x18;
    const KIND_UPDATE: u32 = 0x24;
    const STATE: u32 = 0x18c;
    const MODE_BASE: u32 = 0x184;
    const MODE_OFF: u32 = 0xee7a20;
    const MODE_READY: i8 = 0x10;
    const KIND_HOOK: u32 = 1;
    const PREPARE: u32 = 2;
    const UPDATE_WITH: u32 = 3;
    const UPDATE_NULL: u32 = 4;
    const RESET: u32 = 5;
    const APPEND: u32 = 6;
    unsafe {
        let vtable = (obj as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + VT_SLOT_KIND) as *const u32).read_unaligned() as usize);
        let kind = hook(obj);
        if kind == KIND_APPEND {
            lf_checker_rt::callee_thiscall!(APPEND, u32, obj)
        } else if kind == KIND_RESET {
            lf_checker_rt::callee_thiscall!(RESET, u32, obj)
        } else if kind != KIND_UPDATE {
            kind.wrapping_sub(KIND_UPDATE) & 0xffff_ff00
        } else if ((obj + STATE) as *const u8).read() == 0 {
            lf_checker_rt::callee_thiscall!(UPDATE_NULL, u32, obj, 0)
        } else {
            let base = ((obj + MODE_BASE) as *const u32).read_unaligned();
            let mode = ((base.wrapping_add(MODE_OFF)) as *const i8).read();
            if mode >= MODE_READY {
                lf_checker_rt::callee_thiscall!(UPDATE_NULL, u32, obj, 0)
            } else {
                let prepared = lf_checker_rt::callee_thiscall!(PREPARE, u32, obj);
                lf_checker_rt::callee_thiscall!(UPDATE_WITH, u32, obj, prepared)
            }
        }
    }
});
