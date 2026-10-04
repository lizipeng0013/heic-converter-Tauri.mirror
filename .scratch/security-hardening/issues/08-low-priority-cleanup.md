# 08: 低危收尾

**What to build:** 作为维护者，一批低危项一次性清掉：生产代码无残留调试输出；关键异步调用不再产生未捕获 rejection（关闭/退出/挂载路径失败有反馈）；store 的活动标签页类型与实际值域一致；生产 CSP 补上 `object-src 'none'; base-uri 'self'`；内存池不再留有"复用即越界"的潜伏 panic（鉴于当前无调用方，倾向直接删除，实施时按最小改动取舍）。

**Blocked by:** None (can start immediately)

**Status:** done

- [x] 标题栏调试输出全部移除或改走项目统一日志
- [x] 关闭/托盘退出/面板挂载/事件监听等异步路径补 rejection 兜底，失败有日志或反馈
- [x] store 活动标签页参数类型覆盖全部实际值域
- [x] 生产 CSP 含 `object-src 'none'; base-uri 'self'`
- [x] 内存池：删除，或复用分支恢复缓冲区长度且有测试覆盖
- [x] lint、类型检查、既有测试全部通过
