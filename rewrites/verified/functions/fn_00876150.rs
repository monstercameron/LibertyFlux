// original: 0x00876150 crmt_addsubtract_request_init
/// Build a combiner request: init the embedded source, then set the fields.
///
/// thiscall/2. Installs the base vtable, initializes the embedded source
/// object through the shared data-table slot, then installs the combiner
/// vtable and stores the mode and rate fields. Returns `this`.
export!(thiscall, rw_00876150(this: *mut u8, mode: u32, rate_bits: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0x00fe7fc8);
        let sub = this.add(4);
        *(sub.add(4) as *mut u32) = 0;
        *(sub as *mut u32) = relocated(0x00fe7fb4);
        *(sub.add(8) as *mut u32) = 0;
        *(sub.add(0x0c) as *mut u32) = 0;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*global::<u32>(0x00fe7fb8) as usize);
        init(sub as u32);
        *(this.add(0x14) as *mut u32) = 0;
        *(this as *mut u32) = relocated(0x00fe81e4);
        *(this.add(0x20) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = mode;
        *(this.add(0x1c) as *mut u32) = rate_bits;
        this as u32
    }
});
