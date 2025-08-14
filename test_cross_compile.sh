#!/bin/bash

# Test script for cross-compilation capabilities
# This script checks what can be built on the current system

set -e

echo "======================================"
echo "   Cross-Compilation Test Suite"
echo "======================================"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test results (using simple arrays for compatibility)
TEST_RESULTS_TARGETS=()
TEST_RESULTS_STATUS=()

# Function to test a build target
test_target() {
    local target=$1
    local description=$2
    
    echo -e "\n${BLUE}Testing:${NC} $description"
    echo "Target: $target"
    echo -n "Status: "
    
    # Check if target is installed
    if rustup target list --installed | grep -q "^$target$"; then
        echo -e "${GREEN}Target installed${NC}"
        
        # Try to build with --check (doesn't actually compile, just checks)
        if cargo check --target "$target" 2>/dev/null; then
            echo -e "  ${GREEN}✓ Check passed${NC}"
            TEST_RESULTS_TARGETS+=("$target")
            TEST_RESULTS_STATUS+=("PASS")
            
            # Test actual compilation (will fail on linking for cross-targets without proper toolchain)
            echo -n "  Attempting full build: "
            if timeout 30 cargo build --target "$target" 2>/dev/null; then
                echo -e "${GREEN}✓ Build successful${NC}"
                # Update last status
                TEST_RESULTS_STATUS[${#TEST_RESULTS_STATUS[@]}-1]="BUILD_SUCCESS"
            else
                echo -e "${YELLOW}⚠ Build failed (likely missing linker)${NC}"
                # Update last status
                TEST_RESULTS_STATUS[${#TEST_RESULTS_STATUS[@]}-1]="LINK_FAIL"
            fi
        else
            echo -e "  ${RED}✗ Check failed${NC}"
            TEST_RESULTS_TARGETS+=("$target")
            TEST_RESULTS_STATUS+=("CHECK_FAIL")
        fi
    else
        echo -e "${YELLOW}Target not installed${NC}"
        echo -n "  Installing target: "
        if rustup target add "$target" 2>/dev/null; then
            echo -e "${GREEN}✓${NC}"
            # Retry the test
            test_target "$target" "$description"
        else
            echo -e "${RED}✗${NC}"
            TEST_RESULTS_TARGETS+=("$target")
            TEST_RESULTS_STATUS+=("NOT_AVAILABLE")
        fi
    fi
}

# Function to check for required tools
check_tool() {
    local tool=$1
    local install_hint=$2
    
    if command -v "$tool" >/dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} $tool found"
        return 0
    else
        echo -e "  ${YELLOW}✗${NC} $tool not found"
        if [ -n "$install_hint" ]; then
            echo "      Install with: $install_hint"
        fi
        return 1
    fi
}

# Main test execution
main() {
    echo -e "\n${BLUE}=== System Information ===${NC}"
    echo "OS: $(uname -s) $(uname -m)"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    
    echo -e "\n${BLUE}=== Checking Required Tools ===${NC}"
    
    # Check for various cross-compilation tools
    echo "Cross-compilation tools:"
    check_tool "x86_64-w64-mingw32-gcc" "brew install mingw-w64" || true
    check_tool "i686-w64-mingw32-gcc" "brew install mingw-w64" || true
    check_tool "zig" "brew install zig" || true
    check_tool "lld" "brew install llvm" || true
    
    echo -e "\n${BLUE}=== Testing Native Compilation ===${NC}"
    
    # Test native target
    NATIVE_TARGET=$(rustc -vV | sed -n 's/host: //p')
    test_target "$NATIVE_TARGET" "Native target"
    
    echo -e "\n${BLUE}=== Testing Cross-Compilation Targets ===${NC}"
    
    # Windows targets
    echo -e "\n${YELLOW}Windows Targets:${NC}"
    test_target "x86_64-pc-windows-gnu" "Windows 64-bit (GNU toolchain)"
    test_target "i686-pc-windows-gnu" "Windows 32-bit (GNU toolchain)"
    test_target "x86_64-pc-windows-msvc" "Windows 64-bit (MSVC toolchain)"
    
    # Linux targets (from macOS)
    echo -e "\n${YELLOW}Linux Targets:${NC}"
    test_target "x86_64-unknown-linux-gnu" "Linux 64-bit (GNU)"
    test_target "x86_64-unknown-linux-musl" "Linux 64-bit (MUSL - static)"
    
    # macOS targets
    echo -e "\n${YELLOW}macOS Targets:${NC}"
    test_target "x86_64-apple-darwin" "macOS 64-bit Intel"
    test_target "aarch64-apple-darwin" "macOS ARM64 (Apple Silicon)"
    
    # WebAssembly (for future web version)
    echo -e "\n${YELLOW}WebAssembly:${NC}"
    test_target "wasm32-unknown-unknown" "WebAssembly"
    
    # Print summary
    echo -e "\n${BLUE}======================================"
    echo -e "         Test Summary"
    echo -e "======================================${NC}"
    
    for i in "${!TEST_RESULTS_TARGETS[@]}"; do
        target="${TEST_RESULTS_TARGETS[$i]}"
        result="${TEST_RESULTS_STATUS[$i]}"
        case "$result" in
            "BUILD_SUCCESS")
                echo -e "${GREEN}✓${NC} $target - Full build successful"
                ;;
            "LINK_FAIL")
                echo -e "${YELLOW}⚠${NC} $target - Compilation OK, linking failed (need cross-linker)"
                ;;
            "CHECK_FAIL")
                echo -e "${RED}✗${NC} $target - Compilation check failed"
                ;;
            "NOT_AVAILABLE")
                echo -e "${RED}✗${NC} $target - Target not available"
                ;;
            "PASS")
                echo -e "${GREEN}✓${NC} $target - Check passed"
                ;;
        esac
    done
    
    echo -e "\n${BLUE}=== Recommendations ===${NC}"
    
    # Check if Windows builds are possible
    windows_found=false
    linux_found=false
    for i in "${!TEST_RESULTS_TARGETS[@]}"; do
        if [[ "${TEST_RESULTS_TARGETS[$i]}" == "x86_64-pc-windows-gnu" ]] && [[ "${TEST_RESULTS_STATUS[$i]}" == "LINK_FAIL" ]]; then
            windows_found=true
        fi
        if [[ "${TEST_RESULTS_TARGETS[$i]}" == "x86_64-unknown-linux-musl" ]] && [[ "${TEST_RESULTS_STATUS[$i]}" == "LINK_FAIL" ]]; then
            linux_found=true
        fi
    done
    
    if [ "$windows_found" = true ]; then
        echo "To enable Windows cross-compilation:"
        echo "  1. Install MinGW-w64: brew install mingw-w64"
        echo "  2. Run: ./build_windows.sh"
    fi
    
    if [ "$linux_found" = true ]; then
        echo ""
        echo "To enable Linux cross-compilation:"
        echo "  1. Install cross: cargo install cross"
        echo "  2. Install Docker"
        echo "  3. Use: cross build --target x86_64-unknown-linux-musl --release"
    fi
    
    # Create a simple test report
    echo -e "\n${BLUE}=== Creating Test Report ===${NC}"
    
    cat > cross_compile_report.txt << EOF
Cross-Compilation Test Report
Generated: $(date)
System: $(uname -s) $(uname -m)

Results:
EOF
    
    for i in "${!TEST_RESULTS_TARGETS[@]}"; do
        echo "  - ${TEST_RESULTS_TARGETS[$i]}: ${TEST_RESULTS_STATUS[$i]}" >> cross_compile_report.txt
    done
    
    echo -e "${GREEN}Report saved to: cross_compile_report.txt${NC}"
    
    echo -e "\n${GREEN}Test suite completed!${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Cargo.toml not found${NC}"
    echo "Please run this script from the project root directory"
    exit 1
fi

# Run the tests
main