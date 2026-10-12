// original: 0x0087EBE0 unnamed_0087ebe0

/// Add a nonempty, NUL-terminated string to the state and report success.
///
/// `this` stores the current byte offset at `+0x08` and flag bits at `+0x0c`.
/// A null or empty string returns true without calling helpers. Otherwise the
/// function computes its byte length, asks the capacity helper about the
/// current offset plus the low selected flag bit plus that length, and returns
/// false if capacity is denied. It then forwards the string and length to the
/// insertion helper and returns whether that helper reports the exact length.
/// The boolean is defined in `AL`; upper return bits are unspecified.
///
/// Original: thiscall with one stack pointer. Both direct helper targets are
/// intercepted by the checker; their scripted results drive the branches.
lf_checker_rt::export!(thiscall, rw_0087ebe0(this: u32, text: u32) -> u32 {
    const CURRENT_OFFSET: u32 = 0x08;
    const FLAGS_OFFSET: u32 = 0x0C;
    const CAPACITY_HELPER: u32 = 1;
    const INSERT_HELPER: u32 = 2;

    if text == 0 {
        return 1;
    }

    let mut length = 0u32;
    unsafe {
        while (text.wrapping_add(length) as *const u8).read() != 0 {
            length = length.wrapping_add(1);
        }
        if length == 0 {
            return 1;
        }

        let current_offset = (this.wrapping_add(CURRENT_OFFSET) as *const u32).read_unaligned();
        let flags = (this.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned();
        let selected_flag = (flags >> 2) & 1;
        let required_offset = current_offset.wrapping_add(selected_flag).wrapping_add(length);
        let has_capacity = lf_checker_rt::callee_thiscall!(CAPACITY_HELPER, u32, this, required_offset);
        if has_capacity as u8 == 0 {
            return 0;
        }

        let inserted_length = lf_checker_rt::callee_thiscall!(
            INSERT_HELPER,
            u32,
            this,
            text,
            length
        );
        u32::from(inserted_length == length)
    }
});
