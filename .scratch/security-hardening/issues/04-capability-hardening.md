# 04: Capability 收敛与 stat scope 修复

**What to build:** 作为用户，应用不再持有任何它用不到的权限：`$HOME/**`、`/mnt/**` 的 fs 写权限、无 scope 的打开路径权限、全量通知权限全部移除；同时文件列表里的文件大小能正常加载并显示（修复当前可能永远"加载中…"的 stat 权限问题，实施时先实机复现确认）。作为维护者，有一条回归测试断言这些宽权限不会被加回来。

**Blocked by:** None (can start immediately)

**Status:** done

- [x] 能力配置中 `fs:allow-write-file`、`opener:allow-open-path`、`notification:default` 三条已移除
- [x] `fs:allow-stat` 带精确 scope（限定图片/下载等用户目录，不使用 `$HOME/**`）
- [x] 实机验证：文件列表中文件大小正常显示，不再停在"加载中…"
- [x] 回归测试：解析能力配置，断言被移除的权限不再出现、stat 带 scope，纳入 CI
- [x] 应用启动无权限相关报错，既有功能（选择文件、转换、在目录中显示）正常
