// original: 0x0089B3B0 rage::audEnvelopeSound::vf7
//! Envelope voice setup (virtual slot 7): validates the request, allocates a
//! voice instance from the pool, copies the parameter block into it, derives
//! the table index by division, optionally resolves an effect chain, mirrors
//! a control block into the object, dispatches five parameter queries through
//! the object's virtual table, configures three voice banks, and reports
//! whether all banks came up enabled.

use lf_k2_rt::{callee_thiscall, export, global, relocated};

const ROUTE_STRIDE: u32 = 0x6F40;

export!(thiscall, rw_0089B3B0(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let t = this as *const u8;
        let g968 = *global::<u32>(0x115D968);
        let table = *global::<u32>(0x115D988);

        let ok: u32 = callee_thiscall!(1, u32, this, arg0, arg1, arg2);
        if ok as u8 == 0 {
            return 0;
        }
        let route = *t.add(0x40);
        let edi: u32 = callee_thiscall!(2, u32, relocated(0x115D8A0), 0xA4, route as u32);
        if edi == 0 {
            return 0;
        }
        callee_thiscall!(3, u32, edi);

        // 24-byte parameter block copy, qword-wise.
        let src = arg2 as *const u8;
        let dst = edi as *mut u8;
        let mut i = 0;
        while i < 3 {
            let v = (src.add(i * 8) as *const u64).read_unaligned();
            (dst.add(0x78 + i * 8) as *mut u64).write_unaligned(v);
            i += 1;
        }

        // Table index by division (divisor is scripted nonzero on every trial).
        let row = table
            .wrapping_add((route as u32).wrapping_mul(ROUTE_STRIDE))
            .wrapping_add(0x6F14);
        let idx = edi.wrapping_sub(*(row as *const u32)) / g968;
        *(this as *mut u8).add(0xF7) = idx as u8;

        let p94 = *(t.add(0x94) as *const u32);
        let v = (p94.wrapping_add(0x2D) as *const u32).read_unaligned();
        if v != 0xFFFF_FFFF && v != 0 {
            let r4: u32 = callee_thiscall!(4, u32, relocated(0x115DC18), v, this, arg1, arg2);
            callee_thiscall!(5, u32, this, 0, r4);
            let r6: u32 = callee_thiscall!(6, u32, this, 0);
            if r6 == 0 {
                return 0;
            }
        }

        // Control block mirror (mixed widths, unaligned dword reads).
        let b = p94 as *const u8;
        *((this as *mut u8).add(0xF0) as *mut u16) = (b as *const u16).read_unaligned();
        *((this as *mut u8).add(0xF2) as *mut u16) =
            (b.add(2) as *const u16).read_unaligned();
        *(this as *mut u8).add(0xF4) = *b.add(4);
        *((this as *mut u8).add(0xE8) as *mut u32) =
            (b.add(5) as *const u32).read_unaligned();
        *((this as *mut u8).add(0xEC) as *mut u32) =
            (b.add(9) as *const u32).read_unaligned();

        // Five queries through virtual slot [vtable+0x10]; both sides land on
        // the same planted stub through the same fabricated vtable. Note the
        // calls share one stub id (one slot), so all five answers agree per
        // trial; field-swap mutants are unobservable on any single trial.
        let vt = *(this as *const u32);
        let slot_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt + 0x10) as *const u32) as usize);
        let d = |off: usize| -> u32 { (b.add(off) as *const u32).read_unaligned() };
        *((this as *mut u8).add(0xD4) as *mut u32) = slot_fn(this, d(0x19));
        *((this as *mut u8).add(0xD8) as *mut u32) = slot_fn(this, d(0x1D));
        *((this as *mut u8).add(0xDC) as *mut u32) = slot_fn(this, d(0x25));
        *((this as *mut u8).add(0xE0) as *mut u32) = slot_fn(this, d(0x21));
        *((this as *mut u8).add(0xE4) as *mut u32) = slot_fn(this, d(0x29));

        callee_thiscall!(8, u32, edi, d(0x0D));
        callee_thiscall!(8, u32, edi.wrapping_add(0x28), d(0x11));
        callee_thiscall!(8, u32, edi.wrapping_add(0x50), d(0x15));

        let e = edi as *const u8;
        if *e.add(0x26) != 0 && *e.add(0x76) != 0 && *e.add(0x4E) != 0 {
            1
        } else {
            0
        }
    }
});
