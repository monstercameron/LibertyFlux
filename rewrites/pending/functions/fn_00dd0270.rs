// original: 0x00dd0270 ui_overview_build
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn ld(a: u32) -> u32 {
    *(a as *const u32)
}

#[inline(always)]
unsafe fn st(a: u32, v: u32) {
    *(a as *mut u32) = v;
}

/// Call a planted virtual slot with 0-3 stack arguments (C order, matching
/// the on-stack low-to-high layout the original produces).
#[inline(always)]
unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
    let f: extern "thiscall" fn(u32) -> u32 =
        core::mem::transmute(ld(ld(obj).wrapping_add(slot)) as usize);
    f(obj)
}

#[inline(always)]
unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
    let f: extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(ld(ld(obj).wrapping_add(slot)) as usize);
    f(obj, a0)
}

#[inline(always)]
unsafe fn vcall2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
        core::mem::transmute(ld(ld(obj).wrapping_add(slot)) as usize);
    f(obj, a0, a1)
}

#[inline(always)]
unsafe fn vcall3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        core::mem::transmute(ld(ld(obj).wrapping_add(slot)) as usize);
    f(obj, a0, a1, a2)
}

/// The repeated apply-attributes block: build a 24-byte parameter struct
/// through callee 7, pass its six words with a flag word to slot 0x114,
/// then run the destructor stub (callee 9).
#[inline(always)]
unsafe fn attr_block(obj: u32, c0: u32, c1: u32, flags: u32) {
    let s: u32 = callee_stdcall!(7, u32, c0, c1);
    let w0 = ld(s);
    let w1 = ld(s.wrapping_add(4));
    let w2 = ld(s.wrapping_add(8));
    let w3 = ld(s.wrapping_add(12));
    let w4 = ld(s.wrapping_add(16));
    let w5 = ld(s.wrapping_add(20));
    let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(ld(ld(obj).wrapping_add(0x114)) as usize);
    let _ = f(obj, flags, w0, w1, w2, w3, w4, w5);
    let _ = callee_stdcall!(9, u32,);
}

// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// fn4 0x00DD0270: overview screen builder (input-ui; inventory name
// RP_OVERVIEW).
//
// thiscall(this). Unless a virtual readiness check says the screen is
// already up, allocates and constructs four child widgets (two textures,
// a font string and a row layout; callees 2, 4, 12, 23), initialises two
// of them (callees 5, 6), applies repeated attribute blocks to three
// (callee 7 builds the parameter structs, slot 0x114/0x110 consumes them,
// callee 9 destroys them), configures the font string from HUD colours
// and the display aspect ratio (globals), and finishes through a tail
// helper and a final virtual call whose result is returned. All virtual
// callees (ids 1, 3, 8, 10, 11, 14-19, 21, 22, 24-27, 29) are planted in
// fabricated vtables.
// ---------------------------------------------------------------------------
export!(thiscall, rw_b187_f4(this: u32) -> u32 {
    unsafe {
        let ready = vcall0(this, 0x140);
        if ready & 0xFF != 0 {
            return ready;
        }
        // First texture widget.
        let n1: u32 = callee_cdecl!(2, u32, 0x25C);
        let t1: u32;
        if n1 != 0 {
            let hr = vcall0(this, 0x48);
            t1 = callee_thiscall!(4, u32, n1, relocated(0x00EFA428), hr);
        } else {
            t1 = 0;
        }
        let mut scratch1: u32 = 0;
        let pscratch1 = core::ptr::addr_of_mut!(scratch1) as u32;
        let _ = callee_thiscall!(5, u32, t1, 0, 0, pscratch1, 0xFFFFFFFF);
        st(this.wrapping_add(0x1E0), t1);
        core::hint::black_box(ld(t1));
        attr_block(t1, 0, 0, 4);
        attr_block(t1, 0, 0, 2);
        attr_block(t1, 0, 0, 8);
        attr_block(t1, 0, 0, 0x10);
        let linked = vcall0(this, 0x4C);
        let _ = vcall1(t1, 0x170, linked);
        // Font string widget.
        let n2: u32 = callee_cdecl!(2, u32, 0x610);
        let t2: u32;
        if n2 != 0 {
            let hr = vcall0(this, 0x48);
            t2 = callee_thiscall!(12, u32, n2, relocated(0x00EFA540), hr);
        } else {
            t2 = 0;
        }
        st(this.wrapping_add(0x1E8), t2);
        core::hint::black_box(ld(t2));
        let mut hud_scratch: u32 = 0;
        let phud = core::ptr::addr_of_mut!(hud_scratch) as u32;
        // The original pushes filler words the stub never takes; only the
        // frame pointer is passed, and its address is skipped.
        let hud: u32 = callee_stdcall!(13, u32, phud);
        // The original leaves one stale word below this call's arguments;
        // the callee only takes the two visible words.
        let _ = vcall2(t2, 0x1CC, 0x41900000, hud);
        let mut colour: u32 = 0xFF797979;
        let pcolour = core::ptr::addr_of_mut!(colour) as u32;
        let _ = vcall1(t2, 0x208, pcolour);
        let _ = vcall2(t2, 0x1E0, relocated(0x00EFA550), 0);
        let _ = vcall1(t2, 0x1FC, 0);
        let _ = vcall1(t2, 0x1F8, 0);
        let _ = vcall1(t2, 0x1D4, 0x12);
        // Aspect-ratio dependent adjustment.
        let mode0 = *global::<u8>(0x0116C250);
        let mode1 = *global::<u8>(0x0116C253);
        if mode0 == 0x6A || mode1 != 0 {
            let r1: u32 = callee_cdecl!(20, u32,);
            let mut esel = *global::<u32>(0x0105C884);
            if r1 & 0xFF != 0 {
                esel = *global::<u32>(0x0105C888);
            }
            let r2: u32 = callee_cdecl!(20, u32,);
            let mut csel = *global::<u32>(0x0105C880);
            if r2 & 0xFF != 0 {
                csel = *global::<u32>(0x0105C87C);
            }
            let ratio = (esel as i32 as f32) / (csel as i32 as f32);
            let d = (ratio - *global::<f32>(0x00FE8920)).abs();
            if d < *global::<f32>(0x00FE870C) {
                let fobj = vcall0(t2, 0x210);
                let fx1 = f32::from_bits(ld(fobj.wrapping_add(4)));
                let fx2 = f32::from_bits(ld(fobj));
                let c = *global::<f32>(0x00FE88BC);
                // Stack order low-to-high is xmm2, xmm1, then the pushed
                // zero: the push comes before the fills.
                let _ = vcall3(t2, 0x1DC, (fx2 * c).to_bits(), (fx1 * c).to_bits(), 0);
            }
        }
        // Row layout widget.
        let n3: u32 = callee_cdecl!(2, u32, 0x1F8);
        let t3: u32;
        if n3 != 0 {
            let hr = vcall0(this, 0x48);
            t3 = callee_thiscall!(23, u32, n3, relocated(0x00EFA55C), hr);
        } else {
            t3 = 0;
        }
        st(this.wrapping_add(0x1EC), t3);
        let _ = vcall3(t3, 0x1FC, 2, 0, 0);
        attr_block(t3, 0, 0x3F800000, 4);
        attr_block(t3, 0, 0, 2);
        attr_block(t3, 0xC0000000, 0, 8);
        attr_block(t3, 0, 0xBF800000, 0x10);
        // Second texture widget.
        let n4: u32 = callee_cdecl!(2, u32, 0x25C);
        let t4: u32;
        if n4 != 0 {
            let hr = vcall0(this, 0x48);
            t4 = callee_thiscall!(4, u32, n4, relocated(0x00EFA568), hr);
        } else {
            t4 = 0;
        }
        st(this.wrapping_add(0x1E4), t4);
        let mut hud_scratch2: u32 = 0;
        let phud2 = core::ptr::addr_of_mut!(hud_scratch2) as u32;
        let hud2: u32 = callee_stdcall!(13, u32, phud2);
        let _ = callee_thiscall!(6, u32, t4, 0, 0, hud2, 0);
        core::hint::black_box(ld(t4));
        attr_block(t4, 0, 0x40C00000, 4);
        attr_block(t4, 0, 0xC0C00000, 0x10);
        let s: u32 = callee_stdcall!(7, u32, 0, 0);
        let w0 = ld(s);
        let w1 = ld(s.wrapping_add(4));
        let w2 = ld(s.wrapping_add(8));
        let w3 = ld(s.wrapping_add(12));
        let w4 = ld(s.wrapping_add(16));
        let w5 = ld(s.wrapping_add(20));
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(ld(ld(t4).wrapping_add(0x110)) as usize);
        let _ = f(t4, 2, 8, w0, w1, w2, w3, w4, w5);
        let _ = callee_stdcall!(9, u32,);
        st(t4.wrapping_add(0x1D8), 0);
        let _ = vcall1(t4, 0x94, 0x40400000);
        let _ = vcall1(t4, 0x120, 0);
        let tailr: u32 = callee_thiscall!(28, u32, relocated(0x01981A4C), relocated(0x00EFA57C));
        (this.wrapping_add(0x1F4) as *mut u8).write(1);
        st(this.wrapping_add(0x1F8), 0);
        st(this.wrapping_add(0x1F0), 0);
        st(this.wrapping_add(0x1BC), tailr.wrapping_add(0x1F8));
        vcall1(this, 0x13C, 1)
    }
});
