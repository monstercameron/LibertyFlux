// original: 0x008EE520 notify_64_slots (proposed)

/// Notify the observer of all 64 slots in order.
///
/// Calls the observer (callee 1, cdecl) 64 times with the slot index and
/// the shared argument word at `ARG_OFF` on `obj` (re-read every call).
/// Returns the 64th call's answer, which the original leaves in `eax`.
///
/// Original: 0x008EE520 (thiscall, no stack arguments; true size 40).
lf_checker_rt::export!(thiscall, rw_008EE520(obj: u32) -> u32 {
    unsafe {
        /// Offset of the shared argument word.
        const ARG_OFF: u32 = 0x1C08;
        /// Number of slots.
        const SLOTS: u32 = 64;
        /// Observer callee id.
        const OBS: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut i: u32 = 0;
        let mut ans: u32 = 0;
        while i < SLOTS {
            ans = lf_checker_rt::callee_cdecl!(
                OBS,
                u32,
                i,
                rd32(obj.wrapping_add(ARG_OFF))
            );
            i = i.wrapping_add(1);
        }
        ans
    }
});
