// original: 0x00940900 streaming_request_check_c (proposed)

/// Vet the streaming request against the watcher and emit it in radius.
///
/// Reads the watcher at `obj + 0x6c`. A null watcher skips vetting. A live
/// watcher whose flag byte at `+0xe` is set is stamped with 0x3c at
/// `+0x121` and the call returns 0. Otherwise the watcher's virtual probe
/// at slot `+0x60` runs (thiscall) and a zero answer returns 0. Then the
/// squared distance of the position at `obj + 0x20` from the global anchor
/// is compared against the squared global radius: inside it (strictly,
/// ordered) the object runs through the emit worker (thiscall on the fixed
/// registry object, one stack argument) and the queue worker (cdecl, the
/// object and priority 1). Returns 0 on the early paths, 1 after vetting,
/// always in the low byte only.
///
/// Original: 0x00940900 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00940900(obj: u32) -> u32 {
    unsafe {
        const WATCHER: u32 = 0x6C;
        const WATCHER_FLAG: u32 = 0xE;
        const STAMP: u32 = 0x121;
        const STAMP_VALUE: u8 = 0x3C;
        const VT_PROBE: u32 = 0x60;
        const POS: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const ANCHOR_X: u32 = 0x011D4E40;
        const ANCHOR_Y: u32 = 0x011D4E44;
        const RADIUS: u32 = 0x011A4FB0;
        const REGISTRY: u32 = 0x01394D60;
        const POLL: u32 = 1;
        const EMIT: u32 = 2;
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
        if watcher != 0 {
            if ((watcher + WATCHER_FLAG) as *const u8).read() != 0 {
                ((watcher + STAMP) as *mut u8).write(STAMP_VALUE);
                return 0;
            }
            let vt = (watcher as *const u32).read_unaligned();
            let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + VT_PROBE) as *const u32).read_unaligned() as usize,
            );
            if probe(watcher) & 0xFF == 0 {
                return 0;
            }
        }
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
            let _: u32 = lf_checker_rt::callee_thiscall!(
                EMIT,
                u32,
                lf_checker_rt::relocated(REGISTRY),
                obj
            );
            let _: u32 = lf_checker_rt::callee_cdecl!(QUEUE, u32, obj, 1u32);
        }
        1
    }
});
