//! `lf-checker-proofs`: the checker's proof rewrites (its regression test).
//!
//! Usage: build the 32-bit DLL from the repository root with
//! `RUSTFLAGS="-C panic=abort" cargo build --release --target
//! i686-pc-windows-msvc -p lf-checker-proofs` (a panic must become a fault,
//! never an unwind into the worker), then run
//! `scripts/checker/checker2.py --all`. Every `rw_*` export must pass its
//! contract and every `mut_*` export must fail it.
//!
//! Correct Rust rewrites of original functions plus subtly wrong mutants.
//! Every address discipline goes through `lf-checker-rt` (relocated file
//! VAs, no hard-coded mapped addresses). All arithmetic is wrapping-explicit
//! so debug builds never panic on overflow.

// Rewrites dereference worker-fabricated pointers by design; every export
// body works through raw pointers into the trial's memory.
#![allow(unsafe_code)]
// Proof rewrites are called by the worker, not by humans; pedantic style
// lints stay off here while correctness lints (clippy::all) apply.
#![allow(clippy::pedantic)]
// 32-bit only: every export uses an x86 calling convention (cdecl, stdcall,
// thiscall, fastcall), which is a hard error on other targets. Off x86 this
// crate builds as an empty DLL so the host workspace build keeps working;
// only the i686 build ever runs (see the usage note above).
#![cfg(target_arch = "x86")]
// Every export dereferences worker-fabricated trial memory through raw
// pointers; they are FFI entry points called by address from the worker,
// where a Rust `unsafe` marker is unenforceable, so they stay safe fns.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use lf_checker_rt::{
    callee_addr, callee_cdecl, callee_fastcall, callee_stdcall, callee_thiscall, export,
    f80_to_f32_bits, f80_to_f64_bits, global, relocated, tls_slot, x87_raw, xmm_word,
};

// 1. const58: returns one fixed constant.
export!(cdecl, rw_const58() -> u32 {
    0x5e
});
export!(cdecl, mut_const58() -> u32 {
    0x5f // off by one
});

// 2. addrimm: returns the address of a datum in the data section (relocated by the loader).
export!(cdecl, rw_addrimm() -> u32 {
    relocated(0x103adc4)
});
export!(cdecl, mut_addrimm() -> u32 {
    0x103adc4 // BUG: forgot the relocation delta
});

// 3. be4: big-endian assemble of 4 stack bytes; ret (cdecl)
export!(cdecl, rw_be4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    let mut eax = a & 0xFF;
    eax = (eax << 8) | (b & 0xFF);
    eax = (eax << 8) | (c & 0xFF);
    eax = (eax << 8) | (d & 0xFF);
    eax
});
export!(cdecl, mut_be4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    let mut eax = a & 0xFF;
    eax = (eax << 8) | (c & 0xFF); // BUG: middle bytes swapped
    eax = (eax << 8) | (b & 0xFF);
    eax = (eax << 8) | (d & 0xFF);
    eax
});

// 4. mulstd: returns its argument times a constant plus a value derived from the object register; the callee removes one argument.
export!(thiscall, rw_mulstd(this: u32, a: u32) -> u32 {
    a.wrapping_mul(0xE0).wrapping_add(this.wrapping_add(0x100))
});
export!(thiscall, mut_mulstd(this: u32, a: u32) -> u32 {
    a.wrapping_mul(0xDF).wrapping_add(this.wrapping_add(0x100)) // BUG: 0xDF
});

// 5. al1: returns true in the low byte only; five ignored arguments are removed by the callee.
export!(stdcall, rw_al1(_a: u32, _b: u32, _c: u32, _d: u32, _e: u32) -> u32 {
    1
});
export!(stdcall, mut_al1(_a: u32, _b: u32, _c: u32, _d: u32, _e: u32) -> u32 {
    0x100 // BUG: value lands in AH instead of AL (wrong return register)
});

// 6. al86: returns a fixed value in the low byte only.
export!(cdecl, rw_al86() -> u32 {
    0x86
});
export!(cdecl, mut_al86() -> u32 {
    0x87 // off by one
});

// 7. zero0: stores zero through the object pointer and returns the pointer (constructor shape).
export!(thiscall, rw_zero0(this: *mut u32) -> u32 {
    unsafe {
        *this = 0;
        this as u32
    }
});
export!(thiscall, mut_zero0(this: *mut u32) -> u32 {
    unsafe {
        *((this as *mut u8).add(4) as *mut u32) = 0; // BUG: wrong offset
        this as u32
    }
});

// 8. set2: stores its two arguments into two fields of the object, second field first; the callee removes both.
// Returns eax = last value moved = second stack arg.
export!(thiscall, rw_set2(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        *((this.add(0xC)) as *mut u32) = a;
        *((this.add(8)) as *mut u32) = b;
        b
    }
});
export!(thiscall, mut_set2(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        *((this.add(8)) as *mut u32) = a; // BUG: stores swapped
        *((this.add(0xC)) as *mut u32) = b;
        b
    }
});

// 9. ctorvt: zero +0x4/+0x8/+0xC, immediates at +0x0/+0x10, eax=ecx
export!(thiscall, rw_ctorvt(this: *mut u8) -> u32 {
    unsafe {
        *((this.add(4)) as *mut u32) = 0;
        *((this.add(8)) as *mut u32) = 0;
        *(this as *mut u32) = relocated(0xfd14a0);
        *((this.add(0xC)) as *mut u32) = 0;
        *((this.add(0x10)) as *mut u32) = relocated(0xfe009c);
        this as u32
    }
});
export!(thiscall, mut_ctorvt(this: *mut u8) -> u32 {
    unsafe {
        *((this.add(4)) as *mut u32) = 0;
        *((this.add(8)) as *mut u32) = 0;
        *(this as *mut u32) = relocated(0xfe009c); // BUG: immediates swapped
        *((this.add(0xC)) as *mut u32) = 0;
        *((this.add(0x10)) as *mut u32) = relocated(0xfd14a0);
        this as u32
    }
});

// 10. ssector: zero [ecx+0x18..+0x28], or-bit at +0x130, [ecx+0x28]=ecx+0x30
export!(thiscall, rw_ssector(this: *mut u8) -> u32 {
    unsafe {
        core::ptr::write_bytes(this.add(0x18), 0, 16);
        *this.add(0x130) |= 1;
        *((this.add(0x28)) as *mut u32) = (this as u32).wrapping_add(0x30);
        this as u32
    }
});
export!(thiscall, mut_ssector(this: *mut u8) -> u32 {
    unsafe {
        core::ptr::write_bytes(this.add(0x18), 0, 12); // BUG: short by 4 bytes
        *this.add(0x130) |= 1;
        *((this.add(0x28)) as *mut u32) = (this as u32).wrapping_add(0x30);
        this as u32
    }
});

