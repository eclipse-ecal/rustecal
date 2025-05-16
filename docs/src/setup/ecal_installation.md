# eCAL Library Installation

`rustecal` is built on the eCAL v6 API and is not compatible with earlier eCAL v5 releases.

## Windows

- Install [eCAL](https://github.com/eclipse-ecal/ecal/releases)
- Set the environment variable:

```powershell
$env:ECAL_HOME = "C:\eCAL"
```

Expected structure:

```
%ECAL_HOME%/
├── include/ecal_c/
└── lib/ecal_core_c.lib
```

## Linux

Install system-wide from source or package. Headers and libraries should be in:

- `/usr/include/ecal_c/` or `/usr/local/include/ecal_c/`
- `/usr/lib` or `/usr/local/lib`
