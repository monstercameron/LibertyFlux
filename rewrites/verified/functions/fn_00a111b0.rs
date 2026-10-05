// original: 0x00a111b0 attach_handler_route (proposed)
/// Route an attach request through the early handlers, then link the objects.
///
/// Does nothing when the byte at `a + 0x75` is clear. When the word at `c +
/// 0x40` is 1, runs the direct handler on (`a`, `b`, `c`, 1) and returns.
/// Otherwise, when the global flag is 1 and `c` is the tracked object, runs
/// the tracked handler on (`a`, `b`, `c`). Then marks `a + 0x72`, clears `a
/// + 0x110`, fetches the head through the virtual slot at `+0x2c` of `c`,
/// runs two frame-scratch probers, publishes the head's words at `+0x30`
/// and `+0x38` to `+0x78`/`+0x7c` (clearing the second when the first is not
/// `a`), and finishes with the six-argument linker and the three-argument
/// closer. No return value is set. Cdecl, three stack arguments.
export!(cdecl, rw_00a111b0(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 10;
        const PROBE_A: u32 = 11;
        const PROBE_B: u32 = 12;
        const LINKER: u32 = 13;
        const CLOSER: u32 = 14;
        const DIRECT: u32 = 15;
        const TRACKED_FN: u32 = 16;
        const ENABLE_OFF: u32 = 0x75;
        const KIND_OFF: u32 = 0x40;
        const MARK_OFF: u32 = 0x72;
        const CLEAR_OFF: u32 = 0x110;
        const VTABLE_SLOT: u32 = 0x2c;
        const PROBE_OFF: u32 = 0xb0;
        const PUB_A_OFF: u32 = 0x30;
        const PUB_B_OFF: u32 = 0x38;
        const DST_A_OFF: u32 = 0x78;
        const DST_B_OFF: u32 = 0x7c;
        const FLAG_GLOBAL: u32 = 0x012bd0f0;
        const TRACKED_GLOBAL: u32 = 0x01601088;
        if ((a + ENABLE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        if ((c + KIND_OFF) as *const u32).read_unaligned() == 1 {
            callee_cdecl!(DIRECT, u32, a, b, c, 1);
            return 0;
        }
        if *global::<u8>(FLAG_GLOBAL) == 1 && c == *global::<u32>(TRACKED_GLOBAL) {
            callee_cdecl!(TRACKED_FN, u32, a, b, c);
        }
        ((a + MARK_OFF) as *mut u8).write(1);
        ((a + CLEAR_OFF) as *mut u32).write_unaligned(0);
        let vt = (c as *const u32).read_unaligned();
        let slot = ((vt + VTABLE_SLOT) as *const u32).read_unaligned();
        let head_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let head = head_fn(c);
        let mut scratch_a = [0u32; 8];
        callee_thiscall!(PROBE_A, u32, &mut scratch_a as *mut u32 as u32);
        let mut scratch_b = [0u32; 8];
        callee_thiscall!(
            PROBE_B,
            u32,
            &mut scratch_b as *mut u32 as u32,
            (c + PROBE_OFF) as u32
        );
        let pa = ((head + PUB_A_OFF) as *const u32).read_unaligned();
        let pb = ((head + PUB_B_OFF) as *const u32).read_unaligned();
        ((head + DST_B_OFF) as *mut u32).write_unaligned(pb);
        ((head + DST_A_OFF) as *mut u32).write_unaligned(pa);
        if pa != a {
            ((head + DST_B_OFF) as *mut u32).write_unaligned(0);
        }
        let mut link_tmp = [0u32; 4];
        callee_cdecl!(
            LINKER,
            u32,
            a,
            head,
            &mut link_tmp as *mut u32 as u32,
            b,
            c,
            0
        );
        callee_cdecl!(CLOSER, u32, a, b, c);
        0
    }
});
