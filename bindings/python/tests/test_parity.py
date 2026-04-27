import json
from pathlib import Path

import ootd
import pytest


def test_parity_cases() -> None:
    root = Path(__file__).resolve().parents[3]
    fixture = json.loads((root / "tests" / "parity_cases.json").read_text(encoding="utf-8"))

    for case in fixture["between_cases"]:
        out = ootd.between(case["start"], case["end"], case["locale"], case.get("use_native_ko_number", False))
        assert out == case["expected"]

    for case in fixture["duration_cases"]:
        if "expected_error" in case:
            with pytest.raises(ValueError, match=case["expected_error"]):
                ootd.from_duration(case["seconds"], case["is_future"], case["locale"], case.get("use_native_ko_number", False))
        else:
            out = ootd.from_duration(case["seconds"], case["is_future"], case["locale"], case.get("use_native_ko_number", False))
            assert out == case["expected"]
