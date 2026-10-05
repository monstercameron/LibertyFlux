// original: 0x00941160 streaming_request_check_d (proposed)

/// Fully vet the streaming request and emit it when inside the radius.
///
/// Returns 1 unless every gate passes: the marker byte at `obj + 0x219`
/// is clear, the pair worker (thiscall, two zero arguments) answers
/// non-zero, and the watcher at `obj + 0x6c` is either null or unflagged
/// with a non-zero virtual probe answer (a flagged watcher is stamped
/// with 0x3c at `+0x121` and returns 0 instead). Then the vacancy worker
/// runs (cdecl, no arguments); a live answer sends the object through the
/// admission worker (thiscall, `this` is the answer plus 8) whose non-zero
/// answer ends the call. Finally the squared distance of the position at
/// `obj + 0x20` from the global anchor is compared against the squared
/// global radius: inside it (strictly, ordered) the queued bit 0x80 is set
/// at `obj + 0x264`, the object runs through the emit worker (thiscall on
/// the fixed registry object) and the queue worker (cdecl, priority 1).
/// Returns 0 on the stamp path, 1 everywhere else, in the low byte only.
///
/// Original: 0x00941160 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00941160(obj: u32) -> u32 {
    unsafe {
        const MARKER: u32 = 0x219;
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
        const QUEUED: u32 = 0x264;
        const QUEUED_BIT: u32 = 0x80;
        const REGISTRY: u32 = 0x01394D60;
        const PAIR: u32 = 1;
        const POLL: u32 = 2;
        const VACANCY: u32 = 3;
        const ADMIT: u32 = 4;
        const EMIT: u32 = 5;
        const QUEUE: u32 = 6;
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
        if ((obj + MARKER) as *const u8).read() != 0 {
            return 1;
        }
        let pair: u32 = lf_checker_rt::callee_thiscall!(PAIR, u32, obj, 0u32, 0u32);
        if pair & 0xFF == 0 {
            return 1;
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
        let slot: u32 = lf_checker_rt::callee_cdecl!(VACANCY, u32,);
        if slot != 0 {
            let ok: u32 =
                lf_checker_rt::callee_thiscall!(ADMIT, u32, slot.wrapping_add(8), obj);
            if ok & 0xFF != 0 {
                return 1;
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
            let q = (obj + QUEUED) as *mut u32;
            q.write_unaligned(q.read_unaligned() | QUEUED_BIT);
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
