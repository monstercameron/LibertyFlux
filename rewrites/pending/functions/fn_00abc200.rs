// original: 0x00abc200 sentinel_store_or_shift

/// Store the empty marker, or shift the prefix down forever when given more.
///
/// When the value equals the heap's empty-slot marker it is stored at the
/// pointer and the pointer minus four is returned. Any other value sends the
/// original into a prefix-shifting loop whose exit condition never becomes
/// true, marching down memory until it faults; the rewrite keeps that exact
/// loop. Only the marker path is exercised by the checker, since the other
/// path provokes wild writes.
export!(cdecl, rs64_abc200(ptr: u32, val: u32, _x: u32) -> u32 {
    unsafe {
        const EMPTY_FILEVA: u32 = 0x0151_0A90;
        let empty = relocated(EMPTY_FILEVA);
        if val == empty {
            *(ptr as *mut u32) = val;
            ptr.wrapping_sub(4)
        } else {
            let mut d = ptr;
            let mut s = ptr.wrapping_sub(4);
            loop {
                let v = *(s as *const u32);
                *(d as *mut u32) = v;
                d = s;
                s = s.wrapping_sub(4);
            }
        }
    }
});
