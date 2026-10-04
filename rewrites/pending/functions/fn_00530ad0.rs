// original: 0x00530AD0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race36Standard, player_schema::LeaderboardInfo, 10>::vf12

/// Find the position of one table entry inside the row-id array.
///
/// `index` selects a table entry; the result is that entry's position
/// in the row-id array, or -1. The helper (callee 1) is asked for the
/// tables of kind 0x88: it fills a scratch struct whose word at
/// `+0x14` (INFO_TABLE) points at the indexed table, whose word at
/// `+0x04` (INFO_COUNT) is the row-id count and whose word at `+0x08`
/// (INFO_IDS) points at the row-id array. A zero answer byte, a table
/// entry of -1, or a zero count yields -1. Otherwise the row ids are
/// scanned from 0 while unsigned-below the count (the original
/// compares with `jb`, so a negative count scans on); the first
/// match wins.
///
/// Original: 0x00530AD0 (stdcall, one stack word; the incoming ECX, the
/// virtual-method receiver, is overwritten before any read and ignored).
lf_checker_rt::export!(stdcall, rw_00530AD0(index: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x88;
        const HELPER: u32 = 1;
        const INFO_COUNT: usize = 1;
        const INFO_IDS: usize = 2;
        const INFO_TABLE: usize = 5;
        const NOT_FOUND: u32 = 0xffff_ffff;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut info = [0u32; 8];
        let ok = lf_checker_rt::callee_fastcall!(HELPER, u32, KIND, info.as_mut_ptr() as u32) as u8;
        if ok == 0 {
            return NOT_FOUND;
        }
        let wanted = rd32(info[INFO_TABLE].wrapping_add(index.wrapping_mul(4)));
        if wanted == NOT_FOUND {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT];
        let ids = info[INFO_IDS];
        if count == 0 {
            return NOT_FOUND;
        }
        let mut i = 0u32;
        while i < count {
            if rd32(ids.wrapping_add(i.wrapping_mul(4))) == wanted {
                return i;
            }
            i = i.wrapping_add(1);
        }
        NOT_FOUND
    }
});
