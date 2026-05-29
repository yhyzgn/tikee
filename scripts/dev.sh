#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="${TIKEE_CONFIG:-$ROOT_DIR/config/dev.toml}"
API_PORT="${TIKEE_API_PORT:-9090}"
WEB_PORT="${TIKEE_WEB_PORT:-5173}"
API_URL="${TIKEE_API_URL:-http://localhost:$API_PORT}"
WEB_URL="${TIKEE_WEB_URL:-http://localhost:$WEB_PORT}"
LOG_DIR="$ROOT_DIR/.dev"

export TIKEE_DEV_ADMIN_USERNAME="${TIKEE_DEV_ADMIN_USERNAME:-tikee_init}"
export TIKEE_DEV_ADMIN_PASSWORD="${TIKEE_DEV_ADMIN_PASSWORD:-Tikee@2026!}"
export TIKEE_DEV_ADMIN_TOKEN="${TIKEE_DEV_ADMIN_TOKEN:-tikee-init-token}"

mkdir -p "$LOG_DIR"

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "缺少命令：$1" >&2
    exit 127
  fi
}

need_cmd cargo
need_cmd bun
need_cmd curl
need_cmd python3

extract_sqlite_db_path() {
  python3 -c 'import re, sys
path = sys.argv[1]
try:
    text = open(path, encoding="utf-8").read()
except OSError:
    sys.exit(0)
match = re.search(r"^\s*database_url\s*=\s*\"(sqlite://[^\"]+)\"", text, re.M)
if not match:
    sys.exit(0)
url = match.group(1)[len("sqlite://"):]
url = url.split("?", 1)[0]
print(url)' "$CONFIG_FILE"
}

backup_malformed_sqlite_db() {
  local db_path="$1"
  [[ -n "$db_path" && -f "$db_path" ]] || return 0
  if ! command -v sqlite3 >/dev/null 2>&1; then
    return 0
  fi
  local check_output
  check_output="$(sqlite3 "$db_path" 'PRAGMA integrity_check;' 2>&1 || true)"
  if [[ "$check_output" != "ok" ]]; then
    local stamp backup_dir base
    stamp="$(date +%Y%m%d-%H%M%S)"
    backup_dir="$LOG_DIR/db-backups"
    base="$(basename "$db_path")"
    mkdir -p "$backup_dir"
    echo "检测到 dev SQLite schema 损坏，自动备份并重建：$check_output" >&2
    for suffix in "" "-shm" "-wal"; do
      if [[ -f "$db_path$suffix" ]]; then
        cp -a "$db_path$suffix" "$backup_dir/$base$suffix.$stamp.bak" || true
        rm -f "$db_path$suffix"
      fi
    done
    echo "损坏数据库已备份到：$backup_dir" >&2
  fi
}

cleanup() {
  local code=$?
  echo
  echo "正在停止 tikee 开发进程..."
  if [[ -n "${SERVER_PID:-}" ]] && kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    kill "$SERVER_PID" >/dev/null 2>&1 || true
  fi
  if [[ -n "${WEB_PID:-}" ]] && kill -0 "$WEB_PID" >/dev/null 2>&1; then
    kill "$WEB_PID" >/dev/null 2>&1 || true
  fi
  wait "${SERVER_PID:-0}" 2>/dev/null || true
  wait "${WEB_PID:-0}" 2>/dev/null || true
  exit "$code"
}
trap cleanup INT TERM EXIT

if [[ ! -f "$CONFIG_FILE" ]]; then
  echo "配置文件不存在：$CONFIG_FILE" >&2
  exit 1
fi

DB_PATH="$(extract_sqlite_db_path)"
if [[ -n "$DB_PATH" && "$DB_PATH" != /* ]]; then
  DB_PATH="$ROOT_DIR/$DB_PATH"
fi
backup_malformed_sqlite_db "$DB_PATH"

if [[ ! -d "$ROOT_DIR/web/node_modules" ]]; then
  echo "首次启动：安装 Web 依赖..."
  (cd "$ROOT_DIR/web" && bun install)
fi

echo "启动后端：$API_URL"
(
  cd "$ROOT_DIR"
  cargo run --bin tikee -- serve --config "$CONFIG_FILE"
) >"$LOG_DIR/server.log" 2>&1 &
SERVER_PID=$!

echo -n "等待后端健康检查"
for _ in $(seq 1 60); do
  if curl -fsS "$API_URL/healthz" >/dev/null 2>&1; then
    echo " OK"
    break
  fi
  if ! kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    echo
    echo "后端启动失败，最近日志：" >&2
    tail -n 80 "$LOG_DIR/server.log" >&2 || true
    exit 1
  fi
  echo -n "."
  sleep 1
done

if ! curl -fsS "$API_URL/healthz" >/dev/null 2>&1; then
  echo
  echo "后端健康检查超时，最近日志：" >&2
  tail -n 80 "$LOG_DIR/server.log" >&2 || true
  exit 1
fi

echo "启动 Web：$WEB_URL"
(
  cd "$ROOT_DIR/web"
  bun run dev -- --port "$WEB_PORT"
) >"$LOG_DIR/web.log" 2>&1 &
WEB_PID=$!

echo -n "等待 Web dev server"
for _ in $(seq 1 60); do
  if curl -fsS "$WEB_URL" >/dev/null 2>&1; then
    echo " OK"
    break
  fi
  if ! kill -0 "$WEB_PID" >/dev/null 2>&1; then
    echo
    echo "Web 启动失败，最近日志：" >&2
    tail -n 80 "$LOG_DIR/web.log" >&2 || true
    exit 1
  fi
  echo -n "."
  sleep 1
done

if ! curl -fsS "$WEB_URL" >/dev/null 2>&1; then
  echo
  echo "Web 健康检查超时，最近日志：" >&2
  tail -n 80 "$LOG_DIR/web.log" >&2 || true
  exit 1
fi

echo
echo "开发环境已启动："
echo "  Web UI:       $WEB_URL"
echo "  Backend API:  $API_URL"
echo "  OpenAPI JSON: $API_URL/api-docs/openapi.json"
echo "  初始化账号:  $TIKEE_DEV_ADMIN_USERNAME"
echo "  初始化密码:  $TIKEE_DEV_ADMIN_PASSWORD"
echo "  后端日志:    $LOG_DIR/server.log"
echo "  前端日志:    $LOG_DIR/web.log"
echo
echo "按 Ctrl+C 停止全部进程。"

wait -n "$SERVER_PID" "$WEB_PID"
