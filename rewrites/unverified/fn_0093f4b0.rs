// original: 0x0093f4b0 stream_nodes_in_box (proposed)

/// Call `func` for each unvisited list node whose point is inside `box`.
///
/// Walks the node list at `list` (each node: object at +0, next at +4).
/// A node whose object stamp at +0x3C already equals the epoch at `EPOCH`
/// is skipped; otherwise it is stamped, its point is fetched through its
/// virtual slot `POINT_SLOT` into scratch, and `func(obj, ctx)` runs when
/// the point is strictly inside the box (`lo` at +0/+4/+8, `hi` at
/// +0x10/+0x14/+0x18). A zero answer stops the walk with 0, exhaustion
/// returns 1. Comparisons are strict greater-than, so NaN never passes.
/// Only `al` is set, so only `al` is compared.
///
/// Original: 0x0093f4b0 (cdecl, four stack words; one virtual + one
/// register callee).
lf_checker_rt::export!(cdecl, rw_0093f4b0(list: u32, bx: u32, func: u32, ctx: u32) -> u32 {
    const EPOCH: u32 = 0x11A8908;
    const OBJ_STAMP: u32 = 0x3C;
    const POINT_SLOT: u32 = 0x50;
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
                ((obj + OBJ_STAMP) as *mut u32).write_unaligned(gen);
                let point: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + POINT_SLOT) as *const u32)
                        .read_unaligned()) as usize,
                );
                let mut frame = [0u32; 3];
                point(obj, frame.as_mut_ptr() as u32);
                let p = [
                    f32::from_bits(core::hint::black_box(frame[0])),
                    f32::from_bits(core::hint::black_box(frame[1])),
                    f32::from_bits(core::hint::black_box(frame[2])),
                ];
                let inside = core::hint::black_box(p[0]) > core::hint::black_box(blo[0])
                    && core::hint::black_box(p[1]) > core::hint::black_box(blo[1])
                    && core::hint::black_box(p[2]) > core::hint::black_box(blo[2])
                    && core::hint::black_box(bhi[0]) > core::hint::black_box(p[0])
                    && core::hint::black_box(bhi[1]) > core::hint::black_box(p[1])
                    && core::hint::black_box(bhi[2]) > core::hint::black_box(p[2]);
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
