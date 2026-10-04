// original: 0x00b251e0 read_velocity
// s08_b251e0: read velocity into an output vector. thiscall/1 (out): asks
// the velocity helper (thiscall/0); when it yields a record, copies the
// three floats at record+0x120/+0x124/+0x128 to `out`, otherwise zeroes
// `out`. Returns `out`.
export!(thiscall, rw_b251e0(this: *mut u8, out: *mut u32) -> u32 {
    unsafe {
        let helper: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let rec = helper(this as u32);
        if rec == 0 {
            *out = 0;
            *out.add(1) = 0;
            *out.add(2) = 0;
        } else {
            *out = *((rec + 0x120) as *const u32);
            *out.add(1) = *((rec + 0x124) as *const u32);
            *out.add(2) = *((rec + 0x128) as *const u32);
        }
        out as u32
    }
});
