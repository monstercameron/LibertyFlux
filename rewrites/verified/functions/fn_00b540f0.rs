// original: 0x00b540f0 crmtManagerChannel::vf0
/// Scalar deleting destructor: stamps the channel vtable, destroys
/// the trailing member, destroys 32 channel entries back to front,
/// destroys the head member, and frees `this` when bit 0 of the flag is
/// set. Returns `this`.
export!(thiscall, rw_00b540f0(this: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xEAF328);
        callee_thiscall!(1, u32, (this as u32).wrapping_add(0x80C));
        let mut e = (this as u32).wrapping_add(0x30C);
        let mut i = 0x1Fu32;
        loop {
            e = e.wrapping_sub(0x18);
            callee_thiscall!(1, u32, e);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        callee_thiscall!(2, u32, this as u32);
        if flag & 1 != 0 {
            callee_cdecl!(3, u32, this as u32);
        }
        this as u32
    }
});
