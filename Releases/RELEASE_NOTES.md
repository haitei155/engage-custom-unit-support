# v0.3 — Engage Custom Unit Support

- 特技继承选人页右侧的牵绊等级面板向下延伸至屏幕底部附近，同步增加内容及背景的行高，缓解文字拥挤。
- 修正新增纹章士后，特技继承界面条目显示不全及翻页卡住的问题。
- 面板最多显示 22 项，采用两列、11 行布局；继承详情预留 30 个导航槽，仅显示实际持有的纹章士。
- 包含特技继承翻页越界保护、双人用餐对话回退和六帧 PNG Loading 小人支持。
- 更新时替换完整 ZIP，并完整重启游戏。使用本插件的整合包无需再安装独立副本。

Expands the skill-inheritance lists to address missing entries and freezes when navigating added emblems. The bond-level panel on the inheritance unit-selection screen now extends near the bottom of the screen, with taller content and background rows. It supports 22 entries in two columns and 11 rows. Inheritance details retain 30 navigation slots and display only owned emblems. Navigation bounds protection, paired-dining fallback and six-frame PNG Loading support are included. Replace the complete ZIP and fully restart the game. If your mod collection includes this plugin, no separate copy is needed.

---

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
