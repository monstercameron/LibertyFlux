// original: 0x00875bc0 crmt_extrapolate_node_init
/// Initialize an extrapolate-node object: tag, cleared fields, default rate.
///
/// thiscall/0. Writes a constant tag, zeroes the parameter block, installs
/// the class vtable, sets the default rate to 1.0, clears the child slots,
/// sets the two flag bytes and clears the accumulator. Returns `this`.
export!(thiscall, rw_00875bc0(this: *mut u8) -> u32 {
    unsafe {
        const TAG: u32 = 0x00120000;
        const ONE: u32 = 0x3f800000;
        *(this.add(0x04) as *mut u32) = TAG;
        *(this.add(0x08) as *mut u32) = 0;
        *(this.add(0x0c) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x14) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x1c) as *mut u32) = 0;
        *(this as *mut u32) = relocated(0x00fe81a8);
        *(this.add(0x20) as *mut u32) = ONE;
        *(this.add(0x24) as *mut u32) = 0;
        *(this.add(0x28) as *mut u32) = 0;
        *(this.add(0x2c) as *mut u16) = 0x0101;
        *(this.add(0x30) as *mut u32) = 0;
        this as u32
    }
});
