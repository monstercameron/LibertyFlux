// original: 0x00940860 streaming_request_check_b (proposed)

/// Vet the streaming request against the watcher and queue it in radius.
///
/// Reads the watcher at `obj + 0x6c`: a live watcher whose flag byte at
/// `+0xe` is set vetoes the request. A live watcher is also polled
/// (thiscall, no stack arguments) and a non-zero answer vetoes it too.
/// When neither vetoes, the virtual probe at slot `+0xd0` runs (thiscall)
/// and its record's state word must read 2. Then the squared distance of
/// the position at `obj + 0x20` from the global anchor is compared against
/// the squared global radius: inside it (strictly, ordered) the object is
/// queued with priority 1 (cdecl, two arguments). Always returns 1 in the
/// low byte.
///
/// Original: 0x00940860 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00940860(obj: u32) -> u32 {
    unsafe {
        const WATCHER: u32 = 0x6C;
        const WATCHER_FLAG: u32 = 0xE;
        const VT_PROBE: u32 = 0xD0;
        const PROBE_STATE: u32 = 0x12C;
        const READY_STATE: u32 = 2;
        const POS: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const ANCHOR_X: u32 = 0x011D4E40;
        const ANCHOR_Y: u32 = 0x011D4E44;
        const RADIUS: u32 = 0x011A4FB0;
        const POLL: u32 = 1;
        const PROBE: u32 = 2;
        const QUEUE: u32 = 3;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let watcher = ((obj + WATCHER) as *const u32).read_unaligned();
        let flagged = watcher != 0 && ((watcher + WATCHER_FLAG) as *const u8).read() != 0;
        let mut polled = false;
        if watcher != 0 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(POLL, u32, watcher);
            polled = ok & 0xFF != 0;
        }
        if !flagged && !polled {
            let vt = (obj as *const u32).read_unaligned();
            let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + VT_PROBE) as *const u32).read_unaligned() as usize,
            );
            let record = probe(obj);
            if ((record + PROBE_STATE) as *const u32).read_unaligned() == READY_STATE {
                let pos = ((obj + POS) as *const u32).read_unaligned();
                let dx = sub(
                    f32::from_bits(((pos + POS_Y) as *const u32).read_unaligned()),
                    f32::from_bits(lf_checker_rt::global::<u32>(ANCHOR_Y).read()),
                );
                let dy = sub(
                    f32::from_bits(((pos + POS_X) as *const u32).read_unaligned()),
                    f32::from_bits(lf_checker_rt::global::<u32>(ANCHOR_X).read()),
                );
                let dist = add(mul(dy, dy), mul(dx, dx));
                let radius = f32::from_bits(lf_checker_rt::global::<u32>(RADIUS).read());
                if mul(radius, radius) > dist {
                    let _: u32 = lf_checker_rt::callee_cdecl!(QUEUE, u32, obj, 1u32);
                }
            }
        }
        1
    }
});
