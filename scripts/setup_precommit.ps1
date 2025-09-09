Write-Output "Checking if pre-commit is installed..."

$precommit = Get-Command pre-commit -ErrorAction SilentlyContinue

if (-not $precommit) {
    Write-Output "pre-commit not found, installing..."
    try {
        # Try pipx first
        if (Get-Command pipx -ErrorAction SilentlyContinue) {
            pipx install pre-commit
        } else {
            pip install --user pre-commit

            # Find Python user script directories and add to PATH if needed
            $pythonScriptsDirs = Get-ChildItem -Path (Join-Path $env:USERPROFILE "AppData\Roaming\Python") -Directory -ErrorAction SilentlyContinue |
                                 ForEach-Object { Join-Path $_.FullName "Scripts" }

            foreach ($dir in $pythonScriptsDirs) {
                if (Test-Path $dir) {
                    if (-not ($env:Path -split ";" | Where-Object { $_ -eq $dir })) {
                        Write-Output "Adding $dir to PATH"
                        $env:Path += ";$dir"
                    }
                }
            }
        }
    } catch {
        Write-Error "Failed to install pre-commit. Please run: pip install pre-commit"
        exit 1
    }
} else {
    Write-Output "pre-commit is already installed."
}

Write-Output "Installing git hooks..."
pre-commit install

Write-Output "pre-commit has been successfully installed and enabled."
