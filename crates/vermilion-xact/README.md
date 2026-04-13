# Vermilion - XACT

> [!WARNING]
> This is currently a stub crate for eventual IP-XACT support [Vermilion], it is not the focus until
> the main HDL language support is implemented and working well enough to add ancillary support for
> IP-XACT libraries

This crate provides [IEEE 1685] IP-XACT support for [Vermilion], allowing for snippet generation and autocomplete from provided IP libraries. However, it could also be used in general flows that wish to utilize or generate IP-XACT libraries.

The following IP-XACT schemas are targeted for support:

- SPIRIT 1.0
- SPIRIT 1.1
- SPIRIT 1.2
- SPIRIT 1.4
- SPIRIT 1.5
- IP-XACT 2009 w/ Vendor Extensions
- IP-XACT 2014
- IP-XACT 2022 2/ Vendor Extensions

## License

Vermilion is licensed under the [BSD-3-Clause], the full text of which can be found in the [`LICENSE`] file in the root of the [git repository].

The Vermilion documentation is licensed under the [CC-BY-SA 4.0], the full text of which can be found in the [`LICENSE.docs`] file in the root of the [git repository].

[Vermilion]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion/src/branch/main/crates/vermilion
[IEEE 1685]: https://standards.ieee.org/ieee/1685/10583/
[BSD-3-Clause]: https://spdx.org/licenses/BSD-3-Clause.html
[`LICENSE`]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion/src/branch/main/LICENSE
[CC-BY-SA 4.0]: https://creativecommons.org/licenses/by-sa/4.0/
[`LICENSE.docs`]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion/src/branch/main/LICENSE.docs
[git repository]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion
