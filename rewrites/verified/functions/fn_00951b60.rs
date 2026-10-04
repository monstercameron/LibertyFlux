// original: 0x00951b60 registry_release_pair
/// Releases an object through its virtual pair after registry cleanup.
///
/// The mark word at +0x2e selects the path: marks matching the first two
/// discriminators scan the main registry (releasing and clearing any
/// auxiliary bindings found there), a mark matching the third scans the side
/// registry (clearing a hit), and anything else goes direct. Every path ends
/// by invoking the object's virtual pre-release and release slots in order.
/// Returns the release answer.
export!(cdecl, rw_00951b60(obj: u32) -> u32 {
    unsafe {
        const MARK_OFF: u32 = 0x2E;
        const VT_PRE: u32 = 0x18;
        const VT_RELEASE: u32 = 0;
        const DISC_A: u32 = 0x012F_A2D8;
        const DISC_B: u32 = 0x012F_A0E0;
        const DISC_C: u32 = 0x012F_9FFC;
        const SIDE_REG: u32 = 0x0169_E7B8;
        const MAIN_REG: u32 = 0x0169_E7A0;
        const AUX_A: u32 = 0x0169_E7E8;
        const AUX_B: u32 = 0x0169_E800;
        const AUX_C: u32 = 0x0169_E84C;
        const SLOTS: u32 = 6;
        let mark = *((obj.wrapping_add(MARK_OFF)) as *const i16) as i32;
        let disc_a = *global::<i32>(DISC_A);
        let disc_b = *global::<i32>(DISC_B);
        if mark != disc_a && mark != disc_b {
            if mark != *global::<i32>(DISC_C) {
                // Direct path: no registry holds this mark.
            } else {
                let mut i = 0;
                while i < SLOTS {
                    let slot =
                        (relocated(SIDE_REG).wrapping_add(i.wrapping_mul(4))) as *mut u32;
                    if *slot == obj {
                        *slot = 0;
                        break;
                    }
                    i += 1;
                }
            }
        } else {
            let mut i = 0;
            while i < SLOTS {
                let slot =
                    (relocated(MAIN_REG).wrapping_add(i.wrapping_mul(4))) as *mut u32;
                if *slot == obj {
                    *slot = 0;
                    let a = *((relocated(AUX_A).wrapping_add(i.wrapping_mul(4)))
                        as *const u32);
                    if a != 0 {
                        callee_thiscall!(3, u32, a, 0);
                    }
                    let b = *((relocated(AUX_B).wrapping_add(i.wrapping_mul(4)))
                        as *const u32);
                    if b != 0 {
                        callee_thiscall!(3, u32, b, 0);
                    }
                    let c_addr = relocated(AUX_C).wrapping_add(i.wrapping_mul(4));
                    let c = *(c_addr as *const u32);
                    if c != 0 {
                        callee_thiscall!(4, u32, c);
                        *(c_addr as *mut u32) = 0;
                    }
                    break;
                }
                i += 1;
            }
        }
        // Every path ends with the virtual pre-release + release pair; the
        // table pointer is reloaded between them exactly like the original.
        let vt = *(obj as *const u32);
        let pre: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(VT_PRE)) as *const u32) as usize);
        pre(obj);
        let vt = *(obj as *const u32);
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(VT_RELEASE)) as *const u32) as usize);
        release(obj, 1)
    }
});
