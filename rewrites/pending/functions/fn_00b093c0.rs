// original: 0x00b093c0 scale_field_if_above_one
/// Rescale one field by another when the limit is not exactly one.
///
/// Reads the limit word as a float; when it equals 1.0 there is nothing to
/// do. Otherwise it fetches the peer object, asks for its status through
/// the peer's virtual slot, and only when the status reports ready divides
/// the value field by the limit in place.
export!(thiscall, rw_00b093c0(this_ptr: u32) -> u32 {
    unsafe {
        const LIMIT_OFF: usize = 0x1bc;
        const VALUE_OFF: usize = 0x60;
        const STATUS_SLOT: usize = 0x28;
        const READY_CODE: u32 = 0x1b;
        let limit = *((this_ptr as usize + LIMIT_OFF) as *const f32);
        if limit == 1.0 {
            return 0;
        }
        let peer = callee_thiscall!(1, u32, this_ptr);
        if peer == 0 {
            return 0;
        }
        let table = *(peer as *const u32) as usize;
        let target = *((table + STATUS_SLOT) as *const u32) as usize;
        let status: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target);
        if status(peer) != READY_CODE {
            return 0;
        }
        let num = core::hint::black_box(*((this_ptr as usize + VALUE_OFF) as *const f32));
        let den = *((this_ptr as usize + LIMIT_OFF) as *const f32);
        *((this_ptr as usize + VALUE_OFF) as *mut f32) = num / den;
        0
    }
});
