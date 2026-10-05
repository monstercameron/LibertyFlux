// original: 0x00ca5fe0 CEventHandler::vf39
// ---------------------------------------------------------------------------
// fn5 0x00CA5FE0 CEventHandler::vf39: event switch with twin build paths.
// 0x1ab builds a small result through the factory; 0xc8 clears it; 0x38f
// and 0x76c share a two-stage shape (build a product, tag it, run the tail
// call, then unless a result is already present verify a predicate and run
// a three-call chain that stores a second result). Anything else returns.
// Several callees take donated stack slots from earlier calls; those values
// are passed explicitly here.
// ---------------------------------------------------------------------------
use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

#[inline(always)]
unsafe fn load_u32(addr: u32) -> u32 {
    *(addr as *const u32)
}

#[inline(always)]
unsafe fn store_u32(addr: u32, v: u32) {
    *(addr as *mut u32) = v;
}

export!(thiscall, rw_b08_f5(this: u32, a0: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        let event = load_u32(a0.wrapping_add(0x10));
        if event > 0x38f {
            if event != 0x76c {
                return 0;
            }
            let f = callee_thiscall!(4, u32, *global::<u32>(0x167e2a0),);
            let r = if f == 0 {
                0
            } else {
                let x = callee_cdecl!(10, u32, 0);
                callee_thiscall!(14, u32, f, x, 0)
            };
            store_u32(this.wrapping_add(0xc), r);
            let gate = load_u32(load_u32(this.wrapping_add(4)).wrapping_add(0x224));
            let y = callee_cdecl!(11, u32, 0);
            callee_thiscall!(16, u32, gate, y, 1);
            if load_u32(this.wrapping_add(8)) != 0 {
                return 0;
            }
            if (callee_cdecl!(18, u32, load_u32(this.wrapping_add(4)), a0) & 0xff) == 0 {
                return 0;
            }
            let f2 = callee_thiscall!(5, u32, *global::<u32>(0x167e2a0),);
            if f2 == 0 {
                store_u32(this.wrapping_add(8), 0);
                return 0;
            }
            let z = callee_cdecl!(12, u32, 0);
            let w = callee_cdecl!(20, u32, load_u32(this.wrapping_add(4)));
            let r2 = callee_thiscall!(22, u32, f2, 0, 1, w, z, 0, 0);
            store_u32(this.wrapping_add(8), r2);
            return 0;
        }
        if event == 0x38f {
            let f = callee_thiscall!(3, u32, *global::<u32>(0x167e2a0),);
            let r = if f == 0 {
                0
            } else {
                let x = callee_cdecl!(7, u32, 0);
                callee_thiscall!(13, u32, f, x, 0)
            };
            store_u32(this.wrapping_add(0xc), r);
            *(r.wrapping_add(0x60) as *mut u32) |= 8;
            let gate = load_u32(load_u32(this.wrapping_add(4)).wrapping_add(0x224));
            let y = callee_cdecl!(8, u32, 0);
            callee_thiscall!(15, u32, gate, y, 1);
            if load_u32(this.wrapping_add(8)) != 0 {
                return 0;
            }
            if (callee_cdecl!(17, u32, load_u32(this.wrapping_add(4)), a0) & 0xff) == 0 {
                return 0;
            }
            let f2 = callee_thiscall!(6, u32, *global::<u32>(0x167e2a0),);
            if f2 == 0 {
                store_u32(this.wrapping_add(8), 0);
                return 0;
            }
            let z = callee_cdecl!(9, u32, 0);
            let w = callee_cdecl!(19, u32, load_u32(this.wrapping_add(4)));
            let r2 = callee_thiscall!(21, u32, f2, 1, 1, w, z, 0, 0);
            store_u32(this.wrapping_add(8), r2);
            return 0;
        }
        if event == 0xc8 {
            store_u32(this.wrapping_add(0xc), 0);
            return 0;
        }
        if event != 0x1ab {
            return 0;
        }
        let f = callee_thiscall!(1, u32, *global::<u32>(0x167e2a0),);
        if f == 0 {
            store_u32(this.wrapping_add(0xc), 0);
            return 0;
        }
        store_u32(
            this.wrapping_add(0xc),
            callee_thiscall!(2, u32, f, 0, 0x5f5e0ff, 0xffffffff),
        );
        0
    }
});
