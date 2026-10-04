// original: 0x00e3e950 StatsRecord_InitBasic
// 0x00E3E950: initialise a basic stats record: ready flag with all
// counters and state words cleared. (thiscall/0)
export!(thiscall, rw_00e3e950(this: *mut u8) -> u32 {
    unsafe {
        *this = 1;
        *(this.add(4) as *mut u32) = 0;
        *(this.add(8) as *mut u32) = 0;
        *(this.add(0x0c) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x14) as *mut u16) = 0;
        *(this.add(0x1c) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = 0;
        0
    }
});
