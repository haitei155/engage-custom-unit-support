# v0.2 — Engage Custom Unit Support

- 项目由 engage-dining-pair-fix 更名为 engage-custom-unit-support；NRO 更名为 engage_custom_unit_support.nro。
- 延续 v0.1 的双人用餐对话回退。
- 原生 Loading 缺图时优先读取透明 288×48 六帧 PNG，保留对应 Bundle 后备。
- 修正重复 Loading 的材质清理／重建与贴图卸载保护，失效缓存重新读取。
- 中英 README 提供作者本人制作的琉弥艾尔六帧 PNG 样例及接入说明；其他人物的六帧素材需由 MOD 制作者自行制作。
- 正式构建不输出插件诊断日志。

升级时移除旧 engage_dining_pair_fix.nro，使用新 NRO 后完整重启游戏。如果所使用的整合包已包含本插件，无需另装独立副本。

Renames the project and NRO, retains the paired-dining fallback introduced in v0.1, and adds identity-based six-frame PNG/Bundle support for missing native Loading dots. Corrects material recreation and texture lifetime across repeated Loading screens. An author-made Lumera sheet is provided as a downloadable README example with setup instructions. MOD creators must make their own six-frame sheets for other characters. Standard builds emit no plugin diagnostic logs.

Remove engage_dining_pair_fix.nro when upgrading and fully restart. If your mod collection already includes this plugin, use that copy without a separate installation.
