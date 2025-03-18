# Cloudflare Workers WiFi API

这是一个使用 Rust 开发的 Cloudflare Workers 应用，用于管理 WiFi 信息。

## 功能特点

- 基于 Rust 和 Cloudflare Workers
- 使用 D1 数据库存储数据
- 使用 Axum 框架处理 API 路由
- 提供 WiFi 相关的 API

## API 接口

- `GET /api/wifi/:id`: 根据 ID 查询 WiFi 信息
- `POST /api/wifi`: 添加新的 WiFi 记录

## 准备工作

1. 安装 [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)：

```bash
npm install -g wrangler
```

2. 登录 Cloudflare 账户：

```bash
wrangler login
```

## 数据库设置

1. 创建 D1 数据库：

```bash
wrangler d1 create wifi
```

2. 复制返回的数据库 ID 并更新 wrangler.toml 文件中的 `database_id` 字段。

3. 应用迁移文件创建表结构：

```bash
wrangler d1 execute wifi --file=./migrations/init.sql
```

## 本地开发

```bash
wrangler dev
```

## 部署

```bash
wrangler publish
```

## 项目结构

- `src/lib.rs`: 主入口文件
- `src/models.rs`: 数据模型定义
- `src/database.rs`: 数据库操作
- `src/router.rs`: API 路由处理

## 技术栈

- [Rust](https://www.rust-lang.org/)
- [Cloudflare Workers](https://workers.cloudflare.com/)
- [Cloudflare D1](https://developers.cloudflare.com/d1/)
- [Axum](https://github.com/tokio-rs/axum)
- [wrangler](https://developers.cloudflare.com/workers/wrangler/) 