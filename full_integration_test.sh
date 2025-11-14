#!/usr/bin/env bash
set -euo pipefail
BASE="http://127.0.0.1:8080"
GREEN='\033[0;32m'; RED='\033[0;31m'; NC='\033[0m'
log() { echo -e "${GREEN}[TEST]${NC} $1"; }
fail() { echo -e "${RED}[FAIL]${NC} $1"; exit 1; }

log "Health check"
curl -sf "$BASE/health" > /dev/null || fail "Health unreachable"

log "Create user"
ID=$(curl -sfX POST "$BASE/api/users" \
     -H "Content-Type: application/json" \
     -d '{"name":"A","email":"a@x.co","password":"123","role":"driver"}' | jq -r .data.id)

log "List users"
curl -sf "$BASE/api/users" | jq .data

log "Register vehicle"
VID=$(curl -sfX POST "$BASE/api/users/$ID/vehicles" \
      -H "Content-Type: application/json" \
      -d '{"plate_number":"XY123","make":"Toyota","model":"Corolla","year":2020}' | jq -r .data.id)

log "List vehicles"
curl -sf "$BASE/api/users/$ID/vehicles" | jq .data

log "All endpoints ok ✔"
