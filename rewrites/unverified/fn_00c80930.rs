// original: 0x00c80930 CTaskComplexScenario::vf21

/// Dispatch the child scenario by its kind tag, following one level down.
///
/// Reads the child at `this+8` (null yields 0) and asks its kind through
/// virtual slot `+0xc`. Kind `0x15e` tail-calls virtual slot `+0x54` (the frame
/// is popped first, so the stack is at entry level at the jump); kind `0x123`
/// returns the word at `child+0x28`; kind `0xdf` descends to the grandchild at
/// `child+8` (null yields 0) and returns `grandchild+0x1c` when its kind is
/// `0xdd`. Anything else yields 0. EAX is defined on every path.
///
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00c80930(this: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 8;
        const VT_KIND: u32 = 0x0c;
        const VT_TAIL: u32 = 0x54;
        const KIND_TAIL: u32 = 0x15e;
        const KIND_VALUE: u32 = 0x123;
        const KIND_DOWN: u32 = 0xdf;
        const KIND_GRAND: u32 = 0xdd;
        const VALUE_OFF: u32 = 0x28;
        const GRAND_OFF: u32 = 0x1c;
        type Hook = extern "thiscall" fn(u32) -> u32;
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }
        let vt = (child as *const u32).read_unaligned();
        let kind_of: Hook =
            core::mem::transmute(((vt + VT_KIND) as *const u32).read_unaligned() as usize);
        let r1 = kind_of(child);
        if r1 == KIND_TAIL {
            let tail: Hook =
                core::mem::transmute(((vt + VT_TAIL) as *const u32).read_unaligned() as usize);
            return tail(child);
        }
        let r2 = kind_of(child);
        if r2 == KIND_VALUE {
            return ((child + VALUE_OFF) as *const u32).read_unaligned();
        }
        let r3 = kind_of(child);
        if r3 != KIND_DOWN {
            return 0;
        }
        let grand = ((child + CHILD_OFF) as *const u32).read_unaligned();
        if grand == 0 {
            return 0;
        }
        let gvt = (grand as *const u32).read_unaligned();
        let gkind: Hook =
            core::mem::transmute(((gvt + VT_KIND) as *const u32).read_unaligned() as usize);
        if gkind(grand) != KIND_GRAND {
            return 0;
        }
        ((grand + GRAND_OFF) as *const u32).read_unaligned()
    }
});
