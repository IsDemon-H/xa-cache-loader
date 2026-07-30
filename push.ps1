param(
    [string]$msg = "update",
    [switch]$tag
)

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

git add -A

if ($LASTEXITCODE -ne 0) { throw "git add failed" }

git commit -m $msg

if ($LASTEXITCODE -ne 0) {
    Write-Host "Nothing to commit" -ForegroundColor Yellow
} else {
    Write-Host "Committed: $msg" -ForegroundColor Green
}

git pull --rebase
git push

Write-Host "Push done" -ForegroundColor Green

if ($tag) {
    $v = (Get-Date -Format "yyyyMMdd.HHmm")
    $tagName = "v$v"
    git tag $tagName
    git push origin $tagName
    Write-Host "Tagged & pushed: $tagName" -ForegroundColor Cyan
}
