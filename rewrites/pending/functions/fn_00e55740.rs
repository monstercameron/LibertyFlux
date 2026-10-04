// original: 0x00E55740 find_key_record
/// Search the record source for a key string (original 0x00E55740).
///
/// Opens a lookup handle from two global descriptor words, then reads
/// 32-byte candidate records through it, comparing each against the caller's
/// key with a byte-string comparison. Returns the trailing check's answer
/// after releasing the handle, having found the key or exhausted the source.
export!(stdcall, rw_e55740(key: u32) -> u32 {
    const DESC0: u32 = 0x00F1_A2A0;
    const DESC1: u32 = 0x00F1_A2A8;
    const TAG_OPEN: u32 = 0x00F1_A2B0;
    const TAG_HANDLE: u32 = 0x00F1_A2C8;
    const REC_SIZE: u32 = 0x20;
    let mut desc = [0u32; 4];
    unsafe {
        desc[0] = global::<u32>(DESC0).read();
        desc[1] = global::<u32>(DESC0 + 4).read();
        desc[2] = global::<u32>(DESC1).read();
        desc[3] = global::<u32>(DESC1 + 4).read();
    }
    callee_cdecl!(1, u32, relocated(TAG_OPEN), 0);
    let handle = callee_cdecl!(2, u32, desc.as_ptr() as u32, relocated(TAG_HANDLE));
    if handle == 0 {
        return callee_cdecl!(5, u32,);
    }
    let mut buf = [0u8; 32];
    let mut more = callee_cdecl!(3, u32, handle, buf.as_mut_ptr() as u32, REC_SIZE);
    while more != 0 {
        if str_eq(buf.as_ptr(), key) {
            callee_cdecl!(4, u32, handle);
            return callee_cdecl!(5, u32,);
        }
        more = callee_cdecl!(3, u32, handle, buf.as_mut_ptr() as u32, REC_SIZE);
    }
    callee_cdecl!(4, u32, handle);
    callee_cdecl!(5, u32,)
});

/// Compare two NUL-terminated byte strings for equality (shared helper).
fn str_eq(a: *const u8, b: u32) -> bool {
    let mut i = 0usize;
    loop {
        let x = unsafe { a.add(i).read() };
        let y = unsafe { (b as *const u8).add(i).read() };
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        i = i.wrapping_add(1);
    }
}
