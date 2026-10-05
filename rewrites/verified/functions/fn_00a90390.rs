// original: 0x00a90390 stream_detach_lookup

/// Detaches an object from its lookup slot after preparing it.
///
/// Calls the prepare step (callee 1, thiscall) with `obj+0x80`, then acts
/// only when the kind byte at `obj+0x40` is 0x3F and the key at `obj+0x44`
/// is not 0xFFFF. Looks the key up (callee 2, thiscall) through three
/// own-frame out-slots (two pre-set to 0x3F, compared by snapshot and by
/// the scripted out-word); a 0x3F answer ends the call. Otherwise scans
/// four dwords at `arr[edx*96+0x40]` (`arr` at `this+0x80`) for `obj` and,
/// when found, clears the slot, clears bit 0x08000000 of `obj+0x24` and
/// sets `obj+0x41` to 9. Returns the prepare answer / 0xFFFF / the lookup
/// answer / the found code / the scan-end pointer per path. Two calls.
/// Original: 0x00A90390 (thiscall, ECX + one stack word), 166 bytes.
lf_checker_rt::export!(thiscall, rw_00a90390(this: u32, obj: u32) -> u32 {
    unsafe {
        const ARR_OFF: u32 = 0x80;
        const KIND_OFF: u32 = 0x40;
        const KIND_ACTIVE: u8 = 0x3F;
        const KEY_OFF: u32 = 0x44;
        const KEY_NONE: u16 = 0xFFFF;
        const FLAGS_OFF: u32 = 0x24;
        const FLAGS_KEEP: u32 = 0xF7FF_FFFF;
        const STATE_OFF: u32 = 0x41;
        const STATE_DONE: u8 = 9;
        const SLOT_BASE: u32 = 0x40;
        const STRIDE: u32 = 96;
        const SCAN_N: u32 = 4;
        const PREPARE: u32 = 1;
        const LOOKUP: u32 = 2;
        let prep: u32 = lf_checker_rt::callee_thiscall!(PREPARE, u32, this, obj.wrapping_add(ARR_OFF));
        if (obj.wrapping_add(KIND_OFF) as *const u8).read() != KIND_ACTIVE {
            return prep;
        }
        let key = (obj.wrapping_add(KEY_OFF) as *const u16).read_unaligned();
        if key == KEY_NONE {
            return KEY_NONE as u32;
        }
        let sval = (key as i16) as i32 as u32;
        let mut scratch: u32 = 0;
        let mut answer: u32 = KIND_ACTIVE as u32;
        let mut spare: u32 = KIND_ACTIVE as u32;
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            LOOKUP, u32, this, sval,
            &mut scratch as *mut u32 as u32,
            &mut answer as *mut u32 as u32,
            &mut spare as *mut u32 as u32
        );
        let edx = answer;
        if edx == KIND_ACTIVE as u32 {
            return ans;
        }
        let arr = (this.wrapping_add(ARR_OFF) as *const u32).read_unaligned();
        let base = arr
            .wrapping_add(edx.wrapping_mul(STRIDE))
            .wrapping_add(SLOT_BASE);
        let mut i = 0u32;
        loop {
            let slot = base.wrapping_add(i.wrapping_mul(4));
            if (slot as *const u32).read_unaligned() == obj {
                let code = i.wrapping_add(edx.wrapping_mul(3).wrapping_mul(8));
                (slot as *mut u32).write_unaligned(0);
                let flags = (obj.wrapping_add(FLAGS_OFF) as *const u32).read_unaligned();
                (obj.wrapping_add(FLAGS_OFF) as *mut u32).write_unaligned(flags & FLAGS_KEEP);
                (obj.wrapping_add(STATE_OFF) as *mut u8).write(STATE_DONE);
                return code;
            }
            i = i.wrapping_add(1);
            if i >= SCAN_N {
                return base.wrapping_add(SCAN_N.wrapping_mul(4));
            }
        }
    }
});
