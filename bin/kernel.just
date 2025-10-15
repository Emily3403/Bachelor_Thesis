# Cleans the entire Repo
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
kernel-clean:
    make mrproper

# Clones the kernel if not present and resets the kernel .config to the default
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
kernel-setup:
    ./bin/kernel/setup.sh

# Builds the Kernel
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
kernel-build: kernel-setup
    ./bin/kernel/build.sh

# Deploys the Kernel to the Raspi. Reboot the Raspi to apply the changes
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
kernel-deploy: kernel-build
    ./bin/kernel/deploy.sh

# Runs `listen` with the kernel driver (b2)
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
kernel-listen:
    rsync -a "uart-driver/kernel-driver/" $RASPI_STOCK_KERNEL_HOST:kernel-driver/ --exclude=venv
    ssh -t $RASPI_STOCK_KERNEL_HOST "./kernel-driver/listen.py"

# Runs `send` with the kernel driver (b2)
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
kernel-send:
    rsync -a "uart-driver/kernel-driver/" $RASPI_STOCK_KERNEL_HOST:kernel-driver/ --exclude=venv
    ssh $RASPI_STOCK_KERNEL_HOST "cd kernel-driver; ./send.py"
