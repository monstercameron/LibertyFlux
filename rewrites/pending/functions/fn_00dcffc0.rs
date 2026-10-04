// original: 0x00dcffc0 ui_overview_row_add
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn ld(a: u32) -> u32 {
    *(a as *const u32)
}

#[inline(always)]
unsafe fn st(a: u32, v: u32) {
    *(a as *mut u32) = v;
}

// ---------------------------------------------------------------------------
// fn3 0x00DCFFC0: UI overview row builder (input-ui).
//
// thiscall(this, key, index, flags). Parses a descriptor string obtained
// through a virtual call into three integers (callee id 2, the format
// scanner), derives a count and a scale factor from them with single
// precision arithmetic, optionally creates a UI texture object (callees
// 4-6), inserts it into an index-addressed list owned by a child object
// unless the flags say otherwise, and links everything through virtual
// calls (ids 9-20, planted in fabricated vtables). Returns the last
// virtual call's result.
// ---------------------------------------------------------------------------
export!(thiscall, rw_b187_f3(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let _ = callee_cdecl!(1, u32, relocated(0x00EFA58C), 0);
        let vta = ld(a0);
        let get_desc: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ld(vta.wrapping_add(0x248)) as usize);
        let desc = get_desc(a0);
        let mut o1: u32 = 0;
        let mut o2: u32 = 0;
        let mut o3: u32 = 0;
        let p1 = core::ptr::addr_of_mut!(o1) as u32;
        let p2 = core::ptr::addr_of_mut!(o2) as u32;
        let p3 = core::ptr::addr_of_mut!(o3) as u32;
        // Stack order low-to-high matches the original: string, format,
        // then the three out-pointers.
        let _ = callee_cdecl!(2, u32, desc, relocated(0x00EFA59C), p3, p2, p1);
        let rate = f32::from_bits(ld(this.wrapping_add(0x1F0)))
            * *global::<f32>(0x00FE8BB0);
        let scale = *global::<f32>(0x00FE88E8) / rate;
        let k = o3.wrapping_mul(15);
        let count = o2
            .wrapping_add(k.wrapping_mul(4))
            .wrapping_mul(100)
            .wrapping_add(o1);
        let vt = ld(this);
        let sample: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(ld(vt.wrapping_add(0x8C)) as usize);
        let base = sample(this);
        let b = ld(this.wrapping_add(0x1EC));
        let amount = count as f32;
        let mixed = base * (amount * scale);
        let vb = ld(b);
        let pick: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ld(vb.wrapping_add(0x220)) as usize);
        if pick(b) != 0 {
            let vb2 = ld(b);
            let pick2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vb2.wrapping_add(0x220)) as usize);
            let w = pick2(b);
            let vw = ld(w);
            let open: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vw.wrapping_add(0x1B0)) as usize);
            let _ = open(w);
            let vb3 = ld(b);
            let pick3: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vb3.wrapping_add(0x220)) as usize);
            let w2 = pick3(b);
            let vw2 = ld(w2);
            let use_w: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(ld(vw2.wrapping_add(0x18)) as usize);
            let _ = use_w(w2, 0);
            let vb4 = ld(b);
            let close: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vb4.wrapping_add(0x1B0)) as usize);
            let _ = close(b);
        }
        let _ = callee_thiscall!(3, u32, relocated(0x011737D0), 0);
        let fresh: u32 = callee_cdecl!(4, u32, 0x1F0);
        let mut item = 0;
        if fresh != 0 {
            let vbh = ld(b);
            let helper: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vbh.wrapping_add(0x48)) as usize);
            let _helper_ret = helper(b);
            // The original pushes two filler words above the real argument
            // for this callee; the stub only takes the format word, which
            // is all the comparison sees.
            let cooked: u32 = callee_stdcall!(5, u32, relocated(0x00EFA5A8));
            let built: u32 = callee_thiscall!(6, u32, fresh, cooked);
            item = built;
        }
        if a2 & 0xFF == 0 {
            let vb5 = ld(b);
            let list: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vb5.wrapping_add(0x1D0)) as usize);
            let la = list(b);
            let cw = (la.wrapping_add(4) as *mut u16).read();
            (la.wrapping_add(4) as *mut u16).write(cw.wrapping_add(0xFFFF));
            let vb6 = ld(b);
            let list2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vb6.wrapping_add(0x1D0)) as usize);
            let lb = list2(b);
            let cnt = (lb.wrapping_add(4) as *const u16).read() as u32;
            if cnt > a1 {
                let arr = ld(lb);
                let mut edx = cnt;
                loop {
                    edx -= 1;
                    let v = ld(arr.wrapping_add(edx.wrapping_mul(4)));
                    st(arr.wrapping_add(edx.wrapping_mul(4)).wrapping_add(4), v);
                    if edx <= a1 {
                        break;
                    }
                }
            }
            let cw2 = (lb.wrapping_add(4) as *mut u16).read();
            (lb.wrapping_add(4) as *mut u16).write(cw2.wrapping_add(1));
            let vb7 = ld(b);
            let list3: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(ld(vb7.wrapping_add(0x1D0)) as usize);
            let lc = list3(b);
            st(ld(lc).wrapping_add(a1.wrapping_mul(4)), item);
        }
        let _ = callee_thiscall!(7, u32, item, a0, mixed.to_bits(), 0x42080000);
        let ui_vt = ld(item);
        let vt3 = ld(this);
        let link: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ld(vt3.wrapping_add(0x4C)) as usize);
        let linked = link(this);
        let attach: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ld(ui_vt.wrapping_add(0x17C)) as usize);
        let _ = attach(item, linked);
        let fini: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ld(ld(item).wrapping_add(0x28)) as usize);
        let _ = fini(item, 1);
        let vb8 = ld(b);
        let tail: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ld(vb8.wrapping_add(0x238)) as usize);
        let _ = tail(b);
        st(this.wrapping_add(0x1F8), ld(this.wrapping_add(0x1F8)).wrapping_add(1));
        let _ = callee_thiscall!(8, u32, this);
        let vb9 = ld(b);
        let tail2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ld(vb9.wrapping_add(0x238)) as usize);
        tail2(b)
    }
});
