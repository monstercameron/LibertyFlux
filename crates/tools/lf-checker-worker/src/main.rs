//! `lf-checker-worker`: the checker's 32-bit comparison worker.
//! Maps the original executable once, then runs many trials on stdin/stdout
//! as JSON lines. Driven by `scripts/checker/checker2.py`, whose module
//! documentation describes the protocol and the contract format.
//!
//! Usage (from the repository root):
//! `cargo build --release --target i686-pc-windows-msvc -p lf-checker-worker`
//! then run contracts with the driver; never run this binary by hand.
//! Only the i686 build ever runs: it executes original 32-bit code
//! natively. Commands: setup, trial, teardown, ping. One JSON object per
//! line on stdin, one per line on stdout; stderr is diagnostics (never JSON).
//!
//! v2 abilities: per-callee script slots, stub out-param writes, stub
//! addresses in the setup response (for driver-side vtable planting, which
//! is the indirect-call interception), E9 tail-patch sites, pointer
//! normalization with pointed-to snapshots in call keys, TLS fabrication,
//! XMM entry values, XMM0 call-arg logging with stack transport on the
//! rewrite side, defined stack fills, and below-ESP scratch reset.

// The worker maps and executes original machine code, plants stubs in it,
// and catches faults in-process: raw pointers and FFI are its whole job.
#![allow(unsafe_code)]
// Integrated lane code, proven by the checker's regression suite: address
// casts, long decoders and shared match arms are inherent to the domain,
// so pedantic style lints stay off here. Correctness lints (clippy::all)
// still apply; narrow this to per-lint allows if the code is reworked.
#![allow(clippy::pedantic)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{BufRead, Write};
use std::sync::mpsc;
use std::time::Duration;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(addr: *mut c_void, size: usize, atype: u32, prot: u32) -> *mut c_void;
    fn VirtualProtect(addr: *mut c_void, size: usize, prot: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(proc: *mut c_void, base: *const c_void, len: usize) -> i32;
    fn QueryPerformanceCounter(out: *mut i64) -> i32;
    fn QueryPerformanceFrequency(out: *mut i64) -> i32;
    fn GetCurrentProcess() -> *mut c_void;
    fn AddVectoredExceptionHandler(
        first: u32,
        h: extern "system" fn(*mut c_void) -> i32,
    ) -> *mut c_void;
    fn LoadLibraryW(name: *const u16) -> *mut c_void;
    fn GetProcAddress(h: *mut c_void, name: *const i8) -> *mut c_void;
    fn FreeLibrary(h: *mut c_void) -> i32;
    fn ExitProcess(code: u32) -> !;
}

const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const PAGE_RWX: u32 = 0x40;
const PAGE_NOACCESS: u32 = 0x01;

// ---------------------------------------------------------------------------
// Minimal JSON value + parser (zero dependencies) and emitter helpers.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum J {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Arr(Vec<J>),
    Obj(HashMap<String, J>),
}

impl J {
    fn get(&self, k: &str) -> Option<&J> {
        match self {
            J::Obj(m) => m.get(k),
            _ => None,
        }
    }
    fn as_i64(&self) -> i64 {
        match self {
            J::Int(i) => *i,
            J::Bool(b) => *b as i64,
            J::Str(s) => parse_u32(s) as i64,
            _ => 0,
        }
    }
    fn as_u32(&self) -> u32 {
        self.as_i64() as u32
    }
    fn as_usize(&self) -> usize {
        self.as_i64() as usize
    }
    fn as_str(&self) -> &str {
        match self {
            J::Str(s) => s,
            _ => "",
        }
    }
    fn as_bool(&self, d: bool) -> bool {
        match self {
            J::Bool(b) => *b,
            J::Int(i) => *i != 0,
            _ => d,
        }
    }
    fn as_arr(&self) -> &[J] {
        match self {
            J::Arr(a) => a,
            _ => &[],
        }
    }
}

fn parse_u32(t: &str) -> u32 {
    let t = t.trim();
    if let Some(x) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        u32::from_str_radix(x, 16).unwrap_or(0)
    } else if t.starts_with('-') {
        t.parse::<i32>().unwrap_or(0) as u32
    } else {
        t.parse::<u32>().unwrap_or(0)
    }
}

struct P<'a> {
    b: &'a [u8],
    p: usize,
}

fn parse_json(s: &str) -> Result<J, String> {
    let mut p = P {
        b: s.as_bytes(),
        p: 0,
    };
    let v = p.value()?;
    p.ws();
    if p.p != p.b.len() {
        return Err("trailing chars".to_string());
    }
    Ok(v)
}

impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.p < self.b.len() && matches!(self.b[self.p], b' ' | b'\t' | b'\r' | b'\n') {
            self.p += 1;
        }
    }
    fn peek(&self) -> u8 {
        if self.p < self.b.len() {
            self.b[self.p]
        } else {
            0
        }
    }
    fn value(&mut self) -> Result<J, String> {
        self.ws();
        match self.peek() {
            b'{' => self.obj(),
            b'[' => self.arr(),
            b'"' => Ok(J::Str(self.string()?)),
            b't' => self.lit("true", J::Bool(true)),
            b'f' => self.lit("false", J::Bool(false)),
            b'n' => self.lit("null", J::Null),
            c if c == b'-' || c.is_ascii_digit() => self.num(),
            c => Err(format!("unexpected char {}", c)),
        }
    }
    fn lit(&mut self, s: &str, v: J) -> Result<J, String> {
        if self.b[self.p..].starts_with(s.as_bytes()) {
            self.p += s.len();
            Ok(v)
        } else {
            Err("bad literal".to_string())
        }
    }
    fn num(&mut self) -> Result<J, String> {
        let st = self.p;
        // ints, floats (floats are parsed here, not via the Int path)
        let mut is_float = false;
        while self.p < self.b.len()
            && (self.b[self.p].is_ascii_digit()
                || matches!(self.b[self.p], b'-' | b'+' | b'.' | b'e' | b'E'))
        {
            if matches!(self.b[self.p], b'.' | b'e' | b'E') {
                is_float = true;
            }
            self.p += 1;
        }
        let t = std::str::from_utf8(&self.b[st..self.p]).map_err(|_| "bad num")?;
        if is_float {
            // store floats scaled: keep as string in J::Str would lose type; use Int of bits? simplest: Int(f*1e12)
            let f: f64 = t.parse().map_err(|_| "bad float")?;
            Ok(J::Int((f * 1e12) as i64))
        } else {
            t.parse::<i64>()
                .map(J::Int)
                .map_err(|_| "bad int".to_string())
        }
    }
    fn string(&mut self) -> Result<String, String> {
        // assumes opening quote
        self.p += 1;
        let mut out = String::new();
        while self.p < self.b.len() {
            let c = self.b[self.p];
            self.p += 1;
            match c {
                b'"' => return Ok(out),
                b'\\' => {
                    if self.p >= self.b.len() {
                        break;
                    }
                    let e = self.b[self.p];
                    self.p += 1;
                    match e {
                        b'n' => out.push('\n'),
                        b't' => out.push('\t'),
                        b'r' => out.push('\r'),
                        b'u' => {
                            if self.p + 4 > self.b.len() {
                                return Err("bad \\u".to_string());
                            }
                            let h = std::str::from_utf8(&self.b[self.p..self.p + 4])
                                .map_err(|_| "bad \\u")?;
                            let cp = u32::from_str_radix(h, 16).map_err(|_| "bad \\u")?;
                            out.push(char::from_u32(cp).unwrap_or('?'));
                            self.p += 4;
                        }
                        _ => out.push(e as char),
                    }
                }
                _ => out.push(c as char),
            }
        }
        Err("unterminated string".to_string())
    }
    fn arr(&mut self) -> Result<J, String> {
        self.p += 1;
        let mut v = Vec::new();
        loop {
            self.ws();
            if self.peek() == b']' {
                self.p += 1;
                return Ok(J::Arr(v));
            }
            v.push(self.value()?);
            self.ws();
            match self.peek() {
                b',' => {
                    self.p += 1;
                }
                b']' => {
                    self.p += 1;
                    return Ok(J::Arr(v));
                }
                _ => return Err("bad array".to_string()),
            }
        }
    }
    fn obj(&mut self) -> Result<J, String> {
        self.p += 1;
        let mut m = HashMap::new();
        loop {
            self.ws();
            if self.peek() == b'}' {
                self.p += 1;
                return Ok(J::Obj(m));
            }
            if self.peek() != b'"' {
                return Err("bad obj key".to_string());
            }
            let k = self.string()?;
            self.ws();
            if self.peek() != b':' {
                return Err("bad obj colon".to_string());
            }
            self.p += 1;
            let v = self.value()?;
            m.insert(k, v);
            self.ws();
            match self.peek() {
                b',' => {
                    self.p += 1;
                }
                b'}' => {
                    self.p += 1;
                    return Ok(J::Obj(m));
                }
                _ => return Err("bad obj".to_string()),
            }
        }
    }
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o
}

fn hx(v: u32) -> String {
    format!("\"0x{:x}\"", v)
}

fn hexbytes(b: &[u8]) -> String {
    const H: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(b.len() * 2);
    for &x in b {
        s.push(H[(x >> 4) as usize] as char);
        s.push(H[(x & 15) as usize] as char);
    }
    s
}

// ---------------------------------------------------------------------------
// Worker state (single trial thread at a time; main thread only joins).
// ---------------------------------------------------------------------------

const SSIZE: usize = 0x0010_0000;
const HSIZE: usize = 0x0010_0000;
const HEAP_USE: usize = 0x000F_0000; // top 64KB of heap region is a guard page
const STACK_SNAP_OFF: usize = 0xFC000;
const STACK_SNAP_LEN: usize = 0x1100;
const SCRATCH_FILL_OFF: usize = 0xF4000; // below-ESP scratch reset region
const SCRATCH_FILL_LEN: usize = 0x8000; // 32KB, ends at STACK_SNAP_OFF
const LOG_MAX: usize = 256;
const LOG_ENTRY: usize = 256;
// Log entry v3 layout: 0:id 4:ecx 8:edx 12:ebx 16:esi 20:edi 24:nargs
// 28:args[40] 188:snap_n 192:snap[8] 224:xmm0[4] 240:xmm1[4]
// (v2 logged args[8] only, so trailing call arguments passed uncompared;
// v3 logs and compares every argument up to LOG_MAXW. Contracts declaring
// more are rejected at setup rather than silently truncated.)
const LOG_MAXW: usize = 40;
const SNAP_MAXW: usize = 8;
const WRITEW_PER_ID: usize = 16;
const SEQ_MAX: usize = 16; // max per-call answer steps per callee (v3 item 6)

#[derive(Clone, Default)]
struct Callee {
    id: u32,
    conv: String,
    nargs: usize,
    pop: u32,
    ret: String,
    stub_addr: u32,
    tail_addr: u32,                         // E9 tail-patch variant (0 when unbuilt)
    writes: Vec<(u8, usize, usize, usize)>, // (kind 0=stack arg,1=ecx,2=edx; idx; wstart; nwords)
    snap: Vec<(u8, usize, usize)>,          // (kind 0=stack arg,1=ecx,2=edx; idx; nwords)
    logxmm: bool,                           // log xmm0 words into the call entry
    xmm0_from_stack: Option<usize>,         // rw-side transport: load xmm0 from stack arg
    logxmm1: bool,                          // v3: log xmm1 words into the call entry
    xmm1_from_stack: Option<usize>, // v3: rw-side transport for xmm1-arg callees
}

struct State {
    img: usize,
    delta: u32,
    image_size: usize,
    text_lo: usize,
    text_hi: usize,
    data_ranges: Vec<(usize, usize)>, // mapped (lo,hi) of writable data sections
    pristine: Vec<u8>,                // post-reloc snapshot of data_ranges, concatenated
    #[allow(dead_code)]
    data_off: Vec<usize>,
    s: usize, // scratch stack region
    h: usize, // scratch heap region
    meta: usize,
    m_ctx: u32,
    m_fn: u32,
    m_hostesp: u32,
    m_mxcsr: u32,
    m_tmp: u32,
    m_fault: u32, // fault flag + record (9 dwords)
    m_script_lo: u32,
    m_script_hi: u32,
    m_logidx: u32,
    m_side: u32,     // 0 = original side, 1 = rewrite side (for xmm transport)
    m_save_ecx: u32, // stub-entry spill for register out-param writes
    m_save_edx: u32,
    m_xmm_mirror: u32, // 32 words: scripted xmm0-7 entry values, readable by rewrites
    m_tls_mirror: u32, // 256 words: fabricated TLS slot values, readable by rewrites
    m_script_tab: u32, // 256 x (lo,hi) per-callee script slots
    m_writebuf: u32,   // 256 x 16 per-callee out-param write words
    m_seq_tab: u32,    // v3: 256 x 16 per-call answer steps (lo,hi)
    m_seq_len: u32,    // v3: 256 sequence lengths (trial_body fills)
    m_seq_idx: u32,    // v3: 256 per-side consumption indexes (run_side zeroes)
    m_step: u32,       // v3: stub scratch for the clamped step index
    tls_helper: u32,   // emitted mov eax,fs:[0x2c]; ret
    log_base: u32,
    stub_base: u32,
    stub_off: usize,
    ctable: u32,
    fxbuf: u32,
    tramp: usize,
    fault_pad: usize,
    esp0: u32,
    ctx: Vec<u32>,
    before_stack: Vec<u32>,
    exe_bytes: Vec<u8>,
    segs: Vec<(usize, usize, usize, usize)>, // (vaddr,rva? no: vaddr,size,rawptr,rawsz)
    _nsec: usize,
    _sely: usize,
    _headers_len: usize,
    _opt: usize,
    dll: *mut c_void,
    exports: HashMap<String, u32>,
    callees: HashMap<u32, Callee>,
    patches: Vec<(usize, [u8; 8], usize)>, // (addr, orig bytes, len 5 or 4)
    globals: Vec<(usize, usize)>,          // declared global ranges (mapped lo, len)
    relocs: u32,
    side_rw: bool, // true while the rewrite runs (anti-cheat active)
    mapped: bool,
}

