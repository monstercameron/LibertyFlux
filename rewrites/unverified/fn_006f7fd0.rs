// original: 0x006f7fd0 cloud_request_dispatch (proposed)

/// Dispatch a cloud request selected by the operation code in edx.
///
/// `obj` is the request context (words at `+0x08`/`+0x0c` select a variant
/// passed to the open call); `op` is the operation code; `a1` is an opaque
/// value handed to one helper as its receiver; `name` points at the request
/// name string; `a3` is read into edx for one formatting call (whose stub
/// ignores it); `target` selects an optional second stage (null skips it).
///
/// Behaviour: an open call (callee 2, file address 0x419960) must answer
/// nonzero and a mode call (callee 3, file address 0x419790) must answer 7,
/// else the answer's upper bytes are returned at once. Two setup rounds
/// follow (format call callee 5 at file address 0x6f9630, then check call
/// callee 4 at file address 0x87eb80, whose zero answers also return at
/// once), the first fed by the writable service pointer at file address
/// 0x18b8320, the second by a reader call (callee 6, file address 0x6f7f60).
/// The operation code then selects one of five paths: codes 0-2 validate
/// the name is nonempty and run a helper (callee 7, file address 0x6f7fb0),
/// a format and a check; code 3 runs a format fed by the service pointer
/// and a check; code 4 skips to the tail; any other code returns at once.
/// Each path joins the tail, which formats a scratch path (callee 8, file
/// address 0x671f30, then callee 9 at file address 0x6f9660, then callee 10
/// at file address 0x671fd0), measures it twice (an empty second measure
/// returns at once), checks it (second check identity, callee 17, same file
/// address as callee 4, kept separate because its argument is a stack buffer
/// while the others take words), optionally runs the second stage through
/// `target` (callees 11-14 and check 4), runs a probe/fallback pair
/// (callees 13 and 15, file addresses 0x6dda60 and 0x6ddb40, whose zero
/// answer selects the alternate tag), one more format and a final check
/// (callee 16, file address 0x87ebe0). Success returns the final answer with
/// its low byte set; every early exit returns the triggering answer with
/// its low byte cleared (the string-measure loop clears it the same way).
///
/// The original cleans its four stack words in the caller (plain return)
/// while this Rust fastcall callee-cleans, so the proof leaves the
/// stack-pointer check off and compares the incoming argument words instead;
/// see the contract. All arithmetic is wrapping; all memory access is
/// unaligned-safe.
///
/// Original: 0x006f7fd0 (fastcall with caller cleanup, four stack words).
lf_checker_rt::export!(
    fastcall,
    rw_006f7fd0(obj: u32, op: u32, a1: u32, name: u32, a3: u32, target: u32) -> u32 {
        unsafe {
            const G_SVC: u32 = 0x018b8320;
            const T_OPEN: u32 = 0x00fae0fc;
            const T_S0: u32 = 0x00fae158;
            const T_S1: u32 = 0x00fae144;
            const T_D0A: u32 = 0x00fae140;
            const T_D0B: u32 = 0x00fae13c;
            const T_D1A: u32 = 0x00fae138;
            const T_D1B: u32 = 0x00fae134;
            const T_D2A: u32 = 0x00fae130;
            const T_D2B: u32 = 0x00fae1a0;
            const T_D3A: u32 = 0x00fae19c;
            const T_D3B: u32 = 0x00fae198;
            const T_L: u32 = 0x00fae188;
            const T_MISS: u32 = 0x00fa4f44;
            const T_ALT0: u32 = 0x00fae170;
            const T_ALT1: u32 = 0x00fae17c;
            const T_DX: u32 = 0x00fae148;
            const P_EMPTY: u32 = 0x00f1c38b;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn rd8(a: u32) -> u8 {
                unsafe { (a as *const u8).read_unaligned() }
            }
            // String measure exactly like the original's loop: the length,
            // with the low byte of `eax` cleared by the terminating read.
            #[inline(always)]
            unsafe fn strlen(p: u32, eax: &mut u32) -> u32 {
                let mut q = p;
                while unsafe { rd8(q) } != 0 {
                    q = q.wrapping_add(1);
                }
                *eax &= 0xffffff00;
                q.wrapping_sub(p)
            }
            #[inline(always)]
            unsafe fn fmt(a0: u32, a1: u32, a2: u32) -> u32 {
                unsafe { lf_checker_rt::callee_cdecl!(5, u32, a0, a1, a2) }
            }
            #[inline(always)]
            unsafe fn chk(this: u32, w: u32) -> u32 {
                unsafe { lf_checker_rt::callee_thiscall!(4, u32, this, w) }
            }

            // Scratch: prefix byte pair plus the measured string bytes, laid
            // out contiguously like the original's frame.
            let mut stalk: [u8; 40] = [0; 40];
            let mut ebuf: [u32; 4] = [0; 4];
            let sbase = stalk.as_mut_ptr() as u32;
            let sstr = sbase.wrapping_add(2);
            let ebase = ebuf.as_mut_ptr() as u32;

            let edi = obj;
            let mut ebx = op;
            let mut eax: u32 = 0;

            let _a0: u32 = lf_checker_rt::callee_thiscall!(1, u32, edi);
            let variant = rd32(edi.wrapping_add(8))
                .wrapping_add(7)
                .wrapping_add((rd32(edi.wrapping_add(0x0c)) >> 2) & 1);
            let b: u32 = lf_checker_rt::callee_thiscall!(2, u32, edi, variant);
            if (b as u8) == 0 {
                return (b & 0xffffff00) | (rd8(sbase) as u32);
            }
            let c: u32 = lf_checker_rt::callee_thiscall!(
                3,
                u32,
                edi,
                lf_checker_rt::relocated(T_OPEN),
                7u32
            );
            if c != 7 {
                return (c & 0xffffff00) | (rd8(sbase) as u32);
            }
            let svc = rd32(lf_checker_rt::relocated(G_SVC));
            let e1 = fmt(ebase, lf_checker_rt::relocated(T_S0), rd32(svc.wrapping_add(0x14)));
            let d1 = chk(edi, e1);
            if (d1 as u8) == 0 {
                return (d1 & 0xffffff00) | (rd8(sbase) as u32);
            }
            let d2 = chk(edi, lf_checker_rt::relocated(T_DX));
            if (d2 as u8) == 0 {
                return (d2 & 0xffffff00) | (rd8(sbase) as u32);
            }
            let f: u32 = lf_checker_rt::callee_thiscall!(6, u32, ebx);
            let e2 = fmt(ebase, lf_checker_rt::relocated(T_S1), f);
            let d3 = chk(edi, e2);
            if (d3 as u8) == 0 {
                return (d3 & 0xffffff00) | (rd8(sbase) as u32);
            }

            // Operation dispatch; codes 0-3 run one helper/format/check each.
            if ebx <= 3 {
                let (ta, tb, use_svc_ptr, use_miss): (u32, u32, bool, bool) = match ebx {
                    0 => (T_D0A, T_D0B, false, false),
                    1 => (T_D1A, T_D1B, false, false),
                    2 => (T_D2A, T_D2B, false, false),
                    _ => (T_D3A, T_D3B, true, true),
                };
                if !use_svc_ptr && rd8(name) == 0 {
                    return d3 & 0xffffff00;
                }
                let feed: u32 = if use_svc_ptr {
                    svc
                } else {
                    lf_checker_rt::callee_thiscall!(7, u32, a1)
                };
                let ea = fmt(ebase, lf_checker_rt::relocated(ta), feed);
                let da = chk(edi, ea);
                if (da as u8) == 0 {
                    return da & 0xffffff00;
                }
                let ec_arg: u32 = if use_miss {
                    lf_checker_rt::relocated(T_MISS)
                } else if rd8(name) == 0 {
                    lf_checker_rt::relocated(P_EMPTY)
                } else {
                    name
                };
                let eb = fmt(ebase, lf_checker_rt::relocated(tb), ec_arg);
                let db = chk(edi, eb);
                if (db as u8) == 0 {
                    return db & 0xffffff00;
                }
            } else if ebx != 4 {
                return d3 & 0xffffff00;
            }

            // Tail: format and measure the scratch path twice. Callees 8 and 9
            // take their buffer in ecx but pop nothing (the original cleans
            // up later, proven by its address arithmetic), modelled with the
            // checker's noclean option: plain return on the original side,
            // argument pop on this side.
            let _h: u32 = lf_checker_rt::callee_thiscall!(8, u32, sstr, 0x101u32);
            let _ = a3;
            *((sbase) as *mut u8) = 0x2f;
            *((sbase.wrapping_add(1)) as *mut u8) = 0x5c;
            let len1 = strlen(sstr, &mut eax);
            // Note the order: the F+0xf pointer is pushed last, so it is
            // argument 0 and the F+0xe pointer is argument 1.
            let _i: u32 = lf_checker_rt::callee_thiscall!(
                9,
                u32,
                sstr,
                sbase.wrapping_add(1),
                sbase
            );
            let _ = len1;
            let j: u32 = lf_checker_rt::callee_thiscall!(10, u32, sstr);
            eax = j;
            let len2 = strlen(sstr, &mut eax);
            if len2 == 0 {
                return eax;
            }
            let d_t: u32 = lf_checker_rt::callee_thiscall!(17, u32, edi, sstr);
            if (d_t as u8) == 0 {
                return d_t & 0xffffff00;
            }
            let k: u32 = lf_checker_rt::callee_thiscall!(11, u32, edi);
            ebx = k;
            if target != 0 {
                let _a1c: u32 = lf_checker_rt::callee_thiscall!(1, u32, target);
                let l: u32 = lf_checker_rt::callee_cdecl!(
                    12,
                    u32,
                    ebx,
                    lf_checker_rt::relocated(T_L)
                );
                if l == 0 {
                    return 0;
                }
                let d6 = chk(target, l.wrapping_add(0x0e));
                if (d6 as u8) == 0 {
                    return d6 & 0xffffff00;
                }
            }
            let m: u32 = lf_checker_rt::callee_cdecl!(13, u32, 0x80u32);
            let o_arg: u32 = if m != 0 {
                m
            } else {
                lf_checker_rt::callee_thiscall!(14, u32, svc)
            };
            let o: u32 = lf_checker_rt::callee_cdecl!(15, u32, o_arg);
            let alt: u32 = if (o as u8) == 0 {
                lf_checker_rt::relocated(T_ALT0)
            } else {
                lf_checker_rt::relocated(T_ALT1)
            };
            // The format call's buffer is the same scratch string: the 82ec
            // word is still outstanding, so the lea lands back at its base.
            let _e9 = fmt(sstr, alt, o_arg);
            let p: u32 = lf_checker_rt::callee_thiscall!(16, u32, edi, sstr);
            if (p as u8) == 0 {
                return p & 0xffffff00;
            }
            (p & 0xffffff00) | 1
        }
    }
);