// 11. subf: eax=[ecx+8]-[ecx+0x14]
export!(thiscall, rw_subf(this: *const u8) -> u32 {
    unsafe {
        let a = *((this.add(8)) as *const u32);
        let b = *((this.add(0x14)) as *const u32);
        a.wrapping_sub(b)
    }
});
export!(thiscall, mut_subf(this: *const u8) -> u32 {
    unsafe {
        let a = *((this.add(8)) as *const u32);
        let b = *((this.add(0x14)) as *const u32);
        a.wrapping_add(b) // BUG: add instead of sub
    }
});

// 12. b20: al=[ecx+0x20]
export!(thiscall, rw_b20(this: *const u8) -> u32 {
    unsafe { *this.add(0x20) as u32 }
});
export!(thiscall, mut_b20(this: *const u8) -> u32 {
    unsafe { *this.add(0x21) as u32 } // BUG: off by one
});

// 13. cmpss: al=(*(*(ecx+0xB4)+0x70) as f32 > 0.0)
export!(thiscall, rw_cmpss(this: *const u8) -> u32 {
    unsafe {
        let inner = *((this.add(0xB4)) as *const u32) as *const u8;
        let bits = *((inner.add(0x70)) as *const u32);
        ((f32::from_bits(bits) > 0.0) as u8) as u32
    }
});
export!(thiscall, mut_cmpss(this: *const u8) -> u32 {
    unsafe {
        let inner = *((this.add(0xB4)) as *const u32) as *const u8;
        let bits = *((inner.add(0x70)) as *const u32);
        ((f32::from_bits(bits) >= 0.0) as u8) as u32 // BUG: >= instead of >
    }
});

// 14. gdword: eax=[global dword]
export!(cdecl, rw_gdword() -> u32 {
    unsafe { *(global::<u32>(0x11d6fd4)) }
});
export!(cdecl, mut_gdword() -> u32 {
    unsafe { *(global::<u32>(0x11d6fd8)) } // BUG: wrong global (+4)
});

// 15. gbyte: al=[global byte]
export!(cdecl, rw_gbyte() -> u32 {
    unsafe { *(global::<u8>(0x18b6f2e)) as u32 }
});
export!(cdecl, mut_gbyte() -> u32 {
    unsafe { *(global::<u8>(0x18b6f2f)) as u32 } // BUG: off by one
});

// 16. call0: v=callee(); **pp=v; return v
export!(cdecl, rw_call0(pp: *mut *mut u32) -> u32 {
    let v: u32 = callee_cdecl!(1, u32,);
    unsafe {
        **pp = v;
    }
    v
});
export!(cdecl, mut_call0(pp: *mut *mut u32) -> u32 {
    let v: u32 = callee_cdecl!(1, u32,);
    let _ = pp;
    v // BUG: missed the store
});

// 17. call1: inner=[p+8]; v=callee(*inner); return v (caller cleans)
export!(cdecl, rw_call1(p: *const u8) -> u32 {
    unsafe {
        let inner = *((p.add(8)) as *const u32) as *const u32;
        callee_cdecl!(2, u32, *inner)
    }
});
export!(cdecl, mut_call1(p: *const u8) -> u32 {
    unsafe {
        let inner = *((p.add(8)) as *const u32) as *const u32;
        callee_cdecl!(2, u32, inner as u32) // BUG: missing deref (wrong arg)
    }
});

// 18. callbr: idx=callee(a); if idx!=-1 { RMW bit 0x10 at table+idx*80 }
export!(cdecl, rw_callbr(a: u32, b: u32) -> u32 {
    let idx: u32 = callee_cdecl!(3, u32, a);
    if idx == 0xFFFFFFFF {
        return idx;
    }
    let off = idx.wrapping_mul(5).wrapping_shl(4) as usize; // idx*80
    unsafe {
        let cell = global::<u8>(0x16156a5).add(off);
        let mut al = ((b & 0xFF) << 4) as u8;
        al ^= *cell;
        al &= 0x10;
        *cell ^= al;
        (idx & 0xFFFFFF00) | (al as u32)
    }
});
export!(cdecl, mut_callbr(a: u32, b: u32) -> u32 {
    let idx: u32 = callee_cdecl!(3, u32, a);
    if idx == 0xFFFFFFFF {
        return idx;
    }
    let off = idx.wrapping_mul(5).wrapping_shl(4) as usize;
    unsafe {
        let cell = global::<u8>(0x16156a5).add(off);
        let mut al = ((b & 0xFF) << 4) as u8;
        al ^= *cell;
        al &= 0x01; // BUG: wrong mask bit
        *cell ^= al;
        (idx & 0xFFFFFF00) | (al as u32)
    }
});

// 19. fldz: st0=+0.0
export!(cdecl, rw_fldz() -> f64 {
    0.0
});
export!(cdecl, mut_fldz() -> f64 {
    -0.0 // BUG: wrong sign of zero
});

// 20. fldf: st0=(f32)[ecx+0x1C]
export!(thiscall, rw_fldf(this: *const u8) -> f64 {
    unsafe {
        let bits = *((this.add(0x1C)) as *const u32);
        f32::from_bits(bits) as f64
    }
});
export!(thiscall, mut_fldf(this: *const u8) -> f64 {
    unsafe {
        let bits = *((this.add(0x20)) as *const u32); // BUG: wrong field
        f32::from_bits(bits) as f64
    }
});

// Touch callee_addr so the import is used even if macros change.
#[allow(dead_code)]
fn _use_rt() -> u32 {
    callee_addr(0)
}

// 21. iatcall (bonus): forwards its two arguments to an imported library function and returns the result.
// Callee is stdcall/2 (the @8 suffix); the wrapper itself is cdecl/2.
export!(cdecl, rw_iatcall(a: u32, b: u32) -> u32 {
    callee_stdcall!(4, u32, a, b)
});

// Negative controls for the anti-cheat property: both must be reported as
// `cheat`, never as pass or plain fault.
export!(cdecl, cheat_read() -> u32 {
    unsafe { *(relocated(0x516dc0) as *const u32) } // reads original code bytes
});
export!(cdecl, cheat_call() -> u32 {
    unsafe {
        let f: extern "cdecl" fn() -> u32 =
            core::mem::transmute(relocated(0x516dc0) as usize);
        f() // calls the original function
    }
});

