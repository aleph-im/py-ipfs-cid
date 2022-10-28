# Python IPFS CID

Computes IPFS CIDs from Python code.

This library computes the IPFS DAG of the input data and then returns the CID of the root node.
This returns the same CID as would be computed by `ipfs add <file>`.

## Example

```python
from py_ipfs_cid import compute_cid
cid: str = compute_cid(b"1234")  # Returns QmTPqcLhVnCtjoYuCZwPzfXcFrUviiPComTepHfEEaGf7g
```

## Current limitations

* Only supports CID v0
* Only supports files, not directories
* Only supports the default IPFS block size (256 KiB)
