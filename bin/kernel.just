# Cleans the entire Repo of Artifacts.
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
clean:
    make mrproper

# Clones the kernel if not present and resets the kernel .config to the default.
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
setup:
    ./bin/kernel/setup.sh

# Build the Kernel
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
build: setup
    ./bin/kernel/build.sh

# Deploys the Kernel to the Raspi. Reboot the Raspi to apply the changes.
[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
deploy: build
    ./bin/kernel/deploy.sh


[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
listen:
    rsync -a "uart-driver/kernel-driver/" $RASPI_STOCK_KERNEL_HOST:kernel-driver/ --exclude=venv
    ssh -t $RASPI_STOCK_KERNEL_HOST "./kernel-driver/listen.py"

[group("Kernel")]
[working-directory("/home/emily/Documents/Uni/Study/Bachelor-Thesis")]
send:
    rsync -a "uart-driver/kernel-driver/" $RASPI_STOCK_KERNEL_HOST:kernel-driver/ --exclude=venv
    ssh $RASPI_STOCK_KERNEL_HOST "cd kernel-driver; ./send.py"
