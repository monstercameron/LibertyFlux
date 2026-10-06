// original: 0x006a0130 video_device_enumerator
use lf_checker_rt::{callee_cdecl, callee_stdcall, export, relocated};

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { (base.wrapping_add(off) as *const u32).read() }
}

/// Enumerate video adapters over COM/WMI and match one against the wanted id.
///
/// Initialises COM, creates the WMI locator, connects to the namespace,
/// runs the adapter query, and walks the returned objects. For each object
/// it reads one property, keeps objects whose property is a live string
/// matching the wanted class, extracts two words from it, and compares the
/// composed device id against the wanted dword at +0x0. Every allocated
/// string and interface is released on the way out, COM is uninitialised
/// when initialisation had succeeded, and the result is 1 on a match.
const CLSID_LOCATOR: u32 = 0x00FE_3F10;
const IID_SERVICES: u32 = 0x00FE_3F20;
const LIT_SERVER: u32 = 0x00FA_1484;
const LIT_QUERY: u32 = 0x00FA_14A4;
const LIT_PROP: u32 = 0x00FA_1468;
const LIT_CLASS: u32 = 0x00FA_147C;
const LIT_VID: u32 = 0x00FA_144C;
const LIT_VID_KEY: u32 = 0x00FA_1458;
const LIT_PID: u32 = 0x00FA_1430;
const LIT_PID_KEY: u32 = 0x00FA_143C;

const ID_COINIT: u32 = 1;
const ID_COCREATE: u32 = 2;
const ID_ALLOC_STR: u32 = 3;
const ID_CONNECT: u32 = 4;
const ID_BLANKET: u32 = 5;
const ID_QUERY: u32 = 6;
const ID_NEXT: u32 = 7;
const ID_GET: u32 = 8;
const ID_CMP_CLASS: u32 = 9;
const ID_CMP_VID: u32 = 10;
const ID_CMP_PID: u32 = 11;
const ID_GET_VID: u32 = 12;
const ID_GET_PID: u32 = 13;
const ID_RELEASE: u32 = 14;
const ID_FREE_STR: u32 = 15;
const ID_COUNINIT: u32 = 16;

const BATCH: usize = 0x14;

#[inline]
fn release(obj: u32) {
    let _r = callee_stdcall!(ID_RELEASE, u32, obj);
}

#[inline]
fn free_str(b: u32) {
    let _f = callee_stdcall!(ID_FREE_STR, u32, b);
}

unsafe fn finish(
    items: &mut [u32; BATCH],
    enumerator: u32,
    locator: u32,
    services: u32,
    uninit: bool,
    matched: u8,
) -> u32 {
    for slot in items.iter_mut() {
        if *slot != 0 {
            release(*slot);
            *slot = 0;
        }
    }
    if enumerator != 0 {
        release(enumerator);
    }
    if locator != 0 {
        release(locator);
    }
    if services != 0 {
        release(services);
    }
    if uninit {
        let _u = callee_stdcall!(ID_COUNINIT, u32,);
    }
    matched as u32
}

