// original: 0x00648ce0 rage::rmPtfxShaderVar_Float2::vf7
/// Store two float arguments into the value slots, copying the bits as-is.
export!(thiscall, rw_00648ce0(this: *mut u8, x: u32, y: u32) -> () {
    unsafe {
        *(this.add(0x20) as *mut u32) = x;
        *(this.add(0x24) as *mut u32) = y;
    }
});