// ---------------------------------------------------------------------------
// Checker v2 proofs. Each correct export mirrors the original's convention;
// each mut_ export is a subtly wrong variant the checker must catch.
// ---------------------------------------------------------------------------

// q-07 FN4 (script value rescale) with the v2 fix: the virtual sample hook's
// x87 result is read through an f32 return (the v1 rewrite gave up with 0.0).
export!(thiscall, rw_k2_f4(obj: *mut u8, arg_bits: u32) -> u32 {
    unsafe {
        let reference = if *(obj.add(0x1EC) as *const u32) == 0 {
            let vtable = *(obj as *const u32);
            let target = *((vtable as *const u8).add(0x98) as *const u32);
            let sample: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(target as usize);
            sample(obj as u32)
        } else {
            0.0f32
        };
        let base = f32::from_bits(*(obj.add(0x210) as *const u32));
        let ratio = f32::from_bits(arg_bits) / (base - reference);
        callee_thiscall!(1, u32, obj as u32, ratio.to_bits())
    }
});

// Mutant: add the reference instead of subtracting it.
export!(thiscall, mut_k2_f4(obj: *mut u8, arg_bits: u32) -> u32 {
    unsafe {
        let reference = if *(obj.add(0x1EC) as *const u32) == 0 {
            let vtable = *(obj as *const u32);
            let target = *((vtable as *const u8).add(0x98) as *const u32);
            let sample: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(target as usize);
            sample(obj as u32)
        } else {
            0.0f32
        };
        let base = f32::from_bits(*(obj.add(0x210) as *const u32));
        let ratio = f32::from_bits(arg_bits) / (base + reference);
        callee_thiscall!(1, u32, obj as u32, ratio.to_bits())
    }
});

// q-07 FN1 (flee-and-dive task poll): unchanged logic; under v2 the three
// register-indirect calls land on planted recorder stubs on both sides.
export!(thiscall, rw_k2_f1(task: *mut u8, param: u32) -> u32 {
    unsafe {
        let inner = || *(task.add(8) as *const u32);
        if *(task.add(0x48) as *const u8) == 0 {
            return inner();
        }
        if *(task.add(0x49) as *const u8) != 0 {
            let now = *global::<u32>(0x11735b4);
            *(task.add(0x40) as *mut u32) = now;
            *(task.add(0x49)) = 0;
        }
        let start = *(task.add(0x40) as *const u32);
        let span = *(task.add(0x44) as *const u32);
        let now = *global::<u32>(0x11735b4);
        if (start.wrapping_add(span) as i32) > (now as i32) {
            return inner();
        }
        let obj = inner();
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xC) as *const u32);
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if kind_of(obj) != 0x11d {
            return inner();
        }
        let picked: u32 = callee_thiscall!(1, u32, obj, param);
        if picked == 0 {
            return inner();
        }
        let vtable2 = *(picked as *const u32);
        let target2 = *((vtable2 as *const u8).add(0xC) as *const u32);
        let kind_of2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target2 as usize);
        if kind_of2(picked) != 0x3ae {
            return inner();
        }
        let vtable3 = *(task as *const u32);
        let target3 = *((vtable3 as *const u8).add(0x48) as *const u32);
        let handoff: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target3 as usize);
        handoff(task as u32, param)
    }
});

// Mutant: first kind gate expects 0x11e, so the deep path is never taken.
export!(thiscall, mut_k2_f1(task: *mut u8, param: u32) -> u32 {
    unsafe {
        let inner = || *(task.add(8) as *const u32);
        if *(task.add(0x48) as *const u8) == 0 {
            return inner();
        }
        if *(task.add(0x49) as *const u8) != 0 {
            let now = *global::<u32>(0x11735b4);
            *(task.add(0x40) as *mut u32) = now;
            *(task.add(0x49)) = 0;
        }
        let start = *(task.add(0x40) as *const u32);
        let span = *(task.add(0x44) as *const u32);
        let now = *global::<u32>(0x11735b4);
        if (start.wrapping_add(span) as i32) > (now as i32) {
            return inner();
        }
        let obj = inner();
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xC) as *const u32);
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if kind_of(obj) != 0x11e {
            return inner();
        }
        let picked: u32 = callee_thiscall!(1, u32, obj, param);
        if picked == 0 {
            return inner();
        }
        let vtable2 = *(picked as *const u32);
        let target2 = *((vtable2 as *const u8).add(0xC) as *const u32);
        let kind_of2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target2 as usize);
        if kind_of2(picked) != 0x3ae {
            return inner();
        }
        let vtable3 = *(task as *const u32);
        let target3 = *((vtable3 as *const u8).add(0x48) as *const u32);
        let handoff: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target3 as usize);
        handoff(task as u32, param)
    }
});

// q-07 FN3 (input control dispatch): unchanged logic; under v2 the two
// data-pointer hooks are planted with recorder stubs via global fills.
export!(thiscall, rw_k2_f3(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        if arg == 0 {
            return 0;
        }
        let inner = *(obj.add(0x78) as *const u32);
        let refresh = *global::<u32>(0x17ACD20);
        if refresh != 0 {
            let buf = [0u32; 3];
            let init: extern "cdecl" fn() = core::mem::transmute(refresh as usize);
            init();
            let conv = *global::<u32>(0x17ACD00);
            let convert: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(conv as usize);
            convert(
                buf.as_ptr().add(1) as u32,
                buf.as_ptr() as u32,
            );
            callee_thiscall!(1, u32, inner, arg, buf[0])
        } else {
            let fetch = *global::<u32>(0xE73474);
            let get: extern "cdecl" fn() -> u32 =
                core::mem::transmute(fetch as usize);
            let v = get();
            callee_thiscall!(1, u32, inner, arg, v)
        }
    }
});

