// original: 0x00c66130 CCutsceneObject::is_field_2a0_nonzero

/// Whether the cutscene object's word at +0x2a0 is nonzero.
///
/// `this` is the cutscene object. The word at `FIELD_2A0 (+0x2a0)` is
/// compared against zero (unsigned comparison; only zero versus nonzero
/// matters) and 1 or 0 is returned in AL; nothing is written and no calls
/// are made. Only the low byte of the result is behaviour, so the contract
/// compares AL only.
///
/// Original: thiscall, no stack arguments, byte result in AL.
lf_checker_rt::export!(thiscall, rw_00c66130(this: u32) -> u32 {
    const FIELD_2A0: u32 = 0x2a0;
    unsafe {
        let v = ((this + FIELD_2A0) as *const u32).read_unaligned();
        u32::from(v != 0)
    }
});
