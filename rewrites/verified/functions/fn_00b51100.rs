// original: 0x00b51100 has_active_model (proposed)

/// Test whether the ped attached to a slot has an active model.
///
/// `slot` points to a record whose ped pointer is at `+0x224`; the ped
/// carries a 16-bit model id at `+0x2EC` where 0xFFFF means none.
/// Returns 1 when the id differs from 0xFFFF, else 0.
///
/// Original: 0x00b51100 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b51100(slot: u32) -> u32 {
    unsafe {
        const PED: u32 = 0x224;
        const MODEL_ID: u32 = 0x2ec;
        const NO_MODEL: u16 = 0xffff;
        let ped = ((slot + PED) as *const u32).read_unaligned();
        let id = ((ped + MODEL_ID) as *const u16).read_unaligned();
        u32::from(id != NO_MODEL)
    }
});