// q-07 FN5 (locale code lookup): unchanged logic; under v2 the cursor
// object is synthesized by stub out-param writes on both sides.
export!(cdecl, rw_k2_f5(code: u32, mask: u32, table: u32) -> u32 {
    unsafe {
        let mut st = [0u32; 6];
        let stp = st.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(1, u32, stp, table);
        let entry: u32;
        if code.wrapping_add(1) <= 0x100 {
            let words = *(((st[0]).wrapping_add(0x90)) as *const u32);
            entry = *(((words).wrapping_add(code.wrapping_mul(2))) as *const u16) as u32;
        } else {
            let hi = (code >> 8) & 0xFF;
            let wide: u32 = callee_cdecl!(2, u32, hi, stp);
            let mut buf = [0u8; 4];
            let mut out = [0u16; 2];
            let nbytes: u32;
            if wide != 0 {
                buf[0] = (code >> 8) as u8;
                buf[1] = code as u8;
                buf[2] = 0;
                nbytes = 2;
            } else {
                buf[0] = code as u8;
                buf[1] = 0;
                nbytes = 1;
            }
            let st1 = *((st[0].wrapping_add(4)) as *const u32);
            let ok: u32 = callee_cdecl!(
                3, u32, stp, 1, buf.as_ptr() as u32, nbytes, out.as_mut_ptr() as u32, st1, 1
            );
            if ok == 0 {
                if *((stp.wrapping_add(12)) as *const u8) != 0 {
                    let consumer = *((stp.wrapping_add(8)) as *const u32);
                    *((consumer.wrapping_add(0x70)) as *mut u32) &= !2u32;
                }
                return 0;
            }
            entry = out[0] as u32;
        }
        let result = entry & mask;
        if *((stp.wrapping_add(12)) as *const u8) != 0 {
            let consumer = *((stp.wrapping_add(8)) as *const u32);
            *((consumer.wrapping_add(0x70)) as *mut u32) &= !2u32;
        }
        result
    }
});

export!(cdecl, mut_k2_f5b(code: u32, mask: u32, table: u32) -> u32 {
    unsafe {
        let mut st = [0u32; 6];
        let stp = st.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(1, u32, stp, table);
        let entry: u32;
        if code.wrapping_add(1) <= 0x100 {
            let words = *(((st[0]).wrapping_add(0x90)) as *const u32);
            entry = *(((words).wrapping_add(code.wrapping_mul(2))) as *const u16) as u32;
        } else {
            let hi = (code >> 8) & 0xFF;
            let wide: u32 = callee_cdecl!(2, u32, hi, stp);
            let mut buf = [0u8; 4];
            let mut out = [0u16; 2];
            let nbytes: u32;
            // Mutant: wide-prefix bytes swapped (caught by the id3 buf snapshot).
            if wide != 0 {
                buf[0] = code as u8;
                buf[1] = (code >> 8) as u8;
                buf[2] = 0;
                nbytes = 2;
            } else {
                buf[0] = code as u8;
                buf[1] = 0;
                nbytes = 1;
            }
            let st1 = *((st[0].wrapping_add(4)) as *const u32);
            let ok: u32 = callee_cdecl!(
                3, u32, stp, 1, buf.as_ptr() as u32, nbytes, out.as_mut_ptr() as u32, st1, 1
            );
            if ok == 0 {
                if *((stp.wrapping_add(12)) as *const u8) != 0 {
                    let consumer = *((stp.wrapping_add(8)) as *const u32);
                    *((consumer.wrapping_add(0x70)) as *mut u32) &= !2u32;
                }
                return 0;
            }
            entry = out[0] as u32;
        }
        let result = entry & mask;
        if *((stp.wrapping_add(12)) as *const u8) != 0 {
            let consumer = *((stp.wrapping_add(8)) as *const u32);
            *((consumer.wrapping_add(0x70)) as *mut u32) &= !2u32;
        }
        result
    }
});

// Mutant: the final mask clears the entry's low bit as well.
export!(cdecl, mut_k2_f5(code: u32, mask: u32, table: u32) -> u32 {
    unsafe {
        let mut st = [0u32; 6];
        let stp = st.as_mut_ptr() as u32;
        let _: u32 = callee_thiscall!(1, u32, stp, table);
        let entry: u32;
        if code.wrapping_add(1) <= 0x100 {
            let words = *(((st[0]).wrapping_add(0x90)) as *const u32);
            entry = *(((words).wrapping_add(code.wrapping_mul(2))) as *const u16) as u32;
        } else {
            let hi = (code >> 8) & 0xFF;
            let wide: u32 = callee_cdecl!(2, u32, hi, stp);
            let mut buf = [0u8; 4];
            let mut out = [0u16; 2];
            let nbytes: u32;
            if wide != 0 {
                buf[0] = (code >> 8) as u8;
                buf[1] = code as u8;
                buf[2] = 0;
                nbytes = 2;
            } else {
                buf[0] = code as u8;
                buf[1] = 0;
                nbytes = 1;
            }
            let st1 = *((st[0].wrapping_add(4)) as *const u32);
            let ok: u32 = callee_cdecl!(
                3, u32, stp, 1, buf.as_ptr() as u32, nbytes, out.as_mut_ptr() as u32, st1, 1
            );
            if ok == 0 {
                if *((stp.wrapping_add(12)) as *const u8) != 0 {
                    let consumer = *((stp.wrapping_add(8)) as *const u32);
                    *((consumer.wrapping_add(0x70)) as *mut u32) &= !2u32;
                }
                return 0;
            }
            entry = out[0] as u32;
        }
        let result = entry & mask & !1u32;
        if *((stp.wrapping_add(12)) as *const u8) != 0 {
            let consumer = *((stp.wrapping_add(8)) as *const u32);
            *((consumer.wrapping_add(0x70)) as *mut u32) &= !2u32;
        }
        result
    }
});

// q-16 f2 content mutant (for mut-as-export runs): corrupts the slot local
// whose address is passed to the record query, while forwarding the original
// value as the next argument. Caught only by pointed-to snapshots: the
// skipped address and the forwarded value both still match.
const K2_FLAG_BASE: u32 = 0x167f628;
const K2_TAB_B: u32 = 0x167f640;
const K2_TAB_A: u32 = 0x1682d80;
const K2_ONE_BITS: u32 = 0x3f800000;

#[inline(always)]
unsafe fn k2_put4(dst_row: u32, off: u32, q: u32) {
    // SAFETY: upheld by the caller (worker-fabricated trial memory).
    unsafe {
        let s = q as *const u32;
        let d = global::<u32>(dst_row + off);
        *d = *s;
        *d.add(1) = *s.add(1);
        *d.add(2) = *s.add(2);
        *d.add(3) = *s.add(3);
    }
}

#[inline(always)]
unsafe fn k2_zero3(dst_row: u32, off: u32) {
    // SAFETY: upheld by the caller (worker-fabricated trial memory).
    unsafe {
        let d = global::<u32>(dst_row + off);
        *d = 0;
        *d.add(1) = 0;
        *d.add(2) = 0;
    }
}

