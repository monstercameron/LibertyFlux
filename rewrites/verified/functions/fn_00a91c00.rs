// original: 0x00a91c00 stream_alloc_slot_for_key (proposed)

/// Allocate a slot for a key byte, faulting when the key is marked.
///
/// The set object comes from its global. When the key byte (`arg + off`,
/// `off` at `set+0x04`) has bit 7 set, an id is still allocated (callee 1,
/// cdecl/1 with 4) and then stored through null, faulting exactly like the
/// original. Otherwise the slot `set+0x00 + stride*arg` (stride at
/// `set+0x0c`) receives the allocated id.
///
/// Returns the allocated id on the mapped path. Cdecl, one argument (a
/// pointer into the key bytes).
lf_checker_rt::export!(cdecl, rw_00a91c00(key: u32) -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const BASE_OFF: u32 = 0x00;
        const KEY_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const MARK_BIT: u8 = 0x80;
        const ALLOC_ARG: u32 = 4;
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        let off = ((set + KEY_OFF) as *const u32).read_unaligned();
        if (((key + off) as *const u8).read() & MARK_BIT) != 0 {
            let id = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_ARG);
            core::ptr::write_volatile(0 as *mut u32, id);
            return id;
        }
        let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
        let base = ((set + BASE_OFF) as *const u32).read_unaligned();
        let slot = base.wrapping_add(stride.wrapping_mul(key));
        let id = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_ARG);
        (slot as *mut u32).write_unaligned(id);
        id
    }
});
