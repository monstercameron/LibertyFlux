// original: 0x00a7dda0 CSimpleNMFlinchTaskInfo::CSimpleNMFlinchTaskInfo
/// Construct a flinch task info: a 3-word vector, an owner pointer and a
/// flag byte, plus the owner's network id (or zero without one).
export!(thiscall, rw_00a7dda0(this: u32, src_vec: u32, owner: u32, flags: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1DCC;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        let src = src_vec as *const u32;
        let dst = obj.add(0x20) as *mut u32;
        dst.write_unaligned(src.read_unaligned());
        dst.add(1).write_unaligned(src.add(1).read_unaligned());
        dst.add(2).write_unaligned(src.add(2).read_unaligned());
        (obj.add(0x30) as *mut u32).write_unaligned(owner);
        *obj.add(0x34) = flags as u8;
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
        ((this as *mut u8).add(0x36) as *mut u16).write_unaligned(net_id);
    }
    this
});
