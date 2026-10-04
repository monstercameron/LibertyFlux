// original: 0x0089BF00 rage::audTwinLoopSound::vf7
//! Twin loop setup (virtual slot 7): validates the request, stages the
//! parameter block on the stack, resolves two loop voices through division,
//! mirrors four levels into the object, dispatches four parameter queries
//! through the object's virtual table, brings up the loop engine, lazily
//! initializes two shared tables once per process, and selects the active
//! loop by key match.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

const ROUTE_STRIDE: u32 = 0x6F40;

export!(thiscall, rw_0089BF00(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let t = this as *const u8;
        let g964 = *global::<u32>(0x115D964);
        let table = *global::<u32>(0x115D988);

        let ok: u32 = callee_thiscall!(1, u32, this, arg0, arg1, arg2);
        if ok as u8 == 0 {
            return 0;
        }
        let b = *(t.add(0x94) as *const u32) as *const u8;
        // 24-byte parameter staging, qword-wise (address passed to callee 3).
        let mut stage = [0u64; 3];
        let src = arg2 as *const u8;
        let mut i = 0;
        while i < 3 {
            stage[i] = (src.add(i * 8) as *const u64).read_unaligned();
            i += 1;
        }
        let d = |off: usize| -> u32 { (b.add(off) as *const u32).read_unaligned() };

        let r2a: u32 = callee_thiscall!(2, u32, relocated(0x115DC18), d(0x1D), this, arg1, arg2);
        let row = table
            .wrapping_add((*t.add(0x40) as u32).wrapping_mul(ROUTE_STRIDE))
            .wrapping_add(0x6F10);
        let rowval = *(row as *const u32);
        // Divisors are scripted nonzero on every trial; see the contract.
        let slot_a = if r2a == 0 {
            0xFF
        } else {
            (r2a.wrapping_sub(rowval) / g964) as u8
        };
        *(this as *mut u8).add(0x48) = slot_a;
        let r2b: u32 = callee_thiscall!(
            3, u32, relocated(0x115DC18), d(0x25), this, arg1, stage.as_ptr() as u32
        );
        let slot_b = if r2b == 0 {
            0xFF
        } else {
            (r2b.wrapping_sub(rowval) / g964) as u8
        };
        *(this as *mut u8).add(0x49) = slot_b;

        if slot_a == 0xFF {
            return 0;
        }
        if g964.wrapping_mul(slot_a as u32).wrapping_add(rowval) == 0 {
            return 0;
        }
        if slot_b == 0xFF {
            return 0;
        }
        if g964.wrapping_mul(slot_b as u32).wrapping_add(rowval) == 0 {
            return 0;
        }

        let w16 = |off: usize| -> u16 { (b.add(off) as *const u16).read_unaligned() };
        *((this as *mut u8).add(0xB4) as *mut u32) = w16(4) as u32;
        *((this as *mut u8).add(0xB8) as *mut u32) = w16(6) as u32;
        *((this as *mut u8).add(0xBC) as *mut u32) = w16(0) as u32;
        *((this as *mut u8).add(0xC0) as *mut u32) = w16(2) as u32;

        // Four queries through virtual slot [vtable+0x10]; one stub id (one
        // slot), so all four answers agree per trial.
        let vt = *(this as *const u32);
        let slot_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt + 0x10) as *const u32) as usize);
        *((this as *mut u8).add(0xCC) as *mut u32) = slot_fn(this, d(0x0C));
        *((this as *mut u8).add(0xD0) as *mut u32) = slot_fn(this, d(0x10));
        *((this as *mut u8).add(0xC4) as *mut u32) = slot_fn(this, d(0x14));
        let c4 = slot_fn(this, d(0x18));

        let one = 1.0f32.to_bits();
        callee_thiscall!(5, u32, this.wrapping_add(0xDC), one, one, 0, one);
        *((this as *mut u8).add(0xC8) as *mut u32) = c4;
        let m: f32 = callee_thiscall!(6, f32, this);
        callee_thiscall!(7, u32, this.wrapping_add(0xDC), m.to_bits(), m.to_bits());

        // Lazily initialized shared tables, then loop selection by key.
        let f_init = *global::<u32>(0x115F81C);
        if f_init & 1 == 0 {
            *global::<u32>(0x115F81C) = f_init | 1;
            let r: u32 = callee_cdecl!(8, u32, relocated(0xE79CD0), 0);
            *global::<u32>(0x115F818) = r;
        }
        let f_mid = *global::<u32>(0x115F81C);
        if f_mid & 2 == 0 {
            *global::<u32>(0x115F81C) = f_mid | 2;
            let r: u32 = callee_cdecl!(9, u32, relocated(0xE79CE4), 0);
            *global::<u32>(0x115F820) = r;
        }
        let key = d(8);
        if key == *global::<u32>(0x115F818) {
            *((this as *mut u8).add(0xD8) as *mut u32) = 0;
            return 1;
        }
        if key == *global::<u32>(0x115F820) {
            *((this as *mut u8).add(0xD8) as *mut u32) = 1;
            return 1;
        }
        0
    }
});
