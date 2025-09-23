[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
module:
    ./bin/kernel/setup.sh
    sed -i 's|# CONFIG_UIO_POLLING_UART is not set|CONFIG_UIO_POLLING_UART=m|'  "${KBUILD_OUTPUT}/.config"

    MODULE_PREFIX=polling MODULE_NAME=uio_uart.ko ./bin/kernel/module.sh

# Build, deploys and executes the Rust Userspace Driver for listening on the Raspi.
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
listen: module
    BINARY_NAME=listen FEATURES=driver_polling,io_data ./deploy.sh

# Build, deploys and executes the Rust Userspace Driver for sending on the Raspi.
[group('Polling')]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
send: module
    BINARY_NAME=polling-send FEATURES=driver_polling,io_data ./deploy.sh
