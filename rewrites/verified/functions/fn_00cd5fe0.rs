// original: 0x00cd5fe0 euphoria_effector_apply
/// Scaled effector application over an animation channel object.
///
/// Scales a row of global rate constants by a read-only rate, stores three
/// of them into the channel object, and notifies two sub-objects with the
/// remaining scaled rates. When the channel's owner object is live and in
/// mode 0xC0, and the apply gate byte is set, it measures the absolute
/// difference between a source float and a queried float against a
/// threshold; while the difference is within threshold it normalises the
/// difference, maps it through a scripted curve call taking both vector
/// registers, and while the curve answer is positive it runs an effector
/// slot through a table-dispatched call plus two follow-up calls. A second
/// gate byte selects a further table-dispatched call with two constant
/// pi-multiple triples and an optional ten-argument call, and a third gate
/// byte plus a re-check of the mode selects a flag update on the channel.
///
/// Float operation order matches the original exactly (operands pinned), and
/// the two float branches mirror their vector-compare-plus-branch pairs,
/// including the unordered cases: the threshold skip fires only on ordered
/// greater, the non-positive skip fires on less-or-equal or unordered.
export!(thiscall, rw_cd5fe0(this: *mut u8, arg0: *mut u8) -> u32 {
    unsafe {
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        // Field shorthands: the object layouts are game-defined; offsets are
        // the original's.
        let r32 = |p: *const u8, off: usize| *(p.add(off) as *const u32);
        let rf = |p: *const u8, off: usize| f32::from_bits(r32(p, off));
        let wf = |p: *mut u8, off: usize, v: f32| {
            *(p.add(off) as *mut u32) = v.to_bits();
        };
        const RATE_LO: usize = 0xBB0;
        const MODE_MASK: u32 = 0x3C0;
        const MODE_LIVE: u32 = 0xC0;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const TABLE: u32 = 0x1295CD8;

        let scale = f32::from_bits(*global::<u32>(0xFE8728));
        wf(
            arg0,
            0xC44,
            mul(f32::from_bits(*global::<u32>(0x1051974)), scale),
        );
        wf(
            arg0,
            0xC40,
            mul(f32::from_bits(*global::<u32>(0x1051978)), scale),
        );
        let esi = arg0.add(RATE_LO) as u32;
        callee_thiscall!(
            1,
            u32,
            esi,
            mul(f32::from_bits(*global::<u32>(0x105197C)), scale).to_bits()
        );
        callee_thiscall!(
            2,
            u32,
            esi,
            mul(
                f32::from_bits(*global::<u32>(0x1051980)),
                f32::from_bits(*global::<u32>(0xFE8728))
            )
            .to_bits()
        );
        wf(
            arg0,
            0xC58,
            mul(
                f32::from_bits(*global::<u32>(0x171C944)),
                f32::from_bits(*global::<u32>(0xFE8728))
            ),
        );
        let obj = r32(this, 0x50) as *mut u8;
        if obj.is_null() {
            return 0;
        }
        if r32(obj, 0x28) & MODE_MASK != MODE_LIVE {
            return 0;
        }
        if *global::<u8>(0x171C941) == 0 {
            // Apply gate clear: skip the curve region, join the slot region.
            return slot_region(this, arg0, esi, obj, r32);
        }
        let src = r32(arg0, 0x20) as *const u8;
        let saved = rf(src, 0x38);
        // The query fills a four-word struct; only the third word is read.
        let mut cell = [0u32; 4];
        callee_thiscall!(3, u32, obj as u32, cell.as_mut_ptr() as u32, obj as u32);
        let diff = sub(saved, f32::from_bits(cell[2]));
        let ax = f32::from_bits(diff.to_bits() & ABS_MASK);
        let th = f32::from_bits(*global::<u32>(0x1051984));
        // Threshold skip fires only on ordered greater (NaN falls through).
        if ax > th {
            return slot_region(this, arg0, esi, obj, r32);
        }
        let quot = div(ax, th);
        let x0 = sub(f32::from_bits(*global::<u32>(0xFE88E8)), quot);
        let x1 = f32::from_bits(*global::<u32>(0x1051988));
        // The curve stub answers in XMM0 and EAX; take the bits from EAX.
        let ans = f32::from_bits(callee_cdecl!(4, u32, x0.to_bits(), x1.to_bits()));
        // Non-positive skip fires on less-or-equal or unordered.
        if !(ans > 0.0) {
            return slot_region(this, arg0, esi, obj, r32);
        }
        let idx = *(obj.add(0x2E) as *const i16) as i32;
        let slot_obj = *global::<u32>(TABLE).offset(idx as isize);
        let r5 = slot_call(slot_obj, 5);
        let r6 = callee_thiscall!(6, u32, obj as u32, r5);
        callee_thiscall!(7, u32, esi, r6.wrapping_add(0x30), ans.to_bits());
        slot_region(this, arg0, esi, obj, r32)
    }
});

/// Call the table-dispatched slot shared by both indirect sites.
fn slot_call(slot_obj: u32, arg: u32) -> u32 {
    unsafe {
        let vtable = *(slot_obj as *const u32);
        let target = *((vtable + 0x38) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(slot_obj, arg)
    }
}

/// Slot region and tail of the effector application (shared join point).
///
/// Runs the second table-dispatched slot with its follow-ups behind the
/// slot gate byte, then the flag update behind the tail gate byte.
fn slot_region(
    this: *mut u8,
    arg0: *mut u8,
    esi: u32,
    obj: *mut u8,
    r32: impl Fn(*const u8, usize) -> u32,
) -> u32 {
    unsafe {
        const TABLE: u32 = 0x1295CD8;
        const TAG_ADDR: u32 = 0xEDA2F8;
        const MODE_MASK: u32 = 0x3C0;
        const MODE_LIVE: u32 = 0xC0;
        if *global::<u8>(0x1051970) != 0 {
            let idx = *(obj.add(0x2E) as *const i16) as i32;
            let slot_obj = *global::<u32>(TABLE).offset(idx as isize);
            let r5 = slot_call(slot_obj, 7);
            let r6 = callee_thiscall!(6, u32, obj as u32, r5);
            let mut triple_hi = [0x3FC9_0FDB_u32, 0, 0x4049_0FDB_u32];
            let mut triple_lo = [0xBFC9_0FDB_u32, 0, 0xC049_0FDB_u32];
            let al = callee_thiscall!(
                8,
                u32,
                esi,
                r6.wrapping_add(0x30),
                triple_lo.as_mut_ptr() as u32,
                triple_hi.as_mut_ptr() as u32
            ) as u8;
            if al != 0 {
                callee_thiscall!(
                    9,
                    u32,
                    esi,
                    relocated(TAG_ADDR),
                    0,
                    r32(this, 0x50),
                    0x64,
                    0x4B5,
                    0,
                    0,
                    0x1F4,
                    0x1F4,
                    1
                );
            }
        }
        if *global::<u8>(0x1051971) == 0 {
            return 0;
        }
        if *arg0.add(0x219) != 0 {
            *((esi + 0x30) as *mut u32) |= 2;
            return 0;
        }
        let obj2 = r32(this, 0x50) as *mut u8;
        if r32(obj2, 0x28) & MODE_MASK != MODE_LIVE {
            return 0;
        }
        if *obj2.add(0x219) == 0 {
            return 0;
        }
        *((esi + 0x30) as *mut u32) |= 2;
        0
    }
}