#[inline(always)]
// Test helper mirroring one original's query logic; the flat parameters
// match the call sites one-to-one.
#[allow(clippy::too_many_arguments)]
unsafe fn k2_query(
    flag_id: u32,
    ptr_id: u32,
    index: u32,
    key: u32,
    param: u32,
    code: u32,
    slot_val: u32,
    dst_row: u32,
    off: u32,
    corrupt: bool,
) -> u32 {
    // SAFETY: upheld by the caller (worker-fabricated trial memory).
    unsafe {
        let f = callee_cdecl!(flag_id, u32, index, key, 0) & 0xFF;
        if f != 0 {
            let mut slot: u32 = slot_val;
            if corrupt {
                slot = slot.wrapping_add(1);
            }
            let q = callee_cdecl!(
                ptr_id,
                u32,
                &mut slot as *mut u32 as u32,
                slot_val,
                param,
                code
            );
            k2_put4(dst_row, off, q);
        } else {
            k2_zero3(dst_row, off);
        }
        f
    }
}

export!(cdecl, mut_k2_ptr1c(out: u32, which: u32, index: u32, base: u32, param: u32) -> u32 {
    unsafe {
        let row = index << 6;
        if *global::<u8>(K2_FLAG_BASE.wrapping_add(index)) == 0 {
            let f_d = k2_query(1, 5, index, 0, param, 0x9b, K2_ONE_BITS, K2_TAB_A + row, 0x00, true);
            k2_query(2, 6, index, 1, param, 0x9c, K2_ONE_BITS, K2_TAB_A + row, 0x10, false);
            let f_e = k2_query(3, 7, index, 3, param, 0x9e, K2_ONE_BITS, K2_TAB_A + row, 0x30, false);
            let f_f = k2_query(4, 8, index, 2, param, 0xa0, K2_ONE_BITS, K2_TAB_A + row, 0x20, false);
            if f_d != 0 {
                let mut slot: u32 = 0;
                let q = callee_cdecl!(9, u32, &mut slot as *mut u32 as u32, slot, param, 0x9d);
                k2_put4(K2_TAB_B + row, 0x00, q);
            } else {
                k2_zero3(K2_TAB_A + row, 0x00);
            }
            let r0 = global::<u32>(K2_TAB_B + row) as *const u32;
            let r1 = global::<u32>(K2_TAB_B + row + 0x10);
            *r1 = *r0;
            *r1.add(1) = *r0.add(1);
            *r1.add(2) = *r0.add(2);
            *r1.add(3) = *r0.add(3);
            if f_e != 0 {
                let mut slot: u32 = 0;
                let q = callee_cdecl!(10, u32, &mut slot as *mut u32 as u32, slot, param, 0x9f);
                k2_put4(K2_TAB_B + row, 0x30, q);
            } else {
                k2_zero3(K2_TAB_A + row, 0x30);
            }
            if f_f != 0 {
                let mut slot: u32 = 0;
                let q = callee_cdecl!(11, u32, &mut slot as *mut u32 as u32, slot, param, 0xa1);
                k2_put4(K2_TAB_B + row, 0x20, q);
            } else {
                k2_zero3(K2_TAB_A + row, 0x20);
            }
            *global::<u8>(K2_FLAG_BASE.wrapping_add(index)) = 1;
        }
        let drow = base.wrapping_add(index.wrapping_mul(4)) << 4;
        let src = if which & 0xFF != 0 { K2_TAB_A } else { K2_TAB_B } + drow;
        let s = global::<u32>(src) as *const u32;
        let o = out as *mut u32;
        *o = *s;
        *o.add(1) = *s.add(1);
        *o.add(2) = 0;
        *o.add(3) = *s.add(3);
        *s.add(3)
    }
});

// TLS proof 1 (0x59C150): release-style thiscall/1; dispatches through the
// TLS-slot-0 object graph when the head pointer is set, then clears it.
export!(thiscall, rw_k2_t1(task: *mut u8, _param: u32) -> u32 {
    unsafe {
        let inner = *(task as *const u32);
        if inner != 0 {
            let tls0 = tls_slot(0);
            let obj = *((tls0 + 8) as *const u32);
            let vt = *(obj as *const u32);
            let tgt = *((vt as *const u8).add(0xC) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, inner);
        }
        *(task as *mut u32) = 0;
        *(task.add(4) as *mut u32) = 0;
        0
    }
});

// Mutant: misses the second clear.
export!(thiscall, mut_k2_t1(task: *mut u8, _param: u32) -> u32 {
    unsafe {
        let inner = *(task as *const u32);
        if inner != 0 {
            let tls0 = tls_slot(0);
            let obj = *((tls0 + 8) as *const u32);
            let vt = *(obj as *const u32);
            let tgt = *((vt as *const u8).add(0xC) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, inner);
        }
        *(task as *mut u32) = 0;
        0
    }
});

// TLS proof 2 (0x59A2B0): initializer thiscall/1; stamps two constants, then
// conditionally notifies through the TLS-slot-0 object graph.
export!(thiscall, rw_k2_t2(obj: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(obj.add(0x10) as *mut u32) = relocated(0xFDB744);
        *(obj as *mut u32) = relocated(0xFD898C);
        if flag & 1 != 0 {
            let tls0 = tls_slot(0);
            let inner = *((tls0 + 8) as *const u32);
            let vt = *(inner as *const u32);
            let tgt = *((vt as *const u8).add(0xC) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(inner, obj as u32);
        }
        obj as u32
    }
});

// Mutant: wrong stamp constant.
export!(thiscall, mut_k2_t2(obj: *mut u8, flag: u32) -> u32 {
    unsafe {
        *(obj.add(0x10) as *mut u32) = relocated(0xFDB744).wrapping_add(1);
        *(obj as *mut u32) = relocated(0xFD898C);
        if flag & 1 != 0 {
            let tls0 = tls_slot(0);
            let inner = *((tls0 + 8) as *const u32);
            let vt = *(inner as *const u32);
            let tgt = *((vt as *const u8).add(0xC) as *const u32);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(inner, obj as u32);
        }
        obj as u32
    }
});

// XMM0-arg proof 1 (0x5BF550): rotation-ish initializer; two float
// helpers take their argument in XMM0 (the rewrite passes it on the stack
// and the stub transports it on the rewrite side only).
export!(thiscall, rw_k2_x1(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        let cosv: u32 = callee_cdecl!(1, u32, arg);
        let sinv: u32 = callee_cdecl!(2, u32, arg);
        *(obj as *mut u32) = 0x3F800000;
        *(obj.add(4) as *mut u32) = 0;
        *(obj.add(8) as *mut u32) = 0;
        *(obj.add(0x18) as *mut u32) = sinv;
        let c0 = *global::<u32>(0xFE8FA0);
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = cosv;
        *(obj.add(0x20) as *mut u32) = 0;
        *(obj.add(0x24) as *mut u32) = sinv ^ c0;
        *(obj.add(0x28) as *mut u32) = cosv;
        0
    }
});

