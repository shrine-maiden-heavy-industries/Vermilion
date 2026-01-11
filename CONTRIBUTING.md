# Contribution Guidelines

Hey! Thanks for your interest in contributing to Torii!

A few important things to note up front:

* Contributions to the Vermilion source code are licensed under the [BSD-3-Clause], the full text of which can be found in the [`LICENSE`] file in the root of the [git repository].
* Contributions to the Vermilion documentation are licensed under the [CC-BY-SA 4.0], the full text of which can be found in the [`LICENSE.docs`] file in the root of the [git repository].
* When contributing to Vermilion and interacting with people in/around the project be sure to keep in mind the [Code Of Conduct].
* We have a strict [AI usage policy], so be sure to read that.

Now that we've got that out of the way, there are a few ways to contribute. The first two are by either [reporting issues] or [suggesting features], which we actively encourage users and developers alike to do -
we can't fix what we don't know about and we can't implement what we don't realise users want, making this a fantastic way to contribute. The next two are working on [development and testing] or on the [documentation].

No matter what it is you are looking to contribute, we would love to get you involved! The detailed instructions for each of them are outlined in their own sections below.

We look forward to your involvement!

## Index

* [Reporting Issues]
* [Suggesting Features]
* [Development And Testing]
  * [Setting Up An Environment]
  * [Code Formatting]
  * [Testing]
  * [Commit Guidelines]
  * [Branch Naming]
  * [Submitting A Pull Request]
* [Documentation]
  * [The Vermilion Book]
  * [Crate API Documentation]
* [AI Usage Policy]
  * [TL;DR]

## Reporting Issues

We want Vermilion to be as capable and friendly as possible, as such it is extremely important that we get feedback from users so we know what snags, sharp corners, unclear information, and unexpected behavior they encounter.

It is asked that you please report any and all problems you encounter when using Vermilion, this can be from small things such as typos or weird wording, and unclear/confusing messages and diagnostics, to larger things such as general useability and tool integration.

Reporting an issue with Vermilion is done through the [issue tracker], the issue templates should guide you through most of the needed information. A quick summary of what would be needed is as follows:

* The version of Vermilion being used, this can be shown by running `vermilion --version`
* Which operating system and it's version this happened on, this can help narrow down platform-specific issues.
* A Minimal, Complete, and Verifiable Example that reproduces the issue if possible.

Any other information you can provide will be a huge help in getting your problem solved, be it technical, or if it's a general useability issue with the tooling or language.

## Suggesting Features

> [!NOTE]
> Before suggesting a *very large* feature, it is highly advised that you reach out to the
> maintainers via the [IRC channel] or the [discussion forum] to talk about it first.

We are trying to make Vermilion as widely useable and capable as possible, as such we love hearing about things that would help us achieve that goal.

Suggesting a feature is also done through the [issue tracker], there is a dedicate template for feature suggestions, and it will walk you through everything you need to submit the suggestion to us.

While it's fairly lax at the moment, eventually we will likely develop a formal RFC process for larger more sweeping changes, so keep an eye out for that.

## Development And Testing

> [!NOTE]
> Just like with [Suggesting Features], it is recommended that before you make a large
> change or contribution, to discuss it with the maintainers to see if there are any potential
> snags or foreseeable difficulties with it. This will save you and the maintainers a large
> amount of potential frustration and time.

The guidelines and instructions below are a best-effort attempt at describing the least-friction way of getting started with doing development and testing for Vermilion. If you notice any problems then please do [report an issue], and if you need any help then feel free to use the [discussion forum] or join the [IRC channel] and ask there.

### Setting Up An Environment

Vermilion is written in Rust, so you'll need a reasonably modern rust toolchain (the MSRV is 1.92), and that's about it. If you're using something like VSCode, there are some pre-configured settings in the root of the repo that if you have [`rust-analyzer`] installed will set things up right away. Eventually we'll have a `shell.nix` to automate more environment setup, but that's for the future.

With all of that sorted, you're now ready to get started getting the code:

1. [Fork] the repository to your GitHub account and clone it locally if you've not done so already previously.
1. Within the local repository, add this repo as a remote: `git remote add upstream https://github.com/shrine-maiden-heavy-industries/Vermilion`
1. If you already have an existing clone, make sure it's up to date:
   1. Fetch all pending changes: `git fetch --all --prune`
   1. Switch back to the `main` branch: `git switch main`
   1. Pull the changes locally: `git pull upstream`
   1. Finally make sure your remote is up to date: `git push`
1. Create a new branch for your contribution: `git switch -c type/branch-name` (see the [Branch Naming] section for details on how to name it)

You're now ready to make your changes! Make sure see the [Formatting Code] and [Commit Guidelines] sections, and when you're ready to submit your changes for review, follow the steps in the [Submitting a Pull Request] section.

### Code Formatting