impl State {
    fn new() -> State {
        State {
            img: 0,
            delta: 0,
            image_size: 0,
            text_lo: 0,
            text_hi: 0,
            data_ranges: Vec::new(),
            pristine: Vec::new(),
            data_off: Vec::new(),
            s: 0,
            h: 0,
            meta: 0,
            m_ctx: 0,
            m_fn: 0,
            m_hostesp: 0,
            m_mxcsr: 0,
            m_tmp: 0,
            m_fault: 0,
            m_script_lo: 0,
            m_script_hi: 0,
            m_logidx: 0,
            m_side: 0,
            m_save_ecx: 0,
            m_save_edx: 0,
            m_xmm_mirror: 0,
            m_tls_mirror: 0,
            m_script_tab: 0,
            m_writebuf: 0,
            m_seq_tab: 0,
            m_seq_len: 0,
            m_seq_idx: 0,
            m_step: 0,
            tls_helper: 0,
            log_base: 0,
            stub_base: 0,
            stub_off: 0,
            ctable: 0,
            fxbuf: 0,
            tramp: 0,
            fault_pad: 0,
            esp0: 0,
            ctx: vec![0u32; 17 + 128 + 32], // +32: xmm0-7 entry values (v2)
            before_stack: Vec::new(),
            exe_bytes: Vec::new(),
            segs: Vec::new(),
            _nsec: 0,
            _sely: 0,
            _headers_len: 0,
            _opt: 0,
            dll: std::ptr::null_mut(),
            exports: HashMap::new(),
            callees: HashMap::new(),
            patches: Vec::new(),
            globals: Vec::new(),
            relocs: 0,
            side_rw: false,
            mapped: false,
        }
    }
}

static mut ST: *mut State = std::ptr::null_mut();

fn st() -> &'static mut State {
    // SAFETY: ST is set once in main before any trial runs and never moves.
    unsafe { &mut *std::ptr::addr_of_mut!(ST).read() }
}

fn diag(msg: &str) {
    let _ = writeln!(std::io::stderr(), "worker: {}", msg);
}

// In-process fault trap with recovery: record the fault, then resume at the
// trampoline's fault landing pad (which restores the host stack and returns).
// Never kills the worker; a nested fault exits loudly.
extern "system" fn veh(p: *mut c_void) -> i32 {
    unsafe {
        let s = &mut *std::ptr::addr_of_mut!(ST).read();
        let flag = s.m_fault as *mut u32;
        if *flag != 0 {
            // fault while already handling one: cannot recover
            ExitProcess(11);
        }
        let rec = *(p as *const *const u32);
        let code = *rec.add(0);
        let nparams = *rec.add(4);
        let acc = if nparams >= 1 {
            *rec.add(5)
        } else {
            0xFFFFFFFF
        };
        let badva = if nparams >= 2 { *rec.add(6) } else { 0 };
        let ctxr = *((p as *const u32).add(1) as *const *const u32);
        let g = |off: usize| *(((ctxr as *const u8).add(off)) as *const u32);
        let eip = g(184);
        // record: flag,code,eip,access,badva,eax,ecx,edx,ebx,esi,edi,ebp,esp,eflags
        *flag.add(0) = 1;
        *flag.add(1) = code;
        *flag.add(2) = eip;
        *flag.add(3) = acc;
        *flag.add(4) = badva;
        *flag.add(5) = g(176);
        *flag.add(6) = g(172);
        *flag.add(7) = g(168);
        *flag.add(8) = g(164);
        *flag.add(9) = g(160);
        *flag.add(10) = g(156);
        *flag.add(11) = g(180);
        *flag.add(12) = g(196);
        *flag.add(13) = g(192);
        // resume on the host stack at the landing pad
        let hostesp = *(s.m_hostesp as *const u32);
        let ctxw = *((p as *const u32).add(1) as *const *mut u8);
        *((ctxw.add(184)) as *mut u32) = s.fault_pad as u32;
        *((ctxw.add(196)) as *mut u32) = hostesp;
        -1 // EXCEPTION_CONTINUE_EXECUTION
    }
}

fn u32le(v: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([v[off], v[off + 1], v[off + 2], v[off + 3]])
}
fn u16le(v: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([v[off], v[off + 1]])
}

fn map_at(addr: usize, size: usize) -> Option<usize> {
    let p = unsafe {
        VirtualAlloc(
            addr as *mut c_void,
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_RWX,
        )
    };
    if p.is_null() || p as usize != addr {
        None
    } else {
        Some(addr)
    }
}

fn protect(addr: usize, size: usize, prot: u32) {
    let mut old = 0u32;
    let r = unsafe { VirtualProtect(addr as *mut c_void, size, prot, &mut old) };
    if r == 0 {
        diag(&format!("VirtualProtect {:#x} failed", addr));
    }
}

// Map the executable at a relocated base with relocations applied (runfn logic).
fn map_image(exe: &[u8]) -> Result<(), String> {
    let s = st();
    if exe.len() < 0x200 || u16le(exe, 0) != 0x5A4D {
        return Err("bad MZ".to_string());
    }
    let peoff = u32le(exe, 0x3C) as usize;
    if u32le(exe, peoff) != 0x00004550 {
        return Err("bad PE sig".to_string());
    }
    let coff = peoff + 4;
    let nsec = u16le(exe, coff + 2) as usize;
    let optsz = u16le(exe, coff + 16) as usize;
    let opt = coff + 20;
    if u16le(exe, opt) != 0x10B || optsz < 96 {
        return Err("not PE32".to_string());
    }
    let image_base = u32le(exe, opt + 28) as usize;
    let image_size = (u32le(exe, opt + 56) as usize + 0xFFF) & !0xFFF;
    let headers_len = u32le(exe, opt + 60) as usize;
    let sely = coff + 20 + optsz;

    let image_cands: [usize; 4] = [0x1000_0000, 0x4000_0000, 0x0500_0000, 0x0C00_0000];
    let stack_cands: [usize; 3] = [0x3100_0000, 0x2900_0000, 0x0900_0000];
    let heap_cands: [usize; 3] = [0x3000_0000, 0x2800_0000, 0x0A00_0000];

    let mut ibase = None;
    for &c in &image_cands {
        if let Some(b) = map_at(c, image_size) {
            ibase = Some(b);
            break;
        }
    }
    let img = ibase.ok_or("cannot map image anywhere")?;
    let delta = (img as i64 - image_base as i64) as u32;
    let mut sbase = None;
    for &c in &stack_cands {
        if let Some(b) = map_at(c, SSIZE) {
            sbase = Some(b);
            break;
        }
    }
    let sb = sbase.ok_or("cannot map scratch stack")?;
    let mut hbase = None;
    for &c in &heap_cands {
        if let Some(b) = map_at(c, HSIZE) {
            hbase = Some(b);
            break;
        }
    }
    let hb = hbase.ok_or("cannot map scratch heap")?;

    unsafe {
        std::ptr::copy_nonoverlapping(exe.as_ptr(), img as *mut u8, headers_len.min(exe.len()));
    }
    let mut segs = Vec::new();
    for i in 0..nsec {
        let sh = sely + i * 40;
        let vsize = u32le(exe, sh + 8) as usize;
        let vaddr = u32le(exe, sh + 12) as usize;
        let rawsz = u32le(exe, sh + 16) as usize;
        let rawptr = u32le(exe, sh + 20) as usize;
        if rawsz > 0 && rawptr + rawsz <= exe.len() {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    exe.as_ptr().add(rawptr),
                    (img + vaddr) as *mut u8,
                    rawsz,
                );
            }
        }
        segs.push((img + vaddr, vsize.max(rawsz), rawptr, rawsz));
        let mut nm = [0u8; 8];
        nm.copy_from_slice(&exe[sh..sh + 8]);
        let _ = nm;
    }
    let rva2off = |rva: usize| -> Option<usize> {
        if rva < headers_len {
            return Some(rva);
        }
        for i in 0..nsec {
            let sh = sely + i * 40;
            let vaddr = u32le(exe, sh + 12) as usize;
            let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
            let rawptr = u32le(exe, sh + 20) as usize;
            if vaddr <= rva && rva < vaddr + vsize {
                return Some(rawptr + (rva - vaddr));
            }
        }
        None
    };
    let mut relocs = 0u32;
    let reloc_rva = u32le(exe, opt + 96 + 40) as usize;
    let reloc_size = u32le(exe, opt + 96 + 44) as usize;
    if reloc_rva != 0
        && reloc_size != 0
        && let Some(ro) = rva2off(reloc_rva)
    {
        let mut p = ro;
        let end = ro + reloc_size;
        while p + 8 <= end && p + 8 <= exe.len() {
            let page = u32le(exe, p) as usize;
            let bsz = u32le(exe, p + 4) as usize;
            if bsz < 8 {
                break;
            }
            let mut q = p + 8;
            while q + 2 <= p + bsz && q + 2 <= exe.len() {
                let e = u16le(exe, q);
                q += 2;
                if e >> 12 == 3 {
                    let at = img + page + ((e & 0xFFF) as usize);
                    unsafe {
                        let v = std::ptr::read_unaligned(at as *const u32);
                        std::ptr::write_unaligned(at as *mut u32, v.wrapping_add(delta));
                    }
                    relocs += 1;
                }
            }
            p += (bsz + 3) & !3;
            if p <= ro {
                break;
            }
        }
    }

    s.img = img;
    s.delta = delta;
    s.image_size = image_size;
    s.s = sb;
    s.h = hb;
    s.segs = segs;
    s._nsec = nsec;
    s._sely = sely;
    s._headers_len = headers_len;
    s._opt = opt;
    s.relocs = relocs;

    // .text range (for anti-cheat protection) + writable data ranges
    for i in 0..nsec {
        let sh = sely + i * 40;
        let name = &exe[sh..sh + 8];
        let vaddr = u32le(exe, sh + 12) as usize;
        let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
        let chars = u32le(exe, sh + 36);
        if name.starts_with(b".text") {
            s.text_lo = img + vaddr;
            s.text_hi = img + vaddr + vsize;
        }
        if chars & 0x80000000 != 0 {
            // writable section: track for full diff (skip the stub .rkstr? keep all)
            s.data_ranges.push((img + vaddr, img + vaddr + vsize));
        }
    }
    // pristine snapshot of writable ranges (post-reloc)
    let mut pris = Vec::new();
    let mut offs = Vec::new();
    for &(lo, hi) in &s.data_ranges {
        offs.push(pris.len());
        pris.extend_from_slice(unsafe { std::slice::from_raw_parts(lo as *const u8, hi - lo) });
    }
    s.pristine = pris;
    s.data_off = offs;

    // layout inside scratch stack region
    let meta = sb + 0xFF000;
    s.meta = meta;
    s.m_ctx = meta as u32;
    s.m_fn = (meta + 4) as u32;
    s.m_hostesp = (meta + 8) as u32;
    s.m_mxcsr = (meta + 12) as u32;
    s.m_tmp = (meta + 16) as u32;
    s.m_fault = (meta + 32) as u32;
    s.m_script_lo = (meta + 96) as u32;
    s.m_script_hi = (meta + 100) as u32;
    s.m_logidx = (meta + 104) as u32;
    s.m_side = (meta + 108) as u32;
    s.m_save_ecx = (meta + 112) as u32;
    s.m_save_edx = (meta + 116) as u32;
    s.m_step = (meta + 120) as u32;
    s.m_xmm_mirror = (meta + 128) as u32;
    s.m_tls_mirror = (meta + 384) as u32;
    // v3 layout inside the scratch stack region (all below the snapshot window):
    // stubs 0x1000-0x11000, call log 0x12000-0x22000 (256 x 256B), ctable
    // 0x22000, per-callee script table 0x23000 (256 x 8B), out-param write
    // buffer 0x24000-0x28000 (256 x 16 words), per-call answer sequences
    // 0x28000-0x30000 (256 x 16 steps x 8B), sequence lengths 0x30000
    // (256 dwords), per-side sequence indexes 0x31000 (256 dwords).
    s.log_base = (sb + 0x12000) as u32;
    s.stub_base = (sb + 0x1000) as u32;
    s.ctable = (sb + 0x22000) as u32;
    s.m_script_tab = (sb + 0x23000) as u32;
    s.m_writebuf = (sb + 0x24000) as u32;
    s.m_seq_tab = (sb + 0x28000) as u32;
    s.m_seq_len = (sb + 0x30000) as u32;
    s.m_seq_idx = (sb + 0x31000) as u32;
    s.fxbuf = (sb + 0xFD800) as u32;
    s.tramp = sb + 0xFE000;
    s.esp0 = (sb + 0xFD000) as u32;
    unsafe {
        *((s.m_mxcsr) as *mut u32) = 0x1F80;
        *((s.m_ctx) as *mut u32) = s.ctx.as_ptr() as u32;
        // guard page at the bottom of the stack region
        protect(sb, 0x1000, PAGE_NOACCESS);
        // guard at the top of the heap region
        protect(hb + HEAP_USE, HSIZE - HEAP_USE, PAGE_NOACCESS);
        // callee table: 0 = undeclared (call faults honestly)
        std::ptr::write_bytes(s.ctable as *mut u8, 0, 1024);
        std::ptr::write_bytes(s.m_script_tab as *mut u8, 0, 2048);
        std::ptr::write_bytes(s.m_writebuf as *mut u8, 0, 16384);
        std::ptr::write_bytes(s.m_seq_tab as *mut u8, 0, 32768);
        std::ptr::write_bytes(s.m_seq_len as *mut u8, 0, 1024);
        std::ptr::write_bytes(s.m_seq_idx as *mut u8, 0, 1024);
    }
    build_trampoline();
    // emitted TLS helper: mov eax,fs:[0x2c]; ret (this thread's TEB
    // TLS-slot array pointer). The first 16 stub bytes are reserved for it
    // (see the stub_off reset in setup); a stub emitted at offset 0 once
    // overwrote it and its `ret 4` corrupted the caller frame.
    s.tls_helper = s.stub_base;
    s.stub_off = 16;
    unsafe {
        let code: [u8; 7] = [0x64, 0xA1, 0x2C, 0x00, 0x00, 0x00, 0xC3];
        std::ptr::copy_nonoverlapping(code.as_ptr(), s.tls_helper as *mut u8, 7);
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, s.tls_helper as *const c_void, 7);
    }
    s.mapped = true;
    Ok(())
}

