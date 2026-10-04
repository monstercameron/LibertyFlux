// original: 0x00945fe0 guarded_slot_tail_flush
/// Flush the current slot when every gate agrees, tail-calling the flusher.
///
/// Five gates in order: an enable byte set, a busy byte clear, a global
/// readiness check passing, the slot live flag set, and the slot fill level
/// above threshold (signed). When all pass, the slot is handed to the flusher
/// via a tail call; otherwise returns without effect.
lf_checker_rt::export!(thiscall, rw_00945fe0(this_ptr: u32) -> () {
    unsafe {
        if *(this_ptr.wrapping_add(0x1930) as *const u8) == 0 {
            return;
        }
        if *(this_ptr.wrapping_add(0x1929) as *const u8) != 0 {
            return;
        }
        let ok = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(0x11d7678)
        );
        if ok == 0 {
            return;
        }
        let idx = *(this_ptr.wrapping_add(0x1917) as *const u8) as u32;
        let slot = this_ptr.wrapping_add(idx.wrapping_mul(0xbd0));
        if *(slot.wrapping_add(0xbca) as *const u8) == 0 {
            return;
        }
        if (*(slot.wrapping_add(0x990) as *const u32) as i32) <= 0x7d0 {
            return;
        }
        lf_checker_rt::callee_thiscall!(2, u32, slot);
    }
});
