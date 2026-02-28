# Matrix-Zulip Bridge Development Tasks

基于参考项目 MatrixZulipBridge (Python) 和 matrix-bridge-discord (Rust) 的功能对比分析。

## Phase 1: 项目基础架构 ✅

- [x] 创建 Cargo.toml 项目配置
- [x] 创建 src/main.rs 入口文件
- [x] 实现 CLI 参数解析 (clap)
- [x] 实现日志初始化 (tracing)
- [x] 实现配置加载系统

## Phase 2: 配置模块 (src/config/) ✅

- [x] 创建 config/parser.rs - 配置解析器
- [x] 创建 config/validator.rs - 配置验证
- [x] 支持数据库配置 (PostgreSQL/SQLite/MySQL)
- [x] 支持 Matrix Appservice 配置
- [x] 支持 Zulip API 配置
- [x] 支持 Bridge 行为配置

## Phase 3: 数据库模块 (src/db/)

- [ ] 创建 db/manager.rs - 数据库管理器
- [ ] 创建 db/models.rs - 数据模型定义
  - [ ] RoomMapping - 房间映射
  - [ ] MessageMapping - 消息映射
  - [ ] UserMapping - 用户映射
  - [ ] OrganizationConfig - 组织配置
- [ ] 创建 db/error.rs - 数据库错误类型
- [ ] 创建 db/stores/mod.rs - 存储接口
- [ ] 创建 db/stores/room_store.rs - 房间存储
- [ ] 创建 db/stores/message_store.rs - 消息存储
- [ ] 创建 db/stores/user_store.rs - 用户存储
- [ ] 实现 PostgreSQL 支持
- [ ] 实现 SQLite 支持
- [ ] 实现数据库迁移

## Phase 4: Matrix 模块 (src/matrix/)

- [ ] 创建 matrix.rs - Matrix Appservice 封装
- [ ] 创建 matrix/event_handler.rs - Matrix 事件处理器
- [ ] 创建 matrix/command_handler.rs - Matrix 命令处理
- [ ] 实现幽灵用户 (Ghost User) 管理
- [ ] 实现消息发送 (文本/媒体/回复/编辑)
- [ ] 实现房间管理 (创建/加入/离开)
- [ ] 实现用户资料同步 (显示名/头像)
- [ ] 实现 Presence 状态同步
- [ ] 实现 Typing 指示器

## Phase 5: Zulip 模块 (src/zulip/)

- [ ] 创建 zulip.rs - Zulip API 客户端
- [ ] 创建 zulip/types.rs - Zulip 类型定义
- [ ] 创建 zulip/event_handler.rs - Zulip 事件处理
- [ ] 创建 zulip/http_client.rs - HTTP API 客户端
- [ ] 创建 zulip/websocket.rs - WebSocket 事件流
- [ ] 实现 Zulip 认证 (email + API key)
- [ ] 实现 Stream 消息发送/接收
- [ ] 实现 Direct Message 发送/接收
- [ ] 实现 Topic (线程) 支持
- [ ] 实现消息反应 (Reactions)
- [ ] 实现消息删除/编辑
- [ ] 实现用户管理 API
- [ ] 实现 Stream 订阅管理

## Phase 6: Bridge 核心逻辑 (src/bridge/)

- [ ] 创建 bridge.rs - 桥接核心
- [ ] 创建 bridge/message_flow.rs - 消息流转
- [ ] 创建 bridge/user_sync.rs - 用户同步
- [ ] 创建 bridge/presence_handler.rs - Presence 处理
- [ ] 创建 bridge/provisioning.rs - 桥接配置
- [ ] 创建 bridge/queue.rs - 事件队列

## Phase 7: 房间管理 (src/rooms/)

