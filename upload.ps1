# Xa缓存上传脚本
# 计算MD5 → 生成version.json → 推送到Gitee仓库
param(
    [string]$RepoUrl = "https://gitee.com/dog176/xaupload.git",
    [string]$ZipFile = "Xa缓存.zip",
    [string]$Branch = "master"
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ZipPath = Join-Path $ScriptDir $ZipFile

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Xa缓存 上传工具" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 1. 检查zip文件
if (-not (Test-Path $ZipPath)) {
    Write-Host "[错误] 未找到 $ZipFile" -ForegroundColor Red
    Write-Host "请将 $ZipFile 放在脚本同目录" -ForegroundColor Yellow
    pause
    exit 1
}

Write-Host "[1/5] 计算文件MD5..." -ForegroundColor Green
$fileBytes = [System.IO.File]::ReadAllBytes($ZipPath)
$md5 = [System.Security.Cryptography.MD5]::Create().ComputeHash($fileBytes)
$md5Str = [BitConverter]::ToString($md5).Replace("-", "").ToLower()
$fileSize = (Get-Item $ZipPath).Length
$fileSizeKB = [math]::Round($fileSize / 1024, 1)
$fileSizeMB = [math]::Round($fileSize / 1048576, 2)
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

Write-Host "  文件: $ZipFile" -ForegroundColor White
Write-Host "  大小: ${fileSizeMB}MB (${fileSizeKB}KB)" -ForegroundColor White
Write-Host "  MD5 : $md5Str" -ForegroundColor White
Write-Host "  时间: $timestamp" -ForegroundColor White

# 2. 生成version.json
Write-Host "[2/5] 生成 version.json..." -ForegroundColor Green
$versionJson = @{
    md5 = $md5Str
    size = $fileSize
    timestamp = $timestamp
    filename = $ZipFile
} | ConvertTo-Json -Compress

$versionPath = Join-Path $ScriptDir "version.json"
$versionJson | Set-Content -Path $versionPath -Encoding UTF8
Write-Host "  已生成 version.json" -ForegroundColor White

# 3. 克隆/更新仓库
Write-Host "[3/5] 连接仓库..." -ForegroundColor Green
$tempDir = Join-Path $env:TEMP "xaupload_repo"

if (Test-Path $tempDir) {
    Remove-Item -Recurse -Force $tempDir | Out-Null
}

try {
    git clone --depth 1 --branch $Branch $RepoUrl $tempDir 2>&1 | Out-Null
    Write-Host "  仓库已克隆" -ForegroundColor White
} catch {
    Write-Host "[错误] 克隆仓库失败，请检查网络和仓库地址" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    pause
    exit 1
}

# 4. 复制文件并推送
Write-Host "[4/5] 推送文件到仓库..." -ForegroundColor Green

Copy-Item -Path $ZipPath -Destination $tempDir -Force
Copy-Item -Path $versionPath -Destination $tempDir -Force

Push-Location $tempDir
try {
    git add "$ZipFile" "version.json"
    git commit -m "Update Xa缓存 $timestamp | MD5: $md5Str" 2>&1 | Out-Null
    git push origin $Branch 2>&1 | Out-Null
    Write-Host "  推送成功!" -ForegroundColor Green
} catch {
    Write-Host "[错误] 推送失败" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    Pop-Location
    pause
    exit 1
}
Pop-Location

# 5. 清理
Write-Host "[5/5] 清理..." -ForegroundColor Green
Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
Remove-Item -Path $versionPath -ErrorAction SilentlyContinue
Write-Host "  临时文件已清理" -ForegroundColor White

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  上传完成!" -ForegroundColor Green
Write-Host "  MD5: $md5Str" -ForegroundColor White
Write-Host "  大小: ${fileSizeMB}MB" -ForegroundColor White
Write-Host "========================================" -ForegroundColor Cyan
pause