// Mutant: sine and cosine stores swapped.
export!(thiscall, mut_k2_x1(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        let cosv: u32 = callee_cdecl!(1, u32, arg);
        let sinv: u32 = callee_cdecl!(2, u32, arg);
        *(obj as *mut u32) = 0x3F800000;
        *(obj.add(4) as *mut u32) = 0;
        *(obj.add(8) as *mut u32) = 0;
        *(obj.add(0x18) as *mut u32) = cosv;
        let c0 = *global::<u32>(0xFE8FA0);
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = sinv;
        *(obj.add(0x20) as *mut u32) = 0;
        *(obj.add(0x24) as *mut u32) = sinv ^ c0;
        *(obj.add(0x28) as *mut u32) = cosv;
        0
    }
});

// XMM0-arg proof 2 (0x5BF5C0): sibling initializer, different field layout.
export!(thiscall, rw_k2_x2(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        let cosv: u32 = callee_cdecl!(1, u32, arg);
        let sinv: u32 = callee_cdecl!(2, u32, arg);
        *(obj as *mut u32) = cosv;
        *(obj.add(4) as *mut u32) = 0;
        let c0 = *global::<u32>(0xFE8FA0);
        *(obj.add(8) as *mut u32) = sinv ^ c0;
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = 0x3F800000;
        *(obj.add(0x18) as *mut u32) = 0;
        *(obj.add(0x20) as *mut u32) = sinv;
        *(obj.add(0x24) as *mut u32) = 0;
        *(obj.add(0x28) as *mut u32) = cosv;
        0
    }
});

// Mutant: the 1.0 stamp is missed.
export!(thiscall, mut_k2_x2(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        let cosv: u32 = callee_cdecl!(1, u32, arg);
        let sinv: u32 = callee_cdecl!(2, u32, arg);
        *(obj as *mut u32) = cosv;
        *(obj.add(4) as *mut u32) = 0;
        let c0 = *global::<u32>(0xFE8FA0);
        *(obj.add(8) as *mut u32) = sinv ^ c0;
        *(obj.add(0x10) as *mut u32) = 0;
        *(obj.add(0x14) as *mut u32) = 0;
        *(obj.add(0x18) as *mut u32) = 0;
        *(obj.add(0x20) as *mut u32) = sinv;
        *(obj.add(0x24) as *mut u32) = 0;
        *(obj.add(0x28) as *mut u32) = cosv;
        0
    }
});

// Tail-thunk proof 1 (0xDFBD78): tag in EDX, E9 to a shared routine. The
// worker patches the E9 to a tail stub; the rewrite forwards the tag.
export!(cdecl, rw_k2_tail1() -> u32 {
    callee_fastcall!(1, u32, 0, relocated(0xF0DF6A))
});

// Mutant: wrong tag.
export!(cdecl, mut_k2_tail1() -> u32 {
    callee_fastcall!(1, u32, 0, relocated(0xF0DF6A).wrapping_add(1))
});

// Tail-thunk proof 2 (0xDFBD60): sibling thunk to another routine.
export!(cdecl, rw_k2_tail2() -> u32 {
    callee_fastcall!(1, u32, 0, relocated(0xF0DF6A))
});

// Mutant: wrong tag.
export!(cdecl, mut_k2_tail2() -> u32 {
    callee_fastcall!(1, u32, 0, relocated(0xF0DF6A).wrapping_add(1))
});

// XMM-entry proof 1 (0x75EE00): thiscall/0 with a float in XMM1; gated
// virtual call plus counter state machine plus a computed tail jump.
export!(thiscall, rw_k2_e1(obj: *mut u8) -> u32 {
    unsafe {
        let w = xmm_word(1, 0);
        let vt = *(obj as *const u32);
        let tgt = *((vt as *const u8).add(8) as *const u32);
        let gate: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r = gate(obj as u32, w, w);
        if r & 0xFF != 0 {
            if *(obj.add(0x18) as *const u32) == 1 {
                let c = (*(obj.add(0x20) as *const u32)).wrapping_add(1);
                *(obj.add(0x20) as *mut u32) = c;
                if (c as i32) >= (*(obj.add(0x10) as *const u32) as i32) {
                    *(obj.add(0x18) as *mut u32) = 0;
                }
                if *(obj.add(0x18) as *const u32) == 1 {
                    return c;
                }
            }
            let c = (*(obj.add(0x24) as *const u32)).wrapping_add(1);
            *(obj.add(0x24) as *mut u32) = c;
            if (c as i32) >= (*(obj.add(0x14) as *const u32) as i32) {
                *(obj.add(0x18) as *mut u32) = 2;
            }
            c
        } else if *(obj.add(0x18) as *const u32) == 1 {
            *(obj.add(0x20) as *mut u32) = 0;
            *(obj.add(0x24) as *mut u32) = 0;
            r
        } else {
            let tail = *((vt as *const u8).add(4) as *const u32);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tail as usize);
            f(obj as u32)
        }
    }
});

// Mutant: the saturated second counter stamps state 3 instead of 2.
export!(thiscall, mut_k2_e1(obj: *mut u8) -> u32 {
    unsafe {
        let w = xmm_word(1, 0);
        let vt = *(obj as *const u32);
        let tgt = *((vt as *const u8).add(8) as *const u32);
        let gate: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r = gate(obj as u32, w, w);
        if r & 0xFF != 0 {
            if *(obj.add(0x18) as *const u32) == 1 {
                let c = (*(obj.add(0x20) as *const u32)).wrapping_add(1);
                *(obj.add(0x20) as *mut u32) = c;
                if (c as i32) >= (*(obj.add(0x10) as *const u32) as i32) {
                    *(obj.add(0x18) as *mut u32) = 0;
                }
                if *(obj.add(0x18) as *const u32) == 1 {
                    return c;
                }
            }
            let c = (*(obj.add(0x24) as *const u32)).wrapping_add(1);
            *(obj.add(0x24) as *mut u32) = c;
            if (c as i32) >= (*(obj.add(0x14) as *const u32) as i32) {
                *(obj.add(0x18) as *mut u32) = 3;
            }
            c
        } else if *(obj.add(0x18) as *const u32) == 1 {
            *(obj.add(0x20) as *mut u32) = 0;
            *(obj.add(0x24) as *mut u32) = 0;
            r
        } else {
            let tail = *((vt as *const u8).add(4) as *const u32);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tail as usize);
            f(obj as u32)
        }
    }
});

