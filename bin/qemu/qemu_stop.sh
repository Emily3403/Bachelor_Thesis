#!/bin/bash

SLEEP_TIME=0.1

qemu_running=$(pgrep -af "qemu-system-aar" | grep "\-s -S")
gdb_running=$(pgrep gdb)

while [ -z "$qemu_running" ]
do
#  printf "\rwaiting for qemu to start ... "

  qemu_running=$(pgrep -af "qemu-system-aar" | grep "\-s -S")
  gdb_running=$(pgrep gdb)
  sleep $SLEEP_TIME
done

while [ ! -z "$qemu_running" -a -z "$gdb_running" ]
do
#  printf "\rwaiting for gdb to connect ..."

  qemu_running=$(pgrep -af "qemu-system-aar" | grep "\-s -S")
  gdb_running=$(pgrep gdb)
  sleep $SLEEP_TIME
done

while [ ! -z "$qemu_running" -a ! -z "$gdb_running" ]
do
#  printf "\rwaiting for gdb to stop ...   "

  qemu_running=$(pgrep -af "qemu-system-aar" | grep "\-s -S")
  gdb_running=$(pgrep gdb)
  sleep $SLEEP_TIME
done


echo -e "\nKilling QEMU!\n"


pkill -9 qemu-system-aar