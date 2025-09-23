// SPDX-License-Identifier: GPL-2.0-only
/*
 * drivers/uio/uio_pdrv_genirq.c
 *
 * Userspace I/O Driver for the mini UART of the Raspberry PI 3b+
 *
 * Copyright (C) 2024 Emily Seebeck
 *
 * Based on uio_pdrv_generic.c by Magnus Damm.
 */

#include "../../../include/linux/kern_levels.h"
#include <linux/bitops.h>
#include <linux/interrupt.h>
#include <linux/irq.h>
#include <linux/module.h>
#include <linux/platform_device.h>
#include <linux/pm_runtime.h>
#include <linux/slab.h>
#include <linux/spinlock.h>
#include <linux/stringify.h>
#include <linux/uio_driver.h>

#include <linux/of.h>
#include <linux/of_address.h>
#include <linux/of_platform.h>

#include <linux/init.h>
#include <linux/module.h>

#include "../../../Bachelor-Thesis-Old/include/linux/kern_levels.h"

#define DRIVER_NAME "uio_uart"
#define DRIVER_VERSION "0.0.1"

#define AUX_IRQ 0x00
#define AUX_ENABLES 0x04
#define AUX_MU_IER_REG 0x44
#define AUX_MU_IIR_REG 0x48
#define AUX_MU_LSR_REG 0x54

// This is the private state the driver maintains
struct uio_uart_priv {
    spinlock_t lock;
    u32 enabled_interrupts;
    u32 total_overflows;
};

/* Bits in uio_uart_privdata.flags */
enum {
    IRQ_RX = 0b01,
    IRQ_TX = 0b10,
};

static void enable_interrupts(void *mem, struct uio_uart_priv *priv, bool is_tx) {
    if (is_tx)
        priv->enabled_interrupts |= IRQ_TX;
    else
        priv->enabled_interrupts |= IRQ_RX;

    iowrite8(priv->enabled_interrupts, mem + AUX_MU_IER_REG);
}

static void disable_interrupts(void *mem, struct uio_uart_priv *priv, bool is_tx) {
    if (is_tx)
        priv->enabled_interrupts &= ~IRQ_TX;
    else
        priv->enabled_interrupts &= ~IRQ_RX;

    iowrite8(priv->enabled_interrupts, mem + AUX_MU_IER_REG);
}

static irqreturn_t uio_uart_handler(int irq, struct uio_info *info) {
    struct uio_uart_priv *priv = info->priv;
    void __iomem *mem = info->mem[0].internal_addr;

    unsigned int aux_irq = ioread8(mem + AUX_IRQ);
    if ((aux_irq & 0b001) != 1)
        return IRQ_NONE;

    // Disable the interrupt that just came in (TX or RX)
    unsigned int iir = ioread8(mem + AUX_MU_IIR_REG);

    if (iir & 0b010) {  // TX
        disable_interrupts(mem, priv, true);
    }
    if (iir & 0b100) {  // RX
        disable_interrupts(mem, priv, false);

        unsigned int lsr = ioread8(mem + AUX_MU_LSR_REG);

        if (lsr & 0b10) {
            priv->total_overflows++;
            if (priv->total_overflows % 10 == 0)
                printk(KERN_INFO "ERROR: receiver overflow\n");

            iowrite8(0b11, mem + AUX_MU_IIR_REG);
        }
    }

    return IRQ_HANDLED;
}

static int uio_uart_irqcontrol(struct uio_info *info, s32 irq_on) {
    struct uio_uart_priv *priv = info->priv;
    void __iomem *mem = info->mem[0].internal_addr;

    if (irq_on) {
        enable_interrupts(mem, priv, false);
    }
    else {
        disable_interrupts(mem, priv, false);
        disable_interrupts(mem, priv, true);
    }

    return 0;
}

static int uio_uart_probe(struct platform_device *pdev) {
    struct uio_uart_priv *priv = devm_kzalloc(&pdev->dev, sizeof(struct uio_uart_priv *), GFP_KERNEL);
    struct uio_info *info = devm_kzalloc(&pdev->dev, sizeof(struct uio_info), GFP_KERNEL);

    printk(KERN_ERR "HELP\n");
    if (!priv || !info) {
        dev_err(&pdev->dev, "unable to kmalloc\n");
        return -ENOMEM;
    }

    long irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
        return irq;
    }

    spin_lock_init(&priv->lock);
    priv->enabled_interrupts = 0b01;

    info->priv = priv;
    info->name = DRIVER_NAME;
    info->version = DRIVER_VERSION;
    info->irq = irq;
    info->irq_flags = IRQF_SHARED;
    info->handler = uio_uart_handler;

    info->mmap = NULL;
    info->open = NULL;                       // TODO: Make this a function to enable interrupts for the UART
    info->release = NULL;                    // TODO
    info->irqcontrol = uio_uart_irqcontrol;  // TODO

    BUG_ON(pdev->num_resources == 0);
    WARN_ON(pdev->num_resources > 1);

    struct resource *r = &pdev->resource[0];
    WARN_ON(r->flags != IORESOURCE_MEM);

    struct uio_mem *uiomem = &info->mem[0];
    uiomem->name = r->name;
    uiomem->memtype = UIO_MEM_PHYS;
    uiomem->addr = r->start & PAGE_MASK;
    uiomem->offs = r->start & ~PAGE_MASK;
    uiomem->size = (uiomem->offs + resource_size(r) + PAGE_SIZE - 1) & PAGE_MASK;
    uiomem->internal_addr = ioremap(uiomem->addr, uiomem->size);

    iowrite8(priv->enabled_interrupts, uiomem->internal_addr + AUX_MU_IER_REG);

    int ret = devm_uio_register_device(&pdev->dev, info);
    if (ret)
        printk(KERN_INFO "\n❌ ERROR loading IRQ Module: %d\n\n", ret);
    else
        printk(KERN_INFO "\n✅ Successfully Loaded IRQ Module (disable interrupts)\n\n");

    return ret;
}

#ifdef CONFIG_OF
static struct of_device_id uio_uart_match[] = {
    {.compatible = "uio_uart"},
    {/* Sentinel */},
};

MODULE_DEVICE_TABLE(of, uio_uart_match);
module_param_string(of_id, uio_uart_match[0].compatible, 128, 0);
MODULE_PARM_DESC(of_id, "Openfirmware id of the device to be handled by uio");
#endif

static struct platform_driver uio_uart = {
    .probe = uio_uart_probe,
    .driver =
        {
            .name = DRIVER_NAME,
            .of_match_table = of_match_ptr(uio_uart_match),
        },
};

module_platform_driver(uio_uart);

MODULE_AUTHOR("Emily Seebeck");
MODULE_DESCRIPTION("Userspace I/O platform driver with generic IRQ handling");
MODULE_LICENSE("GPL v2");
MODULE_ALIAS("platform:" DRIVER_NAME);