// XMM-entry proof 2 (0x787390): thiscall/0 with a float in XMM1; broadcasts
// it to every live element of a counted object array through a virtual call.
export!(thiscall, rw_k2_e2(obj: *mut u8) -> u32 {
    unsafe {
        let mut n = ((*(obj as *const u32)).wrapping_sub(1)) as i32;
        let w = xmm_word(1, 0);
        if n > 0 {
            let mut p = obj.add(0x84) as *const u32;
            while n > 0 {
                let o = *p;
                if *((o + 0x220) as *const u32) != 0 {
                    let vt = *(o as *const u32);
                    let tgt = *((vt as *const u8).add(0x4C) as *const u32);
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    f(o, w);
                }
                p = p.add(1);
                n -= 1;
            }
        }
        0
    }
});

// Mutant: the liveness gate is inverted (calls dead elements instead).
export!(thiscall, mut_k2_e2(obj: *mut u8) -> u32 {
    unsafe {
        let mut n = ((*(obj as *const u32)).wrapping_sub(1)) as i32;
        let w = xmm_word(1, 0);
        if n > 0 {
            let mut p = obj.add(0x84) as *const u32;
            while n > 0 {
                let o = *p;
                if *((o + 0x220) as *const u32) == 0 {
                    let vt = *(o as *const u32);
                    let tgt = *((vt as *const u8).add(0x4C) as *const u32);
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    f(o, w);
                }
                p = p.add(1);
                n -= 1;
            }
        }
        0
    }
});

// Mutant: the hook branches are swapped (refresh path taken when unset).
export!(thiscall, mut_k2_f3(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        if arg == 0 {
            return 0;
        }
        let inner = *(obj.add(0x78) as *const u32);
        let refresh = *global::<u32>(0x17ACD20);
        if refresh == 0 {
            let buf = [0u32; 3];
            let init: extern "cdecl" fn() = core::mem::transmute(refresh as usize);
            init();
            let conv = *global::<u32>(0x17ACD00);
            let convert: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(conv as usize);
            convert(
                buf.as_ptr().add(1) as u32,
                buf.as_ptr() as u32,
            );
            callee_thiscall!(1, u32, inner, arg, buf[0])
        } else {
            let fetch = *global::<u32>(0xE73474);
            let get: extern "cdecl" fn() -> u32 =
                core::mem::transmute(fetch as usize);
            let v = get();
            callee_thiscall!(1, u32, inner, arg, v)
        }
    }
});

// ---------------------------------------------------------------------------
// Checker v5 proofs (contracts k5_*). Three run on built-in self-test
// originals (selftest:<name>, emitted by the worker; the verdicts are
// marked as self-tests), because no tracked original is known to take x87
// arguments, pass XMM2-XMM7 to a callee or read through an unrelocated
// absolute address. Two run on a real original with wider snapshots.
// Each mutant is invisible to the v4 checker and caught by the v5 feature
// its contract exercises.
// ---------------------------------------------------------------------------

/// Byte offset of the `f32` store (from ST0) in the x87_store record.
const X87_REC_F32: usize = 0;
/// Byte offset of the `f64` store (from ST1).
const X87_REC_F64: usize = 4;
/// Byte offset of the 80-bit store (from ST2).
const X87_REC_F80: usize = 12;

/// Write the x87_store record: ST(`a`) rounded to `f32`, ST(`b`) rounded to
/// `f64` (both as an x87 store rounds, through the runtime's converters),
/// and ST2's exact 80 bits.
fn write_x87_record(rec: *mut u8, a: usize, b: usize) {
    let (m0, e0) = x87_raw(a);
    let (m1, e1) = x87_raw(b);
    let (m2, e2) = x87_raw(2);
    unsafe {
        (rec.add(X87_REC_F32) as *mut u32).write_unaligned(f80_to_f32_bits(m0, e0));
        (rec.add(X87_REC_F64) as *mut u64).write_unaligned(f80_to_f64_bits(m1, e1));
        (rec.add(X87_REC_F80) as *mut u64).write_unaligned(m2);
        (rec.add(X87_REC_F80 + 8) as *mut u16).write_unaligned(e2);
    }
}

// k5_x87 / k5_x87bal (selftest:x87_store): the original pops three x87
// entry values into an f32, an f64 and an 80-bit slot of the record and
// returns the record pointer. The rewrite reads the entries from the x87
// mirror and rounds them exactly as the stores do.
export!(cdecl, rw_k5_x87(rec: *mut u8) -> u32 {
    write_x87_record(rec, 0, 1);
    rec as u32
});

// Mutant (k5_x87): ST0 and ST1 swapped. The v4 checker could not load x87
// entry values at all, so the order was unobservable.
export!(cdecl, mut_k5_x87(rec: *mut u8) -> u32 {
    write_x87_record(rec, 1, 0);
    rec as u32
});

// Mutant (k5_x87bal): the record is right, but the rewrite returns a value
// in ST0, leaving the x87 stack one deeper than the original's (which
// consumed its entries). Its contract compares no return channel (ret
// "none") and memory matches, so only the v5 x87 state check sees it.
export!(cdecl, mut_k5_x87bal(rec: *mut u8) -> f64 {
    write_x87_record(rec, 0, 1);
    let (m, e) = x87_raw(0);
    f64::from_bits(f80_to_f64_bits(m, e))
});

// k5_xmm (selftest:xmm_call): the original passes its two stack words to
// callee 1 in XMM2 and XMM5 (no stack arguments) and returns the low word
// of the XMM6 entry value. The rewrite passes them on the stack; the stub
// moves them into XMM2/XMM5 on the rewrite side (xmm_from_stack), and the
// logged registers compare.
export!(cdecl, rw_k5_xmm(a: u32, b: u32) -> u32 {
    let _: u32 = callee_cdecl!(1, u32, a, b);
    xmm_word(6, 0)
});

// Mutant: the two vector arguments swapped. v4 logged XMM0/XMM1 only, so
// XMM2 and XMM5 at the call were unobservable.
export!(cdecl, mut_k5_xmm(a: u32, b: u32) -> u32 {
    let _: u32 = callee_cdecl!(1, u32, b, a);
    xmm_word(6, 0)
});

