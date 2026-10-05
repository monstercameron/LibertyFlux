// original: 0x00939C40 stream_meter_update (proposed)

/// Refresh the streaming meter from the row fetcher and probes.
///
/// Fetches table row 12 into a frame slot, clears two marker globals,
/// then truncates the fetched float toward zero to 64 bits exactly like
/// the x87 store (NaN and out-of-range yield the indefinite value) and
/// keeps the low 32 bits. Adds the live value into the floor global,
/// stamps mode 6, copies the fresh stamp, latches 4 for a zero probe low
/// byte and 3 otherwise, and when the handle opens, stores its tag word
/// and answers the handle with its low half replaced by the tag.
lf_checker_rt::export!(cdecl, rw_00939c40() -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const LIVE: u32 = 2;
        const PROBE: u32 = 3;
        const OPEN: u32 = 4;
        const ROW: u32 = 12;
        const FLAG_A: u32 = 0x11A4F14;
        const FLAG_B: u32 = 0x11A4F04;
        const FLOOR: u32 = 0x11A4F08;
        const MODE: u32 = 0x11A4EF4;
        const MARKER: u32 = 0x11A4EFC;
        const STAMP: u32 = 0x1284644;
        const LATCH: u32 = 0x11A4EF8;
        const TAG_OUT: u32 = 0x11A4F10;
        const TAG_OFF: u32 = 0x2C;
        // Truncate a float to 64 bits exactly like fistp with the
        // round-toward-zero control word, answering the low 32 bits.
        let fistp_low = |f: f32| -> u32 {
            const INDEF: u64 = 0x8000_0000_0000_0000;
            const TWO63: f32 = 9223372036854775808.0;
            let q = if f.is_nan() || f >= TWO63 || f < -TWO63 {
                INDEF
            } else {
                f as i64 as u64
            };
            q as u32
        };
        let mut slot = [0u32; 2];
        let got: u32 = lf_checker_rt::callee_cdecl!(
            FETCH,
            u32,
            &mut slot as *mut u32 as u32,
            ROW
        );
        lf_checker_rt::global::<u32>(FLAG_A).write_unaligned(0);
        lf_checker_rt::global::<u8>(FLAG_B).write(0);
        let whole = fistp_low(f32::from_bits(
            (got as *const u32).read_unaligned(),
        ));
        let live: u32 = lf_checker_rt::callee_cdecl!(LIVE, u32,);
        lf_checker_rt::global::<u32>(FLOOR)
            .write_unaligned(live.wrapping_add(whole));
        lf_checker_rt::global::<u32>(MODE).write_unaligned(6);
        lf_checker_rt::global::<u32>(MARKER).write_unaligned(
            lf_checker_rt::global::<u32>(STAMP).read_unaligned(),
        );
        let t: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
        lf_checker_rt::global::<u32>(LATCH)
            .write_unaligned(if (t & 0xFF) == 0 { 4 } else { 3 });
        let h: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, 0);
        if h == 0 {
            0
        } else {
            let tag = ((h + TAG_OFF) as *const u16).read_unaligned();
            lf_checker_rt::global::<u16>(TAG_OUT).write_unaligned(tag);
            (h & 0xFFFF_0000) | tag as u32
        }
    }
});
