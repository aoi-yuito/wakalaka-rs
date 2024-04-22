<!--
 Copyright (c) 2024 Kawaxte

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

<div align="center">
    <img src="resources/waka_lichtstern.png" alt="Waka Lichtstern, drawn by kedoi#4581" width="400" height="400">
    <br>
    <p><small>Waka Lichtstern, drawn by <code>kedoi#4581</code></small></p>
</div>

# Wakalaka

[![gh_release](https://img.shields.io/github/v/release/Kawaxte/wakalaka-rs?sort=date&logo=github&label=latest&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/releases/latest)
[![gh_release_pre](https://img.shields.io/github/v/release/Kawaxte/wakalaka-rs?include_prereleases&sort=date&logo=github&label=pre-release&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/releases)

A work-in-progress, self-hostable Discord bot written in Rust for the general public. She aims to contain enough features to be useful to a small range of users.

In case you haven't tried her out and want to, [click here](https://discord.com/api/oauth2/authorize?client_id=1190718691055251548&permissions=9925535296631&scope=bot) to invite her to your server.

---

## Community

[![gh_discussions](https://img.shields.io/github/discussions/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/discussions)
[![discord](https://img.shields.io/discord/1186451961848008866?logo=discord&style=for-the-badge)](https://discord.gg/jUZVWk7q2q)

If you have any questions or ideas, or if you want to share your thoughts about the project, you're welcome to join our [Discord server](https://discord.gg/jUZVWk7q2q) or start a chat on [GitHub Discussions](https://github.com/Kawaxte/wakalaka-rs/discussions).

I'm always open to feedback and interested in hearing what you think about my projects. Your input can help make Wakalaka even better!

---

## Contributing

[![gh_stars](https://img.shields.io/github/stars/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/stargazers)
[![gh_contribs](https://img.shields.io/github/contributors/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/graphs/contributors)

You can suggest improvements, report issues, or submit changes directly.

If you find this project useful, or meaningful, feel free to give it a ⭐. This helps me understand how popular the project is and encourages me to continue improving it.

While there's no set way to contribute to this project, please do follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). As I'm more familiar with Java, my Rust code may not be perfect. I welcome any corrections or suggestions.

---

## Reporting Issues or Suggesting Features

[![gh_issues_a](https://img.shields.io/github/issues/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/issues)
[![gh_issues_b](https://img.shields.io/github/issues-closed/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/issues?q=is%3Aissue+is%3Aclosed)

If you spot any issues or have any ideas, feel free to open an issue. I'll aim to sort out the problem or add the feature as quickly as I can.

To open an issue, follow these steps:

1. Click the `Issues` tab at the top of the project page.
2. Click the `New issue` button at the top right of the project page.
3. Choose the type of issue you want to open.
4. Fill in the required details and click the `Submit new issue` button.

---

## Forking and Making Pull Requests

[![gh_pr_a](https://img.shields.io/github/issues-pr/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/pulls)
[![gh_pr_b](https://img.shields.io/github/issues-pr-closed/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/pulls?q=is%3Apr+is%3Aclosed)

As a fellow Rust user, I'm always open to new ideas and improvements. If you have any, feel free to make a copy of the project and submit a pull request. I'll look at it as soon as I can.

To make a copy of the project and submit a pull request, follow these steps:

1. Make a copy of the project by clicking the `Fork` button at the top right of the project page.
2. Download the copied project to your computer using [Git](https://git-scm.com/) or [GitHub Desktop](https://desktop.github.com/).
    - _Or, you can click the `Download ZIP` button after clicking the `Code` button at the top right of the project page to download the project as a zip file._
3. Make changes to the project.
4. Commit the changes to your copied project.
5. Submit a pull request by clicking the `Pull request` button at the top right of the project page.

---

## Building from Source

[![gh_build](https://img.shields.io/github/actions/workflow/status/Kawaxte/wakalaka-rs/rust.yml?logo=github&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/actions/workflows/rust.yml)

[![rust](https://img.shields.io/badge/dynamic/json?logo=rust&label=Rust&color=A72145&style=for-the-badge&query=%24.tag_name&url=https%3A%2F%2Fapi.github.com%2Frepos%2Frust-lang%2Frust%2Freleases%2Flatest)](https://www.rust-lang.org/)
[![cargo](https://img.shields.io/badge/cargo-555555?logo=rust&style=for-the-badge)](https://doc.rust-lang.org/cargo/)

### Prerequisites

Before you start, make sure you've got [Rust](https://www.rust-lang.org/) on your computer. You can check if you've got Rust by typing `rustc --version` in your terminal.

If you haven't got Rust, you can get it by following the steps on the [Rust website](https://www.rust-lang.org/learn/get-started).

### Compilation

To compile into an executable, follow these steps:

1. Either clone or manually download the project.
2. Navigate to the workspace of the project.
3. Type `cargo build --release` to build with optimisations or `cargo build` to build with debugging information.

### Execution

To use yours truly, follow these steps:

1. `cd` into `target/releases` or `target/debug` directory, depending on whether you compiled a `release` or `debug` build.
   1. _Linux users have to grant executability by typing_ `chmod +x ./wakalaka`.
2. Run by typing `./wakalaka` on Linux,`wakalaka.exe` on Windows via Command Prompt or `./wakalaka.exe` via PowerShell.

---

## Licence

[![gh_licence](https://img.shields.io/github/license/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](LICENSE)

This project is licenced under the [MIT License](LICENSE). You can use the project for any purpose, but you must include the original copyright and licence.
