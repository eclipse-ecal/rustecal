# Prerequisites

## Rust Toolchain

- Rust >= 1.70
- cargo, rustc

## LLVM + libclang (required for `bindgen`)

| Platform | Install |
|----------|---------|
| Windows  | `choco install llvm` or [LLVM releases](https://github.com/llvm/llvm-project/releases) |
| Linux    | `sudo apt install build-essential llvm-dev clang libclang-dev` |

### Environment Variable for Bindgen (Windows only)

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```
