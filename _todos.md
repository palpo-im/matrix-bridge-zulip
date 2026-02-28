# Matrix-Zulip Bridge Development Tasks

基于 MatrixZulipBridge (Python) 和 matrix-bridge-discord (Rust) 的功能对比分析。

## 优先级说明

- 🔴 **P0 - 阻塞性**: 必须首先完成，其他功能依赖于此
- 🟠 **P1 - 核心**: 核心功能，桥接基本可用
- 🟡 **P2 - 重要**: 提升用户体验的重要功能
- 🟢 **P3 - 增强**: 可选的增强功能

---

## Phase 1: 基础架构 ✅

- [x] 创建 Cargo.toml 项目配置
- [x] 创建 src/main.rs 入口文件
- [x] 实现 CLI 参数解析 (clap)
- [x] 实现日志初始化 (tracing)
- [x] 实现配置加载系统

## Phase 2: 配置模块 ✅

- [x] 创建 config/parser.rs - 配置解析器
- [x] 创建 config/validator.rs - 配置验证
- [x] 支持数据库配置 (PostgreSQL/SQLite/MySQL)
- [x] 支持 Matrix Appservice 配置
- [x] 支持 Zulip API 配置
- [x] 支持 Bridge 行为配置

---

## Phase 3: 数据库模块 ✅

### 3.1 数据库核心
- [x] 创建 db/error.rs - 数据库错误类型
- [x] 创建 db/manager.rs - 数据库管理器（连接池）
- [x] 实现 PostgreSQL 连接支持
- [ ] 实现 SQLite 连接支持
- [ ] 实现 MySQL 连接支持

### 3.2 数据模型
- [x] 创建 db/models.rs - 数据模型定义
  - [x] Organization - 组织配置
  - [x] RoomMapping - 房间映射
  - [x] MessageMapping - 消息映射
  - [x] UserMapping - 用户映射
  - [x] ProcessedEvent - 已处理事件
  - [x] ReactionMapping - 反应映射

### 3.3 数据库迁移
- [x] 创建 migrations/ 目录
- [x] 创建初始迁移脚本 (PostgreSQL)
- [ ] 实现 SQLite 迁移
- [ ] 实现 MySQL 迁移

### 3.4 Store 接口
- [x] 创建 db/stores/mod.rs - Store trait 定义
- [x] 创建 db/stores/organization_store.rs
- [x] 创建 db/stores/room_store.rs
- [x] 创建 db/stores/message_store.rs
- [x] 创建 db/stores/user_store.rs
- [x] 创建 db/stores/event_store.rs
- [x] 创建 db/stores/reaction_store.rs

### 3.5 PostgreSQL 实现
- [x] 创建 db/postgres/mod.rs
- [x] 创建 db/postgres/organization_store.rs
- [x] 创建 db/postgres/room_store.rs
- [x] 创建 db/postgres/user_store.rs
- [x] 创建 db/postgres/message_store.rs
- [x] 创建 db/postgres/event_store.rs
- [x] 创建 db/postgres/reaction_store.rs

### 3.6 其他实现
- [ ] 创建 db/sqlite/ - SQLite 实现
- [ ] 创建 db/mysql/ - MySQL 实现

---

## Phase 4: Matrix 模块 ✅

### 4.1 Matrix 客户端核心
- [x] 完善 matrix.rs - Matrix SDK 封装
- [x] 创建 matrix/event_handler.rs - 事件处理器
- [x] 创建 matrix/ghost.rs - Ghost 用户管理
- [x] 实现 MatrixEvent 类型定义

### 4.2 事件处理
- [x] 实现 MatrixEventHandler trait
  - [x] handle_room_message
  - [x] handle_room_member
  - [x] handle_reaction
  - [x] handle_room_redaction
  - [x] handle_room_encryption
  - [x] handle_room_name/topic/avatar
- [x] 实现 MatrixEventProcessor
  - [x] 事件年龄检查
  - [x] 事件分发逻辑

### 4.3 Ghost 用户管理
- [x] 创建 GhostUserManager
  - [x] get_or_create_ghost
  - [x] update_ghost_profile
  - [x] ensure_ghost_in_room
  - [x] remove_ghost_from_room
  - [x] 用户映射缓存 (LRU Cache)

### 4.4 房间操作
- [x] create_room - 创建房间
- [x] ensure_bot_joined_room - 确保 bot 在房间中
- [x] invite_user - 邀请用户
- [x] kick_user - 踢出用户
- [x] leave_room - 离开房间
- [x] get_room_members - 获取房间成员

