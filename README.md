# `ext_count`

A very basic CLI tool to summarize file extensions and best-guess content types.

### Help
```
root@goatse.cx ext_count % ext_count --help
Counts files by extension and displays MIME types

Usage: ext_count [DIRECTORY]

Arguments:
  [DIRECTORY]  Sets the working directory. Defaults to current directory if not set. [default: .]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Usage
```
root@goatse.cx ext_count % ext_count ~/Downloads/

+--------------------+-------+-----------------------------------+--------------------+
| Extension          | Count | MIME Types                        | Size (kb)          |
+--------------------+-------+-----------------------------------+--------------------+
| .diasqlite         | 3     | {"application/x-sqlite3"}         | 24.00 ± 0.00       |
| .mjs               | 2     | {"text/x-modelica"}               | 5.52 ± 6.11        |
| .py                | 13921 | {"text/plain", "text/x-matlab"}   | 13.49 ± 26.73      |
| .exfat             | 1     | {"application/x-executable"}      | 53.06 ± NaN        |
| .Xml               | 1     | {"text/plain"}                    | 61.33 ± NaN        |
| .chm               | 3     | {"application/octet-stream"}      | 1612.38 ± 2103.92  |
| .msi               | 3     | {"application/x-ole-storage"}     | 58019.07 ± 3862.77 |
| .mgc               | 8     | {"application/octet-stream"}      | 812.59 ± 2293.74   |
| .swp               | 2     | {"application/octet-stream"}      | 12.00 ± 0.00       |
| .prism             | 9     | {"application/zip"}               | 138.31 ± 188.82    |
| .curve_Oc1_SAM_PrL | 1     | {"application/x-ole-storage"}     | 136.50 ± NaN       |
| .svcinfo           | 4     | {"text/plain", "application/xml"} | 10.78 ± 10.32      |
+--------------------+-------+-----------------------------------+--------------------+
```

### TODO
- AsyncIO
- Parametrized output
- Sorting
- Tests
