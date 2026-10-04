// original: 0x00934550 refresh_if_dirty
/// 0x00934550: refresh helper. Clears the dirty flag at +0x164 (one byte
/// when the state flag at +0x169 is set, a word otherwise), refreshes the
/// set through the shared worker, and reports success. The stack argument
/// is ignored; only AL is defined on return.
export!(thiscall, rw_00934550(this: *mut u8, _arg: u32) -> u8 {
    unsafe {
        if core::ptr::read(this.add(0x169)) != 0 {
            core::ptr::write(this.add(0x164), 0u8);
            callee_thiscall!(1, u32, this as u32);
        } else {
            core::ptr::write_unaligned(this.add(0x164) as *mut u16, 0);
            callee_thiscall!(2, u32, this as u32);
        }
    }
    1
});
