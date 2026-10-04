// original: 0x00a7de60 CSimpleNMShotTaskInfo::CSimpleNMShotTaskInfo
/// Construct a shot task info: two words, an owner pointer and a flag
/// byte, plus the owner's network id (or zero without one).
export!(thiscall, rw_00a7de60(this: u32, word_a: u32, word_b: u32, owner: u32, flags: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1C1C;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj.add(0x18) as *mut u32).write_unaligned(word_a);
        (obj.add(0x1c) as *mut u32).write_unaligned(word_b);
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x20) as *mut u32).write_unaligned(owner);
        *obj.add(0x24) = flags as u8;
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
        ((this as *mut u8).add(0x26) as *mut u16).write_unaligned(net_id);
    }
    this
});
