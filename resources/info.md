# Raspi Resources
https://www.raspberrypi.com/documentation/computers/configuration.html#configure-uarts


# BA Resources

https://yurovsky.github.io/2014/10/10/linux-uio-gpio-interrupt.html
https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf

# UIO
https://www.kernel.org/doc/html/v4.18/driver-api/uio-howto.html
https://github.com/gz/rust-uio


# Bare-Metal

## Rust
https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials

## C
https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials



# Future Reading
https://dl.acm.org/doi/10.1145/3626780
https://www.youtube.com/watch?v=KUPlrF_rQHw&t=247s


# Future Drivers

#### Network
- Nginx
- Redis
- application that requires a complete Layer 4 protocol stack

#### DPDK with normal UIO Driver vs FIQ UIO driver
- Main consideration why DPDK uses polling: latency (and throughput)
- I could very easily measure the difference between the following implementations
  - polling
  - IRQ UIO (should be an implementation already)
  - FIQ UIO (only my custom kernel module is needed)
  - FIQ direct jump (should be even faster because the scheduler code is never touched, direct branch)
    - Cache Warmth is still an issue

- Issues
  - Direct Jump = Security Violation
  - DPDK wants to provide deterministic packet processing times; the jitter through the interrupt would make a strich through that
  - Modern NICs have mutliple queues for multiple cores. How could FIQ scale?
  - From the paper I know that handling the IRQ is the main overhead – that will still be a thing
