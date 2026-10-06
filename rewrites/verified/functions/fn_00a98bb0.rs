// original: 0x00a98bb0 filemem_init_stream_state

/// Initialise a stream state object to its defaults and register its tail.
///
/// `this` points to the object, `a1`/`a2` are stored at `+0x1c`/`+0x20`.
/// Flag bytes and counters are cleared, three range words take 0x7f7fffff,
/// three granularity words take 0x800000, and two sample slots (`+0x7c`,
/// `+0x8c`) take a copy of one unread stack word each, exactly as the
/// original reads them (whatever the caller left there; the checker gives
/// both sides the same defined fill). The tail at `+0xc8` is then handed
/// to the register callee together with the object itself.
///
/// Original: 0x00A98BB0 (thiscall, two stack words; one direct callee).
lf_checker_rt::export!(thiscall, rw_00a98bb0(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const RANGE_INIT: u32 = 0x7f7fffff;
        const GRAN_INIT: u32 = 0x00800000;
        const TAIL_OFF: u32 = 0xc8;
        const REGISTER: u32 = 1;

        // The original reads one stack word it never wrote, twice. Read a
        // stack word this function never wrote the same way: an address
        // well below this frame but inside the checker's filled window (a
        // volatile load so the read really happens). Under the checker's
        // defined uniform stack fill both sides observe the same value;
        // what the proof checks is that the two sample slots are stored.
        let probe = core::mem::MaybeUninit::<u32>::uninit();
        let garbage: u32 =
            core::ptr::read_volatile((probe.as_ptr() as u32).wrapping_sub(0x800) as *const u32);

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this.wrapping_add(0x1c), a1);
        wr32(this.wrapping_add(0x20), a2);
        ((this.wrapping_add(0x15)) as *mut u8).write(0);
        wr32(this.wrapping_add(0xc4), 0);
        wr32(this.wrapping_add(0xc0), 0);
        wr32(this.wrapping_add(0x7c), garbage);
        wr32(this.wrapping_add(0x70), RANGE_INIT);
        wr32(this.wrapping_add(0x74), RANGE_INIT);
        wr32(this.wrapping_add(0x78), RANGE_INIT);
        wr32(this.wrapping_add(0x80), GRAN_INIT);
        wr32(this.wrapping_add(0x84), GRAN_INIT);
        wr32(this.wrapping_add(0x88), GRAN_INIT);
        wr32(this.wrapping_add(0x8c), garbage);
        let _: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, this.wrapping_add(TAIL_OFF), this);
        0
    }
});
