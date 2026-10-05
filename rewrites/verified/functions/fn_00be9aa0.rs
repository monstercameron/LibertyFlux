// original: 0x00be9aa0 task_build_from_template (proposed)

/// Build a task data block `dst` from a template `tmpl`, a selector `key`,
/// an output triple slot `out3` and a spare value `extra`.
///
/// Behaviour: the flag byte at `tmpl+0x33` gates every stage. Bit 0 copies
/// the template's leading triple (one dword, two floats, bitwise) into
/// `out3` and records `out3` at `dst+0x14`. The selector word at `key+4`
/// then chooses a path: `-2` takes the direct path (bit 1 set fails; bit
/// 0x20 with `key+8` in {0x20,0x22,0x24} resolves a handle through callee 5
/// with a null base, anything else fails), any other value takes the lookup
/// path (needs bits 0x22; callee 1 maps the selector to a handle, a null
/// handle fails; bit 1 picks a sub-object by the handle's kind field at
/// `+0x28` masked with 0x3c0, either a fixed offset or callee 2; bit 0x20
/// derives a second value from bits 6..9 of the same kind field, via callee
/// 3, callee 4, callee 5 with the handle as base, or a zero for the
/// 0x66..0x68 key range, failing for other keys).
///
/// The tail runs on every non-failed path: bit 0x10 resolves through
/// callee 6 (a fixed shared object) into `dst+0x1c`, bit 8 resolves
/// `key+8` and `extra` through callee 7 into `dst+0x18`, then nine dwords
/// and three bytes are copied from the template (`dst+0x00<-tmpl+0x0c`,
/// `+0x04<-+0x10`, `+0x08<-+0x14`, `+0x24<-+0x18`, `+0x28<-+0x1c`,
/// `+0x2c<-+0x20`, `+0x34<-+0x24`, `+0x3c<-+0x28`, `+0x40<-+0x2c`, bytes
/// `+0x44<-+0x30`, `+0x45<-+0x31`, `+0x46<-+0x32`; the last byte is built
/// by per-bit inserts whose end state is a plain copy).
///
/// Returns 1 on success, 0 on failure; only the low byte is significant
/// (the original leaves the upper bytes from earlier values on failure).
///
/// Original: 0x00be9aa0 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00be9aa0(dst: u32, tmpl: u32, key: u32, out3: u32, extra: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x33;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_DIRECT: u32 = 0x100;
        const FIXED_SUB: u32 = 0x264;
        const ALT_BASE: u32 = 0x3c0;
        const SHARED_OBJ: u32 = 0x0115d9a0;
        const SEL_DIRECT: u32 = 0xffff_fffe;

        const C_MAP: u32 = 1;
        const C_SUB: u32 = 2;
        const C_ALT: u32 = 3;
        const C_ALT2: u32 = 4;
        const C_RESOLVE: u32 = 5;
        const C_SHARED: u32 = 6;
        const C_TAIL: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let flags = rd8(tmpl + FLAG_OFF);
        if flags & 1 != 0 {
            wr32(out3, rd32(tmpl));
            wr32(out3 + 4, rd32(tmpl + 4));
            wr32(out3 + 8, rd32(tmpl + 8));
            wr32(dst + 0x14, out3);
        }
        let sel = rd32(key + 4);
        if sel == SEL_DIRECT {
            if flags & 2 != 0 {
                return 0;
            }
            if flags & 0x20 != 0 {
                let k8 = rd32(key + 8);
                if k8 == 0x20 || k8 == 0x22 || k8 == 0x24 {
                    let r: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, 0u32, rd32(dst + 0x14));
                    wr32(dst + 0x20, r);
                } else {
                    return 0;
                }
            }
        } else {
            if flags & 0x22 == 0 {
                // skip to tail
            } else {
                let h: u32 = lf_checker_rt::callee_cdecl!(C_MAP, u32, 1u32, sel);
                if h == 0 {
                    return 0;
                }
                if flags & 2 != 0 {
                    let kind = rd32(h + 0x28) & KIND_MASK;
                    let q: u32 = if kind == KIND_DIRECT {
                        h.wrapping_add(FIXED_SUB)
                    } else {
                        lf_checker_rt::callee_thiscall!(C_SUB, u32, h)
                    };
                    wr32(dst + 0x0c, q);
                }
                if flags & 0x20 != 0 {
                    let k = (rd32(h + 0x28) >> 6) & 0x0f;
                    if k == 3 {
                        let r: u32 = lf_checker_rt::callee_thiscall!(C_ALT, u32, h.wrapping_add(ALT_BASE));
                        wr32(dst + 0x20, r);
                    } else if k == 2 {
                        let r: u32 = lf_checker_rt::callee_thiscall!(C_ALT2, u32, h);
                        wr32(dst + 0x20, r);
                    } else {
                        let k8 = rd32(key + 8);
                        if k8 == 0x20 || k8 == 0x22 || k8 == 0x24 {
                            let r: u32 = lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, h, rd32(dst + 0x14));
                            wr32(dst + 0x20, r);
                        } else if k8 == 0x66 || k8 == 0x67 || k8 == 0x68 {
                            wr32(dst + 0x20, 0);
                        } else {
                            return 0;
                        }
                    }
                }
            }
        }
        if flags & 0x10 != 0 {
            let r: u32 = lf_checker_rt::callee_thiscall!(C_SHARED, u32, lf_checker_rt::relocated(SHARED_OBJ), rd32(key));
            wr32(dst + 0x1c, r);
        }
        if flags & 8 != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(C_TAIL, u32, rd32(key + 8), extra);
            wr32(dst + 0x18, r);
        }
        wr32(dst, rd32(tmpl + 0x0c));
        wr32(dst + 0x04, rd32(tmpl + 0x10));
        wr32(dst + 0x08, rd32(tmpl + 0x14));
        wr32(dst + 0x24, rd32(tmpl + 0x18));
        wr32(dst + 0x28, rd32(tmpl + 0x1c));
        wr32(dst + 0x2c, rd32(tmpl + 0x20));
        wr32(dst + 0x34, rd32(tmpl + 0x24));
        wr32(dst + 0x3c, rd32(tmpl + 0x28));
        wr32(dst + 0x40, rd32(tmpl + 0x2c));
        wr8(dst + 0x44, rd8(tmpl + 0x30));
        wr8(dst + 0x45, rd8(tmpl + 0x31));
        wr8(dst + 0x46, rd8(tmpl + 0x32));
        1
    }
});
