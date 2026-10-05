// original: 0x00b1c5f0 refresh_channel_gated (proposed)

/// Refreshes the channel's cached levels when the gate flag is set.
///
/// Thiscall with no stack arguments. With the gate byte clear it returns
/// at once (returning the caller's EAX, so that path is excluded from
/// the proof). Otherwise it clears flag bit 0, writes the default
/// header (1.0 at +0x08, 0 at +0x04, 0 byte at +0x3C), scales the fetched
/// rate (cdecl of a scratch buffer and 0x30; word at +4 of the returned
/// block) by the -76.0 constant into +0x38 with +0x34 zeroed, reads one
/// word the filler (cdecl of a scratch buffer) left past its buffer into
/// +0x0C, clears flag bits 2, 4 and 5, zeroes +0x10, writes -1 to +0x28
/// and runs the level scaler (thiscall, no stack arguments), returning
/// its result. The link query (cdecl of 2, a scratch buffer, 0, 0)
/// between the filler and the read leaves nothing the function uses.
lf_checker_rt::export!(thiscall, rw_00b1c5f0(this: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0x011609f6;
        const RATE_SCALE_ADDR: u32 = 0x00eab700;
        const RATE_TAG: u32 = 0x30;
        let gate = lf_checker_rt::global::<u8>(GATE) as *const u8;
        if gate.read() == 0 {
            return 0;
        }
        let flag = (this + 0x41) as *mut u8;
        flag.write(flag.read() & 0xfe);
        ((this + 8) as *mut u32).write_unaligned(0x3f800000);
        ((this + 4) as *mut u32).write_unaligned(0);
        ((this + 0x3c) as *mut u8).write(0);
        let mut scratch = [0u32; 4];
        let block: u32 = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            scratch.as_mut_ptr() as u32,
            RATE_TAG
        );
        let rate = ((block + 4) as *const f32).read_unaligned();
        let scale =
            (lf_checker_rt::relocated(RATE_SCALE_ADDR) as *const f32).read_unaligned();
        ((this + 0x34) as *mut u32).write_unaligned(0);
        ((this + 0x38) as *mut f32).write_unaligned(
            core::hint::black_box(rate) * core::hint::black_box(scale),
        );
        let mut fill = [0u32; 4];
        lf_checker_rt::callee_cdecl!(2, u32, fill.as_mut_ptr() as u32);
        let mut link = [0u32; 4];
        lf_checker_rt::callee_cdecl!(3, u32, 2, link.as_mut_ptr() as u32, 0, 0);
        flag.write(flag.read() & 0xcb);
        ((this + 0x0c) as *mut u32).write_unaligned(fill[1]);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        ((this + 0x28) as *mut u32).write_unaligned(0xffff_ffff);
        lf_checker_rt::callee_thiscall!(4, u32, this)
    }
});
