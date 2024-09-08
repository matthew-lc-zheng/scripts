if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "Require administrator's privilege."
    exit 1
}

for ($i = 0; $i -lt $args.Count; $i++) {
    $env:Path += ";$($args[$i])"
}
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)
