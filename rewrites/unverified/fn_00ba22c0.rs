// original: 0x00ba22c0 SET_NM_MESSAGE_VEC3
/// Send a vector3 natural-motion message.
///
/// Builds a vector from script arguments 1-3 and forwards it with the message target (argument 0) to the engine implementation.
///
/// The engine call takes a pointer to a caller-frame vector in ECX; the
/// contract skips the address and snapshots the three pointed-to words.
export!(cdecl, rw_00ba22c0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let vec = [*args.add(1), *args.add(2), *args.add(3)];
        callee_thiscall!(1, u32, vec.as_ptr() as u32, *args)
    }
});
