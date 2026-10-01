# Fire Emblem Engage — 双人用餐对话回退修复

[English](README.en.md)

首发版本：**[v0.1](https://github.com/haitei155/engage-dining-pair-fix/releases/tag/v0.1)**。

**必须先安装 [Cobalt](https://github.com/Raytwo/Cobalt)。** 感谢 **Raytwo** 开发并维护 Cobalt，为本项目提供 MOD 加载与扩展基础。

当前编译版面向《火焰纹章 Engage》2.0.0，使用 Cobalt 1.31.0 作为兼容基线。

## 功能

- 为部分缺少专用对话标签的双人用餐组合提供回退。
- 只在原标签为空时调用游戏原生料理评价台词选择，根据人物、料理品质和喜好选择好吃／一般／不喜欢的台词。
- 已有正常双人对话保持原标签。
- 当前发行仅安装一个标签钩子；诊断功能默认关闭。

## 修复场景与正常流程截图

下图为自定义新增角色参与双人用餐的开场界面。根据作者实测，没有本补丁时，受影响组合大概率在此界面结束、准备进入后续对话时卡死。部分组合缺少专用双人对话标签，原流程取到空标签后无法正常继续；本补丁在此处改用原生料理评价台词回退。

![双人用餐开场：受影响组合可能在这一界面结束后卡住](images/paired-dining-opening-before-dialogue.png)

启用补丁后，下面两张截图展示了这组角色继续进入正常用餐评价对话的过程。截图中的支援提示是正常游戏流程的显示，本补丁的作用是避免缺失对话标签阻断用餐，不额外创建支援事件。

![修复后继续进入琉尔的用餐评价对话](images/paired-dining-dialogue-alear.png)

![修复后继续进入艾莉可的用餐评价对话](images/paired-dining-dialogue-eirika.png)

## 下载与安装

当前编译版：[`engage_dining_pair_fix.nro`](Releases/engage_dining_pair_fix.nro)；校验值见 [SHA256SUMS](Releases/SHA256SUMS)。

1. 先按 Cobalt 的说明安装基础运行环境。
2. 将 NRO 放在 `engage/mods/engage-dining-pair-fix/` 下，完整重启游戏。
3. 如果 12 人整合包已包含本插件，就使用包内版本，避免再装一个独立副本。

## 源码与构建

见 [构建说明](docs/BUILD.md)。源码包括当前功能实现和实际使用的本地依赖快照；构建产物输出到 `dist/`，不会覆盖 `Releases/` 中的发行编译版。

## 许可与致谢

本项目自行新增的代码与文档采用 [Apache License 2.0](LICENSE)。第三方依赖、上游派生代码及资源保留各自许可与权利声明；详细来源与范围见 [THIRD_PARTY.md](THIRD_PARTY.md) 和 [NOTICE](NOTICE)。

感谢 [Raytwo](https://github.com/Raytwo/Cobalt)、[skyline-rs](https://github.com/skyline-rs) 以及相关依赖贡献者。
