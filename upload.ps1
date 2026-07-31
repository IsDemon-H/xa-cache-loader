# Xa Cache Upload Script
param(
    [string]$RepoUrl = "https://gitee.com/dog176/xaupload.git",
    [string]$Branch = "master"
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$host.UI.RawUI.WindowTitle = "Xa Upload"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Xa Cache Upload Tool" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check git
$git = Get-Command git -ErrorAction SilentlyContinue
if (-not $git) {
    Write-Host "[ERROR] Git not found. Install Git first." -ForegroundColor Red
    Write-Host "        https://git-scm.com/download/win" -ForegroundColor Yellow
    pause
    exit 1
}

# Auto-detect zip file (avoids encoding issues with Chinese filename)
$zipFiles = Get-ChildItem -Path $ScriptDir -Filter "*.zip" | Where-Object { $_.Name -like "Xa*" -or $_.Name -like "*cache*" }
if ($zipFiles.Count -eq 0) {
    Write-Host "[ERROR] No zip file found in: $ScriptDir" -ForegroundColor Red
    Write-Host "        Expected: Xa*.zip or *cache*.zip" -ForegroundColor Yellow
    pause
    exit 1
}

$ZipPath = $zipFiles[0].FullName
$ZipFile = $zipFiles[0].Name

Write-Host "  Found: $ZipFile"

# ============================================================================
# 1. Calculate MD5
# ============================================================================
Write-Host "[1/5] Calculating MD5..." -ForegroundColor Green
try {
    $fileBytes = [System.IO.File]::ReadAllBytes($ZipPath)
    $md5 = [System.Security.Cryptography.MD5]::Create().ComputeHash($fileBytes)
    $md5Str = [BitConverter]::ToString($md5).Replace("-", "").ToLower()
    $fileSize = (Get-Item $ZipPath).Length
    $fileSizeMB = [math]::Round($fileSize / 1048576, 2)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

    Write-Host "  File: $ZipFile"
    Write-Host "  Size: ${fileSizeMB}MB"
    Write-Host "  MD5 : $md5Str"
    Write-Host "  Time: $timestamp"
} catch {
    Write-Host "[ERROR] MD5 failed: $_" -ForegroundColor Red
    pause
    exit 1
}

# ============================================================================
# 2. Generate version.json
# ============================================================================
Write-Host "[2/5] Generating version.json..." -ForegroundColor Green
$versionJson = @{
    md5       = $md5Str
    size      = $fileSize
    timestamp = $timestamp
    filename  = $ZipFile
} | ConvertTo-Json -Compress

$versionPath = Join-Path $ScriptDir "version.json"
try {
    $versionJson | Set-Content -Path $versionPath -Encoding UTF8
    Write-Host "  version.json generated"
} catch {
    Write-Host "[ERROR] Write version.json failed: $_" -ForegroundColor Red
    pause
    exit 1
}

# ============================================================================
# 3. Clone repo
# ============================================================================
Write-Host "[3/5] Connecting to repo..." -ForegroundColor Green
$tempDir = Join-Path $env:TEMP "xaupload_repo_$(Get-Random)"

if (Test-Path $tempDir) {
    Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue | Out-Null
}

Write-Host "  Repo: $RepoUrl"
Write-Host "  Branch: $Branch"
Write-Host "  Cloning..."

$cloneOutput = git clone --depth 1 --branch $Branch $RepoUrl $tempDir 2>&1
$cloneExit = $LASTEXITCODE

if ($cloneExit -ne 0) {
    Write-Host "========================================" -ForegroundColor Red
    Write-Host "  Git clone FAILED!" -ForegroundColor Red
    Write-Host "  Possible causes:" -ForegroundColor Yellow
    Write-Host "  1. Network unreachable (can't access gitee.com)" -ForegroundColor Yellow
    Write-Host "  2. Git credentials not configured" -ForegroundColor Yellow
    Write-Host "  3. Wrong repo URL or branch name" -ForegroundColor Yellow
    Write-Host "" -ForegroundColor Yellow
    Write-Host "  Try this first:" -ForegroundColor Yellow
    Write-Host "    git ls-remote $RepoUrl" -ForegroundColor Yellow
    Write-Host "  If that fails, configure SSH key or Git Credential Manager." -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Red
    Write-Host ""
    Write-Host "Git output:" -ForegroundColor DarkGray
    Write-Host $cloneOutput
    pause
    exit 1
}
Write-Host "  Clone OK"

# ============================================================================
# 4. Copy and push
# ============================================================================
Write-Host "[4/5] Pushing files..." -ForegroundColor Green

try {
    Copy-Item -Path $ZipPath -Destination $tempDir -Force
    Copy-Item -Path $versionPath -Destination $tempDir -Force
    Write-Host "  Files copied"
} catch {
    Write-Host "[ERROR] Copy failed: $_" -ForegroundColor Red
    Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
    pause
    exit 1
}

Push-Location $tempDir
try {
    Write-Host "  git add..."
    git add "$ZipFile" "version.json" 2>&1 | Out-Null

    Write-Host "  git commit..."
    git commit -m "Update Xa $timestamp | MD5: $md5Str" 2>&1 | Out-Null

    Write-Host "  git push..."
    $pushOutput = git push origin $Branch 2>&1
    $pushExit = $LASTEXITCODE

    if ($pushExit -ne 0) {
        Write-Host "========================================" -ForegroundColor Red
        Write-Host "  Push FAILED!" -ForegroundColor Red
        Write-Host "  Possible causes:" -ForegroundColor Yellow
        Write-Host "  1. No write permission on the repo" -ForegroundColor Yellow
        Write-Host "  2. Authentication required (use SSH or PAT)" -ForegroundColor Yellow
        Write-Host "  3. Branch is protected" -ForegroundColor Yellow
        Write-Host "========================================" -ForegroundColor Red
        Write-Host ""
        Write-Host "Git output:" -ForegroundColor DarkGray
        Write-Host $pushOutput
        Pop-Location
        pause
        exit 1
    }
    Write-Host "  Push OK!" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] Push exception: $_" -ForegroundColor Red
    Pop-Location
    pause
    exit 1
}
Pop-Location

# ============================================================================
# 5. Cleanup
# ============================================================================
Write-Host "[5/5] Cleaning up..." -ForegroundColor Green
Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
Remove-Item -Path $versionPath -ErrorAction SilentlyContinue
Write-Host "  Temp files removed"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Upload Complete!" -ForegroundColor Green
Write-Host "  MD5 : $md5Str"
Write-Host "  Size: ${fileSizeMB}MB"
Write-Host "========================================" -ForegroundColor Cyan
