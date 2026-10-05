// original: 0x00a91d60 stream_find_slot_by_hash (proposed)

/// Find the first live slot whose key word equals the argument's hash.
///
/// The argument is hashed first (callee 1, cdecl/1); the hash is kept in
/// the argument's own stack slot. Each slot index below the set count at
/// `set+0x08` (set from its global) whose select byte (`base[i]`, base at
/// `set+0x04`) has bit 7 clear and whose address (`set+0x00 + stride*i`,
/// stride at `set+0x0c`) is nonzero and holds the hash in its key word at
/// `+0x08` is the match.
///
/// Returns the matching index, or -1. Cdecl, one argument. (The inventory
/// size, 86, is short: the body runs past it to its return; the proof runs
/// the whole body.)
lf_checker_rt::export!(cdecl, rw_00a91d60(arg: u32) -> u32 {
    unsafe {
        const SET_GLOBAL: u32 = 0x012fb258;
        const BASE_OFF: u32 = 0x00;
        const SELECT_OFF: u32 = 0x04;
        const COUNT_OFF: u32 = 0x08;
        const STRIDE_OFF: u32 = 0x0c;
        const KEY_OFF: u32 = 0x08;
        const SKIP_BIT: u8 = 0x80;
        const MISS: u32 = 0xffffffff;
        let hash = lf_checker_rt::callee_cdecl!(1, u32, arg);
        let set = lf_checker_rt::global::<u32>(SET_GLOBAL).read_unaligned();
        let count = ((set + COUNT_OFF) as *const i32).read_unaligned();
        if count <= 0 {
            return MISS;
        }
        let mut i = 0i32;
        while i < ((set + COUNT_OFF) as *const i32).read_unaligned() {
            let selbase = ((set + SELECT_OFF) as *const u32).read_unaligned();
            if (((selbase + (i as u32)) as *const u8).read() & SKIP_BIT) == 0 {
                let stride = ((set + STRIDE_OFF) as *const u32).read_unaligned();
                let base = ((set + BASE_OFF) as *const u32).read_unaligned();
                let slot = base.wrapping_add(stride.wrapping_mul(i as u32));
                if slot != 0
                    && ((slot + KEY_OFF) as *const u32).read_unaligned() == hash
                {
                    return i as u32;
                }
            }
            i += 1;
        }
        MISS
    }
});
