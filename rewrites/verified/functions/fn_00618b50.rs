// original: 0x00618b50 render_state_block_apply_param (proposed)

/// Save the current render-state cache into a caller buffer and install a new
/// state set, one entry at a time.
///
/// `this` points to a state-save buffer: a 32-bit entry count at `+0`, then
/// 8-byte entries starting at `+4`, each holding a save key followed by the
/// cached value that was in force. `mode` (EDX) is a caller-chosen value used
/// for two of the installed states.
///
/// The table below is the whole behaviour, applied in order for each of the
/// 17 entries: read the cached value from its global slot, append
/// `(save_key, old_value)` at index `count` and increment `count`, then
/// install the new state. The first ten entries go through setter A: the new
/// cached value is written to the global slot first and then setter A is
/// called with `(arg0, arg1)` (note entries 6 and 7, where the cached value
/// differs from the call's second argument). The last seven entries go
/// through setter B with `(arg0, arg1)` and leave the global slot untouched.
/// All calls are cdecl with two word arguments; every pushed value is a full
/// word. The function result is the return value of the last setter call.
///
/// Original: thiscall with an EDX parameter, no stack arguments; the rewrite
/// is declared fastcall, which passes (ECX, EDX) identically and cleans
/// nothing with no stack arguments. Edge cases: `count` wraps with 32-bit
/// increment; entries past the buffer end write wherever the buffer points,
/// exactly as the original computes the address.
lf_checker_rt::export!(fastcall, rw_00618b50(this: u32, mode: u32) -> u32 {
    unsafe {
        /// One state entry: (save key, cache global file VA, setter id,
        /// call arg0, call arg1 or mode, cached value or mode).
        /// Setter 1 writes the cached value before calling; setter 2 does not.
        const SETTER_A: u32 = 1;
        const SETTER_B: u32 = 2;
        const COUNT_OFF: u32 = 0;
        const ENTRIES_OFF: u32 = 4;
        const ENTRY_STRIDE: u32 = 8;

        #[inline(always)]
        unsafe fn cached(slot_file_va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(slot_file_va)).read() }
        }
        #[inline(always)]
        unsafe fn store_cached(slot_file_va: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(slot_file_va)).write(v) }
        }
        #[inline(always)]
        unsafe fn append(this: u32, key: u32, old: u32) {
            unsafe {
                let n = (this as *const u32).read();
                let slot = this.wrapping_add(ENTRIES_OFF).wrapping_add(n.wrapping_mul(ENTRY_STRIDE));
                (slot as *mut u32).write(key);
                (slot.wrapping_add(4) as *mut u32).write(old);
                ((this.wrapping_add(COUNT_OFF)) as *mut u32).write(n.wrapping_add(1));
            }
        }

        let mut last: u32 = 0;
        // Entry: (key, global VA, setter, arg0, arg1_is_mode, arg1_or_mode_flag, cached_or_mode)
        // Written as straight-line calls so the order matches the original.
        macro_rules! entry_a {
            ($key:expr, $g:expr, $a0:expr, $a1:expr, $new:expr) => {{
                let old = cached($g);
                append(this, $key, old);
                store_cached($g, $new);
                last = lf_checker_rt::callee_cdecl!(SETTER_A, u32, $a0, $a1);
            }};
        }
        macro_rules! entry_b {
            ($key:expr, $g:expr, $a0:expr, $a1:expr) => {{
                let old = cached($g);
                append(this, $key, old);
                last = lf_checker_rt::callee_cdecl!(SETTER_B, u32, $a0, $a1);
            }};
        }

        entry_a!(0x0f, 0x017f5884, 0x13, 0x00, 0x00);
        entry_a!(0x13, 0x017f5894, 0x0b, 0x01, 0x01);
        entry_a!(0x1a, 0x017f58b0, 0x12, 0xff, 0xff);
        entry_a!(0x19, 0x017f58ac, 0x11, 0xff, 0xff);
        entry_a!(0x18, 0x017f58a8, 0x10, mode, mode);
        entry_a!(0x17, 0x017f58a4, 0x0f, 0x08, 0x01);
        entry_a!(0x16, 0x017f58a0, 0x0e, 0x03, 0x02);
        entry_a!(0x14, 0x017f5898, 0x0c, 0x01, 0x00);
        entry_a!(0x15, 0x017f589c, 0x0d, 0x01, 0x00);
        entry_a!(0x1d, 0x017f58bc, 0x23, 0x01, 0x01);
        entry_b!(0x24, 0x017f58d8, 0x24, 0xff);
        entry_b!(0x23, 0x017f58d4, 0x23, 0xff);
        entry_b!(0x22, 0x017f58d0, 0x22, mode);
        entry_b!(0x21, 0x017f58cc, 0x21, 0x01);
        entry_b!(0x20, 0x017f58c8, 0x20, 0x02);
        entry_b!(0x1e, 0x017f58c0, 0x1e, 0x00);
        entry_b!(0x1f, 0x017f58c4, 0x1f, 0x00);
        last
    }
});
