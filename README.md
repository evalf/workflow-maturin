Reusable workflow for building a Python package with Maturin
============================================================

This workflow builds binary distributions using a single version of Python,
hence should only be used for packages using Python's stable ABI.

Usage:

```yaml
jobs:
  build-and-test-dist:
    name: Build and test distribution
    uses: evalf/workflow-maturin/.github/workflows/build-and-test.yaml@release/1
    with:
      # Name of the distribution artifact. (optional)
      artifact: dist
```
