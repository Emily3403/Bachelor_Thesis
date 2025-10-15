# Builds the Kernel Module for disable interrupts
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
irq-dint-module:
    ./bin/kernel/setup.sh
    sed -i "s|# CONFIG_UIO_IRQ_UART_DINT is not set|CONFIG_UIO_IRQ_UART_DINT=m|" "${KBUILD_OUTPUT}/.config"

    MODULE_PREFIX=irq MODULE_NAME=uart_dint.ko ./bin/kernel/module.sh

# Builds the Kernel Module for scratch register
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
irq-scratch-module:
    ./bin/kernel/setup.sh
    sed -i "s|# CONFIG_UIO_IRQ_UART_SCRATCH is not set|CONFIG_UIO_IRQ_UART_SCRATCH=m|" "${KBUILD_OUTPUT}/.config"

    MODULE_PREFIX=irq MODULE_NAME=uart_scratch.ko ./bin/kernel/module.sh

# Builds an runs `listen` with disable interrupts
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-dint-listen: irq-dint-module
    BINARY_NAME=listen FEATURES=driver_irq,io_data REALTIME={{ realtime }} ./deploy.sh

# Builds an runs `send` with disable interrupts
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-dint-send: irq-dint-module
    BINARY_NAME=send FEATURES=driver_irq,io_data REALTIME={{ realtime }} ./deploy.sh

# Builds an runs `listen` with scratch register
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-scratch-listen: irq-scratch-module
    BINARY_NAME=listen FEATURES=driver_irq,io_scratch REALTIME={{ realtime }} ./deploy.sh

# Builds an runs `send` with scratch register
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-scratch-send: irq-scratch-module
    BINARY_NAME=send FEATURES=driver_irq,io_scratch REALTIME={{ realtime }} ./deploy.sh
