// original: 0x00ad4410 audio_enqueue_sorted_triple
// ---------------------------------------------------------------------------
// 0x00AD4410: resolve three ids, order them, append to a work list.
// ---------------------------------------------------------------------------
// Three lookups through one helper turn consecutive argument triples into
// small table indices; the indices are ordered by a two-key comparison over
// a shared word table (equality on the second key selects which pair the
// signed first-key comparison decides), and the ordered triple is appended
// to a counter-indexed work list with two flag bits taken from the low byte
// of the tenth argument. Returns the pre-increment counter plus one. When
// the first arguments of the three triples are all equal, or the second
// arguments are all equal, the function returns the first argument without
// touching the list.
export!(cdecl, rw_00ad4410(a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32) -> u32 {
    unsafe {
        if a1 == a4 && a1 == a7 {
            return a1;
        }
        if a2 == a5 && a2 == a8 {
            return a1;
        }
        let r1 = callee_cdecl!(1, u32, a1, a2, a3);
        let r2 = callee_cdecl!(2, u32, a4, a5, a6);
        let r3 = callee_cdecl!(3, u32, a7, a8, a9);

        let keys = global::<u8>(0x158e860);
        let key0 = |r: u32| -> i16 {
            let off = r.wrapping_mul(8) as usize;
            *(keys.add(off) as *const i16)
        };
        let key1 = |r: u32| -> u16 {
            let off = r.wrapping_mul(8) as usize;
            *(keys.add(off + 2) as *const u16)
        };
        let k1 = key1(r1);
        let (first, second, third);
        if k1 == key1(r2) {
            if key0(r1) < key0(r2) {
                first = r1;
                second = r2;
                third = r3;
            } else {
                first = r2;
                second = r1;
                third = r3;
            }
        } else if k1 == key1(r3) {
            if key0(r1) < key0(r3) {
                first = r1;
                second = r3;
                third = r2;
            } else {
                first = r3;
                second = r1;
                third = r2;
            }
        } else if key0(r2) < key0(r3) {
            first = r2;
            second = r3;
            third = r1;
        } else {
            first = r3;
            second = r2;
            third = r1;
        }

        let n = *global::<u32>(0x154e300);
        let slot = global::<u8>(0x154e308).add(n.wrapping_mul(8) as usize);
        *(slot as *mut u16) = first as u16;
        *(slot.add(2) as *mut u16) = second as u16;
        *(slot.add(4) as *mut u16) = third as u16;
        let mut flags = (*(slot.add(6) as *const u16)) & 0xfffe;
        if (a10 as u8) & 1 != 0 {
            flags &= !2;
        } else {
            flags |= 2;
        }
        if (a10 as u8) & 2 != 0 {
            flags |= 4;
        } else {
            flags &= !4;
        }
        *(slot.add(6) as *mut u16) = flags;
        let next = n.wrapping_add(1);
        *global::<u32>(0x154e300) = next;
        next
    }
});
