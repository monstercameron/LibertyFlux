// original: 0x006611e0 fn_006611e0
/// Detach the inner object, then re-register its two views when active.
///
/// When the incoming mode is 1, sets a flag bit deep inside the inner
/// object found through this object. Resolves the mode and tag through
/// the first helper with this object, clears the inner link, and when
/// the mode is 1 hands the saved inner object with two self-describing
/// frame records (a code mark, a self pointer, two zero words) to the
/// second helper twice. Returns the last helper's answer.
export!(thiscall, rw_006611e0(this_: *mut u8, mode: u32, tag: u32) -> u32 {
    unsafe {
        if mode == 1 {
            let inner = *(this_.add(0x60) as *const u32);
            *((inner + 0x32f8) as *mut u8) |= 0x20;
        }
        let saved = *(this_.add(0x60) as *const u32);
        let first_answer = callee_thiscall!(1, u32, this_ as u32, mode, tag);
        *(this_.add(0x60) as *mut u32) = 0;
        if mode != 1 {
            return first_answer;
        }
        let mut record_a = [relocated(0x00fe341c), 0, 0, 0];
        record_a[1] = record_a.as_ptr() as u32;
        callee_thiscall!(2, u32, saved, record_a.as_ptr() as u32);
        let mut record_b = [relocated(0x00fe2208), 0, 0, 0];
        record_b[1] = record_b.as_ptr() as u32;
        callee_thiscall!(3, u32, saved, record_b.as_ptr() as u32)
    }
});