Formatting is handled by [`rustfmt`], so setting up your editor or [rust-analyzer] to automatically format on save is the recommended way to do things.

> [!IMPORTANT]
> We do use some nightly-only settings for [`rustfmt`], so in order to properly invoke it you also need the nightly
> toolchain installed. Then invoke formatting with `+nightly` for `cargo` and `rustfmt`.

### Testing

Vermilion uses the standard benchmark and testing infrastructure provided by rust, so `cargo test` and `cargo bench` are sufficient, but if you would like using something like [`cargo-nextest`] to speed up test execution is fine.

### Commit Guidelines

It is beneficial for everyone involved in the project for commits to be reasonably small and atomic as possible, along with the messages being detailed.

The contents of a commit should be as tight in scope as possible, regardless if the overall change set is much broader. This helps with doing the review, as well as things such as aiding in `git bisect`, `git revert`, and `git rebase`.

When writing a commit message, the general recommendation is to have the first line, also known as the "short" description of the commit, be a tag which denotes what the commit touched followed by a brief summary. This is then followed by a blank line, and then followed by a more detailed description giving context and rational behind the change where appropriate.

The rules for tag at the front of the message are as follows:

* If the commit is within one of the Vermilion crates, use the name of the crate after the `vermilion-`, unless it's `vermilion` itself, in which case just use that. (e.g. `lsp`, `verilog`, `vermilion`)
* If the commit modifies anything to do with the documentation, then use `docs`.
* If the commit modifies anything to do with the CI or workflows, then use `ci`.
* If it's something not covered here, make a best-effort guess based on the rules above.
* Suffix the tag with `:` before the description.

Some examples of good commit messages are as follows:

```text
docs: Fixed a pile of typos in the diagnostics list
```

