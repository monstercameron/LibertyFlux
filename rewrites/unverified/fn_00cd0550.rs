// original: 0x00CD0550 melee_dispatch_event (proposed)

/// Route a melee event by the target's current action type.
///
/// `this` is the melee task object (linked object at `+LINK`, state id at
/// `+STATE`, mode pair at `+MODE0/MODE1`); `arg` is the subject (vtable-ish
/// link at `+SUB_LINK`, position link at `+SUB_POS`). Both objects are read
/// and partly updated. Returns a handler result, or the linked object when
/// nothing applies (thiscall, one stack word).
///
/// Behaviour: the linked object's secondary slot is queried twice through
/// its virtual slot `+VT_QUERY` (a null slot answers `0xC8`). If the first
/// answer equals `TAG_A`, path A runs: a one-argument primer call, a
/// position triple (queried by id, or zeros when there is no id), and a
/// planar distance check against a constant radius. Inside the radius a
/// single callback runs and the linked object is returned; outside, an event
/// is created through the event call, an empty vtable-gated hook may set a
/// flag bit, a teardown call runs, and the event result is returned. If the
/// first answer differs but the second equals `TAG_B`, path B runs: a primer
/// with zero, a lookup that must also answer `TAG_B` through its own virtual
/// slot, a three-way mode switch over float thresholds (any other mode keeps
/// zeros), an optional float push through a data-table slot that also stores
/// the mode and a flag bit, the same position triple and a squared-distance
/// check, and finally event creation, a three-argument attach call, a second
/// data call, mode reset and teardown, returning the event result. Any other
/// combination returns the linked object.
///
/// Original: 0x00CD0550 (thiscall, one stack word; five indirect calls).
lf_checker_rt::export!(thiscall, rw_00CD0550(this: u32, arg: u32) -> u32 {
    // Path A: first query answered TAG_A.
    unsafe fn path_a(this: u32, arg: u32) -> u32 {
        unsafe {
            const SUB_LINK: u32 = 0xa80;
            const SUB_POS: u32 = 0x20;
            const STATE: u32 = 0x50;
            const K_RADIUS: u32 = 0x1051944;
            const EVT_ID: u32 = 0x1b1;
            const EVT_TAG: u32 = 0xeb3acc;
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
            unsafe fn rd32x(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            lf_checker_rt::callee_thiscall!(1, u32, rd32x(arg + SUB_LINK), 1u32);
            let pid = rd32x(this + STATE);
            let mut buf = [0u32; 3];
            let triple: u32 = if pid == 0 {
                buf.as_ptr() as u32
            } else {
                lf_checker_rt::callee_cdecl!(2, u32, buf.as_mut_ptr() as u32, pid)
            };
            let fx = f32::from_bits(rd32x(triple));
            let fy = f32::from_bits(rd32x(triple + 4));
            let fz = f32::from_bits(rd32x(triple + 8));
            let out3 = [fx.to_bits(), fy.to_bits(), fz.to_bits()];
            let base = rd32x(arg + SUB_POS);
            let dx = sub(
                fx,
                f32::from_bits(rd32x(base + 0x30)),
            );
            let dy = sub(
                fy,
                f32::from_bits(rd32x(base + 0x34)),
            );
            let d2 = add(mul(dx, dx), mul(dy, dy));
            let k = f32::from_bits(rd32x(lf_checker_rt::relocated(K_RADIUS)));
            if !(d2 > mul(k, k)) {
                lf_checker_rt::callee_thiscall!(3, u32, arg, out3.as_ptr() as u32);
                return rd32x(this + 0x08);
            }
            let ev: u32 =
                lf_checker_rt::callee_thiscall!(4, u32, this, arg, EVT_ID, 1u32);
            lf_checker_rt::callee_thiscall!(5, u32, buf.as_ptr() as u32);
            let mut eb = [
                lf_checker_rt::relocated(EVT_TAG),
                buf[1],
                buf[2],
                0u32,
                ev,
                0x4au32,
            ];
            let t8 = rd32x(this + 0x08);
            if unsafe { ((t8 + 0x0c) as *const u8).read() } & 1 == 0 {
                let hook: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32x(rd32x(t8) + 0x14) as usize) };
                let ok: u32 = hook(t8, arg, 2u32, eb.as_ptr() as u32);
                if (ok as u8) != 0 {
                    unsafe {
                        ((t8 + 0x0c) as *mut u32)
                            .write_unaligned(rd32x(t8 + 0x0c) | 2);
                    }
                }
            }
            eb[0] = lf_checker_rt::relocated(EVT_TAG);
            lf_checker_rt::callee_thiscall!(6, u32, eb.as_ptr() as u32);
            ev
        }
    }

    // Path B: first query missed but the second answered TAG_B.
    unsafe fn path_b(this: u32, arg: u32) -> u32 {
        unsafe {
            const SUB_LINK: u32 = 0xa80;
            const SUB_POS: u32 = 0x20;
            const STATE: u32 = 0x50;
            const MODE0: u32 = 0xd4;
            const MODE1: u32 = 0xd8;
            const TAG_B: u32 = 0x38b;
            const EVT_ID: u32 = 0x1b1;
            const EVT_TAG: u32 = 0xeb3acc;
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
            unsafe fn rd32y(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            unsafe fn kf(va: u32) -> f32 {
                unsafe {
                    f32::from_bits(
                        (lf_checker_rt::relocated(va) as *const u32).read_unaligned(),
                    )
                }
            }
            lf_checker_rt::callee_thiscall!(7, u32, rd32y(arg + SUB_LINK), 0u32);
            let t8 = rd32y(this + 0x08);
            let e: u32 = lf_checker_rt::callee_thiscall!(8, u32, t8, arg);
            if e == 0 {
                return rd32y(this + 0x08);
            }
            let kind: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32y(rd32y(e) + 0x0c) as usize) };
            if kind(e) != TAG_B {
                return rd32y(this + 0x08);
            }
            let mode = rd32y(this + MODE1);
            let (x0, xc, bflag) = match mode {
                0 => (kf(0x105190c), kf(0x1051910), 1u32),
                1 => (kf(0x1051914), kf(0x1051918), 0u32),
                2 => (kf(0x105192c), kf(0x1051930), 0u32),
                _ => (0.0, 0.0, 0u32),
            };
            if rd32y(this + MODE0) != mode {
                let o = rd32y(e + 0x14);
                let push: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32y(o + 0x28) as usize) };
                push(e + 0x14, x0.to_bits());
                let old = unsafe { ((e + 0x78) as *const u8).read() };
                let mut al = (bflag as u8) << 3;
                al ^= old;
                al &= 8;
                unsafe { ((e + 0x78) as *mut u8).write(old ^ al) };
                unsafe { ((this + MODE0) as *mut u32).write_unaligned(mode) };
            }
            let pid = rd32y(this + STATE);
            let mut buf = [0u32; 3];
            let triple: u32 = if pid == 0 {
                buf.as_ptr() as u32
            } else {
                lf_checker_rt::callee_cdecl!(9, u32, buf.as_mut_ptr() as u32, pid)
            };
            let base = rd32y(arg + SUB_POS);
            let u = sub(
                f32::from_bits(rd32y(triple)),
                f32::from_bits(rd32y(base + 0x30)),
            );
            let v = sub(
                f32::from_bits(rd32y(triple + 4)),
                f32::from_bits(rd32y(base + 0x34)),
            );
            let d2 = add(mul(u, u), mul(v, v));
            if !(mul(xc, xc) > d2) {
                return rd32y(this + 0x08);
            }
            let ev: u32 =
                lf_checker_rt::callee_thiscall!(10, u32, this, arg, EVT_ID, 1u32);
            let slot = [0u32; 1];
            lf_checker_rt::callee_thiscall!(
                11, u32, slot.as_ptr() as u32, ev, 0x4au32, 0u32
            );
            lf_checker_rt::callee_thiscall!(12, u32, t8, arg, 2u32, buf.as_ptr() as u32);
            unsafe { ((this + MODE0) as *mut u32).write_unaligned(0) };
            unsafe { ((this + MODE1) as *mut u32).write_unaligned(0) };
            let eb = [lf_checker_rt::relocated(EVT_TAG), buf[1], buf[2], 0u32];
            lf_checker_rt::callee_thiscall!(13, u32, eb.as_ptr() as u32);
            ev
        }
    }

    unsafe {
        const LINK: u32 = 0x08;
        const SLOT: u32 = 0x14;
        const VT_QUERY: u32 = 0x0c;
        const TAG_A: u32 = 0x3ae;
        const TAG_B: u32 = 0x38b;
        const ABSENT: u32 = 0xc8;
        let t8 = ((this + LINK) as *const u32).read_unaligned();
        let o = ((t8 + SLOT) as *const u32).read_unaligned();
        let a1: u32 = if o == 0 {
            ABSENT
        } else {
            let f: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(
                    ((((o) as *const u32).read_unaligned() + VT_QUERY) as *const u32)
                        .read_unaligned() as usize,
                )
            };
            f(o)
        };
        let flag_a = u32::from(a1 == TAG_A);
        let o2 = ((t8 + SLOT) as *const u32).read_unaligned();
        let a2: u32 = if o2 == 0 {
            ABSENT
        } else {
            let f: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(
                    ((((o2) as *const u32).read_unaligned() + VT_QUERY) as *const u32)
                        .read_unaligned() as usize,
                )
            };
            f(o2)
        };
        if flag_a != 0 {
            return path_a(this, arg);
        }
        if a2 != TAG_B {
            return ((this + LINK) as *const u32).read_unaligned();
        }
        path_b(this, arg)
    }
});
