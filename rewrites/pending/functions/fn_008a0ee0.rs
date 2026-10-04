// original: 0x008A0EE0 rage::audCrossfadeSound::vf9
/// Retrigger one crossfade voice by slot index (vtable slot 9).
///
/// Looks up the sound entry for the indexed slot and offers it the
/// resolved voice key: when the offer is accepted the entry is started
/// on this voice, otherwise the entry is released and the slot is
/// retired. Empty slots and null entries return without doing anything.
///
/// The slot index arrives as the second argument, the forwarded value as
/// the first. The flag word passed to the offer step carries one meaningful
/// bit from the object flags above the incoming index, which the original
/// keeps in its reused argument slot; the contract therefore disables the
/// stack check while ESP adjustment is still verified.
lf_k2_rt::export!(thiscall, rw_008A0EE0(this: *mut u8, arg0: u32, idx: u32) -> u32 {
    unsafe {
        let stride = *lf_k2_rt::global::<u32>(0x115D964);
        let table = *lf_k2_rt::global::<u32>(0x115D988);
        let bank = *this.add(0x40);
        let row = *((table
            .wrapping_add((bank as u32).wrapping_mul(0x6F40))
            .wrapping_add(0x6F10)) as *const u32);
        let slot = *((this as u32).wrapping_add(idx).wrapping_add(0x48) as *const u8);
        if slot == 0xFF {
            return 0;
        }
        if row.wrapping_add((slot as u32).wrapping_mul(stride)) == 0 {
            return 0;
        }
        let flag = ((*this.add(0x39) >> 5) & 1) as u32;
        let key = *(this.add(0x3C) as *const i16) as i32 as u32;
        let resolved: u32 = lf_k2_rt::callee_cdecl!(1, u32, key);
        let slot2 =
            *((this as u32).wrapping_add(idx).wrapping_add(0x48) as *const u8);
        let entry2 = if slot2 == 0xFF {
            0
        } else {
            row.wrapping_add((slot2 as u32).wrapping_mul(stride))
        };
        let accepted: u32 = lf_k2_rt::callee_thiscall!(
            2,
            u32,
            entry2,
            resolved,
            (idx & 0xFFFFFF00) | flag,
            0
        );
        let slot3 =
            *((this as u32).wrapping_add(idx).wrapping_add(0x48) as *const u8);
        if accepted == 1 {
            let entry3 = if slot3 == 0xFF {
                0
            } else {
                row.wrapping_add((slot3 as u32).wrapping_mul(stride))
            };
            lf_k2_rt::callee_thiscall!(3, u32, entry3, arg0);
        } else {
            let entry3 = if slot3 == 0xFF {
                0
            } else {
                row.wrapping_add((slot3 as u32).wrapping_mul(stride))
            };
            lf_k2_rt::callee_thiscall!(4, u32, entry3);
            lf_k2_rt::callee_thiscall!(5, u32, this as u32, idx);
        }
        0
    }
});
