// original: 0x00b2c9d0 garage_kind5_notify
// 0xB2C9D0 garage_kind5_notify (cdecl/1).
//
// Sweeps all forty records: a record of kind 5 with a nonzero pending byte
// is reported to the shared notifier (thiscall/2: record, table entry for
// the record's index byte, our argument), then marked done (flag bit 0x40
// set, pending cleared). A record of kind 5 with a zero pending byte gets
// the flag bit cleared; any other kind is left untouched.
export!(cdecl, rw_00b2c9d0(arg: u32) -> () {
    unsafe {
        let notify: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut slot = relocated(0x1660353);
        let end = relocated(0x1661433);
        while slot < end {
            if *((slot - 3) as *const u8) == 5 {
                let pending = (slot - 2) as *mut u8;
                if *pending != 0 {
                    let entry = relocated(0x165FD60 + *(slot as *const u8) as u32 * 144);
                    notify(slot - 0x4B, entry, arg);
                    *((slot + 1) as *mut u8) |= 0x40;
                    *pending = 0;
                } else {
                    *((slot + 1) as *mut u8) &= !0x40;
                }
            }
            slot += 0x6C;
        }
    }
});
