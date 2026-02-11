## Vela

基于 `DDD(Domain Driven Design)` 领域驱动设计架构实现的 Rust 版本 web 框架，提供完整的企业级应用开发能力。


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
│   │   │   ├── src     // 源代码
│   │   │   │   ├── api     // API 定义
│   │   │   │   ├── commons // 公共组件
│   │   │   │   ├── entity  // 实体定义
│   │   │   │   ├── repository // 数据持久化接口
│   │   │   │   └── services // 业务服务
│   │   ├── operater_log // 操作日志领域
│   │   │   ├── src     // 源代码
│   │   │   │   ├── api     // API 定义
│   │   │   │   ├── entity  // 实体定义
│   │   │   │   ├── repository // 数据持久化接口
│   │   │   │   └── services // 业务服务
│   │   ├── user        // 用户领域
│   │   │   ├── src     // 源代码
│   │   │   │   ├── api     // API 定义
│   │   │   │   ├── entity  // 实体定义
│   │   │   │   ├── repository // 数据持久化接口
│   │   │   │   └── services // 业务服务
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
│   │   │   │   ├── id_gen.rs // ID生成
│   │   │   │   ├── init.rs // 数据库初始化
│   │   │   │   ├── corn_job.rs // 定时任务数据访问
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

- **接口层 (interfaces)**：负责处理HTTP请求和响应，包含路由定义、控制器、中间件等。作为系统的入口点，负责请求的接收、参数验证、响应格式化等。
- **应用层 (application)**：负责编排领域服务，协调多个领域对象完成业务流程。应用层不包含业务规则，只负责流程协调。
- **领域层 (domain)**：核心业务逻辑所在，包含实体、值对象、领域服务、仓储接口等。领域层是系统的核心，包含所有业务规则和逻辑。
- **基础架构层 (infrastructure)**：提供技术实现细节，如数据库访问、缓存、消息队列等。基础架构层为其他层提供技术支持，实现领域层定义的接口。

### 2. 核心模块说明

- **domain/user**：用户领域，包含用户认证、用户信息管理等核心业务逻辑。
- **domain/operater_log**：操作日志领域，记录系统操作历史，用于审计和问题排查。
- **domain/job**：任务领域，处理定时任务相关逻辑，包括任务的创建、更新、删除和执行。
- **infrastructure/cache**：缓存服务，支持内存缓存和Redis缓存，提高系统性能。
- **infrastructure/cron_scheduled**：定时任务调度，基于tokio-cron-scheduler实现，负责定时任务的调度和执行。
- **infrastructure/persistence**：持久层，基于SeaORM实现，负责数据库访问和数据持久化。
- **infrastructure/processor**：处理器，负责异步任务的执行和管理，提高系统的并发处理能力。
- **interfaces/controller**：控制器，处理HTTP请求并调用相应的服务，实现RESTful API接口。
- **interfaces/middlewares**：中间件，处理认证、日志记录、异常捕获等横切关注点。

### 3. 技术栈

- **语言**：Rust 1.70+
- **Web框架**：基于Rust异步运行时（tokio）
- **数据库**：支持关系型数据库（MySQL/PostgreSQL），通过SeaORM实现ORM
- **缓存**：Redis、内存缓存
- **定时任务**：tokio-cron-scheduler
- **依赖管理**：Cargo
- **错误处理**：thiserror/anyhow
- **日志**：tracing
- **ID生成**：雪花算法
- **配置管理**：yaml

### 4. 设计原则

- **依赖倒置**：高层模块不依赖低层模块，两者都依赖于抽象。例如，领域层定义仓储接口，基础架构层实现这些接口。
- **单一职责**：每个模块和类只负责一个功能领域。例如，用户领域只处理用户相关的业务逻辑。
- **开闭原则**：对扩展开放，对修改关闭。例如，通过接口和抽象类实现扩展，而不是修改现有代码。
- **接口隔离**：使用多个专门的接口，而不是一个统一的接口。例如，为不同的领域服务定义专门的接口。
- **里氏替换**：子类可以替换父类在程序中的位置。例如，不同的仓储实现可以替换使用，而不影响业务逻辑。
- **聚合根**：通过聚合根管理领域对象，确保业务规则的一致性。例如，用户作为聚合根，管理用户相关的所有信息。
- **值对象**：使用值对象表示不需要唯一标识的概念。例如，地址、电话号码等。
- **领域服务**：将不属于单个实体的业务逻辑放在领域服务中。例如，用户认证、权限验证等。


### 5. 快速开始

#### 5.1 环境要求

- Rust 1.70+
- Cargo
- MySQL/PostgreSQL
- Redis (可选)

#### 5.2 构建和运行

```bash
# 构建项目
cargo build

# 运行项目
cargo run
```

#### 5.3 数据库迁移

```bash
# 运行数据库迁移
cargo run --bin migration
```

### 6. 总结

Vela 是一个基于DDD架构实现的Rust web框架，具有以下特点：

- **清晰的架构**：采用标准的DDD分层架构，结构清晰，职责分明
- **强大的功能**：支持用户认证、操作日志、定时任务等企业级应用所需的核心功能
- **高性能**：使用Rust语言和tokio异步运行时，具有优异的性能
- **类型安全**：充分利用Rust的类型系统，确保代码的安全性
- **易于扩展**：模块化设计，易于添加新功能和扩展现有功能

通过本文档的介绍，您应该对Vela的架构设计、核心功能和使用方法有了全面的了解。如果您有任何问题或建议，欢迎提出。