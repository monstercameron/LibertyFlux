// original: 0x00CD0D90 melee_process_event (proposed)

/// Process one melee-task event by id.
///
/// `this` is the melee task object (id word at `+SLOT0`, state id at
/// `+STATE`, flag byte at `+FLAGS`, mode pair at `+MODE0/MODE1`); `a1` is
/// the subject (position link at `+SUB_POS`); `a2` is the event id;
/// `a3` is a flag consumed only by the create event. Returns a handler
/// result, or 0 when the event does not apply (thiscall, three stack words).
///
/// Behaviour: `0x1B1` is the create event: it clears the modes, then either
/// runs the full creation sequence (two setup calls, a flag-bit clear, a
/// seven-argument creation call driven by three flag bits, returning its
/// answer) when `a3` is set and the slot word is nonzero, or a shorter one
/// (setup calls with an inverted flag bit, then, only when flag bit 3 is
/// set, a second creation call, returning its answer). `0x38B` selects a
/// float threshold by mode (any other mode keeps zero) and a flag bit that
/// is set only for mode 0, builds an object through a seven-argument call,
/// sets a bit on it, pushes the threshold through a data-table slot, copies
/// the mode and flag bit over, and finishes through a four-argument call
/// whose answer is returned. `0x3AE` builds a position triple (queried by
/// id, or zeros), normalises the offset from the subject position unless it
/// is exactly zero (a NaN offset is normalised, producing NaNs), scales it,
/// creates an object through a ten-argument call over the scaled triple,
/// stamps it, scales a constant pair onto it, clears the modes and finishes
/// through the same four-argument call. Any other id returns 0.
/// When a factory answer is null on the `0x3AE` or `0x38B` paths the
/// function faults on a null write; the rewrite reproduces the fault.
///
/// Original: 0x00CD0D90 (thiscall, three stack words; one indirect call).
lf_checker_rt::export!(thiscall, rw_00CD0D90(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    // Event 0x3AE: scaled-offset object creation.
    unsafe fn case_c(this: u32, a1: u32) -> u32 {
        unsafe {
            const SUB_POS: u32 = 0x20;
            const STATE: u32 = 0x50;
            const MODE0: u32 = 0xd4;
            const MODE1: u32 = 0xd8;
            const CREATOR: u32 = 0x167e2a0;
            const K_ONE: u32 = 0xfe88e8;
            const K_SCALE: u32 = 0x1051948;
            const K_F3: u32 = 0xee1eb4;
            const K_F4: u32 = 0xee1eb0;
            const K_A: u32 = 0x1051944;
            const K_B: u32 = 0xfe8a24;
            #[inline(always)]
            fn sub(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) - core::hint::black_box(b)
            }
            #[inline(always)]
            fn mul(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) * core::hint::black_box(b)
            }
            #[inline(always)]
            fn add(a: f32, b: f32) -> f32 {
                core::hint::black_box(a) + core::hint::black_box(b)
            }
            unsafe fn rd32c(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            unsafe fn kfc(va: u32) -> f32 {
                unsafe {
                    f32::from_bits(
                        (lf_checker_rt::relocated(va) as *const u32).read_unaligned(),
                    )
                }
            }
            let pid = rd32c(this + STATE);
            let mut buf = [0u32; 3];
            let triple: u32 = if pid == 0 {
                buf.as_ptr() as u32
            } else {
                lf_checker_rt::callee_cdecl!(1, u32, buf.as_mut_ptr() as u32, pid)
            };
            let t0 = f32::from_bits(rd32c(triple));
            let t1 = f32::from_bits(rd32c(triple + 4));
            let t2 = f32::from_bits(rd32c(triple + 8));
            let base = rd32c(a1 + SUB_POS);
            let dx = sub(t0, f32::from_bits(rd32c(base + 0x30)));
            let dy = sub(t1, f32::from_bits(rd32c(base + 0x34)));
            let dz = sub(t2, f32::from_bits(rd32c(base + 0x38)));
            // Note the order: y squared plus x squared, then z squared.
            let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            // The original tests the ucomiss flags through lahf: exactly
            // zero skips the normalisation, anything else (including NaN)
            // takes it.
            let n = if d2 != 0.0 {
                core::hint::black_box(1.0f32) / core::hint::black_box(d2.sqrt())
            } else {
                0.0
            };
            let k = kfc(K_SCALE);
            let rx = add(mul(mul(dx, n), k), t0);
            let ry = add(mul(mul(dy, n), k), t1);
            let rz = add(mul(mul(dz, n), k), t2);
            let out3 = [rx.to_bits(), ry.to_bits(), rz.to_bits()];
            let g = lf_checker_rt::global::<u32>(CREATOR).read();
            let fac: u32 = lf_checker_rt::callee_thiscall!(2, u32, g);
            let edi: u32 = if fac == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    3, u32, fac, 0x40000000u32, out3.as_ptr() as u32,
                    rd32c(lf_checker_rt::relocated(K_F4)),
                    rd32c(lf_checker_rt::relocated(K_F3)),
                    0xffffffffu32, 1u32, 1u32, 0u32, 0u32, 1u32
                )
            };
            let stamp = add(kfc(K_A), kfc(K_SCALE));
            // Faults on a null write when the factory answered null, exactly
            // like the original (see doc comment).
            let stamp_p = (edi + 0xd8) as *mut u32;
            unsafe { stamp_p.write_unaligned(stamp_p.read_unaligned() | 0x2000000) };
            let sc = mul(stamp, kfc(K_B));
            unsafe { ((edi + 0x58) as *mut u32).write_unaligned(sc.to_bits()) };
            unsafe { ((this + MODE0) as *mut u32).write_unaligned(0) };
            unsafe { ((this + MODE1) as *mut u32).write_unaligned(0) };
            let g2 = lf_checker_rt::global::<u32>(CREATOR).read();
            let fac2: u32 = lf_checker_rt::callee_thiscall!(4, u32, g2);
            if fac2 == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(5, u32, fac2, edi, 0u32, 0u32, 0u32)
        }
    }

    // Event 0x38B: threshold object creation.
    unsafe fn case_b(this: u32, _a1: u32) -> u32 {
        unsafe {
            const STATE: u32 = 0x50;
            const MODE0: u32 = 0xd4;
            const MODE1: u32 = 0xd8;
            const CREATOR: u32 = 0x167e2a0;
            const TAG_B: u32 = 0x38b;
            const K_PUSH: u32 = 0x171c938;
            unsafe fn rd32b(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            unsafe fn kfb(va: u32) -> f32 {
                unsafe {
                    f32::from_bits(
                        (lf_checker_rt::relocated(va) as *const u32).read_unaligned(),
                    )
                }
            }
            let _ = TAG_B;
            let mode = rd32b(this + MODE1);
            let (xc, bflag) = match mode {
                0 => (kfb(0x105190c), 1u32),
                1 => (kfb(0x1051914), 0u32),
                2 => (kfb(0x105192c), 0u32),
                _ => (0.0, 0u32),
            };
            let g = lf_checker_rt::global::<u32>(CREATOR).read();
            let fac: u32 = lf_checker_rt::callee_thiscall!(6, u32, g);
            let edi: u32 = if fac == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    7, u32, fac, rd32b(this + STATE), 0xffffffffu32, 0x3e8u32,
                    xc.to_bits(), 0x40000000u32, 0x40000000u32, 1u32
                )
            };
            // Faults on a null write when the factory answered null.
            let bit_p = (edi + 0x78) as *mut u8;
            unsafe { bit_p.write(bit_p.read() | 0x40) };
            let o = rd32b(edi + 0x14);
            unsafe { ((edi + 0x54) as *mut u32).write_unaligned(0x40000000) };
            let push: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32b(o + 0x2c) as usize) };
            push(edi + 0x14, rd32b(lf_checker_rt::relocated(K_PUSH)));
            let old = unsafe { ((edi + 0x78) as *const u8).read() };
            let mut al = (bflag as u8) << 3;
            al ^= old;
            al &= 8;
            unsafe { ((edi + 0x78) as *mut u8).write(old ^ al) };
            unsafe { ((this + MODE0) as *mut u32).write_unaligned(mode) };
            let g2 = lf_checker_rt::global::<u32>(CREATOR).read();
            let fac2: u32 = lf_checker_rt::callee_thiscall!(9, u32, g2);
            if fac2 == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(10, u32, fac2, edi, 0u32, 0u32, 0u32)
        }
    }

    // Event 0x1B1: create.
    unsafe fn case_a(this: u32, a1: u32, a3: u32) -> u32 {
        unsafe {
            const SLOT0: u32 = 0x30;
            const STATE: u32 = 0x50;
            const FLAGS: u32 = 0xf0;
            const MODE0: u32 = 0xd4;
            const MODE1: u32 = 0xd8;
            const CREATOR: u32 = 0x167e2a0;
            unsafe fn rd32a(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            unsafe { ((this + MODE0) as *mut u32).write_unaligned(0) };
            unsafe { ((this + MODE1) as *mut u32).write_unaligned(0) };
            let slot = rd32a(this + SLOT0);
            if a3 != 0 && slot != 0 {
                lf_checker_rt::callee_thiscall!(11, u32, this, a1, slot, 0u32, 0u32);
                lf_checker_rt::callee_thiscall!(12, u32, this, a1);
                let f0 = unsafe { ((this + FLAGS) as *const u8).read() } & 0xfb;
                unsafe { ((this + FLAGS) as *mut u8).write(f0) };
                let g = lf_checker_rt::global::<u32>(CREATOR).read();
                let fac: u32 = lf_checker_rt::callee_thiscall!(13, u32, g);
                if fac == 0 {
                    return 0;
                }
                let f1 = unsafe { ((this + FLAGS) as *const u8).read() };
                return lf_checker_rt::callee_thiscall!(
                    14, u32, fac, slot, a1, rd32a(this + STATE),
                    u32::from((f1 >> 6) & 1), u32::from((f1 >> 1) & 1),
                    u32::from(f1 & 1), 0u32
                );
            }
            let f0 = unsafe { ((this + FLAGS) as *const u8).read() };
            let inv3 = u32::from((f0 >> 3) & 1 == 0);
            lf_checker_rt::callee_thiscall!(15, u32, this, a1, 0u32, 0u32, inv3);
            lf_checker_rt::callee_thiscall!(16, u32, this, a1);
            let f1 = unsafe { ((this + FLAGS) as *const u8).read() };
            if f1 & 8 == 0 {
                return 0;
            }
            unsafe { ((this + FLAGS) as *mut u8).write(f1 & 0xfb) };
            let g = lf_checker_rt::global::<u32>(CREATOR).read();
            let fac: u32 = lf_checker_rt::callee_thiscall!(17, u32, g);
            if fac == 0 {
                return 0;
            }
            let f2 = unsafe { ((this + FLAGS) as *const u8).read() };
            lf_checker_rt::callee_thiscall!(
                18, u32, fac, 0u32, a1, rd32a(this + STATE),
                u32::from((f2 >> 6) & 1), 0u32, 0u32, 0u32
            )
        }
    }

    unsafe {
        match a2 {
            0x1b1 => case_a(this, a1, a3),
            0x38b => case_b(this, a1),
            0x3ae => case_c(this, a1),
            _ => 0,
        }
    }
});
