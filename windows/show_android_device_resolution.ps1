if ($args.Count -eq 0) {
  adb shell wm size
} else {
    foreach ($dev in $args){
        Write-Host "Specified device: $dev "
        adb -s $dev shell wm size
    }
}