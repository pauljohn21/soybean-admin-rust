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
	cargo build --bin server --release

# 运行测试
test:
	cargo test

# 清理构建产物
clean:
	cargo clean

# Docker相关命令
# 启动所有服务
docker-up:
	cd deploy && docker-compose -p soybean-admin-rust up -d

# 停止所有服务
docker-down:
	cd deploy && docker-compose -p soybean-admin-rust down

# 停止所有服务并删除数据卷
docker-down-v:
	cd deploy && docker-compose -p soybean-admin-rust down -v

# 查看服务状态
docker-ps:
	cd deploy && docker-compose -p soybean-admin-rust ps

# 查看服务日志
docker-logs:
	cd deploy && docker-compose -p soybean-admin-rust logs -f

# Redis 集群相关命令
# 启动 Redis 集群
redis-cluster-up:
	cd deploy && docker-compose -p soybean-admin-rust-redis -f docker-compose-redis-cluster.yml up -d

# 停止 Redis 集群
redis-cluster-down:
	cd deploy && docker-compose -p soybean-admin-rust-redis -f docker-compose-redis-cluster.yml down

# 停止 Redis 集群并删除数据卷
redis-cluster-down-v:
	cd deploy && docker-compose -p soybean-admin-rust-redis -f docker-compose-redis-cluster.yml down -v

# 查看 Redis 集群状态
redis-cluster-ps:
	cd deploy && docker-compose -p soybean-admin-rust-redis -f docker-compose-redis-cluster.yml ps

# 查看 Redis 集群日志
redis-cluster-logs:
	cd deploy && docker-compose -p soybean-admin-rust-redis -f docker-compose-redis-cluster.yml logs -f

# 检查 Redis 集群信息
redis-cluster-info:
	@docker exec -it redis_1 sh -c 'echo 123456 | redis-cli -h 127.0.0.1 -p 7001 --user soybean --askpass cluster info'

# 检查 Redis 集群节点
redis-cluster-nodes:
	@docker exec -it redis_1 sh -c 'echo 123456 | redis-cli -h 127.0.0.1 -p 7001 --user soybean --askpass cluster nodes'

# 默认任务：格式化代码并运行服务器
.PHONY: default
default: fmt run-server

# 声明所有任务为伪目标
.PHONY: fmt run-server run-migration migrate-up migrate-down build test clean \
	docker-up docker-down docker-down-v docker-ps docker-logs \
	redis-cluster-up redis-cluster-down redis-cluster-down-v redis-cluster-ps redis-cluster-logs redis-cluster-info redis-cluster-nodes \
	generate-schema-migration generate-data-migration

# 生成表结构迁移文件
# 用法: make generate-schema-migration name=table_name
# 例如: make generate-schema-migration name=sys_role
generate-schema-migration:
	@if [ -z "$(name)" ]; then \
		echo "Error: Please provide a name for the schema migration."; \
		echo "Usage: make generate-schema-migration name=table_name"; \
		exit 1; \
	fi
	sea-orm-cli migrate generate --migration-dir migration/src/schemas create_$(name)

# 生成数据迁移文件
generate-data-migration:
	@if [ -z "$(name)" ]; then \
		echo "Error: Please provide a name for the data migration."; \
		echo "Usage: make generate-data-migration name=insert_default_data"; \
		exit 1; \
	fi
	sea-orm-cli migrate generate --migration-dir migration/src/datas insert_$(name)
