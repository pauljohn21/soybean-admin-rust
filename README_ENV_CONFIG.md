# 环境变量优先配置系统

## 概述

soybean-admin-rust 现在支持环境变量优先的配置管理方式，遵循 12-factor 应用原则。

## 配置优先级

1. **环境变量**（最高优先级）
2. **配置文件**（中等优先级）
3. **默认值**（最低优先级）

## 环境变量命名规范

### 基本规则

- 使用 `APP_` 前缀
- 嵌套配置用下划线分隔
- 所有字母大写

### 示例

#### 数据库配置

```bash
# 基本数据库配置
APP_DATABASE_URL=postgres://user:pass@localhost:5432/dbname
APP_DATABASE_MAX_CONNECTIONS=10
APP_DATABASE_MIN_CONNECTIONS=1
APP_DATABASE_CONNECT_TIMEOUT=30
APP_DATABASE_IDLE_TIMEOUT=600
```

#### 服务器配置

```bash
APP_SERVER_HOST=0.0.0.0
APP_SERVER_PORT=8080
```

#### JWT 配置

```bash
APP_JWT_JWT_SECRET=your-super-secret-key
APP_JWT_ISSUER=https://your-domain.com
APP_JWT_EXPIRE=7200
```

#### Redis 配置

```bash
# 单机模式
APP_REDIS_MODE=single
APP_REDIS_URL=redis://:password@localhost:6379/0

# 集群模式
APP_REDIS_MODE=cluster
APP_REDIS_URLS=redis://:pass@host1:6379,redis://:pass@host2:6379
```

#### MongoDB 配置

```bash
APP_MONGO_URI=mongodb://user:pass@localhost:27017/dbname
```

#### S3 配置

```bash
APP_S3_REGION=us-east-1
APP_S3_ACCESS_KEY_ID=your-access-key
APP_S3_SECRET_ACCESS_KEY=your-secret-key
APP_S3_ENDPOINT=https://s3.amazonaws.com
```

## 使用方法

### 1. 环境变量 + 配置文件（推荐）

```rust
// 在 main.rs 中
server_initialize::initialize_config_with_env("application.yaml", None).await;
```

这种方式会：

1. 先加载配置文件
2. 然后用环境变量覆盖对应的值

### 2. 仅使用环境变量

```rust
// 在 main.rs 中
server_initialize::initialize_config_from_env_only(None).await;
```

这种方式完全依赖环境变量，不读取配置文件。

### 3. 自定义环境变量前缀

```rust
// 使用自定义前缀 "MYAPP_"
server_initialize::initialize_config_with_env("application.yaml", Some("MYAPP")).await;
```

## 实际使用示例

### Docker 环境

```dockerfile
# Dockerfile
ENV APP_DATABASE_URL=postgres://user:pass@db:5432/myapp
ENV APP_SERVER_HOST=0.0.0.0
ENV APP_SERVER_PORT=8080
ENV APP_JWT_JWT_SECRET=production-secret-key
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'
services:
  app:
    image: soybean-admin-rust
    environment:
      - APP_DATABASE_URL=postgres://user:pass@db:5432/myapp
      - APP_SERVER_HOST=0.0.0.0
      - APP_SERVER_PORT=8080
      - APP_JWT_JWT_SECRET=production-secret-key
      - APP_REDIS_URL=redis://:password@redis:6379/0
    ports:
      - "8080:8080"
```

### Kubernetes

```yaml
# k8s-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: soybean-admin-rust
spec:
  template:
    spec:
      containers:
      - name: app
        image: soybean-admin-rust
        env:
        - name: APP_DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        - name: APP_JWT_JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: jwt-secret
              key: secret
        - name: APP_SERVER_HOST
          value: "0.0.0.0"
        - name: APP_SERVER_PORT
          value: "8080"
```

### 本地开发

```bash
# .env 文件
APP_DATABASE_URL=postgres://dev:dev@localhost:5432/soybean_dev
APP_SERVER_HOST=127.0.0.1
APP_SERVER_PORT=9528
APP_JWT_JWT_SECRET=dev-secret-key
APP_REDIS_URL=redis://localhost:6379/0
```

## 配置验证

启动应用时，你会看到类似的日志：

```log
[INFO] Initializing configuration with environment variable override support
[INFO] Config file: application.yaml
[INFO] Environment prefix: APP
[INFO] Loading config from file: application.yaml
[INFO] Loading config from environment variables with prefix: APP
[INFO] Configuration loaded successfully with environment variable override support
[INFO] Configuration initialized successfully with environment variable support
```

## 最佳实践

### 1. 安全性

- 敏感信息（如密码、密钥）优先使用环境变量
- 不要在代码仓库中提交包含敏感信息的 `.env` 文件
- 生产环境使用密钥管理服务

### 2. 配置管理

- 开发环境：使用 `.env` 文件
- 测试环境：使用环境变量覆盖
- 生产环境：完全使用环境变量

### 3. 文档化

- 在 `.env.example` 中记录所有可用的环境变量
- 为每个环境变量添加注释说明
- 保持环境变量命名的一致性

## 故障排除

### 常见问题

1. **环境变量没有生效**
   - 检查环境变量名是否正确（注意大小写）
   - 确认环境变量前缀是否匹配
   - 验证环境变量是否正确设置

2. **配置解析失败**
   - 检查环境变量值的格式是否正确
   - 确认必需的配置项是否都已设置
   - 查看应用启动日志中的错误信息

3. **类型转换错误**
   - 确保数字类型的环境变量值是有效数字
   - 布尔值使用 `true`/`false`
   - 检查 URL 格式是否正确

### 调试技巧

1. 启用详细日志查看配置加载过程
2. 使用 `env | grep APP_` 检查环境变量设置
3. 在测试环境中逐步验证配置项

## 迁移指南

### 从纯配置文件迁移

1. 保留现有的配置文件作为默认值
2. 逐步将敏感配置移到环境变量
3. 更新部署脚本设置环境变量
4. 验证所有环境下的配置正确性

### 示例迁移步骤

```bash
# 步骤 1: 备份现有配置
cp application.yaml application.yaml.backup

# 步骤 2: 创建环境变量文件
cp .env.example .env

# 步骤 3: 设置关键环境变量
export APP_DATABASE_URL="your-database-url"
export APP_JWT_JWT_SECRET="your-jwt-secret"

# 步骤 4: 测试应用启动
cargo run

# 步骤 5: 验证配置生效
# 检查日志确认环境变量被正确加载
```

这样，你就可以逐步迁移到环境变量优先的配置管理方式，同时保持向后兼容性。
