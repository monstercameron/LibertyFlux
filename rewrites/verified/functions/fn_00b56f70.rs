// original: 0x00b56f70 route_record_attach
// rs05f3: route a record through convert-and-attach by its flag bit.
//
// Reads the key at `rec+8`. When bit 2 of the record's first byte is set,
// converts the key (stdcall/1, ignores its ECX) and attaches the result to
// the table at `this+0x11DC` (thiscall/1). Otherwise attaches the converted
// key to both the slot at `this+4` and the table at `this+0x8B8`.
export!(thiscall, rw_b56f70(this: *const u8, rec: *const u8) -> () {
    unsafe {
        let key = *(rec.add(8) as *const u32);
        let attach: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        if *rec & 4 != 0 {
            let handle = callee_stdcall!(1, u32, key);
            attach(this.add(0x11DC) as u32, handle);
        } else {
            let handle = callee_stdcall!(1, u32, key);
            attach(this.add(4) as u32, handle);
            attach(this.add(0x08B8) as u32, handle);
        }
    }
});
