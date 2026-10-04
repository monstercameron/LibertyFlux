// original: 0x00a7c900 update_flags_notify
// Flag update with conditional notify: clear bit 2 and set bit 1 of
// +0x13c, then call the notify step (thiscall/0) when the argument's
// low byte is nonzero. Returns the new flag byte when no call is made,
// otherwise the notify step's answer (its low byte lands in AL).
export!(thiscall, rw_s13_00a7c900(this: *mut u8, arg: u32) -> u8 {
    unsafe {
        let flags = (*this.add(0x13C) & !0x04) | 0x02;
        *this.add(0x13C) = flags;
        if arg & 0xFF != 0 {
            let notify: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            return notify(this as u32) as u8;
        }
        flags
    }
});
