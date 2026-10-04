// original: 0x009e9860 ped_pool_rank_admits
/// True when the pool rank of this object (shared pool lookup,
/// shifted down 8) is non-negative and below the pool bound, and the
/// object is either already accepted (`+0x70` set) or is the pool's
/// current pick. (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e9860(this_ptr: u32) -> u32 {
    unsafe {
        const POOL: u32 = 0x18B6F1C;
        const BOUND_OFF: u32 = 8;
        const ACCEPTED_OFF: u32 = 0x70;
        const RANK_SHIFT: u32 = 8;
        let pool = lf_checker_rt::global::<u32>(POOL).read_unaligned();
        let raw: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool, this_ptr);
        let rank = (raw as i32).wrapping_shr(RANK_SHIFT);
        if rank < 0 {
            return 0;
        }
        let bound = (pool.wrapping_add(BOUND_OFF) as *const u32).read_unaligned() as i32;
        if rank >= bound {
            return 0;
        }
        if (this_ptr.wrapping_add(ACCEPTED_OFF) as *const u32).read_unaligned() != 0 {
            return 1;
        }
        let pick: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0);
        if pick == this_ptr { 1 } else { 0 }
    }
});
