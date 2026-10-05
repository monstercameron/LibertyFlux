// original: 0x0061DAB0 net_serialize_check_08

/// Serialize message kind 8 and check the allocator id and bit size.
///
/// Builds a nine-word message frame and runs it through the two (or three)
/// network helpers: the frame allocator, then this message's serializer.
/// `this` is the endpoint object, `buf` a caller buffer recorded in the
/// frame, `count` the element count (stored scaled by 8), `unused` is read
/// by nothing (the callee pops three words). Returns the accepted byte
/// count class: `(count & ~0xFF) | 1` on success, the failing stage's
/// accumulator with its low byte cleared otherwise.
///
/// Frame layout (words): `BUF`, 0, `count*8`, 0, 0, `BITS`, flag word,
/// spare. The flag word comes from uninitialized stack in the original
/// and is never read back on any path (the helpers' answers are what the
/// checks observe), so the rewrite stores 0 there. The allocator's `ID`
/// out-word lives in the incoming first stack slot on both sides: the
/// original zeroes that slot and hands its address to the allocator, and
/// the rewrite hands `&buf` the same way (the checker compares that
/// above-ESP write). `ID` is checked against the per-message global
/// `GLOBAL`; `BITS` is the serializer's out-word, accepted when
/// `(BITS+7)>>3 == count` (arithmetic shift).
/// Original: 0x0061DAB0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0061DAB0(this: u32, buf: u32, count: u32, _unused: u32) -> u32 {
    unsafe {
        const COUNT_SCALE: u32 = 8;
        const GLOBAL: u32 = 0x019F093C;
        const FRAME_WORDS: usize = 8;
        const BITS_SLOT: usize = 5;
        const ALLOC_CALLEE: u32 = 1;
        const SERIAL_CALLEE: u32 = 2;
        let mut frame = [0u32; FRAME_WORDS];
        frame[0] = buf;
        frame[2] = count.wrapping_mul(COUNT_SCALE);
        let base = frame.as_mut_ptr() as u32;
        // The incoming first stack slot doubles as the allocator's ID
        // out-slot, exactly as in the original; `buf` is not read after this.
        let id_at = &buf as *const u32 as u32;
        let alloc_ok = lf_checker_rt::callee_fastcall!(ALLOC_CALLEE, u32, id_at, base);
        if alloc_ok == 0 {
            return 0;
        }
        let id = (id_at as *const u32).read_unaligned();
        let want = lf_checker_rt::global::<u32>(GLOBAL).read_unaligned();
        if want != id {
            return want & 0xFFFF_FF00;
        }
        let wrote = lf_checker_rt::callee_fastcall!(SERIAL_CALLEE, u32, base, this);
        if wrote & 0xFF == 0 {
            return wrote & 0xFFFF_FF00;
        }

        let got = ((frame[BITS_SLOT].wrapping_add(7)) as i32 >> 3) as u32;
        if got != count {
            return got & 0xFFFF_FF00;
        }
        (count & 0xFFFF_FF00) | 1
    }
});
