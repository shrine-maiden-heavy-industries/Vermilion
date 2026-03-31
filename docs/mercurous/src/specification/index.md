# Overview

> [!CAUTION]
> Mercurous is currently only a draft, as such things may change without warning
> in incompatible ways. Once the specification is stabilized there will be an effort
> made to ensure backwards compatibility in accordance with [SEMVER]

Mercurous is not so much a single protocol as it is three different protocols in a trench coat. They are all closely related, and share many common aspects, such as a [wire format], and [transport] mechanisms.

## Conventions

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [BCP 14] [[RFC 2119]] [[RFC 8174]] when they appear in either all capitals as shown here, regardless of formatting, or in ***boldface italics*** otherwise. (e.g. ***must***, ***must not***, ***required***, etc.)

## Nomenclature

Model
  : A collection of HDL files that make up a single functional unit with a well defined interface for input
    and output signals.

Model Compiler
  : A piece of software that takes a **model** defined in an **HDL** and translates it into traditional code
    that can be either integrated into another piece of software or driven from a **Simulator**.

HDL
  : Hardware Description Language, also commonly called an RTL (Register Transfer Language).

Waveform Data
  : Information describing the state of signals at one or more monotonically increasing points in time.

Simulator
  : A piece of software that evaluates the **HDL** expressions from a given **Model**.

Simulation
  : The process of running a **Model** with a **Simulator**

## Additional Information

You will find two main sections for additional information, the [normative references], which contain links and citations to other documents that contain literature that we reference or expand on here, and the [F.A.Q], which tries to provide quick answers to common questions that might arise.

[SEMVER]: https://semver.org/
[wire format]: ./wire/index.md
[transport]: ./transport/index.md
[BCP 14]: https://www.rfc-editor.org/info/bcp14
[RFC 2119]: https://datatracker.ietf.org/doc/html/rfc2119
[RFC 8174]: https://datatracker.ietf.org/doc/html/rfc8174
[normative references]: ../references.md
[F.A.Q]: ../faq.md
