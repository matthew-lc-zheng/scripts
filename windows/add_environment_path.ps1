for ($i = 0; $i -lt $args.Count; $i++) {
    $env:Path += ";$($args[$i])"
}
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
