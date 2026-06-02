#!/bin/bash
# frontend/e2e/start-servers.sh
set -e

# Kill any existing servers on ports 3000 and 3001
lsof -ti:3000 | xargs kill -9 2>/dev/null || true
lsof -ti:3001 | xargs kill -9 2>/dev/null || true
sleep 1

trap 'kill $MOCK_PID $NEXT_PID 2>/dev/null' EXIT

# Start mock API in background
npx tsx e2e/mock-api.ts &
MOCK_PID=$!

# Wait for mock API to be ready
echo "Waiting for mock API..."
for i in $(seq 1 30); do
  if curl -sf http://localhost:3001/api/settings > /dev/null 2>&1; then
    echo "Mock API ready on :3001"
    break
  fi
  if [ $i -eq 30 ]; then
    echo "ERROR: Mock API failed to start"
    exit 1
  fi
  sleep 1
done

# Build Next.js with mock API
echo "Building Next.js..."
export NEXT_PUBLIC_API_URL=http://localhost:3001
npm run build

# Start Next.js in background
echo "Starting Next.js..."
npm run start &
NEXT_PID=$!

# Wait for Next.js to be ready
echo "Waiting for Next.js..."
for i in $(seq 1 60); do
  if curl -sf http://localhost:3000 > /dev/null 2>&1; then
    echo "Next.js ready on :3000"
    break
  fi
  if [ $i -eq 60 ]; then
    echo "ERROR: Next.js failed to start"
    exit 1
  fi
  sleep 1
done

echo "All servers ready. Waiting..."
wait -n