### 4.5 消息操作
- [x] send_message - 发送消息
- [x] send_message_with_reply - 发送回复
- [x] send_message_edit - 编辑消息
- [x] send_reaction - 发送反应
- [x] redact_event - 删除/撤回消息

### 4.6 其他操作
- [x] set_room_name - 设置房间名
- [x] set_room_topic - 设置房间主题

---

## Phase 5: Zulip 模块 🔴 P0

### 5.1 Zulip 客户端核心
- [ ] 完善 zulip.rs - Zulip API 客户端
- [ ] 完善 zulip/types.rs - Zulip 类型定义
  - [ ] Message 类型
  - [ ] Stream 类型
  - [ ] User 类型
  - [ ] Event 类型
  - [ ] Reaction 类型
- [ ] 创建 zulip/http_client.rs - HTTP API 客户端
  - [ ] 认证 (email + API key)
  - [ ] 发送消息
  - [ ] 获取消息
  - [ ] 上传文件
  - [ ] 获取用户列表
  - [ ] 获取 Stream 列表

### 5.2 WebSocket 事件流
- [ ] 创建 zulip/websocket.rs - WebSocket 客户端
  - [ ] 连接事件队列
  - [ ] 注册事件类型
  - [ ] 接收实时事件
  - [ ] 心跳保活
  - [ ] 重连机制

### 5.3 事件处理
- [ ] 创建 zulip/event_handler.rs - 事件处理器
  - [ ] 处理 message 事件
  - [ ] 处理 reaction 事件
  - [ ] 处理 update_message 事件
  - [ ] 处理 delete_message 事件
  - [ ] 处理 subscription 事件

---

## Phase 6: Bridge 核心逻辑 🟠 P1

### 6.1 Bridge 核心
- [ ] 完善 bridge.rs - 桥接核心
  - [ ] Bridge 状态管理
  - [ ] 启动/停止逻辑
  - [ ] 配置重载

### 6.2 消息流转
- [ ] 创建 bridge/message_flow.rs - 消息流转
  - [ ] Matrix -> Zulip 消息转发
  - [ ] Zulip -> Matrix 消息转发
  - [ ] 消息队列管理
  - [ ] 消息重试机制

### 6.3 用户同步
- [ ] 创建 bridge/user_sync.rs - 用户同步
  - [ ] Zulip 用户 -> Matrix Ghost
  - [ ] Matrix 用户 -> Zulip Puppet (可选)
  - [ ] 用户资料同步
  - [ ] 用户映射管理

### 6.4 事件队列
- [ ] 创建 bridge/queue.rs - 事件队列
  - [ ] 优先级队列
  - [ ] 事件去重
  - [ ] 背压控制

---

## Phase 7: 房间管理 🟠 P1

### 7.1 房间基础
- [ ] 完善 rooms/room.rs - 房间基类
- [ ] 创建 rooms/registry.rs - 房间注册表

### 7.2 房间类型
- [ ] 创建 rooms/control_room.rs - 控制房间
  - [ ] 初始化控制房间
  - [ ] 处理命令
- [ ] 创建 rooms/organization_room.rs - 组织房间
  - [ ] 管理组织配置
  - [ ] 管理订阅的 Stream
- [ ] 创建 rooms/stream_room.rs - Stream 房间
  - [ ] Stream <-> Matrix 房间映射
  - [ ] Topic <-> Thread 映射
- [ ] 创建 rooms/direct_room.rs - 私信房间
  - [ ] DM 双向转发
- [ ] 创建 rooms/space_room.rs - Space 房间
  - [ ] 组织所有 Stream 房间

---

## Phase 8: 命令系统 🟠 P1

### 8.1 命令框架
- [ ] 创建 command/mod.rs - 命令模块
- [ ] 创建 command/parser.rs - 命令解析器
- [ ] 创建 command/executor.rs - 命令执行器

### 8.2 控制房间命令
- [ ] help - 显示帮助
- [ ] addorganization - 添加组织
- [ ] open - 打开组织房间

### 8.3 组织房间命令
- [ ] site - 设置 Zulip 站点 URL
- [ ] email - 设置 Bot 邮箱
- [ ] apikey - 设置 API Key
- [ ] connect - 连接到 Zulip
- [ ] disconnect - 断开连接
- [ ] status - 显示连接状态
- [ ] subscribe - 订阅 Stream
- [ ] unsubscribe - 取消订阅
- [ ] space - 创建 Matrix Space
- [ ] list - 列出订阅的 Stream
- [ ] personalroom - 创建个人房间

