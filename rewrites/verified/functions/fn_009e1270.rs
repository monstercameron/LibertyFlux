// original: 0x009e1270 audio_list_unlink_and_dispatch
/// Unlink the node matching the key from the head-anchored list,
/// then tail-dispatch the removed node to the pool helper. Returns the helper
/// answer, or 0 when the key is absent. (thiscall/1, two tail jumps)
export!(thiscall, rw_009e1270(this: *mut u8, key: u32) -> u32 {
    unsafe {
        let head = *(this as *const u32);
        if head == 0 {
            return 0;
        }
        let mut prev = 0u32;
        let mut cur = head;
        loop {
            if *(cur as *const u32) == key {
                break;
            }
            prev = cur;
            cur = *((cur.wrapping_add(4)) as *const u32);
            if cur == 0 {
                return 0;
            }
        }
        let next = *((cur.wrapping_add(4)) as *const u32);
        if cur == head {
            *(this as *mut u32) = next;
        } else if prev != 0 {
            *((prev.wrapping_add(4)) as *mut u32) = next;
        }
        let pool = *global::<u32>(0x12B4164);
        callee_thiscall!(1, u32, pool, cur)
    }
});
