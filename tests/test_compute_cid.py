from pathlib import Path

from py_ipfs_cid import compute_cid
import pytest


@pytest.mark.parametrize(
    "data,expected_cid", [(b"1234", "QmTPqcLhVnCtjoYuCZwPzfXcFrUviiPComTepHfEEaGf7g")]
)
def test_compute_cid(data: bytes, expected_cid: str):
    cid = compute_cid(data)
    assert cid == expected_cid


def test_compute_cid_no_data():
    cid = compute_cid(b"")
    assert cid == "QmbFMke1KXqnYyBBWxB74N4c5SBnJMVAiMNRcGu6x1AwQH"


def test_compute_cid_large_file():
    """
    Test with a file larger than the IPFS block size (256KiB).
    """

    large_file_path = Path(__file__).parent / "large_file.bin"
    with large_file_path.open("rb") as f:
        data = f.read()

    # Sanity check: the file must be larger than the IPFS block size
    assert len(data) > 256 * 1024

    cid = compute_cid(data)
    assert cid == "QmSzoKhfEGFVdTBQLYckQi3P8cts2C1h8shx6DMCyBq3id"