- [ ] 创建 rooms/mod.rs - 房间模块入口
- [ ] 创建 rooms/room.rs - 房间基类
- [ ] 创建 rooms/control_room.rs - 控制房间
- [ ] 创建 rooms/organization_room.rs - 组织房间
- [ ] 创建 rooms/stream_room.rs - Stream 房间 (频道)
- [ ] 创建 rooms/direct_room.rs - 私信房间
- [ ] 创建 rooms/space_room.rs - Matrix Space 房间
- [ ] 创建 rooms/personal_room.rs - 个人房间

## Phase 8: 消息解析器 (src/parsers/)

- [ ] 创建 parsers/mod.rs - 解析器模块
- [ ] 创建 parsers/matrix_parser.rs - Matrix 消息解析
- [ ] 创建 parsers/zulip_parser.rs - Zulip 消息解析
- [ ] 创建 parsers/common.rs - 通用解析工具
- [ ] 实现 Markdown/HTML 格式转换
- [ ] 实现 Mention 用户转换
- [ ] 实现媒体链接转换

## Phase 9: 工具模块 (src/utils/)

- [ ] 创建 utils/mod.rs - 工具模块入口
- [ ] 创建 utils/error.rs - 错误类型定义
- [ ] 创建 utils/formatting.rs - 格式化工具
- [ ] 创建 utils/logging.rs - 日志配置

## Phase 10: 媒体处理 (src/media.rs)

- [ ] 创建 media.rs - 媒体处理模块
- [ ] 实现 Matrix 媒体下载
- [ ] 实现 Matrix 媒体上传
- [ ] 实现 Zulip 附件上传
- [ ] 实现 URL 转换

## Phase 11: Web 服务 (src/web/)

- [ ] 创建 web.rs - Web 服务入口
- [ ] 创建 web/health.rs - 健康检查端点
- [ ] 创建 web/metrics.rs - Prometheus 指标
- [ ] 实现 Appservice HTTP 监听

## Phase 12: 命令系统

- [ ] 实现控制房间命令
  - [ ] help - 帮助命令
  - [ ] addorganization - 添加组织
  - [ ] open - 打开组织房间
- [ ] 实现组织房间命令
  - [ ] site - 设置 Zulip 站点
  - [ ] email - 设置 Bot 邮箱
  - [ ] apikey - 设置 API Key
  - [ ] connect - 连接组织
  - [ ] disconnect - 断开连接
  - [ ] subscribe - 订阅 Stream
  - [ ] unsubscribe - 取消订阅
  - [ ] space - 创建 Space
  - [ ] status - 显示状态
  - [ ] backfill - 消息回填
  - [ ] personalroom - 创建个人房间

## Phase 13: 功能完善

- [ ] 实现 Matrix Puppet (Zulip 用户在 Matrix 的虚拟用户)
- [ ] 实现 Zulip Puppet (Matrix 用户在 Zulip 的虚拟用户)
- [ ] 实现消息回填 (Backfill)
- [ ] 实现 Topic <-> Thread 映射
- [ ] 实现反应 (Reactions) 双向同步
- [ ] 实现消息编辑同步
- [ ] 实现消息删除同步
- [ ] 实现回复 (Reply) 同步
- [ ] 实现成员同步 (lazy/half/full 模式)
- [ ] 实现权限同步
- [ ] 实现封禁同步 (relay_moderation)

## Phase 14: Docker 和部署

- [ ] 创建 Dockerfile
- [ ] 创建 docker-compose.yml
- [ ] 创建示例配置文件
- [ ] 创建 README.md

## Phase 15: 测试

- [ ] 单元测试
- [ ] 集成测试
- [ ] 配置验证测试

---

## 优先级说明

1. **Phase 1-3**: 必须首先完成，建立项目基础
2. **Phase 4-5**: 核心功能，Matrix 和 Zulip 客户端
3. **Phase 6-7**: 桥接逻辑和房间管理
4. **Phase 8-11**: 辅助功能模块
5. **Phase 12-13**: 高级功能
6. **Phase 14-15**: 部署和测试
