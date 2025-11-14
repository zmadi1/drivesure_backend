#!/usr/bin/env bash
set -euo pipefail
BASE="http://127.0.0.1:8080"
GREEN='\033[0;32m'; RED='\033[0;31m'; NC='\033[0m'
log() { echo -e "${GREEN}[TEST]${NC} $1"; }
fail() { echo -e "${RED}[FAIL]${NC} $1"; exit 1; }

log "Health check"
curl -sf "$BASE/health" > /dev/null || fail "Server down"

log "Create new user"
USER=$(curl -sfX POST "$BASE/api/users" \
       -H "Content-Type: application/json" \
       -d "{\"name\":\"Test Driver\",\"email\":\"test-$(date +%s)@demo.co\",\"password\":\"secret\",\"role\":\"driver\"}") || fail "User creation"
ID=$(echo "$USER" | jq -r .data.id)
log "Created user id=$ID"

log "List all users (first 3 shown)"
curl -sf "$BASE/api/users" | jq '.data[:3]'

log "Register vehicle #1 for user $ID"
V1=$(curl -sfX POST "$BASE/api/users/$ID/vehicles" \
      -H "Content-Type: application/json" \
      -d '{"plate_number":"DM001","make":"VW","model":"Golf","year":2021}') || fail "Vehicle1"
V1_ID=$(echo "$V1" | jq -r .data.id)
log "Vehicle1 id=$V1_ID"

log "Register vehicle #2 for user $ID"
V2=$(curl -sfX POST "$BASE/api/users/$ID/vehicles" \
      -H "Content-Type: application/json" \
      -d '{"plate_number":"DM002","make":"Toyota","model":"Corolla","year":2023}') || fail "Vehicle2"
V2_ID=$(echo "$V2" | jq -r .data.id)
log "Vehicle2 id=$V2_ID"

log "List both vehicles for user $ID"
VEHS=$(curl -sf "$BASE/api/users/$ID/vehicles") || fail "Vehicle list"
echo "$VEHS" | jq '.data'

log "Demo complete – endpoints work 🎉"