/// Bit-exact widening of f32 bits to f64 bits, as `cvtss2sd` does: exact
/// values, infinities kept, NaNs quieted with the payload preserved,
/// denormals normalised, signed zeros kept. (A plain `as f64` widens
/// values but its NaN payload handling is not pinned down, so the proof
/// does the conversion on bits.)
fn cvtss2sd_bits(a: u32) -> u64 {
    let sign = u64::from(a >> 31) << 63;
    let e = (a >> 23) & 0xFF;
    let f = u64::from(a & 0x7F_FFFF);
    if e == 0xFF {
        if f == 0 {
            return sign | 0x7FF0_0000_0000_0000; // infinity
        }
        return sign | 0x7FF8_0000_0000_0000 | (f << 29); // NaN, quieted
    }
    if e == 0 {
        if f == 0 {
            return sign; // signed zero
        }
        // Denormal: value = f * 2^-149 = m * 2^(-127-lz) with m in [1,2).
        let lz = (f as u32).leading_zeros() - 9; // 0..=22 within 23 bits
        let exp = 896 - lz; // 1023 - 127 - lz
        let frac = ((f << (lz + 1)) & 0x7F_FFFF) << 29;
        return sign | (u64::from(exp) << 52) | frac;
    }
    sign | (u64::from(e + 896) << 52) | (f << 29)
}

/// Low and high words of a double, the order `xmm_from_stack64` takes them.
fn lo_hi(d: u64) -> (u32, u32) {
    ((d & 0xFFFF_FFFF) as u32, ((d >> 32) & 0xFFFF_FFFF) as u32)
}

// k6_f64 (selftest:f64_call, doubles extension): the original widens two
// f32 stack words to doubles in XMM0/XMM1, calls callee 1 with no stack
// arguments, stores the double answer from XMM0 at p and returns its low
// word. The rewrite passes each double as two stack words (the stub moves
// them into XMM0/XMM1 on the rewrite side) and reads the answer as the
// callee's u64 return; the logged registers compare bit for bit.
export!(cdecl, rw_k6_f64(a: u32, b: u32, p: *mut u32) -> u32 {
    let (a0, a1) = lo_hi(cvtss2sd_bits(a));
    let (b0, b1) = lo_hi(cvtss2sd_bits(b));
    let ans: u64 = callee_cdecl!(1, u64, a0, a1, b0, b1);
    unsafe {
        (p as *mut u64).write_unaligned(ans);
        *p
    }
});

// Mutant (k6_f64arg): the first double argument negated. Without the
// 8-byte transport the doubles at the call are unobservable and this
// passes (see k6_f64neg).
export!(cdecl, mut_k6_f64_arg(a: u32, b: u32, p: *mut u32) -> u32 {
    let (a0, a1) = lo_hi(cvtss2sd_bits(a) ^ 0x8000_0000_0000_0000);
    let (b0, b1) = lo_hi(cvtss2sd_bits(b));
    let ans: u64 = callee_cdecl!(1, u64, a0, a1, b0, b1);
    unsafe {
        (p as *mut u64).write_unaligned(ans);
        *p
    }
});

// Mutant (k6_f64ans): the double answer ignored, a constant stored. The
// stub answers the same double on both sides, so only the rewrite's use
// of the answer differs; the heap check sees it.
export!(cdecl, mut_k6_f64_ans(a: u32, b: u32, p: *mut u32) -> u32 {
    let (a0, a1) = lo_hi(cvtss2sd_bits(a));
    let (b0, b1) = lo_hi(cvtss2sd_bits(b));
    let _: u64 = callee_cdecl!(1, u64, a0, a1, b0, b1);
    unsafe {
        (p as *mut u64).write_unaligned(0);
        *p
    }
});

// k5_abs (selftest:abs_read with abs_shadow): the original reads a dword
// through an absolute file address with no relocation; the rewrite reads
// the same datum through the relocated image, as rewrites must.
export!(cdecl, rw_k5_abs(file_va: u32) -> u32 {
    unsafe { *global::<u32>(file_va) }
});

// Mutant: reads the unrelocated file address directly. Under v4 both sides
// read the same unrelated worker memory there, so it matched; under v5 the
// window is inaccessible to the rewrite and it faults.
export!(cdecl, mut_k5_abs(file_va: u32) -> u32 {
    unsafe { *(file_va as *const u32) }
});

/// Byte offset (from the object) of the word mut_k5_snap disturbs during
/// the call: inside the k5_snap window (0x1D8 + 16 words), past its eighth
/// word, so no v4 snapshot (8 words from offset 0) could cover it.
const SNAP_POKE_OFF: usize = 0x1F8;
/// Bytes below the object that mut_k5_snapneg disturbs (inside k5_snapneg's
/// window at -16).
const SNAP_POKE_NEG: usize = 8;
/// Value XORed into the disturbed word during the call.
const SNAP_POKE_XOR: u32 = 0x5A5A_5A5A;

/// rw_k2_f4's body with one word disturbed while callee 1 runs and
/// restored afterwards: final memory is identical, only the callee's view
/// at call time differs.
fn k2_f4_with_poke(obj: *mut u8, arg_bits: u32, poke: *mut u32) -> u32 {
    unsafe {
        let reference = if *(obj.add(0x1EC) as *const u32) == 0 {
            let vtable = *(obj as *const u32);
            let target = *((vtable as *const u8).add(0x98) as *const u32);
            let sample: extern "thiscall" fn(u32) -> f32 = core::mem::transmute(target as usize);
            sample(obj as u32)
        } else {
            0.0f32
        };
        let base = f32::from_bits(*(obj.add(0x210) as *const u32));
        let ratio = f32::from_bits(arg_bits) / (base - reference);
        let saved = poke.read_volatile();
        poke.write_volatile(saved ^ SNAP_POKE_XOR);
        let r: u32 = callee_thiscall!(1, u32, obj as u32, ratio.to_bits());
        poke.write_volatile(saved);
        r
    }
}

// Mutant (k5_snap, original 0x9e09f0 with rw_k2_f4 as the rewrite): a
// field past the eighth snapshot word is wrong while the callee runs.
export!(thiscall, mut_k5_snap(obj: *mut u8, arg_bits: u32) -> u32 {
    let poke = unsafe { obj.add(SNAP_POKE_OFF) } as *mut u32;
    k2_f4_with_poke(obj, arg_bits, poke)
});

// Mutant (k5_snapneg): a word just below the object (a header or the
// previous record) is wrong while the callee runs.
export!(thiscall, mut_k5_snapneg(obj: *mut u8, arg_bits: u32) -> u32 {
    let poke = unsafe { obj.sub(SNAP_POKE_NEG) } as *mut u32;
    k2_f4_with_poke(obj, arg_bits, poke)
});
