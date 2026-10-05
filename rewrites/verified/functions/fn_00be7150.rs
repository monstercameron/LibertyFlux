// original: 0x00be7150 CTaskSimpleLeaveGroup::vf17

/// Make the ped leave its group unless it has none or is staying.
///
/// `this` is ignored; `ped` is the ped. Resolves the group through callee 1
/// (thiscall on the ped, no stack words) and returns 1 when there is none.
/// Otherwise asks the group object at `group+8` whether the ped stays
/// (callee 2, thiscall, one word: the ped) and returns 1 when it answers
/// non-zero; if the ped is leaving, removes it (callee 3, thiscall on
/// `group+8`, two words: ped, 1) and releases the group (callee 4, thiscall
/// on the group, no stack words). Returns 1 on every path.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7150(this: u32, ped: u32) -> u32 {
    unsafe {
        const GROUP_INNER: u32 = 8;
        const MEMBER_OF: u32 = 1;
        const STAYS: u32 = 2;
        const REMOVE: u32 = 3;
        const RELEASE: u32 = 4;

        let _ = this;
        let group: u32 = lf_checker_rt::callee_thiscall!(MEMBER_OF, u32, ped);
        if group == 0 {
            return 1;
        }
        let stays: u32 =
            lf_checker_rt::callee_thiscall!(STAYS, u32, group.wrapping_add(GROUP_INNER), ped);
        if stays as u8 != 0 {
            return 1;
        }
        lf_checker_rt::callee_thiscall!(REMOVE, u32, group.wrapping_add(GROUP_INNER), ped, 1);
        lf_checker_rt::callee_thiscall!(RELEASE, u32, group);
        1
    }
});
