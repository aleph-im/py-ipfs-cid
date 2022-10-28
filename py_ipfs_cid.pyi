from enum import Enum
from typing import Optional

class CidVersion(Enum):
    V0: None
    V1: None

def compute_cid(data: bytes, cid_version: Optional[CidVersion] = None) -> str: ...
