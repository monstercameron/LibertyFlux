// original: 0x00cb6cc0 guarded_route_query
/// Forwards to the shared route query when all guards pass; see above.
export!(thiscall, rw_cb6cc0(this_ptr: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let probe = ((this_ptr.wrapping_add(0x64)) as *const u32).read();
        if probe == 0 {
            return 0;
        }
        let armed = (probe as *const u32).read();
        if armed == 0 {
            return probe & 0xFFFF_FF00;
        }
        let sub = ((this_ptr.wrapping_add(8)) as *const u32).read();
        if sub == 0 {
            // NOTE: eax still holds the probe address here (the armed flag
            // was only compared, never loaded), so this exit clears the
            // probe's low byte, not the flag's.
            return probe & 0xFFFF_FF00;
        }
        let vptr = (sub as *const u32).read();
        let slot = ((vptr.wrapping_add(0xC)) as *const u32).read();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let kind = kind_of(sub);
        if kind != 0x389 {
            return kind & 0xFFFF_FF00;
        }
        lf_checker_rt::callee_thiscall!(2, u32, sub, a0, a1)
    }
});
