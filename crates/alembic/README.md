# Alembic

> [!WARNING]
> Alembic is a wishlist part of Vermilion, it's not going to get any focus until the core of Vermilion
> and all of it's features are properly implemented and working well.

Alembic is utility to minimize HDL code while preserving one or more properties of interest. It is in the same family as tools such as [C-Reduce] or more applicably [sv-bugpoint], where the target goal for Alembic is to produce minimal test cases that cause unexpected or incorrect behaviour in EDA tooling.

Alembic is part of the [Vermilion] project, as such it should be able to cover [Verilog], [Verilog-AMS], [SystemVerilog], [VHDL], and [VHDL-AMS] HDL inputs. But also possibly be extended for other supported languages that [Vermilion] knows how to read, such as `SDC` constraint files, or [Synopsys Liberty] libraries.

## License

Alembic is part of the [Vermilion] project, and therefore is licensed under the [BSD-3-Clause], the full text of which can be found in the [`LICENSE`] file in the root of the [git repository].

The Alembic documentation is licensed under the [CC-BY-SA 4.0], the full text of which can be found in the [`LICENSE.docs`] file in the root of the [git repository].

[C-Reduce]: https://github.com/csmith-project/creduce
[sv-bugpoint]: https://github.com/antmicro/sv-bugpoint
[Vermilion]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion
[Verilog]: https://standards.ieee.org/ieee/1364/3641/
[Verilog-AMS]: https://www.accellera.org/downloads/standards/v-ams
[SystemVerilog]: https://standards.ieee.org/ieee/1800/7743/
[VHDL]: https://standards.ieee.org/ieee/1076/5179/
[VHDL-AMS]: https://standards.ieee.org/ieee/1076.1/5180/
[Synopsys Liberty]: https://www.synopsys.com/community/interoperability-programs/tap-in.html
[BSD-3-Clause]: https://spdx.org/licenses/BSD-3-Clause.html
[`LICENSE`]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion/src/branch/main/LICENSE
[CC-BY-SA 4.0]: https://creativecommons.org/licenses/by-sa/4.0/
[`LICENSE.docs`]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion/src/branch/main/LICENSE.docs
[git repository]: https://codeberg.org/shrine-maiden-heavy-industries/Vermilion
