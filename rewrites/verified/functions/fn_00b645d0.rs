// original: 0x00b645d0 refresh_linked_records
// thiscall/0. Refreshes the records linked at +0x14 and +0x18 when each is
// present and carries the active tag (2) at its status byte. Returns nothing.
///
/// Proven scope: both owner links point to one fixed record each and both
/// status bytes equal 2, so refresh and activate each run on every trial.
/// Both callees are stubbed to return 0; null links and other statuses are
/// not covered.
export!(thiscall, rw_rs11f1(this: *mut u8) -> () {
    unsafe {
        let first = *(this.add(0x14) as *const u32);
        if first != 0 && *((first + 0x22a) as *const u8) == 2 {
            let refresh: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            refresh(this as u32, (this as u32).wrapping_add(0x14), 1, 1);
        }
        let second = *(this.add(0x18) as *const u32);
        if second != 0 && *((second + 0x22a) as *const u8) == 2 {
            let activate: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(2) as usize);
            activate(this as u32, 1);
        }
    }
});
