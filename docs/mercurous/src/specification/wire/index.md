<!-- markdownlint-disable MD033 -->
# Wire Format

Mercurous uses a compact binary representation to communicate over its channels rather than something like raw [JSON] or [JSON-RPC].

This was picked over a plain-text format in order to attempt to improve bandwidth usage and ease/speed of serializing and deserializing messages.

## Fundamental Data Types

`i8`
   : A two's complement 8-bit integer representing values in range -2<sup>7</sup> to 2<sup>7</sup>-1 (`-128..=127`)

`u8`
   : An unsigned 8-bit integer representing values in the range 0 to 2<sup>8</sup>-1 (`0..=255`)

`i16`
   : A little endian two's complement 16-bit integer representing values in the range -2<sup>15</sup> to 2<sup>15</sup>-1 (`-32768..=32767`)

`u16`
   : A little endian unsigned 16-bit integer representing values in the range 0 to 2<sup>16</sup>-1 (`0..=65535`)

`i32`
   : A little endian two's complement 32-bit integer representing values in the range -2<sup>31</sup> to 2<sup>31</sup>-1 (`-2147483648..=2147483647`)

`u32`
   : A little endian unsigned 32-bit integer representing values in the range 0 to 2<sup>32</sup>-1 (`0..=4294967295`)

`i64`
   : A little endian two's complement 64-bit integer representing values in the range -2<sup>63</sup> to 2<sup>63</sup>-1 (`-9223372036854775808..=9223372036854775807`)

`u64`
   : A little endian unsigned 64-bit integer representing values in the range 0 to 2<sup>64</sup>-1 (`0..=18446744073709551615`)

`string`
   : A `u16` length-prefixed collection of one or more [UTF-8] code points.

## Message Format

Every message, regardless of channel shares the common superstructure:

```text
    0             0               1               2               3
    0             7               5               3               1
   ╭┴─────────────┴───────────────┴───────────────┴───────────────┴╮
00 │                            Length                             │
   ├──────────────┬───────────────┬────────────────────────────────┤
04 │   Version    │      Type     │            Reserved            │
   ├──────────────┴───────────────┴────────────────────────────────┤
08 ┊                             Data                              ┊
   ╰┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄╯
```

[JSON]: https://www.json.org
[JSON-RPC]: https://www.jsonrpc.org/
[UTF-8]: https://www.unicode.org/versions/Unicode17.0.0/core-spec/chapter-3/#G31703
