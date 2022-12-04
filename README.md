# ANSImulator

Simulates ANSI escape codes on an in-memory double buffer, can be "flipped"
and then rendered to an actual terminal using minimal actual IO commands.

This is intentionally somewhat minimalist in implementation and supported features,
as I don't have that much time to maintain it, nor do I particularly want to build
or maintain it. It is simply a side effect of needing it but not finding it.

If you looking for something more powerful, [`termwiz`][termwiz] may be what you
want.

[termwiz]: https://docs.rs/termwiz/latest/termwiz/
