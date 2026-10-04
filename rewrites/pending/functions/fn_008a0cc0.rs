// original: 0x008A0CC0 audio_dual_slot_apply (proposed)
/// Apply a parameter change to both voice slots (proposed name
/// `audio_dual_slot_apply`).
///
/// Offers the two argument words to each live voice slot in turn: first
/// a parameter update on the slot's entry, then a guarded apply step.
/// Returns 2 immediately if any apply step reports 2, else 1 if every
/// visited apply step reported nonzero, else 0. Empty slots (0xFF) and
/// null entries are skipped without touching the result.
///
/// The flag word passed to the apply step carries one meaningful bit
/// from the object flags; its upper bytes are the defined stack fill.
lf_k2_rt::export!(thiscall, rw_008A0CC0(this: *mut u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let stride = *lf_k2_rt::global::<u32>(0x115D964);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *this.add(0x40);
        let row = *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F10)) as *const u32);
        let mut live = true;
        let mut slot_i = 0u32;
        while slot_i < 2 {
            let slot = *this.add(0x48 + slot_i as usize);
            if slot != 0xFF {
                let entry = row.wrapping_add((slot as u32).wrapping_mul(stride));
                if entry != 0 {
                    lf_k2_rt::callee_thiscall!(
                        1,
                        u32,
                        entry,
                        *(this.add(0x54) as *const u32),
                        0
                    );
                    let flag = ((*this.add(0x39) >> 5) & 1) as u32;
                    let r: u32 = lf_k2_rt::callee_thiscall!(
                        2, u32, entry, arg0, flag, arg1
                    );
                    if r == 2 {
                        return 2;
                    }
                    if r == 0 {
                        live = false;
                    }
                }
            }
            slot_i += 1;
        }
        if live {
            1
        } else {
            0
        }
    }
});
