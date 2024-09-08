param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$EnvironmentVariables
)

foreach ($var in $EnvironmentVariables) {
    if (-not $var.Contains(":")) {
        Write-Error "Wrong format, $var should be like name:value."
        continue
    }

    $varName,$varValue = $var.Split(":", 2)
    [Environment]::SetEnvironmentVariable($varName,$varValue, [EnvironmentVariableTarget]::User)
    Write-Host "$varName added successfully."
}
