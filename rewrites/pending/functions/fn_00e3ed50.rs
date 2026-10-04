// original: 0x00e3ed50 StatsPrinter_Init
// 0x00E3ED50: initialise a stats-printer pair: header plus two labelled
// output slots. Returns the record pointer. (thiscall/0)
export!(thiscall, rw_00e3ed50(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = 0;
        *(this.add(4) as *mut u32) = 1;
        *(this.add(8)) = 0u8;
        callee_cdecl!(1, u32, this.add(9) as u32, relocated(0xF15828), relocated(0xF15573));
        callee_cdecl!(1, u32, this.add(0x1d) as u32, relocated(0xF1582C), relocated(0xF15581));
        this as u32
    }
});
