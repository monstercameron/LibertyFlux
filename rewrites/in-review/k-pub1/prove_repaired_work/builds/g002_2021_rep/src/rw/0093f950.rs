// original: 0x0093f950 stream_nodes_in_sphere (proposed)

/// Call `func` for each unvisited node within range of the box anchor.
///
/// Walks the node list at `list` (object at +0, next at +4), skipping
/// nodes already stamped with the epoch at `EPOCH` and stamping the rest.
/// Each node's anchor triplet comes from virtual slot `ANCHOR_SLOT` (its
/// scratch argument is never read back) and its range from slot
/// `RANGE_SLOT` (an x87 single, read here through the callee's integer
/// channel, which carries the same bits). With `d = box[0..3] - anchor`
/// and `s = range + box[4]`, `func(obj, ctx)` runs when `s*s` strictly
/// exceeds `d0*d0 + d1*d1 + d2*d2`, summed as `(d1*d1 + d0*d0) + d2*d2`
/// exactly as the original accumulates it. A zero answer stops the walk
/// with 0, exhaustion returns 1. Only `al` is set.
///
/// Original: 0x0093f950 (cdecl, four stack words; two virtual + one
/// register callee).
lf_checker_rt::export!(cdecl, rw_0093f950(list: u32, bx: u32, func: u32, ctx: u32) -> u32 {
    const EPOCH: u32 = 0x11A8908;
    const OBJ_STAMP: u32 = 0x3C;
    const ANCHOR_SLOT: u32 = 0x54;
    const RANGE_SLOT: u32 = 0x58;
    unsafe {
        let mut node = (list as *const u32).read_unaligned();
        if node == 0 {
            return 1;
        }
        let gen = (lf_checker_rt::global::<u16>(EPOCH) as *const u16).read_unaligned() as u32;
        let anchor_box = [
            (bx as *const f32).read_unaligned(),
            ((bx + 4) as *const f32).read_unaligned(),
            ((bx + 8) as *const f32).read_unaligned(),
        ];
        let reach = ((bx + 0x10) as *const f32).read_unaligned();
        loop {
            let obj = (node as *const u32).read_unaligned();
            node = ((node + 4) as *const u32).read_unaligned();
            if ((obj + OBJ_STAMP) as *const u32).read_unaligned() != gen {
                ((obj + OBJ_STAMP) as *mut u32).write_unaligned(gen);
                let anchor: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + ANCHOR_SLOT) as *const u32)
                        .read_unaligned()) as usize,
                );
                let mut scratch = [0u32; 4];
                let p = anchor(obj, scratch.as_mut_ptr() as u32);
                let bb = core::hint::black_box;
                let d0 = bb(anchor_box[0]) - bb((p as *const f32).read_unaligned());
                let d1 = bb(anchor_box[1]) - bb(((p + 4) as *const f32).read_unaligned());
                let d2 = bb(anchor_box[2]) - bb(((p + 8) as *const f32).read_unaligned());
                let range: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    (((((obj as *const u32).read_unaligned()) + RANGE_SLOT) as *const u32)
                        .read_unaligned()) as usize,
                );
                let r = f32::from_bits(core::hint::black_box(range(obj)));
                let s = bb(r) + bb(reach);
                let s2 = bb(s) * bb(s);
                let sum = (bb(bb(d1) * bb(d1)) + bb(bb(d0) * bb(d0))) + bb(bb(d2) * bb(d2));
                if bb(s2) > bb(sum) {
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