```text
verilog: tokenizer: fixed attribute tokenization

There was a silly mistake in the tokenization branch of `(` for tokenizing
attributes, this was fixed and now `(* *)` attributes are tokenized
properly.
```

Finally, it is ***highly*** recommended for you to sign all your commits, if you need help with setting this up, see the GitHub documentation on [signing commits] and [setting up your signing key] with git.

We do suggestion you run `git config --global commit.gpgsign true` to ensure git will always sign your commits when you make them.

### Branch Naming

In general, when naming a branch they should follow the `$type/[$function]/$name` pattern.

First the `$type` section of the branch name is what this branch is trying to do, for example, if it's to add a feature then `$type` should be `feature`, for a bug fix, then `fix` should be used, `docs` if it's for Documentation, etc.

Next, the `$function` section can be thought of something like a filesystem path segment that describes where this branch is focused, for instance, `lang/verilog` if it's related to Verilog. If the changes are more wide-reaching than a specific sub-section, then this part can be omitted.

Finally, the `$name` section should be a short, but descriptive name for what this branch is doing, it doesn't need to be super detailed, that's what commit messages are for, but something that gives a general idea, like `formal-support`.

So, to put it all together, the following are good examples of how your branch should be named:

* `fix/lsp/semantic-tokens`
* `feature/vhdl/constraint-mapping`
* `docs/grammar-fixes`

### Submitting A Pull Request

> [!IMPORTANT]
> Vermilion uses a [git rebase] based workflow to maintain a linear history and avoid merge commits.

Once your changes are done, tested, and you're happy with them, then now to open a pull request against the Torii repository.

The first step is to ensure your changes are based on-top of the latest changes in the `main` branch. This can be done two ways, the first is to pull the changes to your local clone and then rebase on top of that, or directly rebase against `upstream/main`. The former is recommended as that way you can keep your local repo in-sync with upstream.

For example:

1. Switch from your working branch to `main`: `git switch main`
2. Update the remotes: `git fetch --all --prune`
3. Pull the changes into your local `main`: `git pull upstream`
4. Push the changes back up to your fork so it is updated as well: `git push`
5. Switch back to your working branch: `git switch <branch-name>`
6. Rebase the changes from `main` onto your feature branch: `git rebase -i main`
7. Go through the steps of running the rebase and resolving any conflicts that arise.
8. Once rebased and you've re-tested and are happy with the changes again, push them up to your fork: `git push -f`

After this, run `cargo clippy` in the root of the repository, this will run the code linter and alert you if anything is amiss. This is also run in the CI workflow, and will prevent your pull request from getting merged if it's not happy.

Once your changes are all up to date, make sure that all the tests pass (see the [Testing] section for more info), and then you can then open a [new pull request] against the Torii repository.

Once you've filled out the details about the PR, it will then be reviewed to ensure that it's in line with the various project standards, sometimes you'll nail it the first try, other times there will be feedback and requested changes.

If changes are requested, then go back and address them, where possible use `git commit --fixup=<sha>` to attach the updates back to the commit which they were associated with (this is why having smaller atomic commits is important), once all the feedback items have been addressed, then run a `git rebase -i main --autosquash` to apply the fixups to the commits in question and resolve any merge issues.

Once that is done, then re-push to your fork with a force push and the pull request will automatically re-trigger CI, you can then re-request review on the pull request.

This cycle will continue until all items have been addressed and we're ready to merge it into `main` if it has been approved.

As noted above, we use a `git rebase` based workflow, as such we require that your change branch is always based on the tip of `main`, so we might ask you to rebase it onto `main` prior to merging. This might result in things breaking that were not broken before, and they must be fixed as appropriate.

And with that, you're done! You've contributed to Vermilion!

## Documentation

The Vermilion documentation is split into two halves, the first is the Vermilion Book, which is all the user-facing documentation, getting started guide, installation instructions, etc. The second is the crate API documentation, while most of the Vermilion crates are internal and project specific, we would still like them documented as best as possible so it makes using and navigating the codebase easier, and would also allow us to possibly publish the crates for others to use as well with minimal extra effort.

### The Vermilion Book

The Vermilion book uses [mdBook] like most other Rust projects, so in order to build and preview the documentation you'll need that installed.

Otherwise there is nothing special about the configuration, simply running `mdbook build docs` from the root of the Vermilion repository will build the documentation for you.

### Crate API Documentation

The crate documentation should be as detailed as needed, but not overwhelming, and should generally follow the guidelines from the [rustdoc book].

## AI Usage Policy

Vermilion explicitly does not allow any contributions that were in either whole or in part made using a Generative AI system or agent, such as large language models (LLMs), chatbots, or image generation systems. This ban includes, but is not limited to Copilot, BARD/Gemini, ChatGPT, Claude, llama, DeepSeek, Stable Diffusion, DALL-E, Midjourney, or Devin AI.

This policy covers ***ALL*** parts of this project, including but not limited to code, tests, documentation, metadata, artworks, issues, comments, suggestions, feature requests, discussions, pull requests, and any other possible contributions that can be made to Vermilion.

If you are found to be using such tools when contributing to Vermilion, all of you contributions will be removed if viable, outstanding contributions closed without recourse or review, all future collaboration efforts denied, and you will be banned from contributing to Vermilion and related projects.

No exceptions to this policy will be made.

> [!NOTE]
> It is also recommended to avoid any and all AI tools when asking questions about Vermilion,
> prefer the documentation when available, as well as things such as the discussion forum, or IRC channel.
> These tools often fabricate plausible sounding information that is entirely incorrect, or often subtly
> incorrect and pass it off with confidence, and are thus misleading.

### TL;DR

The use of Generative AI when contributing or interacting with this project is ***EXPLICITLY FORBIDDEN*** with ***NO EXCEPTIONS*** and if you are found to be using it, you will be banned from the project and your contributions removed.

[BSD-3-Clause]: https://spdx.org/licenses/BSD-3-Clause.html
[`LICENSE`]: https://github.com/shrine-maiden-heavy-industries/Vermilion/blob/main/LICENSE
[CC-BY-SA 4.0]: https://creativecommons.org/licenses/by-sa/4.0/
[`LICENSE.docs`]: https://github.com/shrine-maiden-heavy-industries/Vermilion/blob/main/LICENSE.docs
[git repository]: https://github.com/shrine-maiden-heavy-industries/Vermilion
[Code Of Conduct]: https://github.com/shrine-maiden-heavy-industries/Vermilion/blob/main/CODE_OF_CONDUCT.md
[Reporting Issues]: #reporting-issues
[Suggesting Features]: #suggesting-features
[Development And Testing]: #development-and-testing
[Setting Up An Environment]: #setting-up-an-environment
[Code Formatting]: #code-formatting
[Testing]: #testing
[Commit Guidelines]: #commit-guidelines
[Branch Naming]: #branch-naming
[Submitting A Pull Request]: #submitting-a-pull-request
[Documentation]: #documentation
[The Vermilion Book]: #the-vermilion-book
[Crate API Documentation]: #crate-api-documentation
[AI Usage Policy]: #ai-usage-policy
[TL;DR]: #tldr
[issue tracker]: https://github.com/shrine-maiden-heavy-industries/Vermilion/issues
[report an issue]: https://github.com/shrine-maiden-heavy-industries/Vermilion/issues
[discussion forum]: https://github.com/shrine-maiden-heavy-industries/Vermilion/discussions
[IRC channel]: https://web.libera.chat/#torii
[rust-analyzer]: https://rust-analyzer.github.io/
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`cargo-nextest`]: https://nexte.st/
[fork]: https://github.com/shrine-maiden-heavy-industries/Vermilion/fork
[signing commits]: https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits
[setting up your signing key]: https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key
[git rebase]: https://git-scm.com/book/en/v2/Git-Branching-Rebasing
[new pull request]: https://github.com/shrine-maiden-heavy-industries/Vermilion/compare
[mdBook]: https://rust-lang.github.io/mdBook/
[rustdoc book]: https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
