// original: 0x009407c0 streaming_request_check_a (proposed)

/// Vet the streaming request and queue it when it is inside the radius.
///
/// Returns 1 without doing anything unless every gate passes: the marker
/// byte at `obj + 0x219` is clear, the virtual probe at slot `+0x34`
/// answers non-zero (thiscall), and the linked record's state word reads
/// 2. Then the vacancy worker runs (cdecl, no arguments); a live answer
/// sends the object through the admission worker (thiscall, `this` is the
/// answer plus 8) whose non-zero answer also ends the call. Finally the
/// squared distance of the position at `obj + 0x20` from the global anchor
/// is compared against the squared global radius: inside it (strictly,
/// ordered) the queued bit 0x80 is set at `obj + 0x264` and the object is
/// queued with priority 1 (cdecl, two arguments). Always returns 1 in the
/// low byte.
///
/// Original: 0x009407c0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_009407c0(obj: u32) -> u32 {
    unsafe {
        const MARKER: u32 = 0x219;
        const VT_PROBE: u32 = 0x34;
        const LINK: u32 = 0x21C;
        const LINK_STATE: u32 = 0x12C;
        const READY_STATE: u32 = 2;
        const POS: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const ANCHOR_X: u32 = 0x011D4E40;
        const ANCHOR_Y: u32 = 0x011D4E44;
        const RADIUS: u32 = 0x011A4FB0;
        const QUEUED: u32 = 0x264;
        const QUEUED_BIT: u32 = 0x80;
        const PROBE: u32 = 1;
        const VACANCY: u32 = 2;
        const ADMIT: u32 = 3;
        const QUEUE: u32 = 4;
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
        let vt = (obj as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((vt + VT_PROBE) as *const u32).read_unaligned() as usize,
        );
        if probe(obj) & 0xFF == 0 {
            return 1;
        }
        let link = ((obj + LINK) as *const u32).read_unaligned();
        if ((link + LINK_STATE) as *const u32).read_unaligned() != READY_STATE {
            return 1;
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
            let _: u32 = lf_checker_rt::callee_cdecl!(QUEUE, u32, obj, 1u32);
        }
        1
    }
});
