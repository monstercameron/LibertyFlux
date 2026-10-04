// original: 0x009dd300 CInstanceModelInfo::vf1
// fn_009dd300: CInstanceModelInfo::vf1 (thiscall/0).
//
// Runs the base-class step, then marks the object streamed-in: sets status
// bit 0x100 at +0x40 and resets the slot field at +0x60 to all-ones.
export!(thiscall, rw_009dd300(this: *mut u8) -> u32 {
    unsafe {
        let base: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let r = base(this as u32);
        *(this.add(0x40) as *mut u32) |= 0x100;
        *(this.add(0x60) as *mut u32) = 0xFFFFFFFF;
        r
    }
});
