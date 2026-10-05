// original: 0x00a09fe0 cleanup_dispatch_typed (proposed)
/// Resolve a descriptor through its pool, then find its cleanup record.
///
/// Bits 6..9 of the dword at `item + 0x28` select the pool: 4 resolves via
/// the object pool and searches tag 4, 2 via pool A and tag 1, 3 via pool B
/// and tag 2. Any other value answers null. The find's answer is returned.
/// Thiscall with two stack words.
lf_checker_rt::export!(thiscall, rw_00a09fe0(this: u32, item: u32, extra: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x28;
        const KIND_SHIFT: u32 = 6;
        const KIND_MASK: u32 = 0x0f;
        const OBJ_POOL: u32 = 0x01632c60;
        const POOL_A: u32 = 0x012e22a4;
        const POOL_B: u32 = 0x018b6f1c;
        const RESOLVE: u32 = 0;
        const FIND: u32 = 1;
        let w = ((item + KIND_OFF) as *const u32).read_unaligned();
        let kind = (w >> KIND_SHIFT) & KIND_MASK;
        let (pool_va, tag) = match kind {
            4 => (OBJ_POOL, 4u32),
            2 => (POOL_A, 1u32),
            3 => (POOL_B, 2u32),
            _ => return 0,
        };
        let pool = (lf_checker_rt::global::<u32>(pool_va) as *const u32).read_unaligned();
        let target = lf_checker_rt::callee_thiscall!(RESOLVE, u32, pool, item);
        lf_checker_rt::callee_thiscall!(FIND, u32, this, tag, target, extra)
    }
});
