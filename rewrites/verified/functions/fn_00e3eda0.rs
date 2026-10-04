// original: 0x00e3eda0 HandleRec_Init
// 0x00E3EDA0: initialise a handle record: empty tag, cleared flags.
// Returns the record pointer. (thiscall/0)
export!(thiscall, rw_00e3eda0(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut i32) = -1;
        *(this.add(4)) = 0u8;
        *(this.add(0x40)) = 0u8;
        this as u32
    }
});
