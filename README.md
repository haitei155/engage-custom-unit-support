# Fire Emblem Engage — 自定义人物支持 / Custom Unit Support

[English](README.en.md)

**v0.2** · 双人用餐对话回退与六帧 PNG Loading 小人支持。

**必须先安装 [Cobalt](https://github.com/Raytwo/Cobalt)。** 感谢 **Raytwo** 开发并维护 Cobalt，为本项目提供 MOD 加载与扩展基础。

当前编译版面向《火焰纹章 Engage》2.0.0，使用 Cobalt 1.31.0 作为兼容基线。

## v0.2 新增内容

### 六帧 PNG Loading 小人

原生 Loading 找不到人物贴图时，本插件按人物转换后的实际 UnitIconID 从已启用的 Cobalt 包读取资源。优先路径：`patches/icon/loading/Dot_Run_<UnitIconID>.png`，格式为透明 **288×48 RGBA PNG，横排六个 48×48 帧**，身份键的拼写与大小写须匹配。这是一张六帧横排图，运行时不读取 GIF 或六张独立图片；没有可用 PNG 时仍可回退到对应 Loading Bundle。资源不存在或无效时跳过。

**六帧 PNG Loading 小人需要 MOD 制作者自行制作。** 本插件提供素材读取与原生动画支持，不负责生成图像。目前提供的琉弥艾尔样例为作者本人自行制作的素材；其他人物仍需由其 MOD 制作者自行制作对应的六帧图。

#### 琉弥艾尔 PNG 样例

![琉弥艾尔（Lumera）的六帧横排 Loading 跑步图](images/lumera-loading-run-six-frames.png)

样例文件：[`lumera-loading-run-six-frames.png`](images/lumera-loading-run-six-frames.png)。这是作者本人制作的琉弥艾尔 Loading 动画素材，采用透明 **288×48 RGBA PNG**，横排六个 **48×48** 帧，可供 MOD 制作者参考素材格式与接入方法。

用于琉弥艾尔人物包时，将下载的样例重命名为 `Dot_Run_555Lumiere.png`，放入 `patches/icon/loading/Dot_Run_555Lumiere.png`。样例的英文文件名用于文档展示，游戏读取名称必须匹配琉弥艾尔实际 UnitIconID：`555Lumiere`。其他人物应提供本人六帧图并使用其对应身份键；NRO 不自动生成缺失素材。

Loading 模式、邀请参与者、出场队列和六帧动画继续使用原生规则；原本不显示队列的 Loading 不强制增加队列。v0.2 在清理自定义槽位前恢复保留的材质模板，保护缓存贴图不被 Unity 的资源清理卸载，并在缓存失效时重新读取。这些改动用于修正后续 Loading 显示白色方块的问题。

正式构建不输出本插件诊断日志。游戏期间缓存素材，修改 PNG 或 ZIP 后须完整重启游戏。

## v0.1 功能

### 双人用餐

- 为部分缺少专用对话标签的双人用餐组合提供回退。
- 只在原标签为空时调用游戏原生料理评价台词选择，根据人物、料理品质和喜好选择好吃／一般／不喜欢的台词。
- 已有正常双人对话保持原标签。

### 用餐修复场景与截图

下图为自定义新增角色参与双人用餐的开场界面。根据作者实测，没有本补丁时，受影响组合大概率在此界面结束、准备进入后续对话时卡死。部分组合缺少专用双人对话标签，原流程取到空标签后无法正常继续；本补丁在此处改用原生料理评价台词回退。

![双人用餐开场：受影响组合可能在这一界面结束后卡住](images/paired-dining-opening-before-dialogue.jpg)

启用补丁后，下面两张截图展示了这组角色继续进入正常用餐评价对话的过程。截图中的支援提示是正常游戏流程的显示，本补丁的作用是避免缺失对话标签阻断用餐，不额外创建支援事件。

![修复后继续进入琉尔的用餐评价对话](images/paired-dining-dialogue-alear.jpg)

![修复后继续进入艾莉可的用餐评价对话](images/paired-dining-dialogue-eirika.jpg)

## 下载与安装

v0.2 编译版：[`engage_custom_unit_support.nro`](Releases/engage_custom_unit_support.nro)；校验值见 [SHA256SUMS](Releases/SHA256SUMS)。

1. 先按 Cobalt 的说明安装基础运行环境。
2. 将新版 NRO 放在 `engage/mods/engage-custom-unit-support/` 下，完整重启游戏。
3. **升级时移除旧 `engage_dining_pair_fix.nro`。** 新旧名称包含同一用餐钩子，不可同时运行。如果所使用的整合包已包含本插件，使用包内版本即可，无需另装独立副本。
4. Loading PNG 留在对应人物包中；单独安装 NRO 不会增加人物素材。

## 项目改名与升级

项目由 `engage-dining-pair-fix` 更名为 `engage-custom-unit-support`，v0.2 NRO 为 `engage_custom_unit_support.nro`。新版已经包含原用餐修复，旧名与新名 NRO 同时加载会造成重复钩子；升级时用新 NRO 替换旧 NRO。

## 源码与构建

见 [构建说明](docs/BUILD.md)。源码包括两项功能实现和实际使用的本地依赖快照；构建产物输出到 `dist/`。`Releases/` 保存 v0.2 编译版与校验值。

## 许可与致谢

本项目自行新增的代码与文档采用 [Apache License 2.0](LICENSE)。第三方依赖、上游派生代码及资源保留各自许可与权利声明；详细来源与范围见 [THIRD_PARTY.md](THIRD_PARTY.md) 和 [NOTICE](NOTICE)。

感谢 [Raytwo](https://github.com/Raytwo/Cobalt)、[skyline-rs](https://github.com/skyline-rs) 以及相关依赖贡献者。
