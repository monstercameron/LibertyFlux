// original: 0x005C42E0 ped_factory_try_add_ped

/// Checks a candidate entity through its virtual slot at +0xc0, then tests
/// that result and this object's unsigned slot count. On the accepted path it
/// obtains a 16-bit virtual result, forwards the candidate and index to a
/// helper, and reports true. Null candidates, a zero test result, or a zero
/// slot count use the fallback helper and report false. The method uses ECX
/// plus four callee-cleaned stack words; result comparison is through AL.
lf_checker_rt::export!(thiscall, rw_005c42e0(this: u32, candidate: u32, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const CANDIDATE_VTABLE: u32 = 0x00;
        const CANDIDATE_CHECK_SLOT: u32 = 0xc0;
        const CANDIDATE_INDEX: u32 = 0x6c;
        const SLOT_COUNT: u32 = 0x303e0;
        const MINIMUM_SLOT_COUNT: u32 = 1;
        const CHECK_CALLEE: u32 = 1;
        const FILTER_CALLEE: u32 = 2;
        const GET_INDEX_CALLEE: u32 = 3;
        const ATTACH_CALLEE: u32 = 4;
        const NOTIFY_CALLEE: u32 = 5;
        const FALLBACK_CALLEE: u32 = 6;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn call_thiscall0(target: u32, object: u32) -> u32 {
            let method: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(object)
        }
        #[inline(always)]
        unsafe fn call_thiscall4(target: u32, object: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
            let method: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(object, a0, a1, a2, a3)
        }

        if candidate == 0 {
            return 0;
        }

        if candidate != 0 {
            let candidate_vtable = read_word(candidate + CANDIDATE_VTABLE);
            let check_target = read_word(candidate_vtable + CANDIDATE_CHECK_SLOT);
            let checked_value = call_thiscall0(check_target, candidate);
            let accepted: u32 = lf_checker_rt::callee_cdecl!(FILTER_CALLEE, u32, checked_value, 1);
            if accepted as u8 != 0 && read_word(this + SLOT_COUNT) >= MINIMUM_SLOT_COUNT {
                let raw_index: u32 = lf_checker_rt::callee_thiscall!(GET_INDEX_CALLEE, u32, this);
                let index = (raw_index as u16) as u32;
                let _ = lf_checker_rt::callee_thiscall!(
                    ATTACH_CALLEE, u32, this, candidate, index, 0, 0,
                );
                let _ = lf_checker_rt::callee_cdecl!(
                    NOTIFY_CALLEE, u32, read_word(candidate + CANDIDATE_INDEX),
                );
                return 1;
            }
        }

        let _ = lf_checker_rt::callee_cdecl!(FALLBACK_CALLEE, u32, candidate, 0, 0x0a);
        0
    }
});
