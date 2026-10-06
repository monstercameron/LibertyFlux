// original: 0x00b81b90 GtaThread::vf4
/// Run the thread's mode step, then its tag hooks and base update.
///
/// `this` points to the thread record. The global object at 0x012BD0C4 is
/// asked to retire `this` through its second vtable slot (callee 1,
/// thiscall, one stack word). Then the signed mode byte at offset 0x9E
/// selects one step: mode 0 resolves a slot through helper id 5 (thiscall,
/// one stack word, object from 0x018B6F1C) and, unless the answer is null,
/// clears bit 17 of the word at answer offset 0x260; mode 1 resolves
/// through helper id 4 (object from 0x01632C60) and, unless null, rewrites
/// the word at answer offset 0x210 to hold 0x6 in its top three bits; mode
/// 2 fetches the shared table holder through helper id 2 (cdecl, no
/// arguments), reads the table at answer offset 8, and, unless
/// table-plus-index (the dword at offset 0xA0 times 120) is null, activates
/// the entry found there through its vtable slot 0x3C (callee 3, thiscall,
/// no stack words) and marks the answered record's word at offset 0x6E
/// with 1. Any other mode skips the step. Then, when the tag dword at
/// offset 4 equals the global at 0x0167E2C8, helper id 6 fires (cdecl, no
/// arguments), and when it equals the global at 0x0167E2CC, helper id 7
/// fires. Helper id 8 (thiscall, no stack words) always runs on `this`,
/// and the function leaves through the tail transfer to helper id 9
/// (thiscall, no stack words), answering whatever that call answers.
///
/// Edge cases: a null helper answer skips its store; a null table address
/// skips the entry activation; either tag hook fires only on an exact tag
/// match.
///
/// Original: thiscall, `this` in ECX, no stack arguments, answer in EAX.
lf_checker_rt::export!(thiscall, rw_00b81b90(this: u32) -> u32 {
    unsafe {
        const G_OBJ: u32 = 0x012bd0c4;
        const G_MOD1: u32 = 0x01632c60;
        const G_MOD0: u32 = 0x018b6f1c;
        const G_TAG1: u32 = 0x0167e2c8;
        const G_TAG2: u32 = 0x0167e2cc;
        let edi = this;
        let obj = lf_checker_rt::global::<u32>(G_OBJ).read_unaligned();
        let vt = (obj as *const u32).read_unaligned();
        let tgt = ((vt.wrapping_add(4)) as *const u32).read_unaligned();
        let retire: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(tgt as usize) };
        let _ra = retire(obj, edi);
        let mode = ((edi.wrapping_add(0x9e)) as *const i8).read() as i32;
        if mode == 0 {
            let arg = ((edi.wrapping_add(0xa0)) as *const u32).read_unaligned();
            let m0 = lf_checker_rt::global::<u32>(G_MOD0).read_unaligned();
            let r: u32 = lf_checker_rt::callee_thiscall!(5, u32, m0, arg);
            if r != 0 {
                let p = (r.wrapping_add(0x260)) as *mut u32;
                p.write_unaligned(p.read_unaligned() & 0xfffdffff);
            }
        } else if mode == 1 {
            let arg = ((edi.wrapping_add(0xa0)) as *const u32).read_unaligned();
            let m1 = lf_checker_rt::global::<u32>(G_MOD1).read_unaligned();
            let r: u32 = lf_checker_rt::callee_thiscall!(4, u32, m1, arg);
            if r != 0 {
                let p = (r.wrapping_add(0x210)) as *mut u32;
                p.write_unaligned((p.read_unaligned() & 0x1fffffff) | 0x60000000);
            }
        } else if mode == 2 {
            let idx = ((edi.wrapping_add(0xa0)) as *const u32).read_unaligned();
            let b: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
            let table = ((b.wrapping_add(8)) as *const u32).read_unaligned();
            let entry = table.wrapping_add(idx.wrapping_mul(15).wrapping_mul(8));
            if entry != 0 {
                let vt2 = (entry as *const u32).read_unaligned();
                let tgt2 = ((vt2.wrapping_add(0x3c)) as *const u32).read_unaligned();
                let activate: extern "thiscall" fn(u32) -> u32 =
                    unsafe { core::mem::transmute(tgt2 as usize) };
                let r2 = activate(entry);
                ((r2.wrapping_add(0x6e)) as *mut u16).write_unaligned(1);
            }
        }
        let tag = ((edi.wrapping_add(4)) as *const u32).read_unaligned();
        if tag == lf_checker_rt::global::<u32>(G_TAG1).read_unaligned() {
            let _e: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
        }
        if tag == lf_checker_rt::global::<u32>(G_TAG2).read_unaligned() {
            let _f: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        }
        let _g: u32 = lf_checker_rt::callee_thiscall!(8, u32, edi);
        lf_checker_rt::callee_thiscall!(9, u32, edi)
    }
});
