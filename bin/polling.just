# Builds the Kernel Module for polling
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
polling-module:
    MODULE_PREFIX=polling MODULE_NAME=uio_uart.ko ./bin/kernel/module.sh

# Builds an runs `listen` with polling
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
polling-listen: polling-module
    BINARY_NAME=listen FEATURES=driver_polling,io_data \
      BAUDRATE="{{ baudrate }}" SAVEDIR="{{ save-dir }}" REALTIME="{{ realtime }}" \
      ./deploy.sh

# Builds an runs `send` with polling
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
polling-send: polling-module
    BINARY_NAME=polling-send FEATURES=driver_polling,io_data \
      BAUDRATE="{{ baudrate }}" SAVEDIR="{{ save-dir }}" REALTIME="{{ realtime }}" \
      ./deploy.sh