unsafe fn inner(this: u32, mutant: bool) -> u32 {
    // The original zeroes the item array with a helper call; a zeroed
    // array is the same state, so there is no call here.
    let mut items = [0u32; BATCH];
    let mut locator = 0u32;
    let mut services = 0u32;
    let mut enumerator = 0u32;
    let ci = callee_stdcall!(ID_COINIT, u32, 0);
    let uninit = (ci as i32) >= 0;
    let cc = callee_stdcall!(
        ID_COCREATE, u32,
        relocated(CLSID_LOCATOR), 0, 1, relocated(IID_SERVICES),
        core::ptr::addr_of_mut!(locator) as u32
    );
    if (cc as i32) < 0 || locator == 0 {
        return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
    }
    let bstr_server = callee_stdcall!(ID_ALLOC_STR, u32, relocated(LIT_SERVER));
    if bstr_server == 0 {
        return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
    }
    let bstr_query = callee_stdcall!(ID_ALLOC_STR, u32, relocated(LIT_QUERY));
    if bstr_query == 0 {
        free_str(bstr_server);
        return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
    }
    let bstr_prop = callee_stdcall!(ID_ALLOC_STR, u32, relocated(LIT_PROP));
    if bstr_prop == 0 {
        free_str(bstr_server);
        free_str(bstr_query);
        return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
    }
    let free_all = |bstr_server: u32, bstr_prop: u32, bstr_query: u32| {
        free_str(bstr_server);
        if bstr_prop != 0 {
            free_str(bstr_prop);
        }
        if bstr_query != 0 {
            free_str(bstr_query);
        }
    };
    let cn = callee_stdcall!(
        ID_CONNECT, u32, locator, bstr_server, 0, 0, 0, 0, 0, 0,
        core::ptr::addr_of_mut!(services) as u32
    );
    if (cn as i32) < 0 || services == 0 {
        free_all(bstr_server, bstr_prop, bstr_query);
        return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
    }
    let _b = callee_stdcall!(ID_BLANKET, u32, services, 0xA, 0, 0, 3, 3, 0, 0);
    let q = callee_stdcall!(
        ID_QUERY, u32, services, bstr_query, 0, 0,
        core::ptr::addr_of_mut!(enumerator) as u32
    );
    if (q as i32) < 0 || enumerator == 0 {
        free_all(bstr_server, bstr_prop, bstr_query);
        return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
    }
    loop {
        let mut count = 0u32;
        let nx = callee_stdcall!(
            ID_NEXT, u32, enumerator, 0x2710, 0x14,
            core::ptr::addr_of_mut!(items) as u32,
            core::ptr::addr_of_mut!(count) as u32
        );
        if (nx as i32) < 0 || count == 0 {
            free_all(bstr_server, bstr_prop, bstr_query);
            return unsafe { finish(&mut items, enumerator, locator, services, uninit, 0) };
        }
        let n = (count as usize).min(BATCH);
        for i in 0..n {
            // The property value lands 8 bytes past the type word, as in
            // the original's frame; the stub writes both words.
            let mut vbuf = [0u32; 3];
            let g = callee_stdcall!(
                ID_GET, u32, items[i], bstr_prop, 0,
                core::ptr::addr_of_mut!(vbuf) as u32, 0, 0
            );
            let typ = vbuf[0];
            let value = vbuf[2];
            let mut drop_it = (g as i32) < 0 || (typ & 0xFFFF) != 8 || value == 0;
            if !drop_it {
                let c1 = callee_cdecl!(ID_CMP_CLASS, u32, value, relocated(LIT_CLASS));
                if c1 == 0 {
                    drop_it = true;
                } else {
                    let mut vid = 0u32;
                    let mut pid = 0u32;
                    let c2 = callee_cdecl!(ID_CMP_VID, u32, value, relocated(LIT_VID));
                    if c2 != 0 {
                        let e1 = callee_cdecl!(
                            ID_GET_VID, u32, c2, relocated(LIT_VID_KEY),
                            core::ptr::addr_of_mut!(vid) as u32
                        );
                        if e1 != 1 {
                            vid = 0;
                        }
                    }
                    let c3 = callee_cdecl!(ID_CMP_PID, u32, value, relocated(LIT_PID));
                    if c3 != 0 {
                        let e2 = callee_cdecl!(
                            ID_GET_PID, u32, c3, relocated(LIT_PID_KEY),
                            core::ptr::addr_of_mut!(pid) as u32
                        );
                        if e2 != 1 {
                            pid = 0;
                        }
                    }
                    let device = ((pid & 0xFFFF) << 16) | (vid & 0xFFFF);
                    let want = unsafe { rd32(this, 0) };
                    let hit = if mutant { device != want } else { device == want };
                    if hit {
                        free_all(bstr_server, bstr_prop, bstr_query);
                        return unsafe {
                            finish(&mut items, enumerator, locator, services, uninit, 1)
                        };
                    }
                    drop_it = true;
                }
            }
            if drop_it {
                if items[i] != 0 {
                    release(items[i]);
                    items[i] = 0;
                }
            }
        }
    }
}

export!(thiscall, rw_006A0130(this: u32) -> u32 {
    unsafe { inner(this, false) }
});

export!(thiscall, mut_006A0130(this: u32) -> u32 {
    unsafe { inner(this, true) }
});
