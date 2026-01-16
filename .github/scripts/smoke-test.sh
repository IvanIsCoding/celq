#!/bin/sh
# Smoke test for celq
# Tests basic functionality using return codes and minimal output

set -e

CELQ="$1"

if [ -z "$CELQ" ]; then
    echo "Usage: $0 <path-to-celq-binary>"
    exit 1
fi

if [ ! -x "$CELQ" ]; then
    echo "Error: $CELQ is not executable or does not exist"
    exit 1
fi

echo "Running celq smoke tests..."

# Test 1: Basic arithmetic with boolean exit code
echo "Test 1: Basic arithmetic (2 + 2 == 4)"
echo '{}' | "$CELQ" -b '2 + 2 == 4' || exit 1

# Test 2: Boolean false should return exit code 1
echo "Test 2: Boolean false returns exit code 1"
echo '{}' | "$CELQ" -b '1 > 5' && exit 1 || true

# Test 3: JSON input field access
echo "Test 3: JSON input field access"
echo '{"name":"test","value":42}' | "$CELQ" -b 'this.value == 42' || exit 1

# Test 4: String operations
echo "Test 4: String operations"
echo '{}' | "$CELQ" -b '"hello".contains("ell")' || exit 1

# Test 5: List operations
echo "Test 5: List operations"
echo '{}' | "$CELQ" -b '2 in [1, 2, 3]' || exit 1

# Test 6: Arguments with int type
echo "Test 6: Arguments (int)"
echo '{}' | "$CELQ" -b --arg 'x:int=10' 'x == 10' || exit 1

# Test 7: Raw output for strings
echo "Test 7: Raw output"
OUTPUT=$(echo '{}' | "$CELQ" -r '"hello"')
[ "$OUTPUT" = "hello" ] || exit 1

# Test 8: NDJSON processing
echo "Test 8: NDJSON processing"
printf '{"x":1}\n{"x":2}\n{"x":3}\n' | "$CELQ" --void 'this.x * 2' || exit 1

# Test 9: Nested JSON access
echo "Test 9: Nested JSON access"
echo '{"person":{"name":"Alice","age":30}}' | "$CELQ" -b 'this.person.age == 30' || exit 1

# Test 10: List map operation
echo "Test 10: List map operation"
echo '{}' | "$CELQ" --void '[1, 2, 3].map(x, x * 2)' || exit 1

echo ""
echo "All smoke tests passed!"