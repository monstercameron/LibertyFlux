// original: 0x00648da0 rage::rmPtfxShaderVar_Float3::vf8
/// Store three float arguments into the value slots, copying the bits as-is.
export!(thiscall, rw_00648da0(this: *mut u8, x: u32, y: u32, z: u32) -> () {
    unsafe {
        *(this.add(0x20) as *mut u32) = x;
        *(this.add(0x24) as *mut u32) = y;
        *(this.add(0x28) as *mut u32) = z;
    }
});
