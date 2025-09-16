#!/usr/bin/env pwsh
# PowerShell测试运行脚本

param(
    [switch]$Unit,
    [switch]$Integration,
    [switch]$Bench,
    [switch]$Coverage,
    [switch]$All,
    [switch]$Verbose,
    [switch]$Release,
    [string]$Filter = ""
)

# 设置错误处理
$ErrorActionPreference = "Stop"

# 颜色输出函数
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Write-Success {
    param([string]$Message)
    Write-ColorOutput "✓ $Message" "Green"
}

function Write-Error {
    param([string]$Message)
    Write-ColorOutput "✗ $Message" "Red"
}

function Write-Info {
    param([string]$Message)
    Write-ColorOutput "ℹ $Message" "Cyan"
}

function Write-Warning {
    param([string]$Message)
    Write-ColorOutput "⚠ $Message" "Yellow"
}

# 检查Rust工具链
function Test-RustToolchain {
    Write-Info "检查Rust工具链..."
    
    try {
        $rustVersion = cargo --version
        Write-Success "Rust工具链已安装: $rustVersion"
    }
    catch {
        Write-Error "未找到Rust工具链，请先安装Rust"
        exit 1
    }
}

# 构建项目
function Build-Project {
    Write-Info "构建项目..."
    
    $buildArgs = @("build")
    if ($Verbose) { $buildArgs += "--verbose" }
    if ($Release) { $buildArgs += "--release" }
    
    try {
        & cargo @buildArgs
        Write-Success "项目构建成功"
    }
    catch {
        Write-Error "项目构建失败"
        exit 1
    }
}

# 运行单元测试
function Run-UnitTests {
    Write-Info "运行单元测试..."
    
    $testArgs = @("test", "--lib")
    if ($Verbose) { $testArgs += "--verbose" }
    if ($Release) { $testArgs += "--release" }
    if ($Filter) { $testArgs += $Filter }
    
    try {
        & cargo @testArgs
        Write-Success "单元测试通过"
    }
    catch {
        Write-Error "单元测试失败"
        return $false
    }
    return $true
}

# 运行集成测试
function Run-IntegrationTests {
    Write-Info "运行集成测试..."
    
    $testArgs = @("test", "--test", "*")
    if ($Verbose) { $testArgs += "--verbose" }
    if ($Release) { $testArgs += "--release" }
    if ($Filter) { $testArgs += $Filter }
    
    try {
        & cargo @testArgs
        Write-Success "集成测试通过"
    }
    catch {
        Write-Error "集成测试失败"
        return $false
    }
    return $true
}

# 运行基准测试
function Run-Benchmarks {
    Write-Info "运行基准测试..."
    
    try {
        cargo bench --bench performance_benchmarks
        Write-Success "基准测试完成"
    }
    catch {
        Write-Error "基准测试失败"
        return $false
    }
    return $true
}

# 生成代码覆盖率报告
function Generate-Coverage {
    Write-Info "生成代码覆盖率报告..."
    
    # 检查是否安装了cargo-llvm-cov
    try {
        cargo llvm-cov --version | Out-Null
    }
    catch {
        Write-Warning "未安装cargo-llvm-cov，正在安装..."
        try {
            cargo install cargo-llvm-cov
            Write-Success "cargo-llvm-cov安装成功"
        }
        catch {
            Write-Error "cargo-llvm-cov安装失败"
            return $false
        }
    }
    
    try {
        cargo llvm-cov --all-features --workspace --html
        Write-Success "代码覆盖率报告已生成: target/llvm-cov/html/index.html"
    }
    catch {
        Write-Error "代码覆盖率报告生成失败"
        return $false
    }
    return $true
}

# 运行代码质量检查
function Run-QualityChecks {
    Write-Info "运行代码质量检查..."
    
    # 格式检查
    Write-Info "检查代码格式..."
    try {
        cargo fmt --all -- --check
        Write-Success "代码格式检查通过"
    }
    catch {
        Write-Warning "代码格式不符合标准，运行 'cargo fmt' 进行格式化"
    }
    
    # Clippy检查
    Write-Info "运行Clippy检查..."
    try {
        cargo clippy --all-targets --all-features -- -D warnings
        Write-Success "Clippy检查通过"
    }
    catch {
        Write-Error "Clippy检查发现问题"
        return $false
    }
    
    return $true
}

# 主函数
function Main {
    Write-ColorOutput "=== ZRDDS Rust 测试套件 ===" "Magenta"
    Write-Info "开始时间: $(Get-Date)"
    
    # 检查工具链
    Test-RustToolchain
    
    # 构建项目
    Build-Project
    
    # 运行代码质量检查
    if (-not (Run-QualityChecks)) {
        Write-Error "代码质量检查失败"
        exit 1
    }
    
    $allPassed = $true
    
    # 根据参数运行相应的测试
    if ($All -or $Unit) {
        if (-not (Run-UnitTests)) {
            $allPassed = $false
        }
    }
    
    if ($All -or $Integration) {
        if (-not (Run-IntegrationTests)) {
            $allPassed = $false
        }
    }
    
    if ($All -or $Bench) {
        if (-not (Run-Benchmarks)) {
            $allPassed = $false
        }
    }
    
    if ($Coverage) {
        if (-not (Generate-Coverage)) {
            $allPassed = $false
        }
    }
    
    # 如果没有指定特定的测试类型，默认运行单元测试和集成测试
    if (-not ($Unit -or $Integration -or $Bench -or $Coverage -or $All)) {
        Write-Info "未指定测试类型，运行默认测试套件（单元测试 + 集成测试）"
        if (-not (Run-UnitTests)) {
            $allPassed = $false
        }
        if (-not (Run-IntegrationTests)) {
            $allPassed = $false
        }
    }
    
    # 输出结果
    Write-ColorOutput "\n=== 测试结果 ===" "Magenta"
    Write-Info "结束时间: $(Get-Date)"
    
    if ($allPassed) {
        Write-Success "所有测试通过！"
        exit 0
    } else {
        Write-Error "部分测试失败！"
        exit 1
    }
}

# 显示帮助信息
function Show-Help {
    Write-ColorOutput "ZRDDS Rust 测试运行脚本" "Magenta"
    Write-Host ""
    Write-Host "用法: .\run_tests.ps1 [选项]"
    Write-Host ""
    Write-Host "选项:"
    Write-Host "  -Unit          运行单元测试"
    Write-Host "  -Integration   运行集成测试"
    Write-Host "  -Bench         运行基准测试"
    Write-Host "  -Coverage      生成代码覆盖率报告"
    Write-Host "  -All           运行所有测试"
    Write-Host "  -Verbose       详细输出"
    Write-Host "  -Release       使用release模式"
    Write-Host "  -Filter <名称> 过滤测试（按名称）"
    Write-Host ""
    Write-Host "示例:"
    Write-Host "  .\run_tests.ps1 -Unit -Verbose"
    Write-Host "  .\run_tests.ps1 -All -Coverage"
    Write-Host "  .\run_tests.ps1 -Integration -Filter 'dds'"
}

# 检查是否请求帮助
if ($args -contains "-h" -or $args -contains "--help" -or $args -contains "-?") {
    Show-Help
    exit 0
}

# 运行主函数
Main