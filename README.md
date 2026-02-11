## Vela

基于 `DDD(Domain Driven Design)` 领域驱动设计,架构实现 Rust 版本的 web 框架.


# 架构

```
./Vela
├── bin                 // 可执行文件目录
│   ├── src             // 源代码
│   │   ├── app.rs      // 应用启动逻辑
│   │   ├── lib.rs      // 库定义
│   │   └── main.rs     // 主入口
│   ├── Cargo.toml      // 依赖配置
├── migration           // 数据库迁移
│   ├── src             // 迁移脚本
│   ├── Cargo.toml      // 依赖配置
├── src                 // 源代码主目录
│   ├── common          // 公共模块
│   │   ├── src         // 源代码
│   │   │   ├── config  // 配置管理
│   │   │   ├── logger  // 日志服务
│   │   │   ├── error.rs // 错误定义
│   │   │   ├── snowflake_id.rs // 雪花ID生成
│   │   │   └── traits.rs // 公共 trait
│   │   ├── Cargo.toml  // 依赖配置
│   ├── conf            // 配置文件
│   │   ├── config.dev.yaml // 开发环境配置
│   ├── domain          // 领域层: 核心业务逻辑
│   │   ├── job         // 任务领域
│   │   ├── operater_log // 操作日志领域
│   │   │   ├── api     // API 定义
│   │   │   ├── entity  // 实体定义
│   │   │   ├── repository // 数据持久化接口
│   │   │   └── services // 业务服务
│   │   ├── user        // 用户领域
│   │   │   ├── api     // API 定义
│   │   │   ├── entity  // 实体定义
│   │   │   ├── repository // 数据持久化接口
│   │   │   └── services // 业务服务
│   ├── infrastructure  // 基础架构层: 提供数据来源和基础服务能力
│   │   ├── src         // 源代码
│   │   │   ├── cache   // 缓存服务
│   │   │   │   ├── memory.rs // 内存缓存
│   │   │   │   ├── redis.rs  // Redis缓存
│   │   │   ├── container // 依赖注入容器
│   │   │   ├── cron_scheduled // 定时任务
│   │   │   ├── encrypt // 加密服务
│   │   │   ├── persistence // 持久层
│   │   │   │   ├── entities // 数据库实体
│   │   │   │   ├── user_repo.rs // 用户数据访问
│   │   │   │   ├── sys_oper_log_repo.rs // 操作日志数据访问
│   │   │   ├── processor // 处理器
│   │   │   │   ├── wokers // 工作线程
│   │   │   │   ├── job.rs // 任务处理
│   │   │   │   ├── processor.rs // 处理器核心
│   │   ├── Cargo.toml  // 依赖配置
│   ├── interfaces      // 接口层: 对接不同的端进行适配转化
│   │   ├── src         // 源代码
│   │   │   ├── common  // 公共组件
│   │   │   ├── controller // 控制器
│   │   │   ├── middlewares // 中间件
│   │   │   ├── routes  // 路由定义
│   │   │   ├── types   // 类型定义
│   │   ├── Cargo.toml  // 依赖配置
├── .env                // 环境变量
├── .gitignore          // Git忽略文件
├── Cargo.toml          // 项目依赖配置
├── README.md           // 项目说明
```

## 架构说明

### 1. 领域驱动设计 (DDD) 分层

- **接口层 (interfaces)**：负责处理HTTP请求和响应，包含路由定义、控制器、中间件等。
- **应用层 (application)**：负责编排领域服务，协调多个领域对象完成业务流程。
- **领域层 (domain)**：核心业务逻辑所在，包含实体、值对象、领域服务、仓储接口等。
- **基础架构层 (infrastructure)**：提供技术实现细节，如数据库访问、缓存、消息队列等。

### 2. 核心模块说明

- **domain/user**：用户领域，包含用户认证、用户信息管理等核心业务逻辑。
- **domain/operater_log**：操作日志领域，记录系统操作历史。
- **domain/job**：任务领域，处理定时任务相关逻辑。
- **infrastructure/cache**：缓存服务，支持内存缓存和Redis缓存。
- **infrastructure/cron_scheduled**：定时任务调度，基于tokio-cron-scheduler实现。
- **infrastructure/processor**：处理器，负责异步任务的执行和管理。
- **interfaces/controller**：控制器，处理HTTP请求并调用相应的服务。
- **interfaces/middlewares**：中间件，处理认证、日志记录等横切关注点。

### 3. 技术栈

- **语言**：Rust
- **Web框架**：基于Rust异步运行时
- **数据库**：支持关系型数据库（通过实体定义）
- **缓存**：Redis、内存缓存
- **定时任务**：tokio-cron-scheduler
- **依赖管理**：Cargo

### 4. 设计原则

- **依赖倒置**：高层模块不依赖低层模块，两者都依赖于抽象。
- **单一职责**：每个模块和类只负责一个功能领域。
- **开闭原则**：对扩展开放，对修改关闭。
- **接口隔离**：使用多个专门的接口，而不是一个统一的接口。
- **里氏替换**：子类可以替换父类在程序中的位置。