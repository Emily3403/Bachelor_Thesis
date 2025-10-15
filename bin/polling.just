# Builds the Kernel Module for polling
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
polling-module:
    ./bin/kernel/setup.sh
    sed -i 's|# CONFIG_UIO_POLLING_UART is not set|CONFIG_UIO_POLLING_UART=m|'  "${KBUILD_OUTPUT}/.config"

    MODULE_PREFIX=polling MODULE_NAME=uio_uart.ko ./bin/kernel/module.sh

# Builds an runs `listen` with polling
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
polling-listen: polling-module
    BINARY_NAME=listen FEATURES=driver_polling,io_data ./deploy.sh

# Builds an runs `send` with polling
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
polling-send: polling-module
    BINARY_NAME=polling-send FEATURES=driver_polling,io_data ./deploy.sh