---

## Phase 9: 消息解析器 🟡 P2

### 9.1 解析器框架
- [ ] 完善 parsers/matrix_parser.rs - Matrix 消息解析
- [ ] 完善 parsers/zulip_parser.rs - Zulip 消息解析
- [ ] 创建 parsers/html.rs - HTML 处理
- [ ] 创建 parsers/markdown.rs - Markdown 处理

### 9.2 格式转换
- [ ] Matrix HTML -> Zulip Markdown
- [ ] Zulip Markdown -> Matrix HTML
- [ ] Mention 用户转换 (@user -> @user)
- [ ] Emoji 转换 (zulip_emoji_mapping)
- [ ] 链接转换

---

## Phase 10: 媒体处理 🟡 P2

### 10.1 媒体下载
- [ ] 完善 media.rs - 媒体处理模块
- [ ] Matrix 媒体下载 (mxc:// -> file)
- [ ] Zulip 附件下载

### 10.2 媒体上传
- [ ] Matrix 媒体上传 (file -> mxc://)
- [ ] Zulip 附件上传

### 10.3 媒体转发
- [ ] Matrix -> Zulip 附件转发
- [ ] Zulip -> Matrix 附件转发
- [ ] 媒体大小限制检查
- [ ] 媒体类型检测

---

## Phase 11: 高级功能 🟡 P2

### 11.1 消息高级功能
- [ ] 消息回复 (Reply) 同步
- [ ] 消息编辑同步
- [ ] 消息删除/Redaction 同步
- [ ] 消息反应 (Reactions) 同步
- [ ] Topic <-> Thread 映射

### 11.2 成员同步
- [ ] 成员同步模式 (lazy/half/full)
- [ ] 成员加入/离开事件
- [ ] 权限/角色同步

### 11.3 其他功能
- [ ] 消息回填 (Backfill)
- [ ] Read Receipts
- [ ] 封禁同步 (relay_moderation)

---

## Phase 12: Web 服务 🟢 P3

### 12.1 HTTP 服务
- [ ] 完善 web.rs - Web 服务
- [ ] Appservice HTTP 监听
- [ ] 健康检查端点 (/health)
- [ ] 就绪端点 (/ready)

### 12.2 指标和监控
- [ ] 创建 web/metrics.rs - Prometheus 指标
- [ ] 创建 web/status.rs - 状态端点

---

## Phase 13: 部署 🟢 P3

### 13.1 Docker
- [ ] 创建 Dockerfile
- [ ] 创建 docker-compose.yml
- [ ] 创建 .dockerignore

### 13.2 配置示例
- [ ] 创建 config.example.yaml
- [ ] 创建 registration.example.yaml
- [ ] 创建 README.md
- [ ] 创建 README_CN.md

### 13.3 CI/CD
- [ ] 创建 .github/workflows/ci.yml
- [ ] 创建 .github/workflows/release.yml

---

## Phase 14: 测试 🟢 P3

### 14.1 单元测试
- [ ] 配置模块测试
- [ ] 数据库模块测试
- [ ] 解析器测试
- [ ] 命令测试

### 14.2 集成测试
- [ ] Matrix 客户端集成测试
- [ ] Zulip 客户端集成测试
- [ ] Bridge 集成测试

---

## 实现顺序建议

### 第 1 阶段: 数据库基础 (Phase 3)
1. 数据库错误和连接池
2. 数据模型定义
3. Store trait 和基础实现
4. 数据库迁移

### 第 2 阶段: Matrix 客户端 (Phase 4)
1. Matrix 客户端基础
2. Ghost 用户管理
3. 事件处理器
4. 房间操作

### 第 3 阶段: Zulip 客户端 (Phase 5)
1. HTTP 客户端和类型
2. WebSocket 事件流
3. 事件处理器

### 第 4 阶段: Bridge 核心 (Phase 6-7)
1. 消息流转
2. 用户同步
3. 房间管理
4. 事件队列

### 第 5 阶段: 用户交互 (Phase 8-10)
1. 命令系统
2. 消息解析器
3. 媒体处理

### 第 6 阶段: 完善和部署 (Phase 11-14)
1. 高级功能
2. Web 服务
3. 部署配置
4. 测试
