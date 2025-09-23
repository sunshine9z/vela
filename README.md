## Vela

基于 `DDD(Domain Driven Design)` 领域驱动设计,架构实现 Rust 版本的 web 框架.


# 架构

```
./Vela
├── application         // 应用层: 做domain编排
│   ├── xxx             // xxx 应用层模块
├── conf                // 配置文件
├── consts              // 常量定义
├── docs                // 接口文档
├── domain              // 领域层: 
│   ├── xxx             // xxx 领域层模块
│   │   ├── entity      // xxx 实体定义及充血对象
│   │   ├── repository  // xxx 实体的数据持久化接口
│   │   └── service     // xxx 具体业务逻辑
├── infrastructure      // 基础架构层: 提供数据来源和基础服务能力
│   ├── auth            // 鉴权认证服务
│   ├── common          // 公共服务
│   │   ├── context     // context 上下游管理
│   │   └── log         // log 服务
│   ├── encrypt         // 加密 服务
│   └── persistence     // 持久层
│       ├── dbs         // db数据连接
│       ├── xxx         // xxx 的dao层 访问数据库 xxx 表
├── interfaces          // 接口层: 对接不同的端进行适配转化
│   ├── adapter         // 适配器
│   │   └── initialize  // Web 路由初始化
│   ├── controller      // controller 层
│   ├── midddleware     // 中间件
│   └── types           // 类型
└── logs                // 日志文件存储
└── main.js             // 启动入口
```