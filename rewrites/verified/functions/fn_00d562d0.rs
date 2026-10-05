// original: 0x00d562d0 CCamIdle::vf4

/// Refresh the idle camera's output quad from the blend routine, then adopt
/// the shared slot pointer.
///
/// `this` points to the object. The blend routine runs with three scratch
/// blocks (its second block's four words are copied into `OUT`); the merge
/// routine runs with two scratch blocks against `this + SUB`; then the slot
/// fetch runs with (1, 0, 0) and, when its answer is non-null, the word at
/// `SLOT_LINK` of that answer is adopted into `SLOT`. Returns 1 in AL (upper
/// EAX passes through, so only AL is compared). The scratch addresses differ
/// between the sides by construction, so the frame-pointer arguments are
/// skipped and only the filled words are compared.
///
/// Original: 0x00d562d0 (thiscall, no stack arguments, three calls).
lf_checker_rt::export!(thiscall, rw_00d562d0(this: u32) -> u32 {
    unsafe {
        /// Output quad receiving the blend block.
        const OUT: u32 = 0x40;
        /// Sub-object offset passed to the merge routine.
        const SUB: u32 = 0x10;
        /// Slot receiving the adopted pointer.
        const SLOT: u32 = 0x64;
        /// Link read from the fetch answer.
        const SLOT_LINK: u32 = 0x64;
        /// Blend routine (intercepted; thiscall, three scratch blocks).
        const BLEND: u32 = 1;
        /// Merge routine (intercepted; thiscall, two scratch blocks).
        const MERGE: u32 = 2;
        /// Slot fetch (intercepted; thiscall, three stack words).
        const FETCH: u32 = 3;
        let mut block = [0u32; 4];
        let mut aux_a = 0u32;
        let mut aux_b = 0u32;
        let mut aux_c = 0u32;
        let mut aux_d = 0u32;
        lf_checker_rt::callee_thiscall!(BLEND, u32, this,
            &mut aux_b as *mut u32 as u32,
            &mut aux_a as *mut u32 as u32,
            block.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(MERGE, u32, this.wrapping_add(SUB),
            &mut aux_d as *mut u32 as u32,
            &mut aux_c as *mut u32 as u32);
        ((this + OUT) as *mut u32).write_unaligned(block[0]);
        ((this + OUT + 4) as *mut u32).write_unaligned(block[1]);
        ((this + OUT + 8) as *mut u32).write_unaligned(block[2]);
        ((this + OUT + 12) as *mut u32).write_unaligned(block[3]);
        let answer = lf_checker_rt::callee_thiscall!(FETCH, u32, this, 1, 0, 0);
        if answer != 0 {
            let v = ((answer.wrapping_add(SLOT_LINK)) as *const u32).read_unaligned();
            ((this + SLOT) as *mut u32).write_unaligned(v);
        }
        1
    }
});
