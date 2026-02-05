# Builds the Kernel Module for disable interrupts
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
irq-dint-module:
    MODULE_PREFIX=irq MODULE_NAME=uart_dint.ko ./bin/kernel/module.sh

# Builds the Kernel Module for scratch register
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
irq-scratch-module:
    MODULE_PREFIX=irq MODULE_NAME=uart_scratch.ko ./bin/kernel/module.sh

# Builds an runs `listen` with disable interrupts
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-dint-listen: irq-dint-module
    BINARY_NAME=listen FEATURES=driver_irq,io_data \
      BAUDRATE="{{ baudrate }}" SAVEDIR="{{ save-dir }}" REALTIME="{{ realtime }}" NUM_DATA_BYTES="{{ num-data-bytes }}" \
      ./deploy.sh

# Builds an runs `send` with disable interrupts
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-dint-send: irq-dint-module
    BINARY_NAME=send FEATURES=driver_irq,io_data \
      BAUDRATE="{{ baudrate }}" SAVEDIR="{{ save-dir }}" REALTIME="{{ realtime }}" NUM_DATA_BYTES="{{ num-data-bytes }}" \
      ./deploy.sh

# Builds an runs `listen` with scratch register
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-scratch-listen: irq-scratch-module
    BINARY_NAME=listen FEATURES=driver_irq,io_scratch \
      BAUDRATE="{{ baudrate }}" SAVEDIR="{{ save-dir }}" REALTIME="{{ realtime }}" NUM_DATA_BYTES="{{ num-data-bytes }}" \
      ./deploy.sh

# Builds an runs `send` with scratch register
[group("IRQ")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis/uart-driver")]
irq-scratch-send: irq-scratch-module
    BINARY_NAME=send FEATURES=driver_irq,io_scratch \
      BAUDRATE="{{ baudrate }}" SAVEDIR="{{ save-dir }}" REALTIME="{{ realtime }}" NUM_DATA_BYTES="{{ num-data-bytes }}" \
      ./deploy.sh
