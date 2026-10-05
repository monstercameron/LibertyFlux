// original: 0x00c24df0 cam_vec_blend_call (proposed)
/// Resolve the two blend sources into frame slots, then call the vector
/// blend worker with the +0x30 fields of both (`(first+0x30, second+0x30,
/// out, t)`). Returns the worker's answer with its low byte forced to 1.
///
/// Original: 0x00c24df0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c24df0(obj: u32, tv: u32, out: u32) -> u32 {
    unsafe {
        const VEC_OFF: u32 = 0x30;
        let mut first: u32 = 0;
        let mut second: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            1, u32, obj,
            &mut first as *mut u32 as u32, &mut second as *mut u32 as u32
        );
        let r: u32 = lf_checker_rt::callee_cdecl!(
            2, u32,
            first.wrapping_add(VEC_OFF), second.wrapping_add(VEC_OFF), tv, out
        );
        (r & 0xffff_ff00) | 1
    }
});
