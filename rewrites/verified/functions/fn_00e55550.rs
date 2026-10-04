// original: 0x00E55550 pick_named_record
/// Look up a named record and publish it into the owner's slot (original 0x00E55550).
///
/// Opens a lookup handle from two global descriptor words, then resolves a
/// record name through the record source: each candidate name is checked, and
/// when a check succeeds the name is shortened by three bytes, stamped with
/// the global mark word and stored into a freshly fetched owner slot. Retries
/// with a new name while the source keeps answering; releases the name and the
/// handle and returns the trailing check's answer when the source runs out.
export!(stdcall, rw_e55550(owner: u32) -> u32 {
    const DESC0: u32 = 0x00F1_A364;
    const DESC1: u32 = 0x00F1_A36C;
    const TAG_OPEN: u32 = 0x00F1_A374;
    const TAG_HANDLE: u32 = 0x00F1_A384;
    const MARK: u32 = 0x00F1_A3A0;
    const TAG_CLOSE: u32 = 0x00F1_A3A4;
    const REC_SIZE: u32 = 0x20;
    const TRIM: u32 = 3;
    const SLOT_ARG: u32 = 0x10;
    let mut desc = [0u32; 4];
    unsafe {
        desc[0] = global::<u32>(DESC0).read();
        desc[1] = global::<u32>(DESC0 + 4).read();
        desc[2] = global::<u32>(DESC1).read();
        desc[3] = global::<u32>(DESC1 + 4).read();
    }
    callee_cdecl!(1, u32, relocated(TAG_OPEN), 0);
    let handle = callee_cdecl!(2, u32, desc.as_ptr() as u32, relocated(TAG_HANDLE));
    if handle == 0 {
        return callee_cdecl!(9, u32,);
    }
    let finish = |name: u32| {
        callee_cdecl!(7, u32, name);
        callee_cdecl!(8, u32, handle);
        callee_cdecl!(1, u32, relocated(TAG_CLOSE), 0);
        callee_cdecl!(9, u32,)
    };
    let mut name = callee_cdecl!(3, u32, REC_SIZE);
    if callee_cdecl!(4, u32, handle, name, REC_SIZE) == 0 {
        return finish(name);
    }
    loop {
        if callee_cdecl!(5, u32, name) as u8 != 0 {
            let mut len = 0u32;
            unsafe {
                while (name as *const u8).add(len as usize).read() != 0 {
                    len = len.wrapping_add(1);
                }
                (name.wrapping_add(len).wrapping_sub(TRIM) as *mut u8).write(0);
                let mut end = name as *const u8;
                while end.read() != 0 {
                    end = end.add(1);
                }
                (end as *mut u32).write_unaligned(global::<u32>(MARK).read());
            }
            let slot = callee_thiscall!(6, u32, owner, SLOT_ARG);
            unsafe { (slot as *mut u32).write_unaligned(name) };
            name = callee_cdecl!(3, u32, REC_SIZE);
        }
        if callee_cdecl!(4, u32, handle, name, REC_SIZE) == 0 {
            break;
        }
    }
    finish(name)
});
