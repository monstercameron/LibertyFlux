// original: 0x00e3d8f0 StatsRecord_Init
// 0x00E3D8F0: initialise a stats record: ready flag, zeroed counters,
// unit scale, cleared state words, invalid tag. (thiscall/0)
export!(thiscall, rw_00e3d8f0(this: *mut u8) -> u32 {
    unsafe {
        *this = 1;
        *(this.add(4) as *mut u32) = 0;
        *(this.add(8) as *mut u32) = 0;
        *(this.add(0x20) as *mut f32) = 1.0;
        *(this.add(0x0c) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x14) as *mut u16) = 0;
        *(this.add(0x1c) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0x34) as *mut i32) = -1;
        0
    }
});
