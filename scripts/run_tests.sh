#!/bin/bash
# Linux/macOS测试运行脚本

set -e  # 遇到错误时退出

# 默认参数
UNIT=false
INTEGRATION=false
BENCH=false
COVERAGE=false
ALL=false
VERBOSE=false
RELEASE=false
FILTER=""

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# 输出函数
write_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

write_error() {
    echo -e "${RED}✗ $1${NC}"
}

write_info() {
    echo -e "${CYAN}ℹ $1${NC}"
}

write_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

write_header() {
    echo -e "${MAGENTA}$1${NC}"
}

# 显示帮助信息
show_help() {
    write_header "ZRDDS Rust 测试运行脚本"
    echo ""
    echo "用法: ./run_tests.sh [选项]"
    echo ""
    echo "选项:"
    echo "  -u, --unit          运行单元测试"
    echo "  -i, --integration   运行集成测试"
    echo "  -b, --bench         运行基准测试"
    echo "  -c, --coverage      生成代码覆盖率报告"
    echo "  -a, --all           运行所有测试"
    echo "  -v, --verbose       详细输出"
    echo "  -r, --release       使用release模式"
    echo "  -f, --filter NAME   过滤测试（按名称）"
    echo "  -h, --help          显示帮助信息"
    echo ""
    echo "示例:"
    echo "  ./run_tests.sh -u -v"
    echo "  ./run_tests.sh -a -c"
    echo "  ./run_tests.sh -i -f 'dds'"
}

# 解析命令行参数
while [[ $# -gt 0 ]]; do
    case $1 in
        -u|--unit)
            UNIT=true
            shift
            ;;
        -i|--integration)
            INTEGRATION=true
            shift
            ;;
        -b|--bench)
            BENCH=true
            shift
            ;;
        -c|--coverage)
            COVERAGE=true
            shift
            ;;
        -a|--all)
            ALL=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -r|--release)
            RELEASE=true
            shift
            ;;
        -f|--filter)
            FILTER="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "未知选项: $1"
            show_help
            exit 1
            ;;
    esac
done

# 检查Rust工具链
test_rust_toolchain() {
    write_info "检查Rust工具链..."
    
    if ! command -v cargo &> /dev/null; then
        write_error "未找到Rust工具链，请先安装Rust"
        exit 1
    fi
    
    local rust_version=$(cargo --version)
    write_success "Rust工具链已安装: $rust_version"
}

# 构建项目
build_project() {
    write_info "构建项目..."
    
    local build_args=("build")
    if [[ "$VERBOSE" == "true" ]]; then
        build_args+=("--verbose")
    fi
    if [[ "$RELEASE" == "true" ]]; then
        build_args+=("--release")
    fi
    
    if cargo "${build_args[@]}"; then
        write_success "项目构建成功"
    else
        write_error "项目构建失败"
        exit 1
    fi
}

# 运行单元测试
run_unit_tests() {
    write_info "运行单元测试..."
    
    local test_args=("test" "--lib")
    if [[ "$VERBOSE" == "true" ]]; then
        test_args+=("--verbose")
    fi
    if [[ "$RELEASE" == "true" ]]; then
        test_args+=("--release")
    fi
    if [[ -n "$FILTER" ]]; then
        test_args+=("$FILTER")
    fi
    
    if cargo "${test_args[@]}"; then
        write_success "单元测试通过"
        return 0
    else
        write_error "单元测试失败"
        return 1
    fi
}

# 运行集成测试
run_integration_tests() {
    write_info "运行集成测试..."
    
    local test_args=("test" "--test" "*")
    if [[ "$VERBOSE" == "true" ]]; then
        test_args+=("--verbose")
    fi
    if [[ "$RELEASE" == "true" ]]; then
        test_args+=("--release")
    fi
    if [[ -n "$FILTER" ]]; then
        test_args+=("$FILTER")
    fi
    
    if cargo "${test_args[@]}"; then
        write_success "集成测试通过"
        return 0
    else
        write_error "集成测试失败"
        return 1
    fi
}

# 运行基准测试
run_benchmarks() {
    write_info "运行基准测试..."
    
    if cargo bench --bench performance_benchmarks; then
        write_success "基准测试完成"
        return 0
    else
        write_error "基准测试失败"
        return 1
    fi
}

# 生成代码覆盖率报告
generate_coverage() {
    write_info "生成代码覆盖率报告..."
    
    # 检查是否安装了cargo-llvm-cov
    if ! command -v cargo-llvm-cov &> /dev/null; then
        write_warning "未安装cargo-llvm-cov，正在安装..."
        if cargo install cargo-llvm-cov; then
            write_success "cargo-llvm-cov安装成功"
        else
            write_error "cargo-llvm-cov安装失败"
            return 1
        fi
    fi
    
    if cargo llvm-cov --all-features --workspace --html; then
        write_success "代码覆盖率报告已生成: target/llvm-cov/html/index.html"
        return 0
    else
        write_error "代码覆盖率报告生成失败"
        return 1
    fi
}

# 运行代码质量检查
run_quality_checks() {
    write_info "运行代码质量检查..."
    
    # 格式检查
    write_info "检查代码格式..."
    if cargo fmt --all -- --check; then
        write_success "代码格式检查通过"
    else
        write_warning "代码格式不符合标准，运行 'cargo fmt' 进行格式化"
    fi
    
    # Clippy检查
    write_info "运行Clippy检查..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        write_success "Clippy检查通过"
        return 0
    else
        write_error "Clippy检查发现问题"
        return 1
    fi
}

# 主函数
main() {
    write_header "=== ZRDDS Rust 测试套件 ==="
    write_info "开始时间: $(date)"
    
    # 检查工具链
    test_rust_toolchain
    
    # 构建项目
    build_project
    
    # 运行代码质量检查
    if ! run_quality_checks; then
        write_error "代码质量检查失败"
        exit 1
    fi
    
    local all_passed=true
    
    # 根据参数运行相应的测试
    if [[ "$ALL" == "true" || "$UNIT" == "true" ]]; then
        if ! run_unit_tests; then
            all_passed=false
        fi
    fi
    
    if [[ "$ALL" == "true" || "$INTEGRATION" == "true" ]]; then
        if ! run_integration_tests; then
            all_passed=false
        fi
    fi
    
    if [[ "$ALL" == "true" || "$BENCH" == "true" ]]; then
        if ! run_benchmarks; then
            all_passed=false
        fi
    fi
    
    if [[ "$COVERAGE" == "true" ]]; then
        if ! generate_coverage; then
            all_passed=false
        fi
    fi
    
    # 如果没有指定特定的测试类型，默认运行单元测试和集成测试
    if [[ "$UNIT" == "false" && "$INTEGRATION" == "false" && "$BENCH" == "false" && "$COVERAGE" == "false" && "$ALL" == "false" ]]; then
        write_info "未指定测试类型，运行默认测试套件（单元测试 + 集成测试）"
        if ! run_unit_tests; then
            all_passed=false
        fi
        if ! run_integration_tests; then
            all_passed=false
        fi
    fi
    
    # 输出结果
    echo ""
    write_header "=== 测试结果 ==="
    write_info "结束时间: $(date)"
    
    if [[ "$all_passed" == "true" ]]; then
        write_success "所有测试通过！"
        exit 0
    else
        write_error "部分测试失败！"
        exit 1
    fi
}

# 运行主函数
main