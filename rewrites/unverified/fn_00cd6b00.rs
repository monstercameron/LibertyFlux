// original: 0x00CD6B00 ped_task_chain_five (proposed)
/// Build five chained helper objects, then run a vtable check and a final
/// seven-argument call, returning its low byte.
///
/// Stdcall with four stack words; only the last two (`a2`, `a3`) survive, the
/// first two input slots are reused as scratch before anything reads them, so
/// the rewrite ignores them. Returns the low byte of the final call's answer,
/// or 0 when the chain's status word says to skip it.
///
/// The function owns a 16-byte scratch struct `S` plus two words below it.
/// Five times it calls the allocator (callee 0, cdecl, kind and code from a
/// fixed table, third word 0) and then the initialiser (callee 1, thiscall on
/// `S` with 0x10), storing the allocator's answer into the initialiser's.
/// The initialiser's scripted words seed `S`: word 0 is spare, word 1 packs
/// two 16-bit gates (low: run the main path; high: run the cleanup call),
/// word 3 is the subject object for the rest.
/// When the low gate is set, the two words below `S` are zeroed and the
/// subject's vtable slot `+0xa0` is called: a zero answer reads the argument
/// from the subject at `+0x100`, a nonzero one calls slot `+0xa0` again and
/// then slot `+0xe0` on the returned object, whose answer becomes the
/// argument. The final call (callee 3, cdecl, seven words) takes `a3`, the two
/// zeroed scratch addresses, `a2`, a constant 0 (the clobbered first input
/// slot, always zero at this point), the address of `S`, and that argument.
/// The three addresses are skipped in the call comparison with their contents
/// snapped instead. When the high gate word is nonzero the cleanup callee
/// (callee 4) runs on word 0 of `S`.
lf_checker_rt::export!(stdcall, rw_00CD6B00(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let _ = a0;
        let _ = a1;
        const CHAIN: [(u32, u32); 5] = [(3, 0x61), (3, 0x5c), (3, 0x5d), (3, 0x5e), (4, 0x63)];
        const INIT_ARG: u32 = 0x10;
        const VT_CHECK: u32 = 0xa0;
        const VT_RUN: u32 = 0xe0;
        const SUBJECT_ARG_OFF: u32 = 0x100;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall(slot: u32, obj: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(obj)
            }
        }

        // The two words below S come first so the layout matches the
        // original's: scratch addresses, then S itself.
        let mut frame = [0u32; 6];
        let base = frame.as_mut_ptr() as u32;
        let s = base + 8;
        for (kind, code) in CHAIN {
            let made: u32 = lf_checker_rt::callee_cdecl!(0, u32, kind, code, 0);
            let init: u32 = lf_checker_rt::callee_thiscall!(1, u32, s, INIT_ARG);
            (init as *mut u32).write_unaligned(made);
        }
        let gates = frame[3];
        if gates & 0xFFFF == 0 {
            if gates >> 16 != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, frame[2]);
            }
            return 0;
        }
        frame[0] = 0;
        frame[1] = 0;
        let subject = frame[5];
        let first = vcall(VT_CHECK, subject);
        let arg: u32;
        if first == 0 {
            arg = rd32(subject + SUBJECT_ARG_OFF);
        } else {
            let second = vcall(VT_CHECK, subject);
            arg = vcall(VT_RUN, second);
        }
        let ans: u32 =
            lf_checker_rt::callee_cdecl!(3, u32, a3, base, base + 4, a2, 0, s, arg);
        if gates >> 16 != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, frame[2]);
        }
        ans & 0xFF
    }
});
