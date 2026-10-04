// original: 0x009FD120 frag_inst_gta_delete (proposed)

/// Destroy a frag instance, scalar or array, and optionally free it.
///
/// Merged symbol: `fragInstGta::vf0`. When bit 1 of `flags` is set the
/// object is an array: the element count is read from `obj - 0x10` and the
/// element destructor callee runs on each 0xB0-byte element from last to
/// first. Otherwise the scalar destructor callee runs on `obj` itself. When
/// bit 0 of `flags` is set the storage is released: the array base through
/// the free callee, the scalar object through the heap-release callee on a
/// global heap. Returns the array base, or `obj` on the scalar path.
///
/// Original: 0x009FD120 (thiscall, `obj` in `ecx`, `flags` one stack word).
lf_checker_rt::export!(thiscall, rw_009FD120(obj: u32, flags: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x10;
        const ELEM_SIZE: u32 = 0xB0;
        const HEAP: u32 = 0x0171C11C;
        if flags & 2 != 0 {
            let base = obj.wrapping_sub(COUNT_OFF);
            let count = (base as *const u32).read_unaligned();
            let mut j = count;
            while j > 0 {
                j -= 1;
                lf_checker_rt::callee_thiscall!(1, u32, obj + j * ELEM_SIZE);
            }
            if flags & 1 != 0 {
                lf_checker_rt::callee_cdecl!(2, u32, base);
            }
            base
        } else {
            lf_checker_rt::callee_thiscall!(1, u32, obj);
            if flags & 1 != 0 {
                let heap = (lf_checker_rt::relocated(HEAP) as *const u32).read_unaligned();
                lf_checker_rt::callee_thiscall!(3, u32, heap, obj);
            }
            obj
        }
    }
});
