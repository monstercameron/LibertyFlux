// original: 0x009339A0 tree_imposter_dispatch_indexed_object_followup

/// Dispatch `object_index` to the object's method at vtable byte offset
/// `0x10`. The object pointer comes from a global pointer table, its first
/// word points to a vtable, and the original index is forwarded as the
/// method's one stack argument. The method is thiscall: the object pointer is
/// in ECX, its return value is returned in EAX, and the caller later cleans the
/// stack argument. The table, object, and vtable slot are read before dispatch
/// so invalid pointers fault at the same stage as the original.
lf_checker_rt::export!(cdecl, rw_009339A0(object_index: u32) -> u32 {
    const OBJECT_TABLE_VA: u32 = 0x011A_2958;
    const POINTER_STRIDE: u32 = 4;
    const OBJECT_VTABLE_OFFSET: u32 = 0;
    const METHOD_SLOT_OFFSET: u32 = 0x10;
    const METHOD_CALLEE_ID: u32 = 1;

    let object_slot = OBJECT_TABLE_VA.wrapping_add(object_index.wrapping_mul(POINTER_STRIDE));
    let object = unsafe { lf_checker_rt::global::<u32>(object_slot).read_unaligned() };
    let vtable = unsafe {
        (object.wrapping_add(OBJECT_VTABLE_OFFSET) as *const u32).read_unaligned()
    };
    let method_address = unsafe {
        ((vtable.wrapping_add(METHOD_SLOT_OFFSET)) as *const u32).read_unaligned()
    };
    let _method_address = core::hint::black_box(method_address);
    lf_checker_rt::callee_thiscall!(METHOD_CALLEE_ID, u32, object, object_index)
});