// Trampoline: runfn's setup with a fault landing pad. Layout mirrors runfn so
// the byte sequence stays reviewable against the proven version.
fn build_trampoline() {
    let s = st();
    let (m_ctx, m_fn, m_hostesp, m_mxcsr, m_tmp, fxbuf) =
        (s.m_ctx, s.m_fn, s.m_hostesp, s.m_mxcsr, s.m_tmp, s.fxbuf);
    let mut t: Vec<u8> = Vec::with_capacity(2400);
    let u = |t: &mut Vec<u8>, v: u32| t.extend_from_slice(&v.to_le_bytes());
    t.push(0x60); // pushad
    t.push(0x9C); // pushfd
    t.push(0x89);
    t.push(0x25);
    u(&mut t, m_hostesp); // mov [m_hostesp],esp (after pushes)
    t.push(0xDB);
    t.push(0xE3); // fninit
    t.push(0x0F);
    t.push(0xAE);
    t.push(0x15);
    u(&mut t, m_mxcsr); // ldmxcsr
    t.push(0x8B);
    t.push(0x2D);
    u(&mut t, m_ctx); // mov ebp,[m_ctx]
    t.extend_from_slice(&[0x8B, 0x4D, 0x00]); // ecx
    t.extend_from_slice(&[0x8B, 0x55, 0x04]); // edx
    t.extend_from_slice(&[0x8B, 0x5D, 0x08]); // ebx
    t.extend_from_slice(&[0x8B, 0x75, 0x0C]); // esi
    t.extend_from_slice(&[0x8B, 0x7D, 0x10]); // edi
    // v2: scripted XMM entry values from ctx[145+N*4] (byte disp 580+N*16).
    // ebp still holds the ctx pointer here.
    for n in 0..8u32 {
        t.extend_from_slice(&[0x0F, 0x10, 0x85 | ((n as u8) << 3)]);
        u(&mut t, 580 + n * 16); // movups xmmN,[ebp+disp]
    }
    t.extend_from_slice(&[0x8B, 0x45, 0x18]); // eax_in
    t.push(0xA3);
    u(&mut t, m_tmp + 8);
    t.extend_from_slice(&[0x8B, 0x45, 0x14]); // ebp_in
    t.push(0xA3);
    u(&mut t, m_tmp + 12);
    t.extend_from_slice(&[0x8B, 0x65, 0x1C]); // esp = scratch
    t.push(0xA1);
    u(&mut t, m_tmp + 12);
    t.extend_from_slice(&[0x89, 0xC5]); // ebp
    t.push(0xA1);
    u(&mut t, m_tmp + 8); // eax
    t.push(0xFF);
    t.push(0x15);
    u(&mut t, m_fn); // call [m_fn]
    // normal return path
    t.push(0x89);
    t.push(0x2D);
    u(&mut t, m_tmp); // mov [m_tmp],ebp
    t.push(0x89);
    t.push(0x25);
    u(&mut t, m_tmp + 4); // mov [m_tmp+4],esp
    t.push(0x8B);
    t.push(0x2D);
    u(&mut t, m_ctx);
    t.extend_from_slice(&[0x89, 0x45, 0x20]); // eax
    t.extend_from_slice(&[0x89, 0x4D, 0x24]); // ecx
    t.extend_from_slice(&[0x89, 0x55, 0x28]); // edx
    t.extend_from_slice(&[0x89, 0x5D, 0x2C]); // ebx
    t.extend_from_slice(&[0x89, 0x75, 0x30]); // esi
    t.extend_from_slice(&[0x89, 0x7D, 0x34]); // edi
    t.push(0xA1);
    u(&mut t, m_tmp);
    t.extend_from_slice(&[0x89, 0x45, 0x38]); // ebp
    t.push(0xA1);
    u(&mut t, m_tmp + 4);
    t.extend_from_slice(&[0x89, 0x45, 0x3C]); // esp
    t.push(0x9C);
    t.push(0x58);
    t.extend_from_slice(&[0x89, 0x45, 0x40]); // eflags
    t.push(0x0F);
    t.push(0xAE);
    t.push(0x05);
    u(&mut t, fxbuf); // fxsave
    for i in 0..128u32 {
        t.push(0xA1);
        u(&mut t, fxbuf + i * 4);
        t.push(0x89);
        t.push(0x85);
        u(&mut t, 68 + i * 4);
    }
    t.push(0x8B);
    t.push(0x25);
    u(&mut t, m_hostesp);
    t.push(0x9D); // popfd
    t.push(0x61); // popad
    t.push(0xC3); // ret
    // fault landing pad: VEH resumes here with esp=host (post-pushad/pushfd)
    let pad_off = t.len();
    t.push(0x9D); // popfd
    t.push(0x61); // popad
    t.push(0xC3); // ret
    unsafe {
        std::ptr::copy_nonoverlapping(t.as_ptr(), s.tramp as *mut u8, t.len());
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, s.tramp as *const c_void, t.len());
    }
    s.fault_pad = s.tramp + pad_off;
}

// Per-callsite recorder stub (machine code). Entry: esp->[ret][a0..].
// Logs (id,ecx,edx,ebx,esi,edi,nargs,args[40],snap[8],xmm0,xmm1) to the
// call log, performs scripted out-param writes, then returns the callee's
// current per-call sequence step with the callee's cleanup discipline
// (or pop_override for tail stubs).
fn emit_stub(c: &Callee, tail_pop: Option<u32>) -> Vec<u8> {
    // tail_pop = Some(outer_pop): E9 tail-patch variant. The patched E8
    // pushed a return address (site+5) that must be discarded: the stub
    // does `add esp,4` then `ret outer_pop`, landing back at the
    // trampoline. (A plain `ret outer_pop+4` would resume at site+5.)
    let s = st();
    let mut t: Vec<u8> = Vec::new();
    let u = |t: &mut Vec<u8>, v: u32| t.extend_from_slice(&v.to_le_bytes());
    // v3: tail stubs read stack arguments one word lower. A patched E9 site
    // calls the stub with two return addresses on the stack (site+5, then
    // the trampoline return), so the caller's arg0 is at [esp+8], not
    // [esp+4] (lanes r-n100/r-n104/r-s79: v2 logged the trampoline return
    // address as arg0 for every tail call with stack arguments). This
    // assumes the tail site executes with entry ESP, which holds for
    // register-adjust + jump thunks; the contract author must confirm it.
    let arg_base: u32 = if tail_pop.is_some() { 8 } else { 4 };
    // Spill entry ecx/edx: the arg loop below clobbers ecx, and register
    // out-param writes (emitted later) need the original values.
    t.extend_from_slice(&[0x89, 0x0D]);
    u(&mut t, s.m_save_ecx); // mov [m_save_ecx],ecx
    t.extend_from_slice(&[0x89, 0x15]);
    u(&mut t, s.m_save_edx); // mov [m_save_edx],edx
    // v2 rewrite-side transport for xmm0-arg callees: on the rewrite side
    // only, load xmm0 from the declared stack arg before logging it.
    // NOTE: disp32 SIB forms (modrm 0x84/0x8C): a transport index past 30
    // would overflow disp8 (v2's latent form is fixed here too).
    if let Some(idx) = c.xmm0_from_stack {
        t.extend_from_slice(&[0x83, 0x3D]); // cmp dword [m_side],0
        u(&mut t, s.m_side);
        t.push(0x00);
        t.extend_from_slice(&[0x74, 0x09]); // je +9 (skip the 9-byte movss)
        t.extend_from_slice(&[0xF3, 0x0F, 0x10, 0x84, 0x24]);
        u(&mut t, 4 + idx as u32 * 4); // movss xmm0,[esp+4+idx*4]
    }
    // v3: same transport for xmm1-arg callees (lanes r-b03, r-b24).
    if let Some(idx) = c.xmm1_from_stack {
        t.extend_from_slice(&[0x83, 0x3D]); // cmp dword [m_side],0
        u(&mut t, s.m_side);
        t.push(0x00);
        t.extend_from_slice(&[0x74, 0x09]); // je +9 (skip the 9-byte movss)
        t.extend_from_slice(&[0xF3, 0x0F, 0x10, 0x8C, 0x24]);
        u(&mut t, 4 + idx as u32 * 4); // movss xmm1,[esp+4+idx*4]
    }
    // eax = logidx; if >= 256 skip logging (writes + scripted return still run)
    t.push(0xA1);
    u(&mut t, s.m_logidx); // mov eax,[m_logidx]
    t.extend_from_slice(&[0x3D]);
    u(&mut t, LOG_MAX as u32); // cmp eax,256
    t.extend_from_slice(&[0x0F, 0x87, 0x00, 0x00, 0x00, 0x00]); // ja full (rel32, patched below)
    let jae_pos = t.len() - 4;
    t.extend_from_slice(&[0x69, 0xC0]); // imul eax,eax,128 (imm32 form)
    u(&mut t, LOG_ENTRY as u32);
    t.extend_from_slice(&[0x05]);
    u(&mut t, s.log_base); // add eax,log_base
    // store id, ecx, edx, ebx, esi, edi, nargs
    t.extend_from_slice(&[0xC7, 0x00]);
    u(&mut t, c.id); // mov [eax],id
    t.extend_from_slice(&[0x89, 0x48, 0x04]); // mov [eax+4],ecx
    t.extend_from_slice(&[0x89, 0x50, 0x08]); // mov [eax+8],edx
    t.extend_from_slice(&[0x89, 0x58, 0x0C]); // mov [eax+12],ebx
    t.extend_from_slice(&[0x89, 0x70, 0x10]); // mov [eax+16],esi
    t.extend_from_slice(&[0x89, 0x78, 0x14]); // mov [eax+20],edi
    t.extend_from_slice(&[0xC7, 0x40, 0x18]);
    u(&mut t, c.nargs as u32); // mov [eax+24],nargs
    // v2 pointed-to snapshots: copy declared words through pointer args
    // into [eax+64..] BEFORE the stack-arg loop clobbers ecx/edx.
    let mut snap_off = 0usize;
    for (kind, idx, n) in &c.snap {
        match *kind {
            0 => {
                t.extend_from_slice(&[0x8B, 0x94, 0x24]);
                u(&mut t, arg_base + *idx as u32 * 4); // mov edx,[esp+base+idx*4]
            }
            1 => {
                t.extend_from_slice(&[0x8B, 0xD1]); // mov edx,ecx
            }
            _ => {} // kind 2: pointer already in edx
        }
        for j in 0..*n {
            t.extend_from_slice(&[0x8B, 0x8A]);
            u(&mut t, (j * 4) as u32); // mov ecx,[edx+j*4]
            t.extend_from_slice(&[0x89, 0x88]);
            u(&mut t, (192 + (snap_off + j) * 4) as u32); // mov [eax+..],ecx
        }
        snap_off += *n;
    }
    // NOTE: disp32 form (modrm 0x80): snap_n at byte 188 exceeds the +127
    // reach of disp8, which would sign-extend into the previous entry.
    t.extend_from_slice(&[0xC7, 0x80]);
    u(&mut t, 188);
    u(&mut t, snap_off as u32); // mov [eax+188],snap_n
    // stack args: ecx scratch (saved above); [esp+base+k*4] -> [eax+28+k*4]
    let nargs = c.nargs.min(LOG_MAXW);
    for k in 0..nargs {
        t.extend_from_slice(&[0x8B, 0x8C, 0x24]);
        u(&mut t, arg_base + k as u32 * 4); // mov ecx,[esp+base+4k]
        t.extend_from_slice(&[0x89, 0x88]);
        u(&mut t, 28 + k as u32 * 4); // mov [eax+28+4k],ecx
    }
    // NOTE: disp32 forms throughout (modrm 0x80/0x88): all four vector
    // slots sit past byte 127, out of disp8 reach.
    if c.logxmm {
        t.extend_from_slice(&[0x0F, 0x13, 0x80]);
        u(&mut t, 224); // movlps [eax+224],xmm0
        t.extend_from_slice(&[0x0F, 0x17, 0x80]);
        u(&mut t, 232); // movhps [eax+232],xmm0
    }
    // v3: xmm1 call-argument logging (lanes r-b03, r-b24).
    if c.logxmm1 {
        t.extend_from_slice(&[0x0F, 0x13, 0x88]);
        u(&mut t, 240); // movlps [eax+240],xmm1
        t.extend_from_slice(&[0x0F, 0x17, 0x88]);
        u(&mut t, 248); // movhps [eax+248],xmm1
    }
    // logidx++
    t.push(0xFF);
    t.push(0x05);
    u(&mut t, s.m_logidx); // inc [m_logidx]
    // full: scripted out-param writes + return value
    let full_pos = t.len();
    let rel = (full_pos - (jae_pos + 4)) as u32;
    t[jae_pos..jae_pos + 4].copy_from_slice(&rel.to_le_bytes());
    // v3 per-call answer sequences (lane r-b04): this call consumes step
    // idx = min([seqidx], len-1) of the callee's sequence and advances the
    // per-side index, so a callee polled in a loop can answer token, token,
    // ..., NULL. Contracts without "seq" get [(script lo,hi)] with length
    // 1 (filled by trial_body), which is exactly the v2 behavior. eax is
    // preserved across this: the `al` channel below keeps v2's residue
    // semantics bit-identically.
    t.extend_from_slice(&[0x8B, 0x0D]);
    u(&mut t, s.m_seq_idx + c.id * 4); // mov ecx,[seqidx]
    t.extend_from_slice(&[0x3B, 0x0D]);
    u(&mut t, s.m_seq_len + c.id * 4); // cmp ecx,[seqlen]
    t.extend_from_slice(&[0x72, 0x07]); // jb have (skip 7: mov+dec)
    t.extend_from_slice(&[0x8B, 0x0D]);
    u(&mut t, s.m_seq_len + c.id * 4); // mov ecx,[seqlen]
    t.push(0x49); // dec ecx (len >= 1 always, so len-1 >= 0)
    // have: ecx = clamped step
    t.push(0xFF);
    t.push(0x05);
    u(&mut t, s.m_seq_idx + c.id * 4); // inc [seqidx]
    t.extend_from_slice(&[0x89, 0x0D]);
    u(&mut t, s.m_step); // mov [m_step],ecx (writes below clobber ecx)
    // v2 out-param writes: words from the per-callee write buffer stored
    // through the pointer found at the declared stack arg or register.
    for (kind, idx, wstart, n) in &c.writes {
        match *kind {
            1 => {
                t.extend_from_slice(&[0x8B, 0x15]);
                u(&mut t, s.m_save_ecx); // mov edx,[m_save_ecx]
            }
            2 => {
                t.extend_from_slice(&[0x8B, 0x15]);
                u(&mut t, s.m_save_edx); // mov edx,[m_save_edx]
            }
            _ => {
                t.extend_from_slice(&[0x8B, 0x94, 0x24]);
                u(&mut t, arg_base + *idx as u32 * 4); // mov edx,[esp+base+arg*4]
            }
        }
        for j in 0..*n {
            t.extend_from_slice(&[0x8B, 0x0D]);
            u(&mut t, s.m_writebuf + c.id * 64 + (*wstart + j) as u32 * 4);
            t.extend_from_slice(&[0x89, 0x8A]); // mov ecx,[wbuf]; mov [edx+j*4],ecx
            u(&mut t, (j * 4) as u32);
        }
    }
    // edx = this call's (lo,hi) step address: seq_tab + id*128 + step*8.
    t.extend_from_slice(&[0x8B, 0x0D]);
    u(&mut t, s.m_step); // mov ecx,[m_step]
    t.extend_from_slice(&[0x8B, 0xD1]); // mov edx,ecx
    t.extend_from_slice(&[0xC1, 0xE2, 0x03]); // shl edx,3
    t.extend_from_slice(&[0x81, 0xC2]);
    u(&mut t, s.m_seq_tab + c.id * 128); // add edx,seqbase
    match c.ret.as_str() {
        "u64" => {
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
            t.extend_from_slice(&[0x83, 0xC2, 0x04]); // add edx,4
            t.extend_from_slice(&[0x8B, 0x12]); // mov edx,[edx]
        }
        "al" => {
            t.extend_from_slice(&[0x25]);
            u(&mut t, 0xFFFFFF00); // and eax,0xffffff00
            // or in the script lo byte via edx scratch (clobberable)
            t.extend_from_slice(&[0x8B, 0x12]); // mov edx,[edx]
            t.extend_from_slice(&[0x81, 0xE2]);
            u(&mut t, 0xFF); // and edx,0xff
            t.extend_from_slice(&[0x09, 0xD0]); // or eax,edx
        }
        "f32xmm0" => {
            t.extend_from_slice(&[0xF3, 0x0F, 0x10, 0x02]); // movss xmm0,[edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx] too
        }
        "f64xmm0" => {
            t.extend_from_slice(&[0x0F, 0x12, 0x02]); // movlps xmm0,[edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx] too
        }
        "f32st0" => {
            t.extend_from_slice(&[0xD9, 0x02]); // fld dword [edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
        }
        "f64st0" => {
            t.extend_from_slice(&[0xDD, 0x02]); // fld qword [edx]
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
        }
        _ => {
            // u32 default: eax = step lo
            t.extend_from_slice(&[0x8B, 0x02]); // mov eax,[edx]
        }
    }
    match tail_pop {
        Some(outer) => {
            // discard the E8 return address, return to the trampoline
            t.extend_from_slice(&[0x83, 0xC4, 0x04]); // add esp,4
            if outer > 0 {
                t.push(0xC2);
                t.push((outer & 0xFF) as u8);
                t.push((outer >> 8) as u8); // ret outer_pop
            } else {
                t.push(0xC3); // ret
            }
        }
        None => {
            if c.pop > 0 {
                t.push(0xC2);
                t.push((c.pop & 0xFF) as u8);
                t.push((c.pop >> 8) as u8); // ret pop
            } else {
                t.push(0xC3); // ret
            }
        }
    }
    t
}

