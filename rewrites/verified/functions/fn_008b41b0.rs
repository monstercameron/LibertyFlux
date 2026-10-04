// original: 0x008b41b0 SG_DAM
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Shared count gate: report the slot unless it reaches the shared count.
unsafe fn count_gate(arg: u32, shared: u32) {
    let n = callee_cdecl!(9, u32, shared);
    if (arg as i32) < (n as i32) {
        callee_cdecl!(10, u32, shared, arg, 0);
    }
}

/// Honesty-mutant variant of the count gate: unsigned comparison.
/// Must fail on negative-slot trials with a small positive count.
unsafe fn mut_count_gate(arg: u32, shared: u32) {
    let n = callee_cdecl!(9, u32, shared);
    if arg < n {
        callee_cdecl!(10, u32, shared, arg, 0);
    }
}

/// Tail gate: for slot 12 and above with the tail flag set, report twice
/// and run a second count gate.
unsafe fn tail_gate(arg: u32, shared: u32) {
    if (arg as i32) < 0xC {
        return;
    }
    if global::<u8>(0x0116_0C32).read() == 0 {
        return;
    }
    callee_cdecl!(6, u32, shared, 0, arg, 0, 0, 0, 0);
    callee_cdecl!(6, u32, shared, 1, arg, 0, 0, 0, 0);
    let n = callee_cdecl!(9, u32, shared);
    if (arg as i32) < (n as i32) {
        callee_cdecl!(10, u32, shared, arg, 0);
    }
}

