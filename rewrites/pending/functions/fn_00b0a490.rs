// original: 0x00b0a490 attach_and_notify
/// Attach the peer to a fresh mode object, then notify and release it.
///
/// Builds the mode object, picks the attach kind from this object's flag
/// bits, attaches the peer, marks the mode linked, releases the two
/// registered notifiers when the first one reports active, then detaches
/// the peer through its own virtual slot. Returns the mode object.
export!(thiscall, rw_00b0a490(this_ptr: u32) -> u32 {
    unsafe {
        const FLAGS_OFF: usize = 0x1c4;
        const LINK_OFF: usize = 0x13c;
        const ACTIVE_OFF: usize = 0x38f;
        const FINISH_SLOT: usize = 0x30;
        const ACTIVE_CODE: u8 = 3;
        let mode = callee_thiscall!(1, u32, this_ptr, 0x1bu32, 0u32);
        let peer = callee_cdecl!(2, u32,);
        let flags = *((this_ptr as usize + FLAGS_OFF) as *const u8);
        let kind = if flags & 0x20 != 0 {
            1u32
        } else if flags & 0x10 != 0 {
            4u32
        } else {
            0u32
        };
        callee_thiscall!(3, u32, mode, peer, kind, 1u32);
        *((mode as usize + LINK_OFF) as *mut u8) |= 0x0c;
        let first = callee_thiscall!(4, u32, this_ptr, 1u32, 0u32);
        if first != 0 && *((first as usize + ACTIVE_OFF) as *const u8) == ACTIVE_CODE {
            callee_thiscall!(5, u32, first, 0u32);
            let second = callee_thiscall!(4, u32, this_ptr, 1u32, 1u32);
            if second != 0 {
                callee_thiscall!(5, u32, second, 0u32);
            }
        }
        callee_thiscall!(6, u32, peer, 1u32);
        let table = *(peer as *const u32) as usize;
        let at = *((table + FINISH_SLOT) as *const u32) as usize;
        let finish: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(at);
        finish(peer, 0);
        mode
    }
});
