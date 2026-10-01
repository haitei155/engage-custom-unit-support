# v0.1 — 双人用餐对话回退修复 / Paired Dining Dialogue Fallback Fix

缺少双人用餐专用对话标签时，回退到游戏原生料理评价台词选择；已有有效标签保持原流程。

Falls back to native meal-evaluation dialogue when a paired-dining label is missing, preserving valid paired dialogue.

## 安装 / Installation

先安装 [Cobalt](https://github.com/Raytwo/Cobalt)。下载 `engage_dining_pair_fix.nro`，放入 `engage/mods/engage-dining-pair-fix/`，完整重启游戏。如果整合包已包含本插件，使用包内版本，避免重复加载。

Install Cobalt first. Place `engage_dining_pair_fix.nro` in `engage/mods/engage-dining-pair-fix/` and fully restart the game. Use the included copy when an integration package already provides the plugin.

基线 / Baseline: Fire Emblem Engage 2.0.0 / Cobalt 1.31.0.

## 验证 / Validation

当前 NRO 来自现用12人＋龙妈整合包，二进制内容保持。 / Existing binary from the current 12 Emblems + Lumera integration package, preserved unchanged.

SHA-256: `bf386bc627154e580083f860ed3f3253093e0203d05fac88bd56a32d0cfa7ed3`；下载资产含 `SHA256SUMS`。

许可、来源与致谢见仓库 LICENSE、NOTICE、THIRD_PARTY.md 和 README。
See LICENSE, NOTICE, THIRD_PARTY.md and README for licensing, origins and credits.
