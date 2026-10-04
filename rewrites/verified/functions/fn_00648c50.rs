// original: 0x00648c50 rage::crmtNodePair::vf14
/// Store one float argument into the value slot, copying the bits as-is.
export!(thiscall, rw_00648c50(this: *mut u8, value: u32) -> () {
    unsafe {
        *(this.add(0x20) as *mut u32) = value;
    }
});