// Read this thread's TEB TLS-slot array pointer via the emitted helper.
fn tls_slots() -> u32 {
    let f: extern "C" fn() -> u32 = unsafe { std::mem::transmute(st().tls_helper as usize) };
    f()
}

// Patch an E9 tail-jump site to a recorder stub call. The stub is a tail
// variant that returns directly to the trampoline (see cmd_setup).
fn patch_e9(site_mapped: usize, stub: u32) -> Result<[u8; 8], String> {
    unsafe {
        let op = *(site_mapped as *const u8);
        if op != 0xE9 {
            return Err(format!("site {:#x} is not E9 (op={:#x})", site_mapped, op));
        }
        let mut orig = [0u8; 8];
        std::ptr::copy_nonoverlapping(site_mapped as *const u8, orig.as_mut_ptr(), 5);
        let rel = stub.wrapping_sub((site_mapped + 5) as u32);
        *(site_mapped as *mut u8) = 0xE8;
        std::ptr::write_unaligned((site_mapped + 1) as *mut u32, rel);
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, site_mapped as *const c_void, 5);
        Ok(orig)
    }
}

// Patch an E8 call site to a recorder stub. Returns error if not E8.
fn patch_e8(site_mapped: usize, stub: u32) -> Result<[u8; 8], String> {
    unsafe {
        let op = *(site_mapped as *const u8);
        if op != 0xE8 {
            return Err(format!("site {:#x} is not E8 (op={:#x})", site_mapped, op));
        }
        let mut orig = [0u8; 8];
        std::ptr::copy_nonoverlapping(site_mapped as *const u8, orig.as_mut_ptr(), 5);
        let rel = stub.wrapping_sub((site_mapped + 5) as u32);
        *(site_mapped as *mut u8) = 0xE8;
        std::ptr::write_unaligned((site_mapped + 1) as *mut u32, rel);
        let proc = GetCurrentProcess();
        FlushInstructionCache(proc, site_mapped as *const c_void, 5);
        Ok(orig)
    }
}

