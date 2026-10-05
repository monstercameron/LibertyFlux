// original: 0x009a8e50 flag_byte_store
/// Store a flag byte into the object when the pointer is non-null.
///
/// `obj` points at the object (or is null) and `value` is the byte to
/// store at `obj+0x778`. A null pointer stores nothing. Stdcall, two
/// stack words, no result.
export!(stdcall, rw_009A8E50(obj: u32, value: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x778;
        if obj != 0 {
            ((obj + FLAG_OFF) as *mut u8).write(value as u8);
        }
        0
    }
});
