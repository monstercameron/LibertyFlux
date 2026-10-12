"""global_placement.py: checks where a contract's globals and global values land in the original.

A contract's `globals` ranges and `globals_values` entries are image RVAs (offsets from the image
base, decimal or 0x-hex strings), and the worker writes each value at (image + rva). Nothing else
checks that the address is a global the function reads. Two faults of that kind were found on
2026-10-11 (lane k-chk4): a value named at RVA 0x6482AC lies in .text (code), and the function reads its
divisor at RVA 0xC482AC, so the value was never seen and both sides read the file's -1.

Findings
  refuse  placement: a declared range or value starts in code (.text), in a gap between sections (or the
          header), or past the image. These can never be meant.
  warn    placement_rdata: a declared range or value starts in read-only data (.rdata and similar). The
          worker maps the whole image writable, so the value is written and restored after each side;
          varying a constant the original loads from read-only data is a legitimate test.
  warn    value_unreferenced: a value's words are not absolute data operands of the function's body.
          Typical causes are an image address written where an RVA is meant (the value lands on a
          different word) or a wrong address.
  warn    read_uncovered: the body reads or writes an absolute address in writable data that no
          declared range and no set value covers, in a contract that sets globals. Such a global keeps
          its pristine file value on both sides, and the checker does not compare it by name.

The body is approximate: it is decoded from the function's start to the next verified function start
(or `body_cap` bytes), so operands of neighbouring code can appear. Warnings are for review; refusals
are not waivable.
"""

import capstone
import pefile

IMAGE_BASE_DEFAULT = 0x400000
BODY_CAP_DEFAULT = 0x2000


def _int(v):
    return int(v, 16) if isinstance(v, str) else int(v)


class Image:
    """The original's sections and mapped bytes, read through pefile's mapped image."""

    def __init__(self, exe_path):
        self.pe = pefile.PE(exe_path, fast_load=True)
        self.base = self.pe.OPTIONAL_HEADER.ImageBase
        self.size = self.pe.OPTIONAL_HEADER.SizeOfImage
        self.mapped = self.pe.get_memory_mapped_image()
        self.sections = []
        for s in self.pe.sections:
            lo = s.VirtualAddress
            hi = lo + max(s.Misc_VirtualSize, s.SizeOfRawData)
            self.sections.append({
                "name": s.Name.rstrip(b"\0").decode("ascii", "replace"),
                "lo": lo, "hi": hi,
                "write": bool(s.Characteristics & 0x80000000),
                "exec": bool(s.Characteristics & 0x20000000),
            })

    def section_of(self, rva):
        for s in self.sections:
            if s["lo"] <= rva < s["hi"]:
                return s
        return None

    def placement(self, rva):
        """'data' for writable non-code data; otherwise 'text', 'rdata', 'gap' or 'outside_image'."""
        if rva < 0 or rva >= self.size:
            return "outside_image"
        s = self.section_of(rva)
        if s is None:
            return "gap"
        if s["exec"]:
            return "text"
        if s["write"]:
            return "data"
        return "rdata"

    def body_data_operands(self, fn_rva, next_start_rva=None, body_cap=BODY_CAP_DEFAULT):
        """RVAs of absolute memory operands in writable data within the function's approximate body."""
        end = fn_rva + body_cap
        if next_start_rva is not None and fn_rva < next_start_rva < end:
            end = next_start_rva
        code = bytes(self.mapped[fn_rva:end])
        md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
        md.detail = True
        found = {}
        for ins in md.disasm(code, self.base + fn_rva):
            for op in ins.operands:
                if op.type != capstone.x86.X86_OP_MEM or op.mem.base != 0 or op.mem.index != 0:
                    continue
                rva = (op.mem.disp & 0xFFFFFFFF) - self.base
                s = self.section_of(rva) if 0 <= rva < self.size else None
                if s is not None and s["write"] and not s["exec"]:
                    found.setdefault(rva, hex(ins.address))
        return found  # rva -> first instruction address that uses it


def check_contract(image, contract, fn_rva, next_start_rva=None, body_cap=BODY_CAP_DEFAULT):
    """Return a list of findings (dicts with 'level', 'kind' and details) for one contract."""
    findings = []
    gl = contract.get("globals", []) or []
    gv = contract.get("globals_values", []) or []

    spans = []  # (lo, hi, label) of every declared or valued word range
    for i, g in enumerate(gl):
        lo = _int(g["rva"])
        hi = lo + int(g.get("size", 4))
        spans.append((lo, hi, "globals[%d]" % i))
    for i, g in enumerate(gv):
        lo = _int(g["rva"])
        hi = lo + 4 * len(g.get("words", [None]))
        spans.append((lo, hi, "globals_values[%d]" % i))

    for lo, hi, label in spans:
        where = image.placement(lo)
        if where == "data":
            continue
        if where == "rdata":
            # Read-only data is mapped writable by the worker (the whole image is mapped RWX), and the
            # fill is restored after each side. Varying a constant the original loads from read-only
            # data is a legitimate test, so this is a warning, not a refusal.
            findings.append({"level": "warn", "kind": "placement_rdata", "entry": label,
                             "rva": hex(lo), "where": where})
            continue
        # text (code), a gap between sections or the header, or past the image: never meant.
        findings.append({"level": "refuse", "kind": "placement", "entry": label,
                         "rva": hex(lo), "where": where})

    if not (gl or gv):
        return findings

    ops = image.body_data_operands(fn_rva, next_start_rva, body_cap)
    for lo, hi, label in spans:
        if label.startswith("globals_values") and not any(lo <= r < hi for r in ops):
            findings.append({"level": "warn", "kind": "value_unreferenced", "entry": label,
                             "rva": hex(lo),
                             "note": "no absolute data operand of the body lies in this range"})

    # A read is covered only by a value the contract sets. A declared range (globals) makes writes
    # compare; it says nothing about the value a read sees, which stays the file's pristine word.
    set_spans = [(lo, hi) for lo, hi, label in spans if label.startswith("globals_values")]
    uncovered = []
    for r in sorted(ops):
        if not any(lo <= r < hi for lo, hi in set_spans):
            uncovered.append({"rva": hex(r), "first_insn": ops[r]})
    if uncovered:
        findings.append({"level": "warn", "kind": "read_uncovered", "rvas": uncovered,
                         "count": len(uncovered)})
    return findings
