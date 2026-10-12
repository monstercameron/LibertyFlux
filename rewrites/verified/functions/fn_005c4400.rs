// original: 0x005C4400 ped_task_record_type_status_predicate

/// Looks up the subobject referenced at offset 0x18 through a direct helper
/// call. It returns one when the helper's record has type 3 at offset 8 and a
/// nonzero status at offset 0x0c. The caller uses ECX for the object pointer;
/// the helper is cdecl with one pointer argument, and the result is AL.
lf_checker_rt::export!(thiscall, rw_005c4400(this: u32) -> u32 {
    unsafe {
        const RECORD_POINTER: u32 = 0x18;
        const RECORD_TYPE: u32 = 0x08;
        const RECORD_STATUS: u32 = 0x0c;
        const RECORD_TYPE_MATCH: u32 = 3;
        const LOOKUP_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let lookup_argument = read_word(this + RECORD_POINTER);
        let record: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, lookup_argument);
        u32::from(
            read_word(record + RECORD_TYPE) == RECORD_TYPE_MATCH
                && read_word(record + RECORD_STATUS) != 0,
        )
    }
});
