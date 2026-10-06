// original: 0x005e5dc0 lowered_string_lookup_18
/// Lowercase a string through the allocator, look it up, release it.
///
/// Measures the incoming string, asks the allocator helper to duplicate it
/// into a two-word frame record (bytes pointer, size), lowercases the copy
/// in place when the size's low word is non-zero, then looks the record up
/// through the member helper: a hit returns the word the answer points at,
/// a miss returns {miss:#x}. Releases the copy through the allocator's free
/// entry found via thread storage.
/// Release a duplicated string through the allocator free entry found via
/// thread storage slot 0, following the same chain the original walks.
unsafe fn release_005e5dc0(bytes: u32) -> u32 {
    unsafe {
        let chain = tls_slot(0);
        let holder = *((chain + 8) as *const u32);
        let table = *(holder as *const u32);
        let entry = *((table + 0x0c) as *const u32);
        let free_it: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(entry as usize);
        free_it(holder, bytes)
    }
}
export!(thiscall, rw_005e5dc0(this_: u32, text: u32) -> u32 {
    unsafe {
        let mut frame = [0u32; 2];
        if text != 0 {
            let mut len: u32 = 0;
            let mut p = text as *const u8;
            while *p != 0 {
                len = len.wrapping_add(1);
                p = p.add(1);
            }
            callee_thiscall!(1, u32, frame.as_mut_ptr() as u32, text, len);
            if (frame[1] as u16) != 0 && frame[0] != 0 && *(frame[0] as *const u8) != 0 {
                let mut q = frame[0] as *mut u8;
                loop {
                    let b = *q;
                    if b >= 0x41 && b <= 0x5a {
                        *q = b.wrapping_add(0x20);
                    }
                    q = q.add(1);
                    if *q == 0 {
                        break;
                    }
                }
            }
        }
        let found = callee_thiscall!(2, u32, this_.wrapping_add(0x18), frame.as_mut_ptr() as u32);
        let mut out: u32 = 0x1f;
        if found != 0 {
            out = *(found as *const u32);
        }
        if frame[0] != 0 {
            release_005e5dc0(frame[0]);
        }
        out
    }
});
