// original: 0x00876860 crmt_expression_request_set_source

/// Acquire a reference to the incoming source when it is non-null, release
/// the current source at +0x18 when present, then store the incoming
/// pointer in that member slot. Both virtual calls use the object's vtable:
/// acquire is slot +0x04 and release is slot +0x08. The thiscall receiver is
/// in ECX, one pointer is passed on the stack, and there is no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00876860(this: u32, new_source: u32) -> () {
    const MEMBER_SLOT: u32 = 0x18;
    const ADD_REF_SLOT: u32 = 4;
    const RELEASE_SLOT: u32 = 8;

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
        if new_source != 0 {
            let _ = invoke_slot(new_source, ADD_REF_SLOT);
        }
        let old_source = ((this + MEMBER_SLOT) as *const u32).read_unaligned();
        if old_source != 0 {
            let _ = invoke_slot(old_source, RELEASE_SLOT);
        }
        ((this + MEMBER_SLOT) as *mut u32).write_unaligned(new_source);
    }
});
