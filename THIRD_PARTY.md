# 第三方来源与许可 / Third-party Sources and Licensing

根目录 Apache-2.0 适用于本项目有权许可的新增代码和文档，不覆盖第三方许可，也不代表已获得缺失上游许可的再许可授权。源码中已有声明保持原样。

The root Apache-2.0 license applies to project-owned additions and documentation; it does not replace third-party terms or grant rights absent from upstream licensing. Existing notices are preserved.

| 组件 / Component | 来源 / Origin | 许可范围 / License scope |
|---|---|---|
| `dependencies/unity-nx-0.1.0-local` | [DivineDragonFanClub/unity-nx](https://github.com/DivineDragonFanClub/unity-nx), reused local snapshot | MPL-2.0; original LICENSE-MPL and NOTICE retained. Used by Loading-dot support. |
| `dependencies/cobalt-mods` | [Raytwo/Cobalt](https://github.com/Raytwo/Cobalt), locally used mods crate | Upstream license not specified in this crate snapshot; implementation and authorship retained, not relicensed as Apache-2.0. Used by Loading-dot support. |
| `dependencies/horizon-svc` | [skyline-rs/horizon-svc](https://github.com/skyline-rs/horizon-svc), `ced970c` | No explicit standalone LICENSE in the copied snapshot; retain authorship and review permission before distribution. |
| `tools/dependencies/linkle-0.2.11` | [MegatonHammer/linkle](https://github.com/MegatonHammer/linkle) | MIT / Apache-2.0 per Cargo manifest; bundled license files retained where supplied. |

`tools/link.T` 来自本机 Skyline 链接脚本快照，保留用于 Switch 构建；其来源属于 skyline-rs 工具链，不作为本项目原创代码重新许可。

`tools/link.T` is the Skyline linker script snapshot used for Switch builds; it belongs to the skyline-rs toolchain and is not relicensed as original project code.

Cargo.lock 记录其余 Cargo 依赖及精确版本；这些依赖保持其上游许可。公开发布前需审阅整合发行的许可范围，尤其是 GPL 依赖和没有明确许可的上游部分。

Cargo.lock records other Cargo dependencies and their exact versions; their original terms continue to apply. Review the combined distribution terms before publication, especially GPL dependencies and upstream portions without explicit licensing.
