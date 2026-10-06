// original: 0x005D7700 fill_record_from_rows (proposed)

/// Fills an output record by dispatching each descriptor row to one of
/// nineteen handlers.
///
/// `this` is the owner; the single stack argument `node` points at a
/// node with a descriptor at `+0xDC` (callee pops 4). Output goes to
/// `node+0x14`. Session state lives in TLS slot 0: first `[tls0+8]`
/// and `[tls0+0x10]` are compared for equality (equal increments the
/// counter at `[tls0+0x68]`, otherwise the first word rotates into the
/// spare slot at `[tls0+0x64]` and the second becomes current).
///
/// The main loop runs over the row count, a word at `[desc+0xC]`
/// (skipped as a whole when zero): for each 12-byte row at
/// `[desc+8]+index*12`, a global helper (file VA `FLAG_VA`) is reused
/// when non-zero, otherwise built through the thread allocator as
/// `tls[0] -> [+8] -> vtable[+8]` (thiscall: allocator, `0x24, 0x10,
/// 0`; the twin stub answers on spare-installed trials) and completed
/// by callee 3 (thiscall: fresh object), storing the result back.
/// Callee 4 (thiscall: helper, row word 0) then selects the handler
/// through a 24-entry jump table (file VA `0x5D7AA4`), read as an
/// UNSIGNED index: above `0x17` the iteration does nothing. Five
/// indices (3, 13, 15, 18, 20) also do nothing; the rest: 0/22 prime
/// callee 5 (thiscall/0) and store callee 6's answer (thiscall:
/// prime answer, row word 1) at `+0x38`/`+0x3C`; 1 primes callee 5,
/// queries callee 7 (stdcall: row word 1, out-word) and chains
/// callees 8/9/10 with a struct holding 7, the out-word and 3;
/// 2/5/7/11 prime callee 5, query callee 7 with the row word or the
/// reloaded row pointer, and store the out-word (5 also sets a flag
/// byte); 4 brackets the resolver in the planted global slot (cdecl:
/// table address, row word 1) between two bare first-meg calls and
/// stores the answer; 10 calls callee 15 (thiscall: owner, node, row
/// word 1); 21 calls callee 16 (stdcall: row word 1, output base);
/// 6/8/9/14/16/17/23 convert row word 1 exactly to float and store it
/// (6 also writes tag `0x13` for a zero word else `0x14`, which is what
/// its compare-and-flag-test computes: the conversion of an integer
/// can only be zero for word 0); 12/19 store row word 1 as an integer.
///
/// Callee 5's entry ECX is stale (whatever the previous stub left)
/// and is not compared; the five bare first-meg calls are modeled as
/// stdcall/0 for the same reason. The loop-entry unsigned check is
/// equivalent to a zero test for every count under `0x8000` (all of
/// them here); a count with the high bit set would need tens of
/// thousands of iterations and is not tested. The loop-continue
/// signed check only ever sees non-negative values.
///
/// Returns the session counter minus one, or the spare allocator word
/// when the counter reached zero (restoring and clearing the spare).
///
/// Original: 0x005D7700 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_005D7700(this: u32, node: u32) -> u32 {
    unsafe {
        const DC_OFF: u32 = 0xDC;
        const OUT_OFF: u32 = 0x14;
        const FLAG_VA: u32 = 0x018B7430;
        const TABLE_ADDR: u32 = 0x019E8658;
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const VA: u32 = 1;
        const VB: u32 = 2;
        const MAKE_HELPER: u32 = 3;
        const SELECT: u32 = 4;
        const PRIME: u32 = 5;
        const FINISH: u32 = 6;
        const QUERY: u32 = 7;
        const QUERYB: u32 = 17;
        const CHAIN0: u32 = 8;
        const CHAIN1: u32 = 9;
        const CHAIN2: u32 = 10;
        const PRE: u32 = 11;
        const POST: u32 = 12;
        const RESOLVE: u32 = 13;
        const TOUCH: u32 = 14;
        const EMIT: u32 = 15;
        const STORE2: u32 = 16;
        let tls0 = lf_checker_rt::tls_slot(TLS_SLOT);
        let ea = rd32(tls0.wrapping_add(8));
        let ec = rd32(tls0.wrapping_add(0x10));
        if ea == ec {
            let c = rd32(tls0.wrapping_add(0x68));
            wr32(tls0.wrapping_add(0x68), c.wrapping_add(1));
        } else {
            wr32(tls0.wrapping_add(0x64), ea);
            wr32(tls0.wrapping_add(8), ec);
        }
        let _ = (VA, VB);
        let desc = rd32(node.wrapping_add(DC_OFF));
        let edi = node.wrapping_add(OUT_OFF);
        let count = rd16(desc.wrapping_add(0xC)) as u32;
        if count != 0 {
            let rowbase = rd32(desc.wrapping_add(8));
            let mut ebp = 0u32;
            loop {
                let esi0 = rowbase.wrapping_add(ebp.wrapping_mul(12));
                let w0 = rd32(esi0);
                let row4 = rd32(esi0.wrapping_add(4));
                let mut flag = rd32(lf_checker_rt::relocated(FLAG_VA));
                if flag == 0 {
                    let heap_now = rd32(tls0.wrapping_add(8));
                    let vt = rd32(heap_now);
                    let site: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
                        core::mem::transmute(rd32(vt.wrapping_add(ALLOC_SLOT)) as usize)
                    };
                    let r = site(heap_now, 0x24, 0x10, 0);
                    flag = if r != 0 {
                        lf_checker_rt::callee_thiscall!(MAKE_HELPER, u32, r)
                    } else {
                        0
                    };
                    wr32(lf_checker_rt::relocated(FLAG_VA), flag);
                }
                let sel = lf_checker_rt::callee_thiscall!(SELECT, u32, flag, w0);
                // Unsigned dispatch (original `ja`): above 0x17 joins.
                if sel > 0x17 {
                } else {
                    // Index map from the jump table at file VA 0x5D7AA4.
                    match sel {
                        0 => {
                            let a = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let d = lf_checker_rt::callee_thiscall!(FINISH, u32, a, row4);
                            wr32(edi.wrapping_add(0x38), d);
                        }
                        1 => {
                            let _ = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let mut out = 0u32;
                            let _ = lf_checker_rt::callee_stdcall!(
                                QUERY,
                                u32,
                                &mut out as *mut u32 as u32,
                                row4
                            );
                            let mut st = [0u32; 8];
                            st[0] = 7;
                            st[1] = out;
                            st[3] = 3;
                            let r1 = lf_checker_rt::callee_thiscall!(
                                CHAIN0,
                                u32,
                                this.wrapping_add(0x20),
                                2,
                                st.as_mut_ptr() as u32
                            );
                            let r2 = lf_checker_rt::callee_thiscall!(CHAIN1, u32, r1);
                            let _ = lf_checker_rt::callee_thiscall!(CHAIN2, u32, r2);
                        }
                        2 => {
                            let esi = row4;
                            let _ = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let mut dummy = 0u32;
                            let ans = lf_checker_rt::callee_stdcall!(
                                QUERYB,
                                u32,
                                &mut dummy as *mut u32 as u32,
                                esi
                            );
                            wr32(edi.wrapping_add(0xBC), rd32(ans));
                        }
                        4 => {
                            let _ = lf_checker_rt::callee_cdecl!(PRE, u32,);
                            let r = lf_checker_rt::callee_cdecl!(
                                RESOLVE,
                                u32,
                                lf_checker_rt::relocated(TABLE_ADDR),
                                row4
                            );
                            wr32(edi.wrapping_add(0x24), r);
                            wr8(edi.wrapping_add(0xB8), 1);
                            let _ = lf_checker_rt::callee_cdecl!(POST, u32,);
                        }
                        5 => {
                            wr8(edi.wrapping_add(0xB8), 1);
                            let esi = row4;
                            let _ = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let mut dummy = 0u32;
                            let ans = lf_checker_rt::callee_stdcall!(
                                QUERYB,
                                u32,
                                &mut dummy as *mut u32 as u32,
                                esi
                            );
                            wr32(edi.wrapping_add(0x20), rd32(ans));
                        }
                        6 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            let f = (row4 as i32) as f32;
                            let tag = if row4 == 0 { 0x13 } else { 0x14 };
                            wr32(edi.wrapping_add(0x5C), tag);
                            wr32(edi.wrapping_add(0x68), tag);
                            wr32(edi.wrapping_add(0x74), tag);
                            wr32(edi.wrapping_add(0x80), tag);
                            wr32(edi.wrapping_add(0x60), f.to_bits());
                            wr32(edi.wrapping_add(0x6C), f.to_bits());
                            wr32(edi.wrapping_add(0x78), f.to_bits());
                            wr32(edi.wrapping_add(0x84), f.to_bits());
                        }
                        7 => {
                            let esi = row4;
                            let _ = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let mut dummy = 0u32;
                            let ans = lf_checker_rt::callee_stdcall!(
                                QUERYB,
                                u32,
                                &mut dummy as *mut u32 as u32,
                                esi
                            );
                            wr32(edi.wrapping_add(0x58), rd32(ans));
                            wr32(edi.wrapping_add(0x64), rd32(ans));
                            wr32(edi.wrapping_add(0x70), rd32(ans));
                            wr32(edi.wrapping_add(0x7C), rd32(ans));
                        }
                        8 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            wr32(edi.wrapping_add(0xAC), ((row4 as i32) as f32).to_bits());
                        }
                        9 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            wr32(edi.wrapping_add(0xA8), ((row4 as i32) as f32).to_bits());
                        }
                        10 => {
                            let _ =
                                lf_checker_rt::callee_thiscall!(EMIT, u32, this, node, row4);
                        }
                        11 => {
                            let esi = row4;
                            let _ = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let mut dummy = 0u32;
                            let ans = lf_checker_rt::callee_stdcall!(
                                QUERYB,
                                u32,
                                &mut dummy as *mut u32 as u32,
                                esi
                            );
                            wr32(edi.wrapping_add(0x34), rd32(ans));
                        }
                        12 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            wr32(edi.wrapping_add(0xB0), row4);
                        }
                        14 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            wr32(edi.wrapping_add(8), ((row4 as i32) as f32).to_bits());
                        }
                        16 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            let f = ((row4 as i32) as f32).to_bits();
                            wr32(edi.wrapping_add(0x88), f);
                            wr32(edi.wrapping_add(0x94), f);
                        }
                        17 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            let f = ((row4 as i32) as f32).to_bits();
                            wr32(edi.wrapping_add(0x8C), f);
                            wr32(edi.wrapping_add(0x90), f);
                        }
                        19 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            wr32(edi.wrapping_add(0xB4), row4);
                        }
                        21 => {
                            let _ = lf_checker_rt::callee_stdcall!(STORE2, u32, row4, edi);
                        }
                        22 => {
                            let a = lf_checker_rt::callee_thiscall!(PRIME, u32, flag);
                            let d = lf_checker_rt::callee_thiscall!(FINISH, u32, a, row4);
                            wr32(edi.wrapping_add(0x3C), d);
                        }
                        23 => {
                            let _ = lf_checker_rt::callee_thiscall!(TOUCH, u32, esi0);
                            wr32(edi.wrapping_add(4), ((row4 as i32) as f32).to_bits());
                        }
                        _ => {}
                    }
                }
                ebp = ebp.wrapping_add(1);
                if !((ebp as i32) < (count as i32)) {
                    break;
                }
            }
        }
        let e = rd32(tls0.wrapping_add(0x68));
        if e != 0 {
            wr32(tls0.wrapping_add(0x68), e.wrapping_sub(1));
            e.wrapping_sub(1)
        } else {
            let s = rd32(tls0.wrapping_add(0x64));
            wr32(tls0.wrapping_add(8), s);
            wr32(tls0.wrapping_add(0x64), 0);
            s
        }
    }
});
