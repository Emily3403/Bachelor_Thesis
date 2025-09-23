# 1. First Boot on a new raspi
- Configure rootfs with `rpi-imager`, load to SD Card
- mount `/dev/sda1`, edit `config.txt`
  - add `enable_uart=1` to the config
  - `umount` and boot it up

# 2. Configure SSH Access
- Add MAC Adress to tuport
- Add id_user to emily and root account
- run `setup.sh` on raspi

# 3. Boot the custom kernel
- `just kernel`
- `reboot`

# 4. Get Serial Output to main
- Maybe: `raspi-config` → `3` → `serial` → `yes`
- copy `/boot/firmware/config.txt`
- In `/boot/firmware/cmdline.txt` replace console to `ttyAMA2`; BUT LEAVE tty1!!
- ?? `apt install rsyslog` ??
- In `/etc/sysctl.d/98-rpi.conf` change to `kernel.printk = 7 7 7 7`

# 5. Working with the Serial output
- Log in and `cat /proc/kmsg`
