// original: 0x00b44930 guard_inner_call_release
/// Run the inner object call under the shared guard.
///
/// Acquires the guard into a frame local, runs the inner call with the
/// entry object pointer and releases the guard, returning the inner result.
export!(thiscall, rw_b44930(obj: u32) -> u32 {
    unsafe {
        let mut guard: u32 = 0;
        let slot = (&mut guard as *mut u32) as u32;
        callee_thiscall!(2, u32, slot, relocated(0x16C6710));
        let r = callee_thiscall!(3, u32, obj);
        callee_thiscall!(4, u32, slot);
        r
    }
});
