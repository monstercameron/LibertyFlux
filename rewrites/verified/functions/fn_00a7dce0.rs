// original: 0x00a7dce0 CSimpleNMBraceTaskInfo::CSimpleNMBraceTaskInfo
/// Construct a brace task info: store the owner pointer and a mode byte,
/// then resolve the owner's network id through its handle, or zero when
/// there is no owner or no handle.
export!(thiscall, rw_00a7dce0(this: u32, owner: u32, mode: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1BD4;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x18) as *mut u32).write_unaligned(owner);
        *obj.add(0x1c) = mode as u8;
    }
    let net_id: u16 = if owner != 0 {
        let handle = unsafe { (owner.wrapping_add(0x6C) as *const u32).read_unaligned() };
        if handle != 0 {
            callee_thiscall!(3, u32, handle) as u16
        } else {
            0
        }
    } else {
        0
    };
    unsafe {
        ((this as *mut u8).add(0x1e) as *mut u16).write_unaligned(net_id);
    }
    this
});
