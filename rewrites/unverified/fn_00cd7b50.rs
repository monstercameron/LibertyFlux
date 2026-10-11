// original: 0x00CD7B50 task_seek_vector_dispatch

/// Dispatch a seek-vector request through the object's second function-table
/// entry. The subobject begins at byte offset `0x30`; its table pointer is at
/// offset zero and the selected function pointer is at table offset `4`. The
/// call receives the first argument, the object's word at offset `0x14`, and
/// the second argument, and its return value is forwarded unchanged.
///
/// Calling convention: thiscall with two 32-bit stack arguments. The
/// selected callback is also thiscall and takes three stack arguments.
lf_checker_rt::export!(thiscall, rw_00cd7b50(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    const SUBOBJECT: u32 = 0x30;
    const OBJECT_HANDLE: u32 = 0x14;
    const FUNCTION_SLOT: u32 = 4;

    unsafe {
        let subobject = this + SUBOBJECT;
        let table = (subobject as *const u32).read_unaligned();
        let callback_address = ((table + FUNCTION_SLOT) as *const u32).read_unaligned();
        let object_handle = ((this + OBJECT_HANDLE) as *const u32).read_unaligned();
        let callback: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callback_address as usize);
        callback(subobject, first_arg, object_handle, second_arg)
    }
});
