// original: 0x005e5ad0 lowered_string_store_base
/// Lowercase a string through the allocator, store it, release it.
///
/// Measures the incoming string, asks the allocator helper to duplicate it
/// into a two-word frame record (bytes pointer, size), lowercases the copy
/// in place when the size's low word is non-zero, then stores the record
/// with the incoming slot word through the member helper and releases the
/// copy through the allocator's free entry found via thread storage.
/// Returns the last helper's answer.
/// Release a duplicated string through the allocator free entry found via
/// thread storage slot 0, following the same chain the original walks.
unsafe fn release_005e5ad0(bytes: u32) -> u32 {
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
export!(thiscall, rw_005e5ad0(this_: u32, slot: u32, text: u32) -> u32 {
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
        let stored = callee_thiscall!(2, u32, this_, frame.as_mut_ptr() as u32, &slot as *const u32 as u32);
        if frame[0] != 0 {
            release_005e5ad0(frame[0])
        } else {
            stored
        }
    }
});
