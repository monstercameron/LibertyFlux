// original: 0x00add1e0 ui_store_flag_and_notify
/// Store a flag byte into this object, then forward a value to a worker.
///
/// Writes the low byte of `flag` at offset 0xd51 and calls the worker with
/// `value`; returns the worker's answer.
export!(thiscall, rw_00add1e0(this: *mut u8, flag: u32, value: u32) -> u32 {
    unsafe {
        *(this.add(0xd51)) = (flag & 0xFF) as u8;
        callee_cdecl!(1, u32, value)
    }
});
