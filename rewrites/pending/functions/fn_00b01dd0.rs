// original: 0x00b01dd0 CViewport::vf2
/// If the handle at this+0x540 is valid, release it through the global
/// manager, invalidate the handle, then tail into the sub-object routine
/// at this+0x400. Returns the tail result. The manager address is a
/// relocated absolute (HIGHLOW entry verified).
export!(thiscall, rw_00b01dd0(this_: *mut u8) -> u32 {
    let tag = unsafe { (this_.add(0x540) as *const u32).read_unaligned() };
    if tag != 0xFFFF_FFFF {
        let _: u32 = callee_thiscall!(1, u32, relocated(0x161547c), tag);
    }
    unsafe {
        (this_.add(0x540) as *mut u32).write_unaligned(0xFFFF_FFFF);
    }
    callee_thiscall!(2, u32, unsafe { this_.add(0x400) } as u32)
});
