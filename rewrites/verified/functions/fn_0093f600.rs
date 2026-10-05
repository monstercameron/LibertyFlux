// original: 0x0093f600 stream_nodes_overlap_box (proposed)

/// Call `func` for each unvisited list node whose span overlaps `box`.
///
/// Walks the node list at `list` (each node: object at +0, next at +4).
/// A node whose object stamp at +0x3C already equals the epoch at `EPOCH`
/// is skipped (unlike its sibling, the stamp is never written here); for
/// the rest a six-word span is fetched through virtual slot `SPAN_SLOT`
/// into scratch and `func(obj, ctx)` runs when the span's first triplet
/// is below the box `hi` (+0x10/+0x14/+0x18) and the box `lo` (+0/+4/+8)
/// is below its second triplet. The original's above-or-equal exits make
/// an unordered (NaN) comparison count as below, which `!(a >= b)`
/// reproduces exactly. A zero answer stops the walk with 0, exhaustion
/// returns 1. Only `al` is set, so only `al` is compared.
///
/// Original: 0x0093f600 (cdecl, four stack words; one virtual + one
/// register callee).
lf_checker_rt::export!(cdecl, rw_0093f600(list: u32, bx: u32, func: u32, ctx: u32) -> u32 {
    const EPOCH: u32 = 0x11A8908;
    const OBJ_STAMP: u32 = 0x3C;
    const SPAN_SLOT: u32 = 0x6C;
    unsafe {
        let mut node = (list as *const u32).read_unaligned();
        if node == 0 {
            return 1;
        }
        let gen = (lf_checker_rt::global::<u16>(EPOCH) as *const u16).read_unaligned() as u32;
        let blo = [
            (bx as *const f32).read_unaligned(),
            ((bx + 4) as *const f32).read_unaligned(),
            ((bx + 8) as *const f32).read_unaligned(),
        ];
        let bhi = [
            ((bx + 0x10) as *const f32).read_unaligned(),
            ((bx + 0x14) as *const f32).read_unaligned(),
            ((bx + 0x18) as *const f32).read_unaligned(),
        ];
        loop {
            let obj = (node as *const u32).read_unaligned();
            node = ((node + 4) as *const u32).read_unaligned();
            if ((obj + OBJ_STAMP) as *const u32).read_unaligned() != gen {
                let span: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + SPAN_SLOT) as *const u32)
                        .read_unaligned()) as usize,
                );
                let mut frame = [0u32; 7];
                span(obj, frame.as_mut_ptr() as u32);
                let a = [
                    f32::from_bits(core::hint::black_box(frame[0])),
                    f32::from_bits(core::hint::black_box(frame[1])),
                    f32::from_bits(core::hint::black_box(frame[2])),
                ];
                let b = [
                    f32::from_bits(core::hint::black_box(frame[4])),
                    f32::from_bits(core::hint::black_box(frame[5])),
                    f32::from_bits(core::hint::black_box(frame[6])),
                ];
                let inside = !(core::hint::black_box(a[0]) >= core::hint::black_box(bhi[0]))
                    && !(core::hint::black_box(a[1]) >= core::hint::black_box(bhi[1]))
                    && !(core::hint::black_box(a[2]) >= core::hint::black_box(bhi[2]))
                    && !(core::hint::black_box(blo[0]) >= core::hint::black_box(b[0]))
                    && !(core::hint::black_box(blo[1]) >= core::hint::black_box(b[1]))
                    && !(core::hint::black_box(blo[2]) >= core::hint::black_box(b[2]));
                if inside {
                    let f: extern "cdecl" fn(u32, u32) -> u32 =
                        core::mem::transmute(func as usize);
                    if (f(obj, ctx) & 0xFF) == 0 {
                        return 0;
                    }
                }
            }
            if node == 0 {
                break;
            }
        }
        1
    }
});
