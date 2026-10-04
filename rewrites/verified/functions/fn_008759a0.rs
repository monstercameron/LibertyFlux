// original: 0x008759a0 crmt_filter_request_init
/// Initialize a filter-request object: tag word plus cleared fields.
///
/// thiscall/0. Writes a constant tag, zeroes the parameter block, installs
/// the class vtable and clears the tail field. Returns `this`.
export!(thiscall, rw_008759a0(this: *mut u8) -> u32 {
    unsafe {
        const TAG: u32 = 0x00130000;
        *(this.add(0x04) as *mut u32) = TAG;
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x0c) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x14) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x1c) as *mut u32) = 0;
        *(this as *mut u32) = relocated(0x00fe816c);
        *(this.add(0x20) as *mut u32) = 0;
        this as u32
    }
});
