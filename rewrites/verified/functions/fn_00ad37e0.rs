// original: 0x00ad37e0 audio_bank_refresh
/// Refresh audio banks when the shared flag is set, else return early.
///
/// With no arguments. Reads the shared object pointer; when null, or when
/// its slot-0x24 probe answers anything but 15, or when the shared flag byte
/// is clear, returns at once. Otherwise clears the flag and runs three
/// fetch/set/commit groups (fetch takes (16, 0); set takes the fetch result
/// with (10, 1), (6, 1), (15, 0); a null fetch commits 0), then scans the
/// shared array object: for each of its `count` entries whose marker byte
/// lacks bit 7, whose computed slot is non-null and whose state word is 2,
/// calls slot 0x8c with (29, 0, 255, -1). Finally runs three measure blocks
/// (fetch (16, 0) then set (10, 1), (6, 1), (15, 15)): each calls slot 8
/// twice, combines the answers with signed remainder-16 arithmetic and
/// installs bits 14..24 of the object's word at +4.
///
/// Original: 0x00ad37e0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00ad37e0() -> u32 {
    unsafe {
        const SHARED_PTR: u32 = 0x012fb1b8;
        const FLAG_BYTE: u32 = 0x013b0ee4;
        const ARRAY_OBJ: u32 = 0x012e22a4;
        const PROBE_CALLEE: u32 = 1;
        const FETCH_CALLEE: u32 = 2;
        const SET_CALLEE: u32 = 3;
        const COMMIT_CALLEE: u32 = 4;
        const LOOP_CALLEE: u32 = 5;
        const MEASURE_CALLEE: u32 = 6;
        const READY_CODE: u32 = 15;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let shared = lf_checker_rt::global::<u32>(SHARED_PTR).read();
        if shared == 0 {
            return 0;
        }
        let probe_ans: u32 = {
            let vt = rd32(shared);
            let tgt = rd32(vt.wrapping_add(0x24));
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(shared)
        };
        let _ = PROBE_CALLEE;
        if probe_ans != READY_CODE {
            return probe_ans;
        }
        let flag = lf_checker_rt::global::<u8>(FLAG_BYTE).read();
        if flag == 0 {
            return probe_ans;
        }
        lf_checker_rt::global::<u8>(FLAG_BYTE).write(0);
        #[inline(always)]
        unsafe fn group(a0: u32, a1: u32) {
            unsafe {
                const FETCH_CALLEE: u32 = 2;
                const SET_CALLEE: u32 = 3;
                const COMMIT_CALLEE: u32 = 4;
                let v: u32 = lf_checker_rt::callee_cdecl!(
                    FETCH_CALLEE, u32, 0x10u32, 0u32);
                let c: u32 = if v != 0 {
                    lf_checker_rt::callee_thiscall!(
                        SET_CALLEE, u32, v, a0, a1)
                } else {
                    0
                };
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    COMMIT_CALLEE, u32, c);
            }
        }
        group(0x0au32, 1u32);
        group(6u32, 1u32);
        group(0x0fu32, 0u32);
        let arr = lf_checker_rt::global::<u32>(ARRAY_OBJ).read();
        let count = rd32(arr.wrapping_add(8)) as i32;
        let mut i: i32 = 0;
        if count > 0 {
            loop {
                let bytes = rd32(arr.wrapping_add(4));
                let mark = ((i as u32).wrapping_add(bytes) as *const u8).read();
                if mark & 0x80 == 0 {
                    let stride = rd32(arr.wrapping_add(0x0c)) as i32;
                    let base = rd32(arr.wrapping_add(0)) as i32;
                    let slot = base.wrapping_add(stride.wrapping_mul(i));
                    if slot != 0 {
                        let su = slot as u32;
                        if rd32(su.wrapping_add(0x1304)) == 2 {
                            let vt = rd32(su);
                            let tgt = rd32(vt.wrapping_add(0x8c));
                            let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                                core::mem::transmute(tgt as usize);
                            let _ = LOOP_CALLEE;
                            f(su, 0x1du32, 0u32, 0xffu32, 0xffffffffu32);
                        }
                    }
                }
                i += 1;
                if i >= count {
                    break;
                }
            }
        }
        #[inline(always)]
        unsafe fn measure(a0: u32, a1: u32) -> u32 {
            unsafe {
                const FETCH_CALLEE: u32 = 2;
                const SET_CALLEE: u32 = 3;
                const MEASURE_CALLEE: u32 = 6;
                let v: u32 = lf_checker_rt::callee_cdecl!(
                    FETCH_CALLEE, u32, 0x10u32, 0u32);
                let obj: u32 = if v != 0 {
                    lf_checker_rt::callee_thiscall!(
                        SET_CALLEE, u32, v, a0, a1)
                } else {
                    0
                };
                let vt = (obj as *const u32).read_unaligned();
                let tgt = ((vt.wrapping_add(8)) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let _ = MEASURE_CALLEE;
                let r1 = (f(obj) as i32) % 16;
                let r2 = (16i32.wrapping_sub(r1)) % 16;
                let vt2 = (obj as *const u32).read_unaligned();
                let tgt2 = ((vt2.wrapping_add(8)) as *const u32).read_unaligned();
                let g: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(tgt2 as usize);
                let sum = (g(obj) as i32).wrapping_add(r2);
                let q = sum / 16;
                let field = (q as u32).wrapping_shl(14);
                let slot_ptr = obj.wrapping_add(4) as *mut u32;
                let old = slot_ptr.read_unaligned();
                let tmp = (field ^ old) & 0x01ffc000u32;
                slot_ptr.write_unaligned(old ^ tmp);
                tmp
            }
        }
        measure(0x0au32, 1u32);
        measure(6u32, 1u32);
        measure(0x0fu32, 0x0fu32)
    }
});
