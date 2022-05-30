# Borderlands 3 Save Editor

[源仓库](https://github.com/ZakisM/bl3_save_edit)
[English](../README.md)

一个帮助你修改《无助之地 3》的存档和配置的工具。

> 本仓库为 fork 仓库

目前支持在 Windows，Mac OS 和 Linux 平台运行。他支持修改 PC 端存档以及已经解密的 PS4 存档（并且在两个平台相互转换）。

# 截图

<img width="1762" alt="Screenshot 2021-10-13 at 16 03 15" src="https://user-images.githubusercontent.com/8143258/137160314-81ff5ba1-b89c-4c9c-a7e8-ae905a101fe9.png">

# 使用？

Visit [Releases](https://github.com/ZakisM/bl3_save_edit/releases) and download the corresponding version for your
platform.

Unzip and open the editor, then double click to run it. On the first start it will ask you to point it to the
folder/directory where your saves/profiles are stored. Once you have pointed it to a valid folder it will remember this
folder the next time you open the program.

# Notices

编辑器会在保存文件之前为您备份，但我建议您自己备份以防万一出现什么奇怪的问题。

# 从稳定分支构建

首先你需要安装 [Rust](https://www.rust-lang.org/).

然后克隆仓库并运行指令：

`cargo build`

发布一个稳定版本：

`cargo build --release`

# Credits

Huge credits to apocalyptech for their editor at https://github.com/apocalyptech/bl3-cli-saveedit. The majority of this
code was based off of their work.

Thanks to HackerSmacker for their PS4
bitmasks: https://github.com/HackerSmacker/CSave/blob/022c4e78ac7fa68e2338804bc0148ac9be3296f7/FileTranslator.c.

Huge thanks to Levin from [Lootlemon](https://www.lootlemon.com/) for providing items that are available to import
inside the editor!

Thanks to those who created these docs:

- https://docs.google.com/spreadsheets/d/1XYG30B6CulmcmmVDuq-PkLEJVtjAFacx7cuSkqbv5N4
- https://docs.google.com/spreadsheets/d/16b7bGPFKIrNg_cJm_WCMO6cKahexBs7BiJ6ja0RlD04
- https://docs.google.com/spreadsheets/d/1v-F_3C2ceaFKJae1b6wmbelw_jLjmPPriBLzGTZMqRc

Tool to download data from these docs can be found [here](https://github.com/ZakisM/bl3_save_edit_resource_downloader).

Thanks to the [iced](https://github.com/iced-rs/iced) project for allowing me to create a user interface like this!

Thanks to the [Ajour](https://github.com/ajour/ajour) project for their methods of bundling Rust Apps as well as
updating them.
