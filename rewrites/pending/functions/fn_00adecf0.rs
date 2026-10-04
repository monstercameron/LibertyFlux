// original: 0x00adecf0 publish_render_state
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Publish one render-state block and fill its slots (original 0x00ADECF0).
///
/// Stores the two header words into the block, shifts one indexed entry down,
/// publishes the block pointer plus three derived row pointers and two
/// scalar parameters into the shared state table, queries the current view
/// through callee 1, runs the two setup calls (callees 2 and 3, whose pointer
/// arguments address the caller's own frame and are skipped by the contract),
/// lazily creates the shared slot handle on first use (callee 4), appends one
/// filled slot per requested count (callee 5) bumping the block's fill cursor,
/// then walks the live-object table stamping each row. Takes the cookie registry
/// value into the security check like the original (the stack words cancel, so
/// the checked value is the registry word itself). Returns nothing meaningful.
export!(thiscall, rw_00adecf0(this: u32, head0: u32, head1: u32, count: u32) -> u32 {
    const STATE: u32 = 0x1593B20;
    const VIEW_DEPTH_OFF: u32 = 0xE98;
    const VT_SLOT: u32 = 0x24;
    const COOKIE_REG: u32 = 0x1057FB4;
    unsafe {
        let rd = |off: u32| (this.wrapping_add(off) as *const u32).read();
        let wr = |off: u32, v: u32| (this.wrapping_add(off) as *mut u32).write(v);
        let g = |va: u32| global::<u32>(va);
        let gr = |va: u32| (g(va) as *const u32).read();
        let gw = |va: u32, v: u32| g(va).write(v);
        let row = rd(0xC);
        wr(4, head0);
        wr(8, head1);
        wr(row.wrapping_mul(4).wrapping_add(0x14), rd(row.wrapping_mul(4).wrapping_add(0x1C)));
        gw(STATE + 0xC, this.wrapping_add(row.wrapping_add(5).wrapping_mul(4)));
        wr(0, head0);
        let tint = (global::<[u32; 4]>(0x15C15E0) as *const [u32; 4]).read();
        gw(STATE + 0x4, this);
        gw(STATE + 0x8, head1);
        gw(STATE + 0x10, this.wrapping_add(row.wrapping_add(9).wrapping_mul(4)));
        (global::<[u32; 4]>(STATE + 0x20) as *mut [u32; 4]).write(tint);
        gw(STATE + 0x30, gr(0x15B0E68));
        gw(STATE + 0x14, this.wrapping_add(row.wrapping_add(0xB).wrapping_mul(4)));
        gw(STATE + 0x34, gr(0x103F6C0));
        let view = callee_thiscall!(1, u32, this);
        gw(STATE + 0x38, (view.wrapping_add(VIEW_DEPTH_OFF) as *const u32).read());
        gw(0x179764C, relocated(STATE));
        // Frame scratch for the two setup calls; the contract skips these
        // addresses (frame layouts differ) and compares the zeroed contents.
        let frame = [0u32; 4];
        let frame_at = frame.as_ptr() as u32;
        callee_thiscall!(2, u32, frame_at);
        callee_cdecl!(3, u32, frame_at, 0, 0x94);
        if gr(0x103F68C) == 0xFFFF_FFFF {
            let handle = callee_cdecl!(4, u32, relocated(0xEA716C), 0x8000, 0x2000, 1, 7, 1);
            gw(0x103F68C, handle);
        }
        if (count as i32) > 0 {
            for _ in 0..(count as i32) {
                let slot_arg = gr(0x103F68C);
                let got = callee_cdecl!(
                    5, u32, relocated(0xEA7184), relocated(0xD68FB0), frame_at, slot_arg
                );
                let at = rd(0x74);
                (at as *mut u32).write(got);
                wr(0x74, at.wrapping_add(4));
            }
        }
        let n = rd(0xC);
        wr(n.wrapping_mul(4).wrapping_add(0x78), gr(0x118D7E0));
        let mut i = 0u32;
        while (gr(0x118D7E0) as i32) > (i as i32) {
            let obj = (relocated(0x118D760).wrapping_add(i.wrapping_mul(4)) as *const u32).read();
            let vt = (obj as *const u32).read();
            let tgt = (vt.wrapping_add(VT_SLOT) as *const u32).read();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
            let r = f(obj);
            let n2 = rd(0xC);
            wr(i.wrapping_add(n2.wrapping_mul(24)).wrapping_mul(4).wrapping_add(0x80), r);
            i = i.wrapping_add(1);
        }
        callee_thiscall!(7, u32, gr(COOKIE_REG));
    }
    0
});
