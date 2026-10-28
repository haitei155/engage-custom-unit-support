# 更新记录 / Changelog

## v0.3 — 2026-10-29

- 选人页右侧牵绊面板由 19 项扩为 22 项，使用两列、11 行布局，并补足背景行；面板向下延伸至屏幕底部附近，增加内容与背景行高，缓解文字拥挤。
- 通过配套界面资源将继承详情页扩为 30 个导航槽，仅显示实际持有数量；缩小圆点行，并为导航索引增加越界保护。
- 沿用原生纹章士排序、左右切换与首尾循环，不改技能、SP 消耗或牵绊等级。

Expands the bond-level panel from 19 to 22 entries in two columns and 11 rows. The panel extends near the bottom of the screen, with taller content and background rows. Companion UI resources provide 30 inheritance navigation slots, showing only the owned emblems. Scales the indicator row to fit and guards navigation indices, preserving native sorting, left/right navigation, wraparound, skills, SP costs and bond levels.

## v0.2 — 2026-10-03

- 项目／NRO 改名为 engage-custom-unit-support／engage_custom_unit_support.nro；保留 v0.1 的双人用餐回退。
- 通用 Loading 缺图回退优先读取 288×48 六帧 PNG，保留 Bundle 后备及原生参与者／出场队列／动画。
- 修正重复加载材质生命周期；缓存贴图增加 Unity 卸载保护与失效重读。
- 提供作者本人制作的琉弥艾尔六帧 PNG 样例及接入说明；其他人物的六帧素材需由 MOD 制作者自行制作。
- 正式构建不输出插件诊断日志。

Renames the project and NRO, retains the paired-dining fallback introduced in v0.1, adds native-mode six-frame PNG/Bundle Loading support, fixes custom material reuse and protects cached textures. Both READMEs provide an author-made Lumera PNG example and setup instructions. MOD creators must make their own six-frame sheets for other characters.

## v0.1 — 2026-10-02

缺少双人用餐专用对话标签时，回退到游戏原生料理评价台词选择；已有有效标签保持原流程。

Falls back to native meal-evaluation dialogue when a paired-dining label is missing, preserving valid paired dialogue.


