// original: 0x009D5A40 parse_digit_gated_append (proposed)
//
/// Parses a tag, derives a value on a digit gate, and appends a record.
///
/// Parses `text` with the scanner (callee 1) into record, aux and tag slots;
/// the out-pointers are skipped and observed via snapshots. The tag's low
/// byte minus '0' is compared UNSIGNED against 9 (`ja`): a digit reparses
/// the tag with a second format (callee 2) into the value slot, else the tag
/// is converted (callee 3) into it. The record slot is then resolved
/// (callee 5); a null answer stops. Otherwise a header (callee 6) holding a
/// count at `+4` and a base at `+8` yields slot `base + count * 8`, the
/// count is written back incremented, the slot is registered on the resolved
/// object (callee 7, thiscall), and the aux and value slots are stored into
/// the slot pair. Returns the value slot on the full path, else the null
/// object. The trailing
/// security cookie check (callee 8) preserves registers and is called for
/// sequence parity. Cdecl, one word.
lf_checker_rt::export!(cdecl, rw_009D5A40(text: u32) -> u32 {
    unsafe {
        const FMT1: u32 = 0xe96774;
        const FMT2: u32 = 0xe96780;
        const DIGIT_BASE: u8 = 0x30;
        const DIGIT_MAX: u8 = 9;
        const COUNT_OFF: u32 = 1;
        const BASE_OFF: u32 = 2;
        const SLOT_STRIDE: u32 = 8;
        const SCAN: u32 = 1;
        const RESCAN: u32 = 2;
        const CONVERT: u32 = 3;
        const SYNC: u32 = 4;
        const RESOLVE: u32 = 5;
        const HEADER: u32 = 6;
        const REGISTER: u32 = 7;
        const COOKIE: u32 = 8;
        let f1 = lf_checker_rt::relocated(FMT1);
        let f2 = lf_checker_rt::relocated(FMT2);
        let mut o_rec = 0u32;
        let mut o_aux = 0u32;
        let mut o_tag = 0u32;
        let mut o_val = 0u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, text, f1,
            (&mut o_rec as *mut u32) as u32,
            (&mut o_aux as *mut u32) as u32,
            (&mut o_tag as *mut u32) as u32);
        let digit: u8 = (o_tag as u8).wrapping_sub(DIGIT_BASE);
        if digit > DIGIT_MAX {
            o_val = lf_checker_rt::callee_cdecl!(
                CONVERT, u32, (&mut o_tag as *mut u32) as u32, 0);
        } else {
            let _: u32 = lf_checker_rt::callee_cdecl!(
                RESCAN, u32, (&mut o_tag as *mut u32) as u32, f2,
                (&mut o_val as *mut u32) as u32);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(SYNC, u32,);
        let obj: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE, u32, (&mut o_rec as *mut u32) as u32, 0);
        if obj == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
            return obj;
        }
        let hdr = lf_checker_rt::callee_cdecl!(HEADER, u32,) as *mut u32;
        let cnt: u32 = hdr.add(COUNT_OFF as usize).read();
        let base: u32 = hdr.add(BASE_OFF as usize).read();
        hdr.add(COUNT_OFF as usize).write(cnt.wrapping_add(1));
        let slot: u32 = base.wrapping_add(cnt.wrapping_mul(SLOT_STRIDE));
        let _: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, obj, slot);
        (slot as *mut u32).write(o_aux);
        (slot.wrapping_add(4) as *mut u32).write(o_val);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE, u32,);
        o_val
    }
});
