// original: 0x00648e80 rage::rmPtfxShaderVar_Float4::vf9
/// Store four float arguments into the value slots, copying the bits as-is.
export!(thiscall, rw_00648e80(this: *mut u8, x: u32, y: u32, z: u32, w: u32) -> () {
    unsafe {
        *(this.add(0x20) as *mut u32) = x;
        *(this.add(0x24) as *mut u32) = y;
        *(this.add(0x28) as *mut u32) = z;
        *(this.add(0x2c) as *mut u32) = w;
    }
});
