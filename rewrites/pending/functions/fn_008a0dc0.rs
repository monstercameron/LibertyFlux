// original: 0x008A0DC0 audio_slot_probe (proposed)
/// Probe the voice slots for a live entry (proposed name `audio_slot_probe`).
///
/// If the indexed entry is active, runs the entry's two setup steps, then
/// scans both voice slots: a slot tagged 2 is offered the stimulus and
/// latches the result when the offer is accepted, a slot tagged 1 latches
/// unconditionally, anything else is skipped. Returns 1 if any slot
/// latched, else 0.
///
/// The second setup call is declared stdcall in the contract even though
/// the real callee reads ECX: under interception ECX there holds the
/// previous stub's exit value on the original side, which is an artifact
/// of the interception rather than game behaviour, so it is compared on
/// neither side and only the stack argument is verified.
lf_k2_rt::export!(thiscall, rw_008A0DC0(this: *mut u8, arg0: u32) -> u32 {
    unsafe {
        let stride = *lf_k2_rt::global::<u32>(0x115D964);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *this.add(0x40);
        let row = *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F10)) as *const u32);
        let mut latched = false;
        let indexed =
            row.wrapping_add((*this.add(0xB0) as u32).wrapping_mul(stride));
        if *((indexed + 0x72) as *const u8) != 0 {
            lf_k2_rt::callee_thiscall!(1, u32, this as u32, indexed);
            lf_k2_rt::callee_stdcall!(2, u32, indexed);
        }
        let mut slot_i = 0u32;
        while slot_i < 2 {
            let slot = *this.add(0x48 + slot_i as usize);
            if slot != 0xFF {
                let entry =
                    row.wrapping_add((slot as u32).wrapping_mul(stride));
                if entry != 0 {
                    let tag = *((entry + 6) as *const u16);
                    if tag == 2 {
                        let r: u32 = lf_k2_rt::callee_thiscall!(
                            3, u32, entry, arg0
                        );
                        if (r & 0xFF) != 0 {
                            latched = true;
                        }
                    } else if tag == 1 {
                        latched = true;
                    }
                }
            }
            slot_i += 1;
        }
        if latched {
            1
        } else {
            0
        }
    }
});
