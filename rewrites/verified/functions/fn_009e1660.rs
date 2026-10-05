// original: 0x009e1660 audio_table_scan
/// Scan two tagged record tables and report live entries (cdecl).
///
/// `arg0` selects an object through a word-sized key into a global pointer
/// table (null selects nothing and returns 0); `arg1` is a count that must
/// be positive (otherwise `arg0` is returned); the low byte of `arg2` picks
/// between two report calls. The first pass walks `count1` slots: each slot
/// indexes a global word table, values inside the object's range resolve
/// through a scripted lookup, and records whose tag word is valid are
/// reported. The second pass walks `count2` groups of four entries the same
/// way. Returns the last value produced along the executed path, or 0/arg0
/// on the early exits.
///
/// Both lookups run as this-call style callees with the object's word at
/// `0x70` in the register argument.
export!(cdecl, rw_9e1660(arg0: *const u8, arg1: i32, arg2: u32) -> u32 {
    unsafe {
        if arg0.is_null() {
            return 0;
        }
        if arg1 <= 0 {
            return arg0 as u32;
        }
        let mut eax: u32;
        let idx = *(arg0.add(0x2E) as *const i16) as i32;
        eax = idx as u32;
        let obj = (*(global::<u32>(0x1295CD8).add(idx as usize))) as usize as *const u8;
        let ebx0 = *(obj.add(0x70) as *const u32);
        let ans1 = callee_thiscall!(1, u32, ebx0, arg1 as u32);
        eax = ans1;
        let rec = (ans1 as usize) as *const u8;
        let count1 = *(rec as *const u32);
        eax = count1;
        let flag = (arg2 & 0xFF) as u8;
        if count1 != 0 {
            let mut esi: u32 = 0;
            loop {
                let base = *(rec.add(8) as *const u32);
                eax = base.wrapping_add(esi);
                let tword = *(global::<u16>(0x164BA98).add(eax as usize)) as i16 as i32;
                eax = tword as u32;
                let edx = *(obj.add(0x60) as *const u32);
                let diff = (*(obj.add(0x64) as *const u32)).wrapping_sub(edx);
                eax = diff;
                if tword < diff as i32 {
                    let slot = edx.wrapping_add(tword as u32);
                    let ans2 = callee_cdecl!(2, u32,);
                    eax = ans2;
                    let ptr =
                        (slot << 5).wrapping_add(*((ans2 as usize as *const u32).add(2)));
                    if ptr != 0 {
                        let sv = (*((ptr as usize as *const u32).add(3)) as i32) >> 12;
                        eax = sv as u32;
                        if sv > -1 {
                            let g = *global::<u32>(0x12B4138);
                            if flag != 0 {
                                eax = callee_cdecl!(4, u32, sv as u32, g, 0x80);
                            } else {
                                eax = callee_cdecl!(5, u32, sv as u32, g);
                            }
                        }
                    }
                }
                esi = esi.wrapping_add(1);
                if esi >= count1 {
                    break;
                }
            }
        }
        let count2 = *(rec.add(4) as *const u32);
        eax = count2;
        if count2 != 0 {
            let mut counter: u32 = 0;
            loop {
                let base2 = *(rec.add(0x10) as *const u32);
                eax = base2.wrapping_add(counter);
                let w = *(global::<u16>(0x16556D8).add(eax as usize)) as i16 as i32;
                eax = w as u32;
                let ans3 = callee_thiscall!(3, u32, ebx0, w as u32);
                eax = ans3;
                let mut inner = ans3.wrapping_add(0x50);
                let mut n: u32 = 4;
                loop {
                    let s = (*(inner as usize as *const u32))
                        .wrapping_add(*(obj.add(0x60) as *const u32));
                    let ans2b = callee_cdecl!(2, u32,);
                    eax = ans2b;
                    let ptr2 =
                        (s << 5).wrapping_add(*((ans2b as usize as *const u32).add(2)));
                    if ptr2 != 0 {
                        let sv2 = (*((ptr2 as usize as *const u32).add(3)) as i32) >> 12;
                        eax = sv2 as u32;
                        if sv2 > -1 {
                            let g = *global::<u32>(0x12B4138);
                            if flag != 0 {
                                eax = callee_cdecl!(4, u32, sv2 as u32, g, 0x80);
                            } else {
                                eax = callee_cdecl!(5, u32, sv2 as u32, g);
                            }
                        }
                    }
                    inner = inner.wrapping_add(4);
                    n -= 1;
                    if n == 0 {
                        break;
                    }
                }
                counter = counter.wrapping_add(1);
                if counter >= count2 {
                    break;
                }
            }
        }
        eax
    }
});
