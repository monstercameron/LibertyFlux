// original: 0x0097EFD0 PED_WATER_LAND

/// Fire the ped-water-land audio event for a ped task object.
///
/// `this` is the task object. Besides the shared audio-ready guards (quit
/// flag clear, two session words equal, mode word not the skip value) the
/// task's last-fire time at `+0x140` must be older than 0x3E8 ticks
/// before the clock word: otherwise the call is a no-op. On the main path
/// the task records 1.0 at `+0x144` and the current clock at `+0x140`.
///
/// A scratch request buffer is initialised by callee 2 and filled with a
/// nested vehicle word (`[[this+0x120]+0x20]`, biased by 0x30) at word 5
/// and a sub-object word (`+0x8`) at word 8; callee 3 resolves a handle,
/// callee 4 derives a parameter, and callee 5 (this = task, name word
/// first) arbitrates: a zero low byte releases the handle through
/// callee 7, otherwise callee 1 hashes the event name and callee 6 posts
/// the event with a (0, -1, 0x35) descriptor and the buffer.
///
/// Original: 0x0097EFD0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_0097EFD0(this: u32) -> u32 {
    unsafe {
        const G_QUIT: u32 = 0x011F7060;
        const G_SESS_A: u32 = 0x012088B4;
        const G_SESS_B: u32 = 0x00F1C040;
        const G_MODE: u32 = 0x01037720;
        const G_CLOCK: u32 = 0x011735B4;
        const SKIP_MODE: u32 = 0x12;
        const COOLDOWN: u32 = 0x3E8;
        const ONE_F: u32 = 0x3F800000;
        const NAME_ARB: u32 = 0x00E8CCA0;
        const NAME_EV: u32 = 0x00E8CCB0;
        const OFF_LAST: u32 = 0x140;
        const OFF_LEVEL: u32 = 0x144;
        const OFF_PED: u32 = 0x120;
        const OFF_SUB: u32 = 0x08;
        const PED_INNER: u32 = 0x20;
        const INNER_BIAS: u32 = 0x30;
        const BUF_INNER_WORD: usize = 5;
        const BUF_SUB_WORD: usize = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }

        if gget(G_QUIT) == 1 {
            return 0;
        }
        if gget(G_SESS_A) != gget(G_SESS_B) {
            return 0;
        }
        if gget(G_MODE) == SKIP_MODE {
            return 0;
        }
        let clock = gget(G_CLOCK);
        if rd32(this.wrapping_add(OFF_LAST)).wrapping_add(COOLDOWN) >= clock {
            return 0;
        }
        wr32(this.wrapping_add(OFF_LEVEL), ONE_F);
        wr32(this.wrapping_add(OFF_LAST), clock);
        let mut buf = [0u32; 16];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, buf_ptr);
        buf[BUF_SUB_WORD] = rd32(this.wrapping_add(OFF_SUB));
        buf[BUF_INNER_WORD] = rd32(rd32(this.wrapping_add(OFF_PED)).wrapping_add(PED_INNER))
            .wrapping_add(INNER_BIAS);
        let handle: u32 = lf_checker_rt::callee_thiscall!(3, u32, buf_ptr);
        let param: u32 = lf_checker_rt::callee_cdecl!(4, u32, handle);
        let arb: u32 = lf_checker_rt::callee_thiscall!(
            5,
            u32,
            this,
            lf_checker_rt::relocated(NAME_ARB),
            buf_ptr,
            handle,
            param,
            0
        );
        if (arb & 0xFF) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, handle);
            return 0;
        }
        let mut desc = [0u32, 0xFFFF_FFFF, 0x35];
        let desc_ptr = desc.as_mut_ptr() as u32;
        let hash: u32 = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            lf_checker_rt::relocated(NAME_EV),
            0
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(
            6,
            u32,
            hash,
            0,
            0,
            1,
            buf_ptr,
            desc_ptr,
            rd32(this.wrapping_add(OFF_PED)),
            handle
        );
        0
    }
});
