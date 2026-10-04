// original: 0x00891320 aud_handle_adopt
/// Adopts handles into the looked-up target after a chain of guards.
///
/// Resolves the target from the byte at 0x48 (0xff means null) and the table
/// index byte. Returns early while the global kill-switch is set, the gate
/// callee vetoes (nonzero low byte), the bytes at 5/4 read 0xff (the latter
/// skipping the probe call whose pointed-to byte must read 0xff), or bit 7
/// of the byte at 0x39 is clear. Then each non-null handle field is adopted
/// into the matching target field unless that field is already set. A zero
/// word at 0x44 tail-calls the finish routine, otherwise flag bit 0 is set.
export!(thiscall, rw_00891320(this: *mut u8) -> () {
    unsafe {
        let b48 = *(this.add(0x48));
        let target = if b48 == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x115d964);
            let table = *global::<u32>(0x115d988);
            let idx = *(this.add(0x40)) as u32;
            let entry = *((table
                .wrapping_add(idx.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32);
            stride.wrapping_mul(b48 as u32).wrapping_add(entry)
        };
        if *global::<u8>(0x1030393) != 0 {
            return;
        }
        let gate: u32 = callee_cdecl!(1, u32,);
        if gate & 0xff != 0 {
            return;
        }
        if *(this.add(5)) == 0xff {
            return;
        }
        if *(this.add(4)) != 0xff {
            let p: u32 = callee_thiscall!(2, u32, this as u32, 0);
            if *((p.wrapping_add(4)) as *const u8) != 0xff {
                return;
            }
        }
        if *(this.add(0x39)) & 0x80 == 0 {
            return;
        }
        let f0 = *(this.add(0x0c) as *const u32);
        if f0 != 0 && *((target.wrapping_add(0x0c)) as *const u32) != 0 {
            return;
        }
        let f1 = *(this.add(0x10) as *const u32);
        if f1 != 0 && *((target.wrapping_add(0x10)) as *const u32) != 0 {
            return;
        }
        let f2 = *(this.add(0x14) as *const u32);
        if f2 != 0 && *((target.wrapping_add(0x14)) as *const u32) != 0 {
            return;
        }
        let f3 = *(this.add(0x18) as *const u32);
        if f3 != 0 && *((target.wrapping_add(0x18)) as *const u32) != 0 {
            return;
        }
        if f0 != 0 {
            *((target.wrapping_add(0x0c)) as *mut u32) = f0;
        }
        if f1 != 0 {
            *((target.wrapping_add(0x10)) as *mut u32) = f1;
        }
        if f2 != 0 {
            *((target.wrapping_add(0x14)) as *mut u32) = f2;
        }
        if f3 != 0 {
            *((target.wrapping_add(0x18)) as *mut u32) = f3;
        }
        if *(this.add(0x44) as *const u32) == 0 {
            let _: u32 = callee_thiscall!(3, u32, this as u32);
            return;
        }
        *(this.add(0x3a)) |= 1;
    }
});
