// original: 0x00c7fbe0 CTaskComplexScenario::vf22

/// Dispatch the child scenario: tail handler for kind `0x15e`, field for `0x123`.
///
/// Reads the child at `this+8` (null yields 0) and asks its kind through
/// virtual slot `+0xc`. Kind `0x15e` tail-jumps to virtual slot `+0x58` and
/// returns its result. Otherwise the kind is asked a second time: kind
/// `0x123` returns the word at `child+0x24`, anything else returns 0. The
/// original's tail is a computed jump through the vtable; the stub's plain
/// return reaches the caller the same way, so the rewrite calls it normally.
///
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00c7fbe0(this: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 8;
        const VT_KIND: u32 = 0x0c;
        const VT_TAIL: u32 = 0x58;
        const KIND_TAIL: u32 = 0x15e;
        const KIND_FIELD: u32 = 0x123;
        const FIELD_OFF: u32 = 0x24;
        type Hook = extern "thiscall" fn(u32) -> u32;
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }
        let vt = (child as *const u32).read_unaligned();
        let kind_of: Hook =
            core::mem::transmute(((vt + VT_KIND) as *const u32).read_unaligned() as usize);
        if kind_of(child) == KIND_TAIL {
            let tail: Hook =
                core::mem::transmute(((vt + VT_TAIL) as *const u32).read_unaligned() as usize);
            return tail(child);
        }
        if kind_of(child) == KIND_FIELD {
            return ((child + FIELD_OFF) as *const u32).read_unaligned();
        }
        0
    }
});
