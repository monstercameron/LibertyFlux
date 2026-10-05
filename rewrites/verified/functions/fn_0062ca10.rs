// original: 0x0062CA10 streaming_global_init (proposed)

/// Create and publish the shared streaming state on first use.
///
/// When the shared slot is already set, returns at once. Otherwise allocates
/// a 0x40-byte block through the thread-local allocator's slot `+8`, builds
/// it through the construct callee (patched), publishes it to the shared
/// slot, and fills the descriptor words: the magic at `+4`, capacity `0x60`
/// at `+0x10`, the callback pointers, zeroed spares, and the masked flags
/// half at `+0x1c`. Then registers the block through the register callee
/// (thiscall on the block: the address list) and links it into the global
/// table through the link callee (thiscall on table `+0x18`: the addresses of
/// two scratch words holding the magic and the shared-slot address, compared
/// by snapshot). The accumulator at return holds a callee answer or an
/// incoming leftover, so it is not compared (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0062ca10() -> u32 {
    unsafe {
        const SHARED: u32 = 0x18B74BC;
        const ADDR_LIST: u32 = 0x18B74B4;
        const TABLE: u32 = 0x1BB5520;
        const MAGIC: u32 = 0xF96978;
        const CAPACITY: u32 = 0x60;
        const CALLBACK_A: u32 = 0x62E410;
        const CALLBACK_B: u32 = 0x43EAD0;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        unsafe fn wr(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        if *lf_checker_rt::global::<u32>(SHARED) != 0 {
            return 0;
        }
        let holder = lf_checker_rt::tls_slot(0);
        let frobj = rd(holder, 8);
        let fvt = rd(frobj, 0);
        let fa = rd(fvt, 8);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(fa as usize);
        let mem = alloc(frobj, 0x40, 0x10, 0);
        let g: u32 = if mem == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, mem)
        };
        *lf_checker_rt::global::<u32>(SHARED) = g;
        wr(g, 4, lf_checker_rt::relocated(MAGIC));
        wr(g, 0x10, CAPACITY);
        wr(g, 0x20, 0);
        wr(g, 0x24, lf_checker_rt::relocated(CALLBACK_A));
        wr(g, 0x28, 0);
        wr(g, 0x2c, lf_checker_rt::relocated(CALLBACK_B));
        wr(g, 8, 0);
        wr(g, 0x0c, 0);
        wr(g, 0x1c, rd(g, 0x1c) & 0xFFFF0000);
        ((g + 0x1e) as *mut u16).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, g, lf_checker_rt::relocated(ADDR_LIST));
        let tab = *lf_checker_rt::global::<u32>(TABLE);
        let mut qa = lf_checker_rt::relocated(SHARED);
        let mut qb = rd(g, 4);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, tab.wrapping_add(0x18), &qb as *const u32 as u32,
            &qa as *const u32 as u32);
        0
    }
});
