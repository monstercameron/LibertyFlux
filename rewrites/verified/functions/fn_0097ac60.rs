// original: 0x0097ac60 audio_listeners_create_all
/// Allocate and initialise all three global audio listener records.
///
/// Allocates each 0x2c-byte record, initialises it from its 8-byte
/// descriptor and stores it in the global table; a failed allocation
/// leaves a null slot.
export!(cdecl, rw_0097ac60() -> u32 {
    unsafe {
        let table = global::<u32>(0x12312D4);
        let mut src = relocated(0x103891C);
        for i in 0..3usize {
            let obj = callee_cdecl!(1, u32, 0x2c);
            let val = if obj != 0 {
                callee_thiscall!(2, u32, obj, src)
            } else {
                0
            };
            *table.add(i) = val;
            src = src.wrapping_add(8);
        }
        0
    }
});
