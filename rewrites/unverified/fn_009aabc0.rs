// original: 0x009AABC0 audio_voice_slot_update (proposed)

/// Voice-slot update: stores two values into one voice record and notifies
/// a callee about the middle of it.
///
/// Record selector: index byte at `this + a + 0x368` plus `3 * a`, times the
/// 96-byte record size. Stores `b` at record `+ 0x14`, calls the notify
/// callee with (`c`, address of record `+ 0x18`), then stores `d` at record
/// `+ 0x58`. The callee's answer is ignored. Returns nothing.
/// Original: 0x009AABC0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_009AABC0(this: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe {
        const INDEX_BASE: u32 = 0x368;
        const RECORD_SIZE: u32 = 96;
        const STORE_A_OFF: u32 = 0x14;
        const NOTIFY_OFF: u32 = 0x18;
        const STORE_B_OFF: u32 = 0x58;
        const NOTIFY: u32 = 1;
        let t = ((this.wrapping_add(a).wrapping_add(INDEX_BASE)) as *const u8).read() as u32;
        let rec = (t.wrapping_add(a.wrapping_mul(3))).wrapping_mul(RECORD_SIZE);
        let base = this.wrapping_add(rec);
        ((base.wrapping_add(STORE_A_OFF)) as *mut u32).write_unaligned(b);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            NOTIFY,
            u32,
            c,
            base.wrapping_add(NOTIFY_OFF)
        );
        ((base.wrapping_add(STORE_B_OFF)) as *mut u32).write_unaligned(d);
        0
    }
});
