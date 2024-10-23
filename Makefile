# 格式化所有 Rust 代码
fmt:
	cargo fmt --all

# 运行服务器
run-server:
	cargo run --bin server

# 运行数据库迁移
# 注意：不带参数时，默认执行 "up" 操作
run-migration:
	cargo run --bin migration

# 执行向上迁移（应用所有未应用的迁移）
migrate-up:
	cargo run --bin migration -- up

# 执行向下迁移（回滚最后一次迁移）
migrate-down:
	cargo run --bin migration -- down

# 构建项目
build:
	cargo build

# 运行测试
test:
	cargo test

# 清理构建产物
clean:
	cargo clean

# 默认任务：格式化代码并运行服务器
.PHONY: default
default: fmt run-server

# 声明所有任务为伪目标
.PHONY: fmt run-server run-migration migrate-up migrate-down build test clean generate-migration

# 生成新的迁移文件
# 用法: make generate-migration name=table_name
# 例如: make generate-migration name=sys_role
generate-migration:
	@if [ -z "$(name)" ]; then \
		echo "Error: Please provide a name for the migration."; \
		echo "Usage: make generate-migration name=table_name"; \
		exit 1; \
	fi
	sea-orm-cli migrate generate --migration-dir migration/src/migrations create_$(name)
