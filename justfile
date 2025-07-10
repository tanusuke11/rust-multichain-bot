# Justfile for multichain bot project

# Build all components
build:
    echo "Building all components..."

# Deploy to production
deploy:
    echo "Deploying to production..."

# Run tests
test:
    echo "Running tests..."

# Start development environment
dev:
    echo "Starting development environment..."

# Clean build artifacts
clean:
    echo "Cleaning build artifacts..."

# Start Rust worker bot using Docker (clean build with volume purge to avoid caching issues)
worker:
    cd runtime/worker && docker-compose down -v && docker-compose up --build