// Minimal import directory walk: find the IAT slot for dll!name.
fn find_iat(dll_want: &str, name_want: &str) -> Option<usize> {
    let s = st();
    let exe = &s.exe_bytes;
    let opt = s._opt;
    let idt_rva = u32le(exe, opt + 96 + 8) as usize;
    let idt_sz = u32le(exe, opt + 96 + 12) as usize;
    if idt_rva == 0 {
        return None;
    }
    let rva2mapped = |rva: usize| -> Option<usize> {
        // map via section table recompute
        let nsec = s._nsec;
        let sely = s._sely;
        let headers_len = s._headers_len;
        if rva < headers_len {
            return Some(s.img + rva);
        }
        for i in 0..nsec {
            let sh = sely + i * 40;
            let vaddr = u32le(exe, sh + 12) as usize;
            let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
            if vaddr <= rva && rva < vaddr + vsize {
                return Some(s.img + rva);
            }
        }
        None
    };
    let rva2bytes = |rva: usize| -> Option<usize> {
        // offset into file bytes
        let nsec = s._nsec;
        let sely = s._sely;
        let headers_len = s._headers_len;
        if rva < headers_len {
            return Some(rva);
        }
        for i in 0..nsec {
            let sh = sely + i * 40;
            let vaddr = u32le(exe, sh + 12) as usize;
            let vsize = u32le(exe, sh + 8).max(u32le(exe, sh + 16)) as usize;
            let rawptr = u32le(exe, sh + 20) as usize;
            if vaddr <= rva && rva < vaddr + vsize {
                return Some(rawptr + (rva - vaddr));
            }
        }
        None
    };
    let n = idt_sz / 20;
    for i in 0..n {
        let d = rva2bytes(idt_rva + i * 20)?;
        let ilt = u32le(exe, d) as usize;
        let name_rva = u32le(exe, d + 12) as usize;
        let iat = u32le(exe, d + 16) as usize;
        if name_rva == 0 {
            break;
        }
        let no = rva2bytes(name_rva)?;
        let mut dn = Vec::new();
        let mut q = no;
        while q < exe.len() && exe[q] != 0 {
            dn.push(exe[q]);
            q += 1;
        }
        let dll = String::from_utf8_lossy(&dn).to_ascii_uppercase();
        if dll != dll_want.to_ascii_uppercase() {
            continue;
        }
        // walk ILT/IAT in parallel
        let mut k = 0usize;
        loop {
            let lto = rva2bytes(ilt + k * 4)?;
            if lto + 4 > exe.len() {
                return None;
            }
            let thunk = u32le(exe, lto);
            if thunk == 0 {
                return None;
            }
            if thunk & 0x80000000 == 0 {
                let hn = rva2bytes((thunk & 0x7FFFFFFF) as usize)?;
                let mut nm = Vec::new();
                let mut q = hn + 2;
                while q < exe.len() && exe[q] != 0 {
                    nm.push(exe[q]);
                    q += 1;
                }
                if String::from_utf8_lossy(&nm) == name_want {
                    return rva2mapped(iat + k * 4);
                }
            }
            k += 1;
            if k > 4096 {
                return None;
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Trial execution.
// ---------------------------------------------------------------------------

// v3: deterministic per-word fill with full avalanche (murmur3 fmix32 over
// the mixed key). Every bit of (seed, trial, range, word) affects every
// output bit, so fills genuinely vary between trials, between ranges and
// between words. This replaces the v1/v2 per-trial xorshift stream seeded
// `(trial ^ seed*G ^ C)|1`, whose `|1` erased the bit consecutive trials
// differed in, making trial pairs share bit-identical first words.
fn fill_word(seed: u32, trial: u32, range: u32, word: u32) -> u32 {
    let mut h = seed
        ^ trial.wrapping_mul(0x85EBCA6B)
        ^ range.wrapping_mul(0xC2B28077)
        ^ word.wrapping_mul(0x27D4EB2F);
    h ^= 16;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EBCA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B28077);
    h ^= h >> 16;
    h
}

fn heap_pattern(i: u32, trial: u32, seed: u32) -> u32 {
    let mut h = i.wrapping_mul(0x9E3779B1) ^ 0xC6C6C6C6;
    h ^= trial.wrapping_mul(0x85EBCA6B).wrapping_add(0x3C3C3C3C);
    h ^= seed.wrapping_mul(0xC2B28077);
    if h == 0 {
        h = 0xA5A5A5A5;
    }
    h
}

fn fnv1a(mut h: u64, off: usize, val: u32) -> u64 {
    for b in (off as u64)
        .to_le_bytes()
        .iter()
        .chain(val.to_le_bytes().iter())
    {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001B3);
    }
    h
}

type CallRec = (
    u32,
    u32,
    u32,
    Vec<u32>,
    Vec<u32>,
    [u32; 4],
    [u32; 4],
);

#[derive(Clone, Default)]
struct Obs {
    status: String, // ok | fault | cheat
    regs: [u32; 8], // eax ecx edx ebx esi edi ebp esp
    eflags: u32,
    _mxcsr: u32,
    st0: String,
    xmm0: String,
    esp_delta: i64,
    heap_n: u32,
    heap_writes: Vec<(usize, u32)>,
    heap_hash: u64,
    stack_n: u32,
    stack_writes: Vec<(usize, u32)>,
    stack_hash: u64,
    globals_writes: Vec<(u32, u32)>, // (rva, val)
    undeclared: Vec<(u32, u32)>,
    undeclared_n: u32,
    calls: Vec<CallRec>, // (id, ecx, edx, args, snap, xmm0, xmm1)
    fault: String,       // "" or "code=.. eip=.. ..."
    fault_code: u32,
    fault_eip: u32,
    fault_badva: u32,
}

fn obs_json(o: &Obs) -> String {
    let jw = |v: &Vec<(usize, u32)>| -> String {
        v.iter()
            .map(|(x, y)| format!("[{},\"0x{:x}\"]", x, y))
            .collect::<Vec<_>>()
            .join(",")
    };
    let jg = |v: &Vec<(u32, u32)>| -> String {
        v.iter()
            .map(|(x, y)| format!("[\"0x{:x}\",\"0x{:x}\"]", x, y))
            .collect::<Vec<_>>()
            .join(",")
    };
    let jc = o
        .calls
        .iter()
        .map(|(id, cx, dx, a, snap, x0, x1)| {
            format!(
                "{{\"id\":{},\"ecx\":{},\"edx\":{},\"args\":[{}],\"snap\":[{}],\"xmm0\":[{}],\"xmm1\":[{}]}}",
                id,
                hx(*cx),
                hx(*dx),
                a.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                snap.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                x0.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(","),
                x1.iter().map(|x| hx(*x)).collect::<Vec<_>>().join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"status\":\"{}\",\"eax\":{},\"ecx\":{},\"edx\":{},\"ebx\":{},\"esi\":{},\"edi\":{},\"ebp\":{},\"esp\":{},\"eflags\":{},\"st0\":\"{}\",\"xmm0\":\"{}\",\"esp_delta\":{},\"heap_n\":{},\"heap_writes\":[{}],\"heap_hash\":\"0x{:x}\",\"stack_n\":{},\"stack_writes\":[{}],\"stack_hash\":\"0x{:x}\",\"globals_writes\":[{}],\"undeclared\":[{}],\"undeclared_n\":{},\"calls\":[{}],\"fault\":\"{}\",\"fault_code\":{},\"fault_eip\":{},\"fault_badva\":{}}}",
        o.status,
        hx(o.regs[0]),
        hx(o.regs[1]),
        hx(o.regs[2]),
        hx(o.regs[3]),
        hx(o.regs[4]),
        hx(o.regs[5]),
        hx(o.regs[6]),
        hx(o.regs[7]),
        hx(o.eflags),
        o.st0,
        o.xmm0,
        o.esp_delta,
        o.heap_n,
        jw(&o.heap_writes),
        o.heap_hash,
        o.stack_n,
        jw(&o.stack_writes),
        o.stack_hash,
        jg(&o.globals_writes),
        jg(&o.undeclared),
        o.undeclared_n,
        jc,
        esc(&o.fault),
        hx(o.fault_code),
        hx(o.fault_eip),
        hx(o.fault_badva)
    )
}

// Snapshot data ranges back to pristine, then apply a globals fill.
fn restore_data() {
    let s = st();
    let mut off = 0usize;
    for &(lo, hi) in &s.data_ranges.clone() {
        let len = hi - lo;
        unsafe {
            std::ptr::copy_nonoverlapping(s.pristine.as_ptr().add(off), lo as *mut u8, len);
        }
        off += len;
    }
}

fn in_declared(mapped: usize) -> bool {
    let s = st();
    for &(lo, len) in &s.globals {
        if lo <= mapped && mapped < lo + len {
            return true;
        }
    }
    false
}

// The trial parameters are a flat physical interface (registers, stack,
// heap, globals, TLS, XMM); bundling them would only hide the protocol.
#[allow(clippy::too_many_arguments)]
fn run_side(
    fn_addr: u32,
    regs: &[u32; 7], // eax ecx edx ebx esi edi ebp
    stack_args: &[u32],
    heapsegs: &[(usize, Vec<u32>)],
    globals_fill: &[(usize, Vec<u32>)], // mapped addr + words
    trial: u32,
    seed: u32,
    tls: &[(u32, u32)], // (slot, value) fabricated TLS slots
    xmm: &[u32; 32],    // xmm0-7 entry values
    stack_fill: Option<u32>,
    is_rw: bool,
    fulldata: bool,
) -> Obs {
    let s = st();
    let h = s.h;
    let sb = s.s;
    let esp0 = s.esp0;
    // 1. fill heap with trial pattern
    unsafe {
        let hp = h as *mut u32;
        for i in 0..(HEAP_USE / 4) {
            *hp.add(i) = heap_pattern(i as u32, trial, seed);
        }
        // stack snapshot window: trial pattern, or the contract's defined fill
        let sp = (sb + 0xFC000) as *mut u32;
        for i in 0..(STACK_SNAP_LEN / 4) {
            *sp.add(i) = match stack_fill {
                Some(v) => v,
                None => (i as u32).wrapping_mul(0x85EBCA6B) ^ 0x3C3C3C3C ^ trial,
            };
        }
        // v2: reset the below-ESP scratch (alloca-style regions) identically
        // on both sides, so stub-shaped buffers do not read cross-side garbage.
        let bp = (sb + SCRATCH_FILL_OFF) as *mut u32;
        for i in 0..(SCRATCH_FILL_LEN / 4) {
            *bp.add(i) = match stack_fill {
                Some(v) => v,
                None => {
                    let ai = ((SCRATCH_FILL_OFF / 4) + i) as u32;
                    ai.wrapping_mul(0x85EBCA6B) ^ 0x3C3C3C3C ^ trial
                }
            };
        }
        // heap segments
        for (off, words) in heapsegs {
            for (k, w) in words.iter().enumerate() {
                *((h + off + k * 4) as *mut u32) = *w;
            }
        }
        // stack args at esp0
        let ap = esp0 as *mut u32;
        for (i, &a) in stack_args.iter().enumerate() {
            *ap.add(i) = a;
        }
        // snapshot stack window BEFORE
        std::ptr::copy_nonoverlapping(
            (sb + STACK_SNAP_OFF) as *const u32,
            s.before_stack.as_mut_ptr(),
            STACK_SNAP_LEN / 4,
        );
        // fault flag + log + side marker (script tables are filled once
        // per trial in trial_body, shared by both sides)
        std::ptr::write_bytes(s.m_fault as *mut u8, 0, 56);
        *((s.m_logidx) as *mut u32) = 0;
        // v3: each side consumes per-call sequences from step 0.
        std::ptr::write_bytes(s.m_seq_idx as *mut u8, 0, 1024);
        *((s.m_side) as *mut u32) = if is_rw { 1 } else { 0 };
        *((s.m_fn) as *mut u32) = fn_addr;
        // ctx inputs: ecx edx ebx esi edi ebp eax esp0
        s.ctx[0] = regs[1];
        s.ctx[1] = regs[2];
        s.ctx[2] = regs[3];
        s.ctx[3] = regs[4];
        s.ctx[4] = regs[5];
        s.ctx[5] = regs[6];
        s.ctx[6] = regs[0];
        s.ctx[7] = esp0;
        // v2: scripted XMM entry values (+ a mirror the rewrite can read,
        // since Rust cannot observe incoming vector registers any other way)
        for (i, &w) in xmm.iter().enumerate() {
            s.ctx[145 + i] = w;
            *((s.m_xmm_mirror + i as u32 * 4) as *mut u32) = w;
        }
        *((s.m_ctx) as *mut u32) = s.ctx.as_ptr() as u32;
    }
    // v2: fabricated TLS slots on this trial thread + a mirror the
    // rewrite can read, since Rust cannot read FS. The mirror is
    // worker-shared, so it is zeroed on every side. The TEB slots are
    // saved and restored around each side: low slots may belong to a
    // loaded module (thread teardown reads them; leaving a heap pointer
    // behind killed the worker).
    unsafe {
        std::ptr::write_bytes(s.m_tls_mirror as *mut u8, 0, 1024);
    }
    // NOTE: faults before the trampoline's hostesp save (this plant
    // included) resume with esp=0 and die by nested fault; keep this
    // sequence to mapped-memory operations only (TEB slots + mirror are).
    let mut tls_save: Vec<(u32, u32)> = Vec::new();
    if !tls.is_empty() {
        let slots = tls_slots();
        for (slot, val) in tls {
            unsafe {
                let p = (slots + slot * 4) as *mut u32;
                tls_save.push((*slot, *p));
                *p = *val;
                if *slot < 256 {
                    *((s.m_tls_mirror + slot * 4) as *mut u32) = *val;
                }
            }
        }
    }
    // 2. data ranges: pristine + declared fill
    let need_data = fulldata || !globals_fill.is_empty() || !st().globals.is_empty();
    if need_data {
        restore_data();
    }
    // v2: fills outside the writable sections (.rdata hook slots) are
    // captured and restored around the call so they never leak across trials.
    let mut rda_restores: Vec<(usize, u32)> = Vec::new();
    for (addr, words) in globals_fill {
        for (k, w) in words.iter().enumerate() {
            let a = addr + k * 4;
            let in_writable = st().data_ranges.iter().any(|(lo, hi)| *lo <= a && a < *hi);
            unsafe {
                if !in_writable && !rda_restores.iter().any(|(x, _)| *x == a) {
                    rda_restores.push((a, *(a as *const u32)));
                }
                *(a as *mut u32) = *w;
            }
        }
    }
    // snapshot declared ranges AFTER fill (diffs are callee-only)
    let mut gbefore: Vec<(usize, Vec<u32>)> = Vec::new();
    for &(lo, len) in &st().globals {
        let mut v = Vec::with_capacity(len / 4);
        for k in 0..(len / 4) {
            v.push(unsafe { *((lo + k * 4) as *const u32) });
        }
        gbefore.push((lo, v));
    }
    // full-data snapshot after fill for undeclared-write discovery
    let mut dbefore: Vec<u8> = Vec::new();
    if fulldata {
        for &(lo, hi) in &st().data_ranges {
            dbefore
                .extend_from_slice(unsafe { std::slice::from_raw_parts(lo as *const u8, hi - lo) });
        }
    }
    // 3. anti-cheat: revoke .text while the rewrite runs
    s.side_rw = is_rw;
    if is_rw {
        protect(s.text_lo, s.text_hi - s.text_lo, PAGE_NOACCESS);
    }
    // 4. call
    let tramp_fn: extern "C" fn() = unsafe { std::mem::transmute(st().tramp) };
    tramp_fn();
    for (slot, old) in &tls_save {
        unsafe {
            *((tls_slots() + slot * 4) as *mut u32) = *old;
        }
    }
    if is_rw {
        let s = st();
        protect(s.text_lo, s.text_hi - s.text_lo, PAGE_RWX);
        s.side_rw = false;
    }
    for (a, v) in &rda_restores {
        unsafe {
            *(*a as *mut u32) = *v;
        }
    }
    // 5. collect
    let s = st();
    let mut o = Obs::default();
    let faulted = unsafe { *((s.m_fault) as *const u32) != 0 };
    if faulted {
        let f = unsafe { std::slice::from_raw_parts(s.m_fault as *const u32, 14) };
        o.fault_code = f[1];
        o.fault_eip = f[2];
        let acc = f[3];
        o.fault_badva = f[4];
        o.regs = [f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12]];
        o.eflags = f[13];
        o.esp_delta = (o.regs[7] as i64) - (esp0 as i64);
        o.fault = format!(
            "code=0x{:x} eip=0x{:x} access={} badva=0x{:x}",
            o.fault_code, o.fault_eip, acc, o.fault_badva
        );
        // cheat? rewrite touching original code pages
        let in_text = |a: u32| (s.text_lo as u32) <= a && a < (s.text_hi as u32);
        if is_rw && (in_text(o.fault_eip) || in_text(o.fault_badva)) {
            o.status = "cheat".to_string();
        } else {
            o.status = "fault".to_string();
        }
        o.st0 = String::new();
        o.xmm0 = String::new();
    } else {
        o.status = "ok".to_string();
        o.regs = [
            s.ctx[8], s.ctx[9], s.ctx[10], s.ctx[11], s.ctx[12], s.ctx[13], s.ctx[14], s.ctx[15],
        ];
        o.eflags = s.ctx[16];
        o.esp_delta = (o.regs[7] as i64) - (esp0 as i64);
        let fx = &s.ctx[17..17 + 128];
        let fxb = unsafe { std::slice::from_raw_parts(fx.as_ptr() as *const u8, 512) };
        o.st0 = hexbytes(&fxb[32..42]);
        o.xmm0 = hexbytes(&fxb[160..176]);
        o._mxcsr = u32::from_le_bytes([fxb[24], fxb[25], fxb[26], fxb[27]]);
    }
    // heap diff
    let mut hh = 0xcbf29ce484222325u64;
    unsafe {
        let hp = h as *const u32;
        for i in 0..(HEAP_USE / 4) {
            // expected = pattern, unless a segment overrode it
            let mut expect = heap_pattern(i as u32, trial, seed);
            let byte = i * 4;
            for (off, words) in heapsegs {
                if *off <= byte && byte < off + words.len() * 4 {
                    expect = words[(byte - off) / 4];
                    break;
                }
            }
            let got = *hp.add(i);
            if got != expect {
                o.heap_n += 1;
                hh = fnv1a(hh, byte, got);
                if o.heap_writes.len() < 64 {
                    o.heap_writes.push((byte, got));
                }
            }
        }
    }
    o.heap_hash = hh;
    // stack diff
    let mut sh = 0xcbf29ce484222325u64;
    let esp_after = o.regs[7] as usize;
    unsafe {
        let sp = (sb + STACK_SNAP_OFF) as *const u32;
        let sb0 = sb + STACK_SNAP_OFF;
        for i in 0..s.before_stack.len() {
            let addr = sb0 + i * 4;
            if (addr as u32) < esp0 {
                continue; // below incoming ESP: callee's own frame, not observable
            }
            if !faulted && addr == esp_after.wrapping_sub(4) {
                continue; // trampoline's own pushfd slot
            }
            // args area was written before snapshot, so diffs are callee-only;
            // but skip the exact arg slots only if unchanged... no: include all diffs
            let got = *sp.add(i);
            if got != s.before_stack[i] {
                // ignore slots that are exactly our own pre-placed stack args
                let rel = addr as u32 as i64 - esp0 as i64;
                if rel >= 0 && (rel as usize) < stack_args.len() * 4 {
                    let want = stack_args[rel as usize / 4];
                    if got == want {
                        continue;
                    }
                }
                o.stack_n += 1;
                sh = fnv1a(sh, STACK_SNAP_OFF + i * 4, got);
                if o.stack_writes.len() < 64 {
                    o.stack_writes.push((STACK_SNAP_OFF + i * 4, got));
                }
            }
        }
    }
    o.stack_hash = sh;
    // declared globals diff + full-data undeclared discovery
    for (gi, &(lo, len)) in st().globals.clone().iter().enumerate() {
        for k in 0..(len / 4) {
            let got = unsafe { *((lo + k * 4) as *const u32) };
            if got != gbefore[gi].1[k] {
                o.globals_writes.push(((lo - st().img) as u32, got));
            }
        }
    }
    if fulldata {
        let s = st();
        let img = s.img;
        let mut off = 0usize;
        for &(lo, hi) in &s.data_ranges.clone() {
            let len = hi - lo;
            let now = unsafe { std::slice::from_raw_parts(lo as *const u8, len) };
            let bef = &dbefore[off..off + len];
            // word-wise compare
            let nw = len / 4;
            for k in 0..nw {
                let a = u32::from_le_bytes([
                    now[k * 4],
                    now[k * 4 + 1],
                    now[k * 4 + 2],
                    now[k * 4 + 3],
                ]);
                let b = u32::from_le_bytes([
                    bef[k * 4],
                    bef[k * 4 + 1],
                    bef[k * 4 + 2],
                    bef[k * 4 + 3],
                ]);
                if a != b && !in_declared(lo + k * 4) {
                    o.undeclared_n += 1;
                    if o.undeclared.len() < 64 {
                        o.undeclared.push(((lo + k * 4 - img) as u32, a));
                    }
                }
            }
            off += len;
        }
    }
    // call log
    {
        let s = st();
        let n = unsafe { *((s.m_logidx) as *const u32) }.min(LOG_MAX as u32);
        for i in 0..n {
            let e = (s.log_base + i * LOG_ENTRY as u32) as *const u32;
            unsafe {
                let id = *e;
                let cx = *e.add(1);
                let dx = *e.add(2);
                let nargs = (*e.add(6)).min(LOG_MAXW as u32);
                let mut args = Vec::new();
                for k in 0..nargs {
                    args.push(*e.add(7 + k as usize));
                }
                let snapn = (*e.add(47)).min(SNAP_MAXW as u32);
                let mut snap = Vec::new();
                for k in 0..snapn {
                    snap.push(*e.add(48 + k as usize));
                }
                let mut x0 = [0u32; 4];
                for (k, slot) in x0.iter_mut().enumerate() {
                    *slot = *e.add(56 + k);
                }
                let mut x1 = [0u32; 4];
                for (k, slot) in x1.iter_mut().enumerate() {
                    *slot = *e.add(60 + k);
                }
                o.calls.push((id, cx, dx, args, snap, x0, x1));
            }
        }
    }
    o
}

// Compare two observations per the checks object. Returns (pass, checks_json, first_mismatch).
fn compare(a: &Obs, b: &Obs, checks: &J) -> (bool, String, String) {
    let mut parts: Vec<String> = Vec::new();
    let mut first = String::new();
    let mut ok_all = true;
    let mut chk = |name: &str, pass: bool, detail: String| {
        if !pass {
            ok_all = false;
            if first.is_empty() {
                first = format!("{}: {}", name, detail);
            }
        }
        parts.push(format!(
            "{{\"name\":\"{}\",\"passed\":{},\"detail\":\"{}\"}}",
            name,
            if pass { "true" } else { "false" },
            esc(&detail)
        ));
    };
    let want = |k: &str, d: bool| checks.get(k).map(|v| v.as_bool(d)).unwrap_or(d);
    // termination
    if want("fault", true) {
        let same = a.status == b.status || (a.status == "ok" && b.status == "ok");
        // fault codes must match too when both fault
        let pass = if a.status == "fault" && b.status == "fault" {
            a.fault_code == b.fault_code
        } else {
            same
        };
        chk(
            "termination",
            pass,
            format!("orig={} rw={} {}", a.status, b.status, b.fault),
        );
    }
    // cheat is always a failure for the rewrite side
    if b.status == "cheat" {
        chk(
            "no_cheat",
            false,
            format!("rewrite touched original code: {}", b.fault),
        );
    } else {
        chk("no_cheat", true, String::new());
    }
    if a.status != "ok" || b.status != "ok" {
        // fault-vs-fault with same code passes termination; nothing else to compare
        return (ok_all, format!("[{}]", parts.join(",")), first);
    }
    // return value
    let ret = checks.get("ret").map(|v| v.as_str()).unwrap_or("eax");
    match ret {
        "none" => chk("ret", true, String::new()),
        "al" => {
            let p = (a.regs[0] & 0xFF) == (b.regs[0] & 0xFF);
            chk(
                "ret",
                p,
                format!(
                    "al orig=0x{:x} rw=0x{:x}",
                    a.regs[0] & 0xFF,
                    b.regs[0] & 0xFF
                ),
            );
        }
        "ax" => {
            let p = (a.regs[0] & 0xFFFF) == (b.regs[0] & 0xFFFF);
            chk(
                "ret",
                p,
                format!(
                    "ax orig=0x{:x} rw=0x{:x}",
                    a.regs[0] & 0xFFFF,
                    b.regs[0] & 0xFFFF
                ),
            );
        }
        "edx_eax" => {
            let p = a.regs[0] == b.regs[0] && a.regs[2] == b.regs[2];
            chk(
                "ret",
                p,
                format!(
                    "edx:eax orig=0x{:x}:0x{:x} rw=0x{:x}:0x{:x}",
                    a.regs[2], a.regs[0], b.regs[2], b.regs[0]
                ),
            );
        }
        "st0" => {
            let (p, d, _exact) = fp_cmp(&a.st0, &b.st0, 10, checks);
            chk("ret", p, format!("st0 {}", d));
        }
        "xmm0" => {
            let (p, d, _exact) = fp_cmp(&a.xmm0, &b.xmm0, 16, checks);
            chk("ret", p, format!("xmm0 {}", d));
        }
        _ => {
            let p = a.regs[0] == b.regs[0];
            chk(
                "ret",
                p,
                format!("eax orig=0x{:x} rw=0x{:x}", a.regs[0], b.regs[0]),
            );
        }
    }
    if want("esp", true) {
        let p = a.esp_delta == b.esp_delta;
        chk(
            "esp",
            p,
            format!("delta orig={} rw={}", a.esp_delta, b.esp_delta),
        );
    }
    if want("heap", true) {
        let p = a.heap_n == b.heap_n && a.heap_hash == b.heap_hash;
        let mut d = format!("n orig={} rw={}", a.heap_n, b.heap_n);
        if !p {
            d.push_str(&format!(
                " orig_first={:?} rw_first={:?}",
                a.heap_writes.iter().take(4).collect::<Vec<_>>(),
                b.heap_writes.iter().take(4).collect::<Vec<_>>()
            ));
        }
        chk("heap", p, d);
    }
    if want("stack", true) {
        let p = a.stack_n == b.stack_n && a.stack_hash == b.stack_hash;
        chk("stack", p, format!("n orig={} rw={}", a.stack_n, b.stack_n));
    }
    if want("globals", true) {
        let p = a.globals_writes == b.globals_writes;
        chk(
            "globals",
            p,
            format!(
                "orig={:?} rw={:?}",
                lim(&a.globals_writes, 4),
                lim(&b.globals_writes, 4)
            ),
        );
    }
    if want("calls", true) {
        // Only the callee's real inputs participate: id + declared reg args
        // (cdecl/stdcall: none; thiscall: ecx; fastcall: ecx+edx; override
        // via checks.call_regs {id:[regs]}). Scratch regs at the call site
        // differ legitimately between original and rewrite codegen.
        let ka: Vec<String> = a.calls.iter().map(|c| callkey(c, checks)).collect();
        let kb: Vec<String> = b.calls.iter().map(|c| callkey(c, checks)).collect();
        let p = ka == kb;
        chk(
            "calls",
            p,
            format!("orig={:?} rw={:?}", lims(&ka, 3), lims(&kb, 3)),
        );
    }
    if want("undeclared", true) {
        let p = a.undeclared_n == 0 && b.undeclared_n == 0;
        chk(
            "undeclared",
            p,
            format!(
                "orig_n={} rw_n={} rw_first={:?}",
                a.undeclared_n,
                b.undeclared_n,
                lim(&b.undeclared, 4)
            ),
        );
    }
    (ok_all, format!("[{}]", parts.join(",")), first)
}

// v2 pointer normalization: an address inside one of the worker's fixed
// windows compares as a region-relative offset, so a pointer to the
// function's own frame, the trial heap or the image matches when it refers
// to the same place on both sides even though raw addresses differ.
fn norm_ptr(v: u32) -> String {
    let s = st();
    let h = s.h as u32;
    if h <= v && v < h.wrapping_add(HEAP_USE as u32) {
        return format!("H+{:x}", v - h);
    }
    let sb = s.s as u32;
    if sb <= v && v < sb.wrapping_add(SSIZE as u32) {
        if v >= s.esp0 {
            return format!("S+{:x}", v - s.esp0);
        } else {
            return format!("S-{:x}", s.esp0 - v);
        }
    }
    let img = s.img as u32;
    if img <= v && v < img.wrapping_add(s.image_size as u32) {
        return format!("I+{:x}", v - img);
    }
    format!("0x{:x}", v)
}

fn callkey(c: &CallRec, checks: &J) -> String {
    let id = c.0.to_string();
    let cal = st().callees.get(&c.0).cloned().unwrap_or_default();
    let regs: Vec<String> = match checks.get("call_regs").and_then(|m| m.get(id.as_str())) {
        Some(v) => v.as_arr().iter().map(|r| r.as_str().to_string()).collect(),
        None => match cal.conv.as_str() {
            "thiscall" => vec!["ecx".to_string()],
            "fastcall" => vec!["ecx".to_string(), "edx".to_string()],
            _ => Vec::new(),
        },
    };
    let mut k = format!("id={}", c.0);
    for r in &regs {
        match r.as_str() {
            "ecx" => k.push_str(&format!(" ecx={}", norm_ptr(c.1))),
            "edx" => k.push_str(&format!(" edx={}", norm_ptr(c.2))),
            _ => {}
        }
    }
    // v2: stack args are normalized too; volatile indexes can be dropped
    // with checks.call_skip {id:[idx,...]} (folded from lane q-16). Args of
    // an xmm0-transport callee are skipped: the original side pushes nothing.
    let skip: Vec<usize> = match checks.get("call_skip").and_then(|m| m.get(id.as_str())) {
        Some(v) => v.as_arr().iter().map(|x| x.as_usize()).collect(),
        None => Vec::new(),
    };
    if cal.xmm0_from_stack.is_none() && cal.xmm1_from_stack.is_none() {
        let args: Vec<String> =
            c.3.iter()
                .enumerate()
                .filter(|(i, _)| !skip.contains(i))
                .map(|(_, &a)| norm_ptr(a))
                .collect();
        k.push_str(&format!(" args={:?}", args));
    } else {
        k.push_str(" args=transport");
    }
    // Pointed-to snapshots compare by value (raw words: contents, not addresses).
    if !c.4.is_empty() {
        k.push_str(&format!(" snap={:?}", c.4));
    }
    if cal.logxmm {
        k.push_str(&format!(" xmm0={:?}", c.5));
    }
    if cal.logxmm1 {
        k.push_str(&format!(" xmm1={:?}", c.6));
    }
    k
}

fn lims(v: &[String], n: usize) -> Vec<String> {
    v.iter().take(n).cloned().collect()
}

fn lim(v: &[(u32, u32)], n: usize) -> Vec<String> {
    v.iter()
        .take(n)
        .map(|(a, b)| format!("0x{:x}=0x{:x}", a, b))
        .collect()
}

// FP compare: exact bits first; else f32/f64 abs+rel tolerance. Returns (pass, detail, exact).
fn fp_cmp(ah: &str, bh: &str, nbytes: usize, checks: &J) -> (bool, String, bool) {
    if ah == bh {
        return (true, "exact".to_string(), true);
    }
    let unhex = |h: &str| -> Vec<u8> {
        let mut v = Vec::new();
        let b = h.as_bytes();
        let mut i = 0;
        while i + 1 < b.len() {
            let hi = (b[i] as char).to_digit(16).unwrap_or(0);
            let lo = (b[i + 1] as char).to_digit(16).unwrap_or(0);
            v.push((hi * 16 + lo) as u8);
            i += 2;
        }
        v
    };
    let ab = unhex(ah);
    let bb = unhex(bh);
    let mut detail = format!("orig={} rw={} ", ah, bh);
    if ab.len() < nbytes || bb.len() < nbytes {
        return (false, detail + "short", false);
    }
    // Bitwise difference with zero tolerance is a failure, full stop.
    // (IEEE == would call -0.0 and +0.0 equal; the checker must not.)
    let tol = checks.get("fp_tol").map(|v| v.as_str()).unwrap_or("");
    let (tabs, trel) = if tol.is_empty() {
        (0.0f64, 0.0f64)
    } else {
        let mut sp = tol.split(',');
        let a = sp.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let r = sp.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        (a, r)
    };
    if tabs == 0.0 && trel == 0.0 {
        return (false, detail + "bitwise differ, zero tolerance", false);
    }
    // f32 lane (low 4 bytes little-endian) and f64 lane (low 8)
    let a32 = f32::from_le_bytes([ab[0], ab[1], ab[2], ab[3]]);
    let b32 = f32::from_le_bytes([bb[0], bb[1], bb[2], bb[3]]);
    let (abs, rel) = (tabs, trel);
    let close32 = (a32 == b32)
        || (!a32.is_nan()
            && !b32.is_nan()
            && ((a32 - b32).abs() as f64 <= abs + rel * (b32.abs() as f64)));
    if nbytes == 16
        && checks
            .get("fp64")
            .map(|v| v.as_bool(false))
            .unwrap_or(false)
    {
        let a64 = f64::from_le_bytes([ab[0], ab[1], ab[2], ab[3], ab[4], ab[5], ab[6], ab[7]]);
        let b64 = f64::from_le_bytes([bb[0], bb[1], bb[2], bb[3], bb[4], bb[5], bb[6], bb[7]]);
        let close64 = (a64 == b64)
            || (!a64.is_nan() && !b64.is_nan() && ((a64 - b64).abs() <= abs + rel * b64.abs()));
        detail.push_str(&format!("f64 {} vs {}", a64, b64));
        return (close64, detail, false);
    }
    // x87 st0: 80-bit extended; compare via f64 conversion of the 64-bit mantissa+exponent
    if nbytes == 10 {
        let cvt = |x: &[u8]| -> f64 {
            let man = u64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
            let ex = u16::from_le_bytes([x[8], x[9]]);
            let sign = if ex & 0x8000 != 0 { -1.0 } else { 1.0 };
            let e = (ex & 0x7FFF) as i32;
            if e == 0 && man == 0 {
                return 0.0 * sign;
            }
            if e == 0x7FFF {
                return if man & 0x7FFFFFFFFFFFFFFF == 0 {
                    sign * f64::INFINITY
                } else {
                    f64::NAN
                };
            }
            sign * (man as f64) * 2f64.powi(e - 16383 - 63)
        };
        let a = cvt(&ab);
        let b = cvt(&bb);
        let close =
            (a == b) || (!a.is_nan() && !b.is_nan() && ((a - b).abs() <= abs + rel * b.abs()));
        detail.push_str(&format!("st0 {} vs {}", a, b));
        return (close, detail, false);
    }
    detail.push_str(&format!("f32 {} vs {}", a32, b32));
    (close32, detail, false)
}

// ---------------------------------------------------------------------------
// Commands.
// ---------------------------------------------------------------------------

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn cmd_setup(q: &J) -> String {
    let s = st();
    if !s.mapped {
        let exe_path = q.get("exe").map(|v| v.as_str()).unwrap_or("");
        if exe_path.is_empty() {
            return "{\"ok\":false,\"error\":\"no exe\"}".to_string();
        }
        let bytes = std::fs::read(exe_path).unwrap_or_else(|_| Vec::new());
        if bytes.is_empty() {
            return "{\"ok\":false,\"error\":\"cannot read exe\"}".to_string();
        }
        s.before_stack = vec![0u32; STACK_SNAP_LEN / 4];
        s.exe_bytes = bytes.clone();
        if let Err(e) = map_image(&bytes) {
            return format!("{{\"ok\":false,\"error\":\"{}\"}}", esc(&e));
        }
    }
    // callees (stub space is reused per setup, but the first 16 bytes
    // hold the emitted TLS/ESP helpers and are never reused)
    s.callees.clear();
    s.stub_off = 16;
    if let Some(cs) = q.get("callees") {
        for c in cs.as_arr() {
            let id = c.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let conv = c
                .get("conv")
                .map(|v| v.as_str())
                .unwrap_or("cdecl")
                .to_string();
            let nargs = c.get("nargs").map(|v| v.as_usize()).unwrap_or(0);
            if nargs > LOG_MAXW {
                return format!(
                    "{{\"ok\":false,\"error\":\"callee {} nargs {} exceeds log window {}\"}}",
                    id, nargs, LOG_MAXW
                );
            }
            let ret = c
                .get("ret")
                .map(|v| v.as_str())
                .unwrap_or("u32")
                .to_string();
            // (Setup-time scripts are vestigial: per-trial answers arrive
            // with each trial request, so the setup list is not stored.)
            // v3: callee-cleaned stubs pop the full argument count. (v2 capped
            // the cleanup at 8 words like the log window, under-popping for
            // nargs > 8 and corrupting the caller's frame: lane r-b01's fix.)
            let pop = match conv.as_str() {
                "cdecl" => 0,
                _ => (nargs as u32) * 4, // stdcall/thiscall/fastcall/custom pop
            };
            // v2: out-param writes [{arg|reg, at, n}], snapshots, xmm logging.
            let mut writes = Vec::new();
            if let Some(ws) = c.get("writes") {
                for w in ws.as_arr() {
                    let (kind, idx) = if let Some(r) = w.get("reg") {
                        match r.as_str() {
                            "edx" => (2u8, 0usize),
                            _ => (1u8, 0usize),
                        }
                    } else {
                        (0u8, w.get("arg").map(|v| v.as_usize()).unwrap_or(0))
                    };
                    let at = w.get("at").map(|v| v.as_usize()).unwrap_or(0);
                    let n = w.get("n").map(|v| v.as_usize()).unwrap_or(0);
                    if at + n > WRITEW_PER_ID {
                        return format!(
                            "{{\"ok\":false,\"error\":\"callee {} writes overflow\"}}",
                            id
                        );
                    }
                    writes.push((kind, idx, at, n));
                }
            }
            let mut snap = Vec::new();
            let mut snap_total = 0usize;
            if let Some(ss) = c.get("snap") {
                for sn in ss.as_arr() {
                    let kind = sn.get("kind").map(|v| v.as_str()).unwrap_or("arg");
                    let k = match kind {
                        "ecx" => 1u8,
                        "edx" => 2u8,
                        _ => 0u8,
                    };
                    let idx = sn.get("idx").map(|v| v.as_usize()).unwrap_or(0);
                    let n = sn.get("n").map(|v| v.as_usize()).unwrap_or(0);
                    snap_total += n;
                    snap.push((k, idx, n));
                }
            }
            if snap_total > SNAP_MAXW {
                return format!("{{\"ok\":false,\"error\":\"callee {} snap overflow\"}}", id);
            }
            let logxmm = c.get("logxmm").map(|v| v.as_bool(false)).unwrap_or(false);
            let xmm0_from_stack = c.get("xmm0_from_stack").map(|v| v.as_usize());
            let logxmm1 = c.get("logxmm1").map(|v| v.as_bool(false)).unwrap_or(false);
            let xmm1_from_stack = c.get("xmm1_from_stack").map(|v| v.as_usize());
            let cal = Callee {
                id,
                conv,
                nargs,
                pop,
                ret,
                stub_addr: 0,
                tail_addr: 0,
                writes,
                snap,
                logxmm,
                xmm0_from_stack,
                logxmm1,
                xmm1_from_stack,
            };
            let bytes = emit_stub(&cal, None);
            let addr = s.stub_base + s.stub_off as u32;
            if s.stub_off + bytes.len() > 0x10000 {
                return "{\"ok\":false,\"error\":\"stub overflow\"}".to_string();
            }
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
                let proc = GetCurrentProcess();
                FlushInstructionCache(proc, addr as *const c_void, bytes.len());
                *((s.ctable + id * 4) as *mut u32) = addr;
            }
            s.stub_off += (bytes.len() + 15) & !15;
            let mut cal = cal;
            cal.stub_addr = addr;
            s.callees.insert(id, cal);
        }
    }
    // E8 patches
    let mut errors: Vec<String> = Vec::new();
    if let Some(ps) = q.get("patches") {
        for p in ps.as_arr() {
            let site_rva = p.get("site").map(|v| v.as_u32()).unwrap_or(0);
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let stub = match s.callees.get(&id) {
                Some(c) => c.stub_addr,
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            let site = s.img + site_rva as usize;
            match patch_e8(site, stub) {
                Ok(orig) => s.patches.push((site, orig, 5)),
                Err(e) => errors.push(e),
            }
        }
    }
    // v2 E9 tail-jump patches: the tail stub logs like a normal stub
    // but returns straight to the trampoline (outer_pop + the E8 slot).
    let outer_pop = q.get("outer_pop").map(|v| v.as_u32()).unwrap_or(0);
    if let Some(ps) = q.get("tailpatches") {
        for p in ps.as_arr() {
            let site_rva = p.get("site").map(|v| v.as_u32()).unwrap_or(0);
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let cal = match s.callees.get(&id) {
                Some(c) => c.clone(),
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            let tail = if cal.tail_addr != 0 {
                cal.tail_addr
            } else {
                let bytes = emit_stub(&cal, Some(outer_pop));
                let addr = s.stub_base + s.stub_off as u32;
                if s.stub_off + bytes.len() > 0x10000 {
                    return "{\"ok\":false,\"error\":\"stub overflow\"}".to_string();
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
                    let proc = GetCurrentProcess();
                    FlushInstructionCache(proc, addr as *const c_void, bytes.len());
                }
                s.stub_off += (bytes.len() + 15) & !15;
                if let Some(c) = s.callees.get_mut(&id) {
                    c.tail_addr = addr;
                }
                addr
            };
            let site = s.img + site_rva as usize;
            match patch_e9(site, tail) {
                Ok(orig) => s.patches.push((site, orig, 5)),
                Err(e) => errors.push(e),
            }
        }
    }
    // IAT patches
    let mut iat_done: Vec<String> = Vec::new();
    if let Some(ps) = q.get("iat") {
        for p in ps.as_arr() {
            let dll = p.get("dll").map(|v| v.as_str()).unwrap_or("");
            let name = p.get("name").map(|v| v.as_str()).unwrap_or("");
            let id = p.get("id").map(|v| v.as_u32()).unwrap_or(0);
            let stub = match s.callees.get(&id) {
                Some(c) => c.stub_addr,
                None => {
                    errors.push(format!("no callee {}", id));
                    continue;
                }
            };
            match find_iat(dll, name) {
                Some(slot) => unsafe {
                    let mut orig = [0u8; 8];
                    std::ptr::copy_nonoverlapping(slot as *const u8, orig.as_mut_ptr(), 4);
                    *(slot as *mut u32) = stub;
                    s.patches.push((slot, orig, 4));
                    iat_done.push(format!("{}!{}", dll, name));
                },
                None => errors.push(format!("iat {}!{} not found", dll, name)),
            }
        }
    }
    // declared globals
    s.globals.clear();
    if let Some(gs) = q.get("globals") {
        for g in gs.as_arr() {
            let rva = g.get("rva").map(|v| v.as_u32()).unwrap_or(0);
            let size = g.get("size").map(|v| v.as_usize()).unwrap_or(0);
            s.globals.push((s.img + rva as usize, size));
        }
    }
    // DLL
    let mut dll_base = String::from("null");
    let mut expmap = String::new();
    if let Some(d) = q.get("dll").map(|v| v.as_str()).filter(|x| !x.is_empty()) {
        if !s.dll.is_null() {
            unsafe {
                FreeLibrary(s.dll);
            }
            s.dll = std::ptr::null_mut();
        }
        s.exports.clear();
        let w = wide(d);
        let h = unsafe { LoadLibraryW(w.as_ptr()) };
        if h.is_null() {
            errors.push(format!("LoadLibrary failed: {}", d));
        } else {
            s.dll = h;
            dll_base = format!("\"0x{:x}\"", h as usize);
            // patch CHECKER_XBASE + CHECKER_CTABLE + CHECKER_XMM data exports
            // (CHECKER_XMM is v2-only; older lane DLLs lack it and report an
            // error string here, which is harmless unless the contract uses it)
            for (sym, val) in [
                ("CHECKER_XBASE\0", s.img as u32),
                ("CHECKER_CTABLE\0", s.ctable),
                ("CHECKER_XMM\0", s.m_xmm_mirror),
                ("CHECKER_TLS\0", s.m_tls_mirror),
            ] {
                let p = unsafe { GetProcAddress(h, sym.as_ptr() as *const i8) };
                if !p.is_null() {
                    unsafe {
                        *(p as *mut u32) = val;
                    }
                } else {
                    errors.push(format!("dll export {} missing", sym.trim_end_matches('\0')));
                }
            }
            let mut parts = Vec::new();
            if let Some(es) = q.get("exports") {
                for e in es.as_arr() {
                    let name = e.as_str().to_string();
                    let sym = format!("{}\0", name);
                    let p = unsafe { GetProcAddress(h, sym.as_ptr() as *const i8) };
                    if p.is_null() {
                        errors.push(format!("export {} missing", name));
                    } else {
                        s.exports.insert(name.clone(), p as u32);
                        parts.push(format!("\"{}\":\"0x{:x}\"", name, p as usize));
                    }
                }
            }
            expmap = parts.join(",");
        }
    }
    // v2: stub addresses for driver-side planting (vtables, data slots).
    let mut stubs: Vec<(&u32, &Callee)> = s.callees.iter().collect();
    stubs.sort_by_key(|(id, _)| *id);
    let stubmap = stubs
        .iter()
        .map(|(id, c)| format!("\"{}\":\"0x{:x}\"", id, c.stub_addr))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"ok\":true,\"img_base\":{},\"delta\":{},\"relocs\":{},\"heap\":{},\"stack\":{},\"dll_base\":{},\"text_lo\":{},\"text_hi\":{},\"exports\":{{{}}},\"stub_addrs\":{{{}}},\"iat_patched\":[{}],\"errors\":[{}]}}",
        hx(s.img as u32),
        hx(s.delta),
        s.relocs,
        hx(s.h as u32),
        hx(s.s as u32),
        dll_base,
        hx(s.text_lo as u32),
        hx(s.text_hi as u32),
        expmap,
        stubmap,
        iat_done
            .iter()
            .map(|x| format!("\"{}\"", x))
            .collect::<Vec<_>>()
            .join(","),
        errors
            .iter()
            .map(|x| format!("\"{}\"", esc(x)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn cmd_teardown() -> String {
    let s = st();
    for (addr, orig, len) in s.patches.clone() {
        unsafe {
            std::ptr::copy_nonoverlapping(orig.as_ptr(), addr as *mut u8, len);
        }
    }
    s.patches.clear();
    unsafe {
        std::ptr::write_bytes(s.ctable as *mut u8, 0, 1024);
    }
    s.callees.clear();
    s.globals.clear();
    if !s.dll.is_null() {
        unsafe {
            FreeLibrary(s.dll);
        }
        s.dll = std::ptr::null_mut();
    }
    s.exports.clear();
    restore_data();
    "{\"ok\":true}".to_string()
}

#[derive(Clone)]
struct TrialReq {
    fn_addr: u32,
    rw_addr: u32,
    regs: [u32; 7],
    stack: Vec<u32>,
    heapsegs: Vec<(usize, Vec<u32>)>,
    globals_fill: Vec<(usize, Vec<u32>)>,
    trial: u32,
    seed: u32,
    fulldata: bool,
    tls: Vec<(u32, u32)>, // (slot, value) fabricated TLS slots
    xmm: [u32; 32],       // xmm0-7 entry values
    stack_fill: Option<u32>,
}

fn parse_trial(q: &J) -> Result<(TrialReq, J), String> {
    let s = st();
    let fn_rva = q.get("fn_rva").map(|v| v.as_u32()).unwrap_or(0);
    let fn_addr = (s.img + fn_rva as usize) as u32;
    let rw_addr = if let Some(e) = q.get("export") {
        *s.exports
            .get(e.as_str())
            .ok_or_else(|| format!("unknown export {}", e.as_str()))?
    } else if let Some(a) = q.get("rw_rva") {
        (s.img + a.as_u32() as usize) as u32
    } else {
        return Err("trial needs export or rw_rva".to_string());
    };
    let regs_v = q
        .get("regs")
        .map(|v| v.as_arr().to_vec())
        .unwrap_or_default();
    if regs_v.len() != 7 {
        return Err("regs must have 7 entries".to_string());
    }
    let mut regs = [0u32; 7];
    for i in 0..7 {
        regs[i] = regs_v[i].as_u32();
    }
    let stack: Vec<u32> = q
        .get("stack")
        .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
        .unwrap_or_default();
    let mut heapsegs = Vec::new();
    if let Some(hs) = q.get("heapsegs") {
        for hseg in hs.as_arr() {
            let off = hseg.get("off").map(|v| v.as_usize()).unwrap_or(0);
            let words: Vec<u32> = hseg
                .get("words")
                .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
                .unwrap_or_default();
            if off + words.len() * 4 > HEAP_USE {
                return Err("heapseg out of range".to_string());
            }
            heapsegs.push((off, words));
        }
    }
    let mut globals_fill = Vec::new();
    if let Some(gf) = q.get("globals_fill") {
        if gf.as_str() == "random" {
            // deterministic per-trial fill of declared ranges
            let trial = q.get("trial").map(|v| v.as_u32()).unwrap_or(0);
            let seed = q.get("seed").map(|v| v.as_u32()).unwrap_or(0);
            for (ri, &(lo, len)) in s.globals.iter().enumerate() {
                let mut words = Vec::new();
                for wi in 0..(len / 4) {
                    words.push(fill_word(seed, trial, ri as u32, wi as u32));
                }
                globals_fill.push((lo, words));
            }
        } else if let Some(arr) = match gf {
            J::Arr(a) => Some(a),
            _ => None,
        } {
            for g in arr {
                let rva = g.get("rva").map(|v| v.as_u32()).unwrap_or(0);
                let words: Vec<u32> = g
                    .get("words")
                    .map(|v| v.as_arr().iter().map(|x| x.as_u32()).collect())
                    .unwrap_or_default();
                globals_fill.push((s.img + rva as usize, words));
            }
        }
        // "pristine" or missing: no fill
    }
    let trial = q.get("trial").map(|v| v.as_u32()).unwrap_or(0);
    let seed = q.get("seed").map(|v| v.as_u32()).unwrap_or(0);
    let checks = q.get("checks").cloned().unwrap_or(J::Obj(HashMap::new()));
    let fulldata = checks
        .get("fulldata")
        .map(|v| v.as_bool(true))
        .unwrap_or(true);
    // v2: fabricated TLS slots [{slot|slot_rva, value}]
    let mut tls = Vec::new();
    if let Some(ts) = q.get("tls") {
        for te in ts.as_arr() {
            let slot = if let Some(rva) = te.get("slot_rva") {
                let addr = s.img + rva.as_u32() as usize;
                unsafe { *((addr & !3) as *const u32) } // slot index word
            } else {
                te.get("slot").map(|v| v.as_u32()).unwrap_or(0)
            };
            let val = te.get("value").map(|v| v.as_u32()).unwrap_or(0);
            if slot < 64 {
                tls.push((slot, val));
            } else {
                return Err("tls slot >= 64 needs expansion slots (unsupported)".to_string());
            }
        }
    }
    // v2: XMM entry values (sparse {reg:[4 dwords]} or dense [32])
    let mut xmm = [0u32; 32];
    if let Some(x) = q.get("xmm") {
        match x {
            J::Arr(a) => {
                for (i, v) in a.iter().take(32).enumerate() {
                    xmm[i] = v.as_u32();
                }
            }
            J::Obj(m) => {
                for (k, v) in m {
                    if let Ok(r) = k.parse::<usize>()
                        && r < 8
                    {
                        for (i, w) in v.as_arr().iter().take(4).enumerate() {
                            xmm[r * 4 + i] = w.as_u32();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    // v2: defined uninitialized-stack fill (integer or "pattern")
    let stack_fill = match q.get("stack_fill") {
        Some(J::Int(i)) => Some(*i as u32),
        Some(J::Str(st)) if st.as_str() != "pattern" => Some(parse_u32(st)),
        _ => None,
    };
    Ok((
        TrialReq {
            fn_addr,
            rw_addr,
            regs,
            stack,
            heapsegs,
            globals_fill,
            trial,
            seed,
            fulldata,
            tls,
            xmm,
            stack_fill,
        },
        checks,
    ))
}

// The trial body runs on a dedicated thread so a hang can be cut off.
fn trial_body(
    req: TrialReq,
    checks: J,
    script_vals: Vec<(u32, u32, u32, Vec<u32>, Vec<(u32, u32)>)>,
) -> String {
    // v2: per-callee script slots + out-param write words, filled once per
    // trial and shared by both sides. v3: per-call answer sequences; a
    // callee without "seq" gets [(script lo,hi)] with length 1, exactly v2.
    unsafe {
        std::ptr::write_bytes(st().m_script_tab as *mut u8, 0, 2048);
        std::ptr::write_bytes(st().m_writebuf as *mut u8, 0, 16384);
        std::ptr::write_bytes(st().m_seq_tab as *mut u8, 0, 32768);
        std::ptr::write_bytes(st().m_seq_len as *mut u8, 0, 1024);
        for (id, lo, hi, w, seq) in &script_vals {
            if *id < 256 {
                *((st().m_script_tab + id * 8) as *mut u32) = *lo;
                *((st().m_script_tab + id * 8 + 4) as *mut u32) = *hi;
                for (j, word) in w.iter().take(WRITEW_PER_ID).enumerate() {
                    *((st().m_writebuf + id * 64 + j as u32 * 4) as *mut u32) = *word;
                }
                let owned: Vec<(u32, u32)> = if seq.is_empty() {
                    vec![(*lo, *hi)]
                } else {
                    seq[..seq.len().min(SEQ_MAX)].to_vec()
                };
                for (k, (slo, shi)) in owned.iter().enumerate() {
                    *((st().m_seq_tab + id * 128 + k as u32 * 8) as *mut u32) = *slo;
                    *((st().m_seq_tab + id * 128 + k as u32 * 8 + 4) as *mut u32) = *shi;
                }
                *((st().m_seq_len + id * 4) as *mut u32) = owned.len() as u32;
            }
        }
    }
    let mut f0 = 0i64;
    let mut f1 = 0i64;
    let mut fq = 0i64;
    unsafe {
        QueryPerformanceFrequency(&mut fq);
        QueryPerformanceCounter(&mut f0);
    }
    let orig = run_side(
        req.fn_addr,
        &req.regs,
        &req.stack,
        &req.heapsegs,
        &req.globals_fill,
        req.trial,
        req.seed,
        &req.tls,
        &req.xmm,
        req.stack_fill,
        false,
        req.fulldata,
    );
    let rw = run_side(
        req.rw_addr,
        &req.regs,
        &req.stack,
        &req.heapsegs,
        &req.globals_fill,
        req.trial,
        req.seed,
        &req.tls,
        &req.xmm,
        req.stack_fill,
        true,
        req.fulldata,
    );
    unsafe {
        QueryPerformanceCounter(&mut f1);
    }
    let us = if fq > 0 {
        ((f1 - f0) as f64 * 1e6 / fq as f64) as u64
    } else {
        0
    };
    let (pass, checks_json, first) = compare(&orig, &rw, &checks);
    // exact-fp flag for the driver's exact-match counts
    let ret = checks.get("ret").map(|v| v.as_str()).unwrap_or("eax");
    let exact = match ret {
        "st0" => orig.st0 == rw.st0,
        "xmm0" => orig.xmm0 == rw.xmm0,
        _ => true,
    };
    format!(
        "{{\"ok\":true,\"pass\":{},\"checks\":{},\"first_mismatch\":\"{}\",\"fp_exact\":{},\"trial_us\":{},\"orig\":{},\"rw\":{}}}",
        if pass { "true" } else { "false" },
        checks_json,
        esc(&first),
        if exact { "true" } else { "false" },
        us,
        obs_json(&orig),
        obs_json(&rw)
    )
}

fn main() {
    unsafe {
        let boxed = Box::new(State::new());
        ST = Box::into_raw(boxed);
        AddVectoredExceptionHandler(1, veh);
    }
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut out = std::io::stdout();
    // ready banner (driver waits for this)
    writeln!(out, "{{\"ready\":true,\"checker_version\":\"checker3\"}}").unwrap();
    out.flush().unwrap();
    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }
        let q = match parse_json(&line) {
            Ok(J::Obj(m)) => J::Obj(m),
            Ok(_) => {
                writeln!(out, "{{\"ok\":false,\"error\":\"not an object\"}}").unwrap();
                out.flush().unwrap();
                continue;
            }
            Err(e) => {
                writeln!(out, "{{\"ok\":false,\"error\":\"json: {}\"}}", esc(&e)).unwrap();
                out.flush().unwrap();
                continue;
            }
        };
        let cmd = q.get("cmd").map(|v| v.as_str()).unwrap_or("");
        match cmd {
            "ping" => {
                writeln!(out, "{{\"ok\":true,\"mapped\":{}}}", st().mapped).unwrap();
                out.flush().unwrap();
            }
            "setup" => {
                let r = cmd_setup(&q);
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "teardown" => {
                let r = cmd_teardown();
                writeln!(out, "{}", r).unwrap();
                out.flush().unwrap();
            }
            "trial" => {
                let timeout = q.get("timeout_ms").map(|v| v.as_u32()).unwrap_or(10000);
                // explicit scripted values (driver-resolved per callee scripts)
                let sv: Vec<(u32, u32, u32, Vec<u32>, Vec<(u32, u32)>)> = q
                    .get("script_vals")
                    .map(|v| {
                        v.as_arr()
                            .iter()
                            .map(|e| {
                                (
                                    e.get("id").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("lo").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("hi").map(|x| x.as_u32()).unwrap_or(0),
                                    e.get("w")
                                        .map(|w| w.as_arr().iter().map(|x| x.as_u32()).collect())
                                        .unwrap_or_default(),
                                    // v3: per-call answer steps [[lo,hi],...]
                                    e.get("seq")
                                        .map(|s| {
                                            s.as_arr()
                                                .iter()
                                                .map(|st| match st {
                                                    J::Arr(a) => (
                                                        a.first().map(|x| x.as_u32()).unwrap_or(0),
                                                        a.get(1).map(|x| x.as_u32()).unwrap_or(0),
                                                    ),
                                                    x => (x.as_u32(), 0),
                                                })
                                                .collect()
                                        })
                                        .unwrap_or_default(),
                                )
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let parsed = parse_trial(&q);
                match parsed {
                    Err(e) => {
                        writeln!(out, "{{\"ok\":false,\"error\":\"{}\"}}", esc(&e)).unwrap();
                        out.flush().unwrap();
                    }
                    Ok((req, checks)) => {
                        let (tx, rx) = mpsc::channel();
                        std::thread::Builder::new()
                            .name("trial".to_string())
                            .stack_size(8 * 1024 * 1024)
                            .spawn(move || {
                                let r = trial_body(req, checks, sv);
                                let _ = tx.send(r);
                            })
                            .unwrap();
                        match rx.recv_timeout(Duration::from_millis(timeout as u64)) {
                            Ok(r) => {
                                writeln!(out, "{}", r).unwrap();
                                out.flush().unwrap();
                            }
                            Err(_) => {
                                // hang: the trial thread is abandoned (tainted worker).
                                // Report, then exit so the driver starts a fresh worker.
                                writeln!(
                                    out,
                                    "{{\"ok\":true,\"pass\":false,\"hang\":true,\"checks\":[],\"first_mismatch\":\"watchdog timeout\",\"fp_exact\":false,\"trial_us\":{}}}",
                                    (timeout as u64) * 1000
                                )
                                .unwrap();
                                out.flush().unwrap();
                                diag("watchdog: trial timed out, exiting");
                                unsafe {
                                    ExitProcess(0);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                writeln!(out, "{{\"ok\":false,\"error\":\"unknown cmd\"}}").unwrap();
                out.flush().unwrap();
            }
        }
    }
}
