// original: 0x00cfbef0 CEventCallForCover::CEventCallForCover
/// Call the editable-response base constructor, install this event's vtable,
/// and store the handle at offset 0x18. For a non-null handle, call the attach
/// helper with the handle and the address of that field. Return this object.
lf_checker_rt::export!(thiscall, rw_00cfbef0(this: *mut u8, handle: u32) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00EDFC1C;
        const HANDLE_SLOT: usize = 0x18;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = lf_checker_rt::relocated(OBJECT_VTABLE);
        let handle_slot = this.add(HANDLE_SLOT) as *mut u32;
        *handle_slot = handle;
        if handle != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, handle, handle_slot as u32);
        }
        this as u32
    }
});
