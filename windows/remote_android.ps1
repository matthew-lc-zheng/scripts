# require adb and scrcpy installed in environment path
if ($args.Count -eq 0) {
    # used for default adb connection with usb usually
    scrcpy
  } else {
      foreach ($ip in $args){
          Write-Host "Specified address: $ip "
          adb connect ${ip}:5555
          scrcpy -s ${ip}:5555
      }
  }