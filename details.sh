arm-none-eabi-size $1
arm-none-eabi-size --format=SysV $1
readelf -x .rodata $1
