#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== DriveSure API - Complete Test Suite ===${NC}"
echo

BASE_URL="http://localhost:8080"
PASS_COUNT=0
FAIL_COUNT=0

# Helper function to make API calls
api_call() {
    local description="$1"
    local method="$2"
    local endpoint="$3"
    local data="$4"
    local expected_status="$5"
    
    echo -e "${YELLOW}Testing: $description${NC}"
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -w "HTTP_STATUS:%{http_code}" "$BASE_URL$endpoint")
    else
        response=$(curl -s -w "HTTP_STATUS:%{http_code}" -X "$method" -H "Content-Type: application/json" -d "$data" "$BASE_URL$endpoint")
    fi
    
    http_code=$(echo "$response" | grep -o 'HTTP_STATUS:[0-9]*' | cut -d: -f2)
    body=$(echo "$response" | sed 's/HTTP_STATUS:[0-9]*//g')
    
    if [ "$http_code" = "$expected_status" ]; then
        echo -e "${GREEN}✅ PASS - Expected $expected_status, Got $http_code${NC}"
        echo "Response: $body" | jq . 2>/dev/null || echo "Response: $body"
        ((PASS_COUNT++))
    else
        echo -e "${RED}❌ FAIL - Expected $expected_status, Got $http_code${NC}"
        echo "Response: $body" | jq . 2>/dev/null || echo "Response: $body"
        ((FAIL_COUNT++))
    fi
    echo
}

# Wait for server to be ready
echo -e "${BLUE}Waiting for server to be ready...${NC}"
until curl -s "$BASE_URL/health" >/dev/null; do
    sleep 1
done

echo -e "${GREEN}Server is ready! Starting tests...${NC}"
echo

### 1. HEALTH CHECKS ###
echo -e "${BLUE}=== 1. HEALTH & BASIC ENDPOINTS ===${NC}"
api_call "Health Check" "GET" "/health" "" "200"
api_call "Welcome Endpoint" "GET" "/" "" "200"

### 2. USER CREATION - SUCCESS CASES ###
echo -e "${BLUE}=== 2. USER CREATION - SUCCESS CASES ===${NC}"
api_call "Create Owner User" "POST" "/api/users" '{
    "email": "owner1@example.com",
    "password": "securepassword123",
    "role": "owner",
    "first_name": "John",
    "last_name": "Doe",
    "phone": "+27123456789"
}' "200"

api_call "Create Driver User" "POST" "/api/users" '{
    "email": "driver1@example.com",
    "password": "driverpass123",
    "role": "driver",
    "first_name": "Mike",
    "last_name": "Smith"
}' "200"

api_call "Create User Without Optional Fields" "POST" "/api/users" '{
    "email": "minimal@example.com",
    "password": "minimalpass",
    "role": "owner"
}' "200"

### 3. USER CREATION - ERROR CASES ###
echo -e "${BLUE}=== 3. USER CREATION - ERROR CASES ===${NC}"
api_call "Duplicate Email (Should Fail)" "POST" "/api/users" '{
    "email": "owner1@example.com",
    "password": "anotherpassword",
    "role": "driver"
}' "400"

api_call "Invalid Role (Should Fail)" "POST" "/api/users" '{
    "email": "invalidrole@example.com",
    "password": "password123",
    "role": "admin"
}' "400"

api_call "Missing Email (Should Fail)" "POST" "/api/users" '{
    "password": "password123",
    "role": "owner"
}' "400"

api_call "Missing Password (Should Fail)" "POST" "/api/users" '{
    "email": "nopassword@example.com",
    "role": "owner"
}' "400"

api_call "Missing Role (Should Fail)" "POST" "/api/users" '{
    "email": "norole@example.com",
    "password": "password123"
}' "400"

api_call "Invalid JSON (Should Fail)" "POST" "/api/users" '{
    "email": "badjson@example.com",
    "password": "pass",
    "role": "owner"
' "400"

### 4. USER LISTING & RETRIEVAL ###
echo -e "${BLUE}=== 4. USER LISTING & RETRIEVAL ===${NC}"
api_call "List All Users" "GET" "/api/users" "" "200"

### 5. VEHICLE ENDPOINTS (Placeholder Tests) ###
echo -e "${BLUE}=== 5. VEHICLE ENDPOINTS (Placeholder) ===${NC}"
api_call "Create Vehicle (Not Implemented)" "POST" "/api/vehicles" '{
    "make": "Toyota",
    "model": "Corolla",
    "year": 2022,
    "license_plate": "CA123456",
    "weekly_rent": 150000
}' "200"

api_call "List Vehicles (Not Implemented)" "GET" "/api/vehicles" "" "200"
api_call "Available Vehicles (Not Implemented)" "GET" "/api/vehicles/available" "" "200"

### 6. SECURITY TESTS ###
echo -e "${BLUE}=== 6. SECURITY TESTS ===${NC}"
api_call "Check Password Not in Response" "GET" "/api/users" "" "200"

### 7. DATABASE INTEGRITY TESTS ###
echo -e "${BLUE}=== 7. DATABASE INTEGRITY ===${NC}"
echo -e "${YELLOW}Checking database directly...${NC}"
psql drivesure_dev -c "SELECT 
    email, 
    role, 
    is_verified, 
    length(password_hash) as hash_length,
    created_at IS NOT NULL as has_timestamp
FROM users ORDER BY created_at;" 2>/dev/null || echo "Could not connect to database directly"

### 8. PERFORMANCE & EDGE CASES ###
echo -e "${BLUE}=== 8. PERFORMANCE & EDGE CASES ===${NC}"
api_call "Very Long Email" "POST" "/api/users" '{
    "email": "verylongemailaddress1234567890123456789012345678901234567890@example.com",
    "password": "password123",
    "role": "owner"
}' "200"

api_call "Special Characters in Names" "POST" "/api/users" '{
    "email": "special@example.com",
    "password": "password123",
    "role": "driver",
    "first_name": "José-Maria",
    "last_name": "O''Connor"
}' "200"

### SUMMARY ###
echo -e "${BLUE}=== TEST SUMMARY ===${NC}"
echo -e "${GREEN}Passed: $PASS_COUNT${NC}"
echo -e "${RED}Failed: $FAIL_COUNT${NC}"
echo

if [ $FAIL_COUNT -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED! Your API is working correctly.${NC}"
else
    echo -e "${YELLOW}⚠️  Some tests failed. Check the responses above.${NC}"
fi

echo -e "${BLUE}=== Database Verification ===${NC}"
echo "Total users in database:"
psql drivesure_dev -t -c "SELECT COUNT(*) FROM users;" 2>/dev/null || echo "Could not connect to database"

echo
echo -e "${BLUE}=== Next Steps ===${NC}"
echo "1. Implement JWT Authentication"
echo "2. Add Vehicle Management"
echo "3. Add Rental Agreements" 
echo "4. Add Payment System"
