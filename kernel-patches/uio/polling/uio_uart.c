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

#define DRIVER_NAME "uio_uart"
#define DRIVER_VERSION "0.0.1"

// This is the private state the driver maintains
struct uio_uart_priv {
    struct uio_info *info;
    spinlock_t lock;
    unsigned long flags;
    struct platform_device *pdev;
};

/* Bits in uio_uart_privdata.flags */
enum {
    UIO_IRQ_DISABLED = 0,
};

static int uio_uart_probe(struct platform_device *pdev) {
	struct uio_uart_priv *priv = devm_kzalloc(&pdev->dev, sizeof(struct uio_uart_priv *), GFP_KERNEL);
    struct uio_info *info = devm_kzalloc(&pdev->dev, sizeof(struct uio_info), GFP_KERNEL);

	if (!priv || !info) {
        dev_err(&pdev->dev, "unable to kmalloc\n");
        return -ENOMEM;
    }

    priv->info = info;
    spin_lock_init(&priv->lock);  // TODO: Why do we need a spinlock?
    priv->flags = 0;
    priv->pdev = pdev;

    info->name = DRIVER_NAME;
    info->version = DRIVER_VERSION;
    info->irq = 0;

    info->mmap = NULL;
    info->open = NULL;
    info->release = NULL;
    info->irqcontrol = NULL;

    BUG_ON(pdev->num_resources != 1);
    WARN_ON(pdev->num_resources > 1);

    struct resource *r = &pdev->resource[0];
    WARN_ON(r->flags != IORESOURCE_MEM);

    struct uio_mem *uiomem = &info->mem[0];
    uiomem->name = r->name;
    uiomem->memtype = UIO_MEM_PHYS;
    uiomem->addr = r->start & PAGE_MASK;
    uiomem->offs = r->start & ~PAGE_MASK;
    uiomem->size = (uiomem->offs + resource_size(r) + PAGE_SIZE - 1) & PAGE_MASK;

    int ret = devm_uio_register_device(&pdev->dev, priv->info);
    if (ret)
        printk(KERN_INFO "\n❌ ERROR loading polling Module: %d\n\n", ret);
    else
        printk(KERN_INFO "\n✅ Successfully Loaded polling Module\n\n");

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
