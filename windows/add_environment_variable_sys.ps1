param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$EnvironmentVariables
)

if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "Require administrator's privilege."
    exit 1
}

foreach ($var in $EnvironmentVariables) {
    if (-not $var.Contains(":")) {
        Write-Error "Wrong format, $var should be like name:value."
        continue
    }

    $varName,$varValue = $var.Split(":", 2)
    [Environment]::SetEnvironmentVariable($varName,$varValue, [EnvironmentVariableTarget]::Machine)
    Write-Host "$varName added to system successfully."
}
