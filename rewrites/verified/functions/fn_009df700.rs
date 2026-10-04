// original: 0x009df700 tracker_init
// fn_009df700: tracker init (thiscall/0).
//
// Runs the attach step above (stubbed by the checker, so its stores happen
// on neither side here), then installs this class's vtable and clears its
// own fields. Returns `this`.
export!(thiscall, rw_009df700(this: *mut u8) -> u32 {
    unsafe {
        let attach: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        attach(this as u32);
        *(this as *mut u32) = relocated(0xE9819C);
        *(this.add(0x74) as *mut u16) = 0x100;
        *(this.add(0x34) as *mut u32) = 0;
        *(this.add(0x78) as *mut u32) = 0;
        *(this.add(0x7C) as *mut u32) = 0;
        *(this.add(0xC0) as *mut u32) = 0;
        this as u32
    }
});
