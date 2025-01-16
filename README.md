# limine-genconf

Generates `limine.conf` from your `/boot` directory

## Usage

```shell
limine-genconf --cmdline "loglevel=4 cryptdevice=/dev/nvme0n1p1:cryptlvm root=/dev/mapper/cryptlvm rw" --timeout 1 2>/dev/null
```
