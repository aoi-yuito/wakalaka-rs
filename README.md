<!--
 Copyright (c) 2024 Kawaxte

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

<div align="center">
    <img src="res/waka_lichtstern.png" alt="Waka Lichtstern, drawn by kedoi#4581" width="400" height="400">
    <br>
    <p><small>Waka Lichtstern, drawn by <code>kedoi#4581</code></small></p>
</div>

# Wakalaka

[![gh_release](https://img.shields.io/github/v/release/Kawaxte/wakalaka-rs?sort=date&logo=github&label=latest&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/releases/latest)
[![gh_release_pre](https://img.shields.io/github/v/release/Kawaxte/wakalaka-rs?include_prereleases&sort=date&logo=github&label=pre-release&style=for-the-badge)](https://github.com/Kawaxte/wakalaka-rs/releases)

A work-in-progress, self-hostable Discord bot written purely in Rust, inspired by [Red](https://github.com/Cog-Creators/Red-DiscordBot) and [Dyno](https://dyno.gg).

_While in Beta, Wakalaka is partially ready for general use, but not so much for full-time use. She's still, and will remain to be  available for testing and feedback. If you feel like testing her, click [here](https://discord.com/api/oauth2/authorize?client_id=1190718691055251548&permissions=9899241204854&scope=bot) to bring her to your server._

_As a matter of development, one of the big worries is the database. I don't expect her to be used in many servers (if any at all) at the moment, but if by chance she blows up, the database has to be able to handle it. Nobody should be afraid to give critism of her, and provide ways to improve her, be it as minor of a detail as a typo, or as major as a feature request. A certain video game character once said, "Anything goes, even Chinese!"._

> I'm not perfect and I'm not a great developer by any means. If something is bothering you in the code, please just open an issue or make a pull request with your changes. As long as they aren't outlandish, I'll be glad to take them with open arms.

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

All Rust users are invited to contribute to this project. You can suggest improvements, report issues, or submit changes directly.

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

To compile the project, follow these steps:

1. Download the project to your computer.
2. Go to the project directory.
3. Type `sqlx database create` to create a new database `.db` file.
   - _If you can't run `sqlx`, you can install it by typing `cargo install sqlx-cli --no-default-features --features sqlite`._
4. Run `sqlx migrate run` to create the database tables if they don't exist, which they don't if you've just created the database.
5. Type `cargo build --release` to build the project for general use. Or, type `cargo build` to build the project for use in development.

The executable will be in the `target` directory.

---

## Licence

[![gh_licence](https://img.shields.io/github/license/Kawaxte/wakalaka-rs?logo=github&style=for-the-badge)](LICENSE)

This project is licenced under the [GNU Lesser General Public Licence v3.0](https://www.gnu.org/Licences/lgpl-3.0.en.html).

The GNU Lesser General Public Licence (LGPL) is a free-software Licence published by the Free Software Foundation (FSF). The Licence allows developers and companies to use and integrate a software component released under the LGPL into their own (even proprietary) software without being required by the terms of a strong copyleft Licence to release the source code of their own components.

However, any modifications to the original software must be released under the same LGPL Licence. These modifications may be distributed under any Licence if they are not distributed with the original software. This includes, in the case of a library, if the program dynamically links to the library.

For more details, please refer to the [GNU Lesser General Public Licence v3.0](https://www.gnu.org/Licences/lgpl-3.0.en.html).
