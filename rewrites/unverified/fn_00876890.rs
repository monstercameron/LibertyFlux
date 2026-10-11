// original: 0x00876890 rage::crmtRequestExpression::vf2


/// Allocate a new expression request using the first incoming stack word.
/// Add a reference to the current source at +0x18 when present, release the
/// allocated object's previous child at +0x20 when present, replace that
/// child with the current source, call the request update helper with both
/// incoming words and the allocation, then return the allocation. The method
/// is thiscall with two stack words and callee cleanup.
lf_checker_rt::export!(thiscall, rw_00876890(this: u32, first_argument: u32, second_argument: u32) -> u32 {
    const SOURCE_MEMBER: u32 = 0x18;
    const CHILD_MEMBER: u32 = 0x20;
    const ALLOCATE: u32 = 1;
    const ADD_REF_SLOT: u32 = 4;
    const RELEASE_SLOT: u32 = 8;
    const UPDATE: u32 = 4;

    #[inline(always)]
    unsafe fn invoke_slot(object: u32, slot: u32) -> u32 {
        unsafe {
            let vtable = (object as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(slot) as *const u32).read_unaligned();
            let method: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            method(object)
        }
    }

    unsafe {
        let allocated = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, first_argument);
        let source = ((this + SOURCE_MEMBER) as *const u32).read_unaligned();
        if source != 0 { let _ = invoke_slot(source, ADD_REF_SLOT); }
        let previous_child = ((allocated + CHILD_MEMBER) as *const u32).read_unaligned();
        if previous_child != 0 { let _ = invoke_slot(previous_child, RELEASE_SLOT); }
        ((allocated + CHILD_MEMBER) as *mut u32).write_unaligned(source);
        let _ = lf_checker_rt::callee_thiscall!(UPDATE, u32, this, first_argument,
                                                 second_argument, allocated);
        allocated
    }
});
