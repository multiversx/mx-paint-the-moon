cleanup() {
    echo "Stopping all processes inside the microservice"
    
    # Try to terminate all processes with SIGTERM
    kill $MICROSERVICE_PID $REDIS_PID 2>/dev/null
    
    echo "All processes stopped."
    exit 0
}

trap cleanup SIGINT SIGTERM SIGTSTP
trap cleanup EXIT

BASE_DIR=$(pwd)

# Start Redis
redis-server &
REDIS_PID=$!

# Check if Redis started successfully
if ! kill -0 $REDIS_PID 2>/dev/null; then
    echo "Failed to start Redis."
    exit 1
fi

# Start microservice
cargo run --bin microservice &
MICROSERVICE_PID=$!

# Check if microservice started successfully
if ! kill -0 $MICROSERVICE_PID 2>/dev/null; then
    echo "Failed to start the microservice."
    kill $REDIS_PID
    exit 1
fi

# Wait for all processes to finish
wait $MICROSERVICE_PID
wait $REDIS_PID
