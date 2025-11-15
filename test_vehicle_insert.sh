#!/usr/bin/env bash
set -euo pipefail

DB="drivesure_dev"
API="http://127.0.0.1:8080"

echo "🔧 Fetch first user id from DB"
USER_ID=$(psql -d "$DB" -tAc "SELECT id FROM users LIMIT 1")
[[ -n "$USER_ID" ]] || { echo "❌ No users in DB"; exit 1; }
echo "Using user-id: $USER_ID"

echo "🚗 Inserting vehicle..."
curl -sfX POST "$API/api/users/$USER_ID/vehicles" \
  -H "Content-Type: application/json" \
  -d '{"plate_number":"QUICK","make":"VW","model":"Golf","year":2021}' \
| jq .

echo "✅ Done – row should exist in DB:"
psql -d "$DB" -c "SELECT id, plate_number, make, model, year FROM vehicles WHERE owner_user_id = '$USER_ID';"
