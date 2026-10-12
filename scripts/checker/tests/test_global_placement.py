"""Tests for global_placement.py on the contract of record for 0x00B3F120 (RVAs only, no game code)."""

import copy
import os
import sys
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import global_placement as gp  # noqa: E402

# The repository root is three levels up from this file; LIBERTYFLUX_ROOT overrides it (for a copy).
ROOT = Path(os.environ.get("LIBERTYFLUX_ROOT", str(HERE.parent.parent.parent)))
EXE = ROOT / "orig" / "GTAIV.exe"
FN_RVA = 0x73F120  # 0x00B3F120 with image base 0x400000

RECORD = {
    "globals": [
        {"rva": 19289084, "size": 12}, {"rva": 19694944, "size": 16},
        {"rva": 19627064, "size": 24}, {"rva": 6587052, "size": 8},
    ],
    "globals_values": [
        {"rva": "0x1265404", "words": [{"cycle": [0, 1, 1, 1]}]},
        {"rva": "0x1265400", "words": [{"cycle": [4194304, 4194304, 262144, 5]}]},
        {"rva": "0x12653FC", "words": [{"cycle": [{"heap": 3, "plus": 0}]}]},
        {"rva": "0x6482AC", "words": [{"cycle": [4, 2, 6, 0, 5, 1, -7]}]},
        {"rva": "0x6482B0", "words": [{"cycle": [4, 2, 6, 3, 5, 1, 0]}]},
    ],
}


def fixed(contract):
    """The same contract with the divisor and size at the RVAs the original reads (0xC482AC, 0xC482B0)."""
    c = copy.deepcopy(contract)
    for g in c["globals_values"]:
        if g["rva"] == "0x6482AC":
            g["rva"] = "0xC482AC"
        elif g["rva"] == "0x6482B0":
            g["rva"] = "0xC482B0"
    c["globals"][3]["rva"] = 0xC482AC
    return c


@unittest.skipUnless(EXE.is_file(), "orig/GTAIV.exe is not present")
class PlacementTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.image = gp.Image(str(EXE))

    def kinds(self, findings):
        return [(f["level"], f["kind"]) for f in findings]

    def test_record_places_values_in_code(self):
        found = gp.check_contract(self.image, RECORD, FN_RVA)
        refusals = [f for f in found if f["level"] == "refuse"]
        self.assertTrue(refusals, found)
        self.assertEqual(refusals[0]["where"], "text")
        self.assertIn(refusals[0]["rva"], ("0x6482ac", "0x6482AC"))

    def test_fixed_addresses_pass_placement_and_are_referenced(self):
        found = gp.check_contract(self.image, fixed(RECORD), FN_RVA)
        self.assertEqual([f for f in found if f["level"] == "refuse"], [], found)
        self.assertNotIn(("warn", "value_unreferenced"), self.kinds(found), found)

    def test_image_address_form_is_flagged_as_unreferenced(self):
        c = fixed(RECORD)
        for g in c["globals_values"]:
            if g["rva"] == "0xC482AC":
                g["rva"] = "0x10482AC"
            elif g["rva"] == "0xC482B0":
                g["rva"] = "0x10482B0"
        found = gp.check_contract(self.image, c, FN_RVA)
        # the image address is a writable word, so placement passes; the body never reads it
        self.assertEqual([f for f in found if f["level"] == "refuse"], [], found)
        unref = [f for f in found if f["kind"] == "value_unreferenced"]
        self.assertEqual(len(unref), 2, found)

    def test_unset_size_is_reported_as_uncovered_read(self):
        c = fixed(RECORD)
        c["globals_values"] = [g for g in c["globals_values"] if g["rva"] != "0xC482B0"]
        found = gp.check_contract(self.image, c, FN_RVA)
        uncovered = [f for f in found if f["kind"] == "read_uncovered"]
        self.assertTrue(uncovered, found)
        rvas = [r["rva"] for r in uncovered[0]["rvas"]]
        self.assertIn("0xc482b0", rvas)

    def test_read_only_placement_is_a_warning_not_a_refusal(self):
        c = fixed(RECORD)
        c["globals_values"].append({"rva": "0xA73100", "words": [{"cycle": [1, 2]}]})  # inside .rdata
        found = gp.check_contract(self.image, c, FN_RVA)
        self.assertEqual([f for f in found if f["level"] == "refuse"], [], found)
        rd = [f for f in found if f["kind"] == "placement_rdata"]
        self.assertEqual(len(rd), 1, found)
        self.assertEqual(rd[0]["where"], "rdata")

    def test_gap_and_outside_image_are_refused(self):
        c = fixed(RECORD)
        c["globals_values"].append({"rva": "0x100", "words": [5]})  # header, in no section
        c["globals"].append({"rva": self.image.size + 0x10, "size": 4})  # past the image
        found = gp.check_contract(self.image, c, FN_RVA)
        wheres = sorted(f["where"] for f in found if f["level"] == "refuse")
        self.assertEqual(wheres, ["gap", "outside_image"], found)

    def test_body_reads_divisor_and_size(self):
        ops = self.image.body_data_operands(FN_RVA)
        self.assertIn(0xC482AC, ops)
        self.assertIn(0xC482B0, ops)
        self.assertNotIn(0x6482AC, ops)


if __name__ == "__main__":
    unittest.main()