/// Resolve one slot: gather, check, report.
///
/// The collector fills five scratch slots: a 48-byte text, a second small
/// buffer, and three gate bytes (entry, lookup, return). A set entry gate
/// takes the alternate path; otherwise the checker call refreshes the
/// status word and a clear lookup gate together with the shared object's
/// flag byte selects between the text path and the lookup path.
///
/// The text path measures the text (capped at 42, or at a mode limit of 20
/// or 29 selected by two stored bytes), truncates overlong text with a
/// marker word, copies both buffers, and reports twice. The lookup path
/// resolves one or two names and reports with the resolution when it is
/// non-null. The alternate path resolves its own name and reports the
/// same way, with two extra reports plus a count report for slot 12 and
/// above. Every path
/// then runs the count gate (report unless the slot reaches the shared
/// count) and the tail gate (two reports plus a second count gate for
/// slot 12 and above when the tail flag is set).
///
/// Returns 1 on the text path and the collected return byte otherwise.
export!(cdecl, rw_008b41b0(arg: u32) -> u32 {
    /// Stored byte picking the 20 limit (equals 0x6a) or the 29 limit.
    const MODE_A: u32 = 0x0116_C250;
    /// Stored byte confirming the 29 limit when clear.
    const MODE_B: u32 = 0x0116_C253;
    /// Shared handle passed by value to the counter and reporter.
    const SHARED: u32 = 0x0116_0C10;
    /// Tail flag byte.
    const TAIL_FLAG: u32 = 0x0116_0C32;
    /// Shared object pointer.
    const OBJECT: u32 = 0x01BB_5624;
    /// Flag offset inside the shared object.
    const OBJECT_FLAG: usize = 0x169;
    /// Marker stored after a 42-cap truncation.
    const MARKER_WIDE: u32 = 0x00E7_D274;
    /// Marker stored after a mode-limit truncation.
    const MARKER_MODE: u32 = 0x00E7_D278;
    /// Object the measurer runs against.
    const MEASURER_THIS: u32 = 0x0118_D7F0;
    /// Object the name resolver runs against.
    const RESOLVER_THIS: u32 = 0x0116_BFF0;
    /// Name keys for the lookup, second lookup, and alternate paths.
    const NAME_MAIN: u32 = 0x00E7_D260;
    const NAME_SECOND: u32 = 0x00E7_D268;
    const NAME_ALT: u32 = 0x00E7_D27C;
    /// Text cap on the wide path.
    const WIDE_CAP: usize = 0x2A;

    /// Index of the first NUL in `buf` (a NUL is always stored in range).
    fn strlen(buf: &[u8; 48]) -> usize {
        let mut i = 0;
        while buf[i] != 0 {
            i += 1;
        }
        i
    }

    unsafe {
        let mut text = [0u8; 48];
        let mut aux = [0u32; 4];
        let mut gate_inner = 0u32;
        let mut gate_outer = 0u32;
        let mut status = 0u32;
        let mut spare = 0u32;
        let mut copy_a = [0u32; 4];
        let mut copy_b = [0u32; 4];
        // The slots start zeroed (mirrors the pre-collect zeroing); the
        // collector stub then fills them.
        callee_cdecl!(
            1,
            u32,
            arg,
            text.as_mut_ptr() as u32,
            aux.as_mut_ptr() as u32,
            core::ptr::addr_of_mut!(gate_inner) as u32,
            core::ptr::addr_of_mut!(gate_outer) as u32,
            core::ptr::addr_of_mut!(spare) as u32
        );
        let entry_gate = ((gate_outer >> 8) & 0xFF) as u8;
        let mut ret_byte = ((gate_outer >> 16) & 0xFF) as u8;
        let shared = global::<u32>(SHARED).read();
        if entry_gate != 0 {
            // Alternate path.
            let found = callee_thiscall!(
                7,
                u32,
                relocated(RESOLVER_THIS),
                relocated(NAME_ALT)
            );
            if found != 0 {
                callee_cdecl!(
                    8,
                    u32,
                    found,
                    arg.wrapping_add(1),
                    0xFFFFFFFF,
                    0xFFFFFFFF,
                    0xFFFFFFFF,
                    0xFFFFFFFF,
                    0xFFFFFFFF,
                    0,
                    0xFFFFFFFF,
                    copy_b.as_mut_ptr() as u32
                );
            }
            callee_cdecl!(
                6,
                u32,
                shared,
                0,
                arg,
                copy_b.as_mut_ptr() as u32,
                0,
                0,
                0
            );
            callee_cdecl!(6, u32, shared, 1, arg, 0, 0, 0, 0);
            if global::<u8>(TAIL_FLAG).read() == 0 {
                if (arg as i32) >= 0xC {
                    callee_cdecl!(6, u32, shared, 0, arg, 0, 0, 0, 0);
                    callee_cdecl!(6, u32, shared, 1, arg, 0, 0, 0, 0);
                    callee_cdecl!(10, u32, shared, arg, 0);
                }
                count_gate(arg, shared);
            }
            tail_gate(arg, shared);
        } else {
            // Main path: refresh the status word, then dispatch.
            status = 0;
            callee_cdecl!(
                2,
                u32,
                arg,
                0,
                core::ptr::addr_of_mut!(status) as u32
            );
            let lookup_gate = (gate_outer & 0xFF) as u8;
            let on_lookup;
            if lookup_gate != 0 {
                on_lookup = true;
            } else {
                let obj = global::<u32>(OBJECT).read();
                callee_thiscall!(3, u32, obj);
                let flag = ((obj.wrapping_add(OBJECT_FLAG as u32)) as *const u8).read();
                on_lookup = flag != 0 && status == 0;
            }
            if !on_lookup {
                // Text path.
                ret_byte = 1;
                let mode_limit: usize =
                    if global::<u8>(MODE_A).read() == 0x6A {
                        0x14
                    } else if global::<u8>(MODE_B).read() == 0 {
                        0x1D
                    } else {
                        0x14
                    };
                let measure =
                    callee_thiscall!(4, u32, relocated(MEASURER_THIS));
                if (measure & 0xFF) != 0 {
                    let len = strlen(&text);
                    if len + 1 > WIDE_CAP {
                        text[WIDE_CAP] = 0;
                        let cut = strlen(&text);
                        let mark = global::<u32>(MARKER_WIDE).read();
                        text[cut + 1..cut + 5]
                            .copy_from_slice(&mark.to_le_bytes());
                    }
                } else {
                    let len = strlen(&text);
                    if len + 1 > mode_limit {
                        if mode_limit >= 0x80 {
                            // Provably unreachable: the limit is 20 or 29.
                            callee_cdecl!(12, u32,);
                            core::hint::unreachable_unchecked();
                        }
                        text[mode_limit] = 0;
                        let cut = strlen(&text);
                        let mark = global::<u32>(MARKER_MODE).read();
                        text[cut + 1..cut + 5]
                            .copy_from_slice(&mark.to_le_bytes());
                    }
                }
                callee_cdecl!(
                    5,
                    u32,
                    text.as_mut_ptr() as u32,
                    copy_a.as_mut_ptr() as u32
                );
                callee_cdecl!(
                    5,
                    u32,
                    aux.as_mut_ptr() as u32,
                    copy_b.as_mut_ptr() as u32
                );
                callee_cdecl!(
                    6,
                    u32,
                    shared,
                    0,
                    arg,
                    copy_a.as_mut_ptr() as u32,
                    0,
                    0,
                    0
                );
                callee_cdecl!(
                    6,
                    u32,
                    shared,
                    1,
                    arg,
                    copy_b.as_mut_ptr() as u32,
                    0,
                    0,
                    0
                );
                tail_gate(arg, shared);
            } else {
                // Lookup path.
                let mut found = callee_thiscall!(
                    7,
                    u32,
                    relocated(RESOLVER_THIS),
                    relocated(NAME_MAIN)
                );
                if lookup_gate == 0 {
                    let obj = global::<u32>(OBJECT).read();
                    callee_thiscall!(3, u32, obj);
                    let flag =
                        ((obj.wrapping_add(OBJECT_FLAG as u32)) as *const u8)
                            .read();
                    if flag != 0 && status == 0 {
                        found = callee_thiscall!(
                            7,
                            u32,
                            relocated(RESOLVER_THIS),
                            relocated(NAME_SECOND)
                        );
                    }
                }
                if found != 0 {
                    callee_cdecl!(
                        8,
                        u32,
                        found,
                        arg.wrapping_add(1),
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                        0xFFFFFFFF,
                        0,
                        0xFFFFFFFF,
                        copy_b.as_mut_ptr() as u32
                    );
                }
                callee_cdecl!(
                    6,
                    u32,
                    shared,
                    0,
                    arg,
                    copy_b.as_mut_ptr() as u32,
                    0,
                    0,
                    0
                );
                callee_cdecl!(6, u32, shared, 1, arg, 0, 0, 0, 0);
                if global::<u8>(TAIL_FLAG).read() == 0 {
                    count_gate(arg, shared);
                }
                tail_gate(arg, shared);
            }
        }
        callee_cdecl!(11, u32,);
        ret_byte as u32
    }
});
