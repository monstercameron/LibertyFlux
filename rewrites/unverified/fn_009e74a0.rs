// original: 0x009e74a0 ped_task_id_or_default
/// Task id for the key at `+0xB94`: the first lookup's word at
/// `+4`, falling back to the second lookup when the first misses, and
/// to `0x31` when the word is -1. (thiscall, two callee lookups.)
lf_checker_rt::export!(thiscall, rw_009e74a0(this_ptr: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0xB94;
        const ID_OFF: u32 = 4;
        const DEFAULT_ID: u32 = 0x31;
        let key = (this_ptr.wrapping_add(KEY_OFF) as *const u32).read_unaligned();
        let mut hit: u32 = lf_checker_rt::callee_cdecl!(1, u32, key);
        if hit == 0 {
            hit = lf_checker_rt::callee_cdecl!(2, u32, key);
        }
        let v = (hit.wrapping_add(ID_OFF) as *const u32).read_unaligned();
        if v == 0xFFFFFFFF { DEFAULT_ID } else { v }
    }
});
