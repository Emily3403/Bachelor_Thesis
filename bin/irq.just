[group("Polling")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
dint-module:
    ./bin/kernel/setup.sh
    sed -i "s|# CONFIG_UIO_IRQ_UART_DINT is not set|CONFIG_UIO_IRQ_UART_DINT=m|" "${KBUILD_OUTPUT}/.config"

    MODULE_PREFIX=irq MODULE_NAME=uart_dint.ko ./bin/kernel/module.sh

[group("Polling")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
scratch-module:
    ./bin/kernel/setup.sh
    sed -i "s|# CONFIG_UIO_IRQ_UART_SCRATCH is not set|CONFIG_UIO_IRQ_UART_SCRATCH=m|" "${KBUILD_OUTPUT}/.config"

    MODULE_PREFIX=irq MODULE_NAME=uart_scratch.ko ./bin/kernel/module.sh


# Build, deploys and executes the Rust Userspace Driver for listening on the Raspi.
[group("Polling")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
dint-listen: dint-module
    BINARY_NAME=listen FEATURES=driver_irq,io_data ./deploy.sh

# Build, deploys and executes the Rust Userspace Driver for sending on the Raspi.
[group("Polling")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
dint-send: dint-module
    BINARY_NAME=send FEATURES=driver_irq,io_data ./deploy.sh


# Build, deploys and executes the Rust Userspace Driver for listening on the Raspi.
[group("Polling")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
scratch-listen: scratch-module
    BINARY_NAME=listen FEATURES=driver_irq,io_scratch ./deploy.sh

# Build, deploys and executes the Rust Userspace Driver for sending on the Raspi.
[group("Polling")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
scratch-send: scratch-module
    BINARY_NAME=send FEATURES=driver_irq,io_scratch ./deploy.sh
