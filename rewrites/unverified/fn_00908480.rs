// original: 0x00908480 input_slot_probe (proposed)

/// Probe one input slot: initialise a scratch record, feed its fields and the
/// slot's fields to a (pointer, length) sink, and return 1.
///
/// `index` selects the record. A negative value (as i32) uses the scratch
/// record the initialiser callee filled; otherwise the record is the table
/// entry `TABLE[index]` (`TABLE` holds one pointer per slot). The active flag
/// is byte `+0x08` of the record (0 for the scratch record).
///
/// The sink callee (cdecl, two arguments) is called with the saved index
/// (length 4), a byte beside the scratch record (length 1), then the record
/// fields at offsets `0x20` (2), `0x30` (16), `0x40` (4), `0x24` (4),
/// `0x60` (60), `0x00` (2), `0x04` (4), `0x44` (4), `0x48` (4), `0x4c` (4),
/// `0x50` (4), `0x54` (4), `0x58` (1), then a computed word (length 4) and a
/// fixed global (length 8). The computed word is 0 unless the record is
/// active and the pointer at `+0x5c` is non-null, in which case it is the
/// word past that pointer plus one.
///
/// Returns the sink's last answer with its low byte set to 1. Ends with the
/// standard cookie check. Faults exactly when the original does: a wild
/// index faults on the table load, a null or wild entry on the flag or field
/// reads. Original: 0x00908480 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00908480(index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118F6F8;
        const FIXED_GLOBAL: u32 = 0x01193C5C;
        const COOKIE: u32 = 0x01057FB4;
        const FLAG_OFF: u32 = 0x08;
        const LINK_OFF: u32 = 0x5c;

        // Mirror of the original's frame: saved index, computed word, then
        // the scratch record the initialiser fills through `entry`.
        let mut frame = [0u32; 48];
        frame[2] = index;
        let base = frame.as_mut_ptr() as u32;
        let entry = base.wrapping_add(16);
        // Initialiser (thiscall, record pointer in ecx).
        lf_checker_rt::callee_thiscall!(1, u32, entry);

        let neg = (index as i32) < 0;
        let esi = if neg {
            entry
        } else {
            let tab = lf_checker_rt::relocated(TABLE);
            (tab.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
        };
        let flag: u8 = if neg {
            0
        } else {
            ((esi.wrapping_add(FLAG_OFF) as *const u8).read() != 0) as u8
        };

        lf_checker_rt::callee_cdecl!(2, u32, base.wrapping_add(8), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, base.wrapping_add(15), 1u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x20), 2u32);
        lf_checker_rt::callee_cdecl!(3, u32, esi.wrapping_add(0x30), 0x10u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x40), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x24), 4u32);
        lf_checker_rt::callee_cdecl!(4, u32, esi.wrapping_add(0x60), 0x3cu32);
        lf_checker_rt::callee_cdecl!(2, u32, esi, 2u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(4), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x44), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x48), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x4c), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x50), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x54), 4u32);
        lf_checker_rt::callee_cdecl!(2, u32, esi.wrapping_add(0x58), 1u32);

        let mut out = 0u32;
        if flag != 0 {
            let p = (esi.wrapping_add(LINK_OFF) as *const u32).read_unaligned();
            if p != 0 {
                out = ((p.wrapping_add(4)) as *const u32)
                    .read_unaligned()
                    .wrapping_add(1);
            }
        }
        frame[3] = out;
        lf_checker_rt::callee_cdecl!(2, u32, base.wrapping_add(12), 4u32);
        let last =
            lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(FIXED_GLOBAL), 8u32);
        let cookie = (lf_checker_rt::relocated(COOKIE) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(5, u32, cookie);
        (last & 0xFFFFFF00) | 1
    }
});
