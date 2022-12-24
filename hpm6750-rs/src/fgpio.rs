#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO input value"]
    pub di_gpioa_value: DI_GPIOA_VALUE,
    #[doc = "0x04 - GPIO input set"]
    pub di_gpioa_set: DI_GPIOA_SET,
    #[doc = "0x08 - GPIO input clear"]
    pub di_gpioa_clear: DI_GPIOA_CLEAR,
    #[doc = "0x0c - GPIO input toggle"]
    pub di_gpioa_toggle: DI_GPIOA_TOGGLE,
    #[doc = "0x10 - GPIOB input"]
    pub di_gpiob_value: DI_GPIOB_VALUE,
    #[doc = "0x14 - GPIO input set"]
    pub di_gpiob_set: DI_GPIOB_SET,
    #[doc = "0x18 - GPIO input clear"]
    pub di_gpiob_clear: DI_GPIOB_CLEAR,
    #[doc = "0x1c - GPIO input toggle"]
    pub di_gpiob_toggle: DI_GPIOB_TOGGLE,
    #[doc = "0x20 - GPIOC input"]
    pub di_gpioc_value: DI_GPIOC_VALUE,
    #[doc = "0x24 - GPIO input set"]
    pub di_gpioc_set: DI_GPIOC_SET,
    #[doc = "0x28 - GPIO input clear"]
    pub di_gpioc_clear: DI_GPIOC_CLEAR,
    #[doc = "0x2c - GPIO input toggle"]
    pub di_gpioc_toggle: DI_GPIOC_TOGGLE,
    #[doc = "0x30 - GPIOD input"]
    pub di_gpiod_value: DI_GPIOD_VALUE,
    #[doc = "0x34 - GPIO input set"]
    pub di_gpiod_set: DI_GPIOD_SET,
    #[doc = "0x38 - GPIO input clear"]
    pub di_gpiod_clear: DI_GPIOD_CLEAR,
    #[doc = "0x3c - GPIO input toggle"]
    pub di_gpiod_toggle: DI_GPIOD_TOGGLE,
    #[doc = "0x40 - GPIOE input"]
    pub di_gpioe_value: DI_GPIOE_VALUE,
    #[doc = "0x44 - GPIO input set"]
    pub di_gpioe_set: DI_GPIOE_SET,
    #[doc = "0x48 - GPIO input clear"]
    pub di_gpioe_clear: DI_GPIOE_CLEAR,
    #[doc = "0x4c - GPIO input toggle"]
    pub di_gpioe_toggle: DI_GPIOE_TOGGLE,
    #[doc = "0x50 - GPIOF input"]
    pub di_gpiof_value: DI_GPIOF_VALUE,
    #[doc = "0x54 - GPIO input set"]
    pub di_gpiof_set: DI_GPIOF_SET,
    #[doc = "0x58 - GPIO input clear"]
    pub di_gpiof_clear: DI_GPIOF_CLEAR,
    #[doc = "0x5c - GPIO input toggle"]
    pub di_gpiof_toggle: DI_GPIOF_TOGGLE,
    _reserved24: [u8; 0x70],
    #[doc = "0xd0 - GPIOX input"]
    pub di_gpiox_value: DI_GPIOX_VALUE,
    #[doc = "0xd4 - GPIO input set"]
    pub di_gpiox_set: DI_GPIOX_SET,
    #[doc = "0xd8 - GPIO input clear"]
    pub di_gpiox_clear: DI_GPIOX_CLEAR,
    #[doc = "0xdc - GPIO input toggle"]
    pub di_gpiox_toggle: DI_GPIOX_TOGGLE,
    #[doc = "0xe0 - GPIOY input"]
    pub di_gpioy_value: DI_GPIOY_VALUE,
    #[doc = "0xe4 - GPIO input set"]
    pub di_gpioy_set: DI_GPIOY_SET,
    #[doc = "0xe8 - GPIO input clear"]
    pub di_gpioy_clear: DI_GPIOY_CLEAR,
    #[doc = "0xec - GPIO input toggle"]
    pub di_gpioy_toggle: DI_GPIOY_TOGGLE,
    #[doc = "0xf0 - GPIOZ input"]
    pub di_gpioz_value: DI_GPIOZ_VALUE,
    #[doc = "0xf4 - GPIO input set"]
    pub di_gpioz_set: DI_GPIOZ_SET,
    #[doc = "0xf8 - GPIO input clear"]
    pub di_gpioz_clear: DI_GPIOZ_CLEAR,
    #[doc = "0xfc - GPIO input toggle"]
    pub di_gpioz_toggle: DI_GPIOZ_TOGGLE,
    #[doc = "0x100 - GPIO output value"]
    pub do_gpioa_value: DO_GPIOA_VALUE,
    #[doc = "0x104 - GPIO output set"]
    pub do_gpioa_set: DO_GPIOA_SET,
    #[doc = "0x108 - GPIO output clear"]
    pub do_gpioa_clear: DO_GPIOA_CLEAR,
    #[doc = "0x10c - GPIO output toggle"]
    pub do_gpioa_toggle: DO_GPIOA_TOGGLE,
    #[doc = "0x110 - GPIOB output"]
    pub do_gpiob_value: DO_GPIOB_VALUE,
    #[doc = "0x114 - GPIO output set"]
    pub do_gpiob_set: DO_GPIOB_SET,
    #[doc = "0x118 - GPIO output clear"]
    pub do_gpiob_clear: DO_GPIOB_CLEAR,
    #[doc = "0x11c - GPIO output toggle"]
    pub do_gpiob_toggle: DO_GPIOB_TOGGLE,
    #[doc = "0x120 - GPIOC output"]
    pub do_gpioc_value: DO_GPIOC_VALUE,
    #[doc = "0x124 - GPIO output set"]
    pub do_gpioc_set: DO_GPIOC_SET,
    #[doc = "0x128 - GPIO output clear"]
    pub do_gpioc_clear: DO_GPIOC_CLEAR,
    #[doc = "0x12c - GPIO output toggle"]
    pub do_gpioc_toggle: DO_GPIOC_TOGGLE,
    #[doc = "0x130 - GPIOD output"]
    pub do_gpiod_value: DO_GPIOD_VALUE,
    #[doc = "0x134 - GPIO output set"]
    pub do_gpiod_set: DO_GPIOD_SET,
    #[doc = "0x138 - GPIO output clear"]
    pub do_gpiod_clear: DO_GPIOD_CLEAR,
    #[doc = "0x13c - GPIO output toggle"]
    pub do_gpiod_toggle: DO_GPIOD_TOGGLE,
    #[doc = "0x140 - GPIOE output"]
    pub do_gpioe_value: DO_GPIOE_VALUE,
    #[doc = "0x144 - GPIO output set"]
    pub do_gpioe_set: DO_GPIOE_SET,
    #[doc = "0x148 - GPIO output clear"]
    pub do_gpioe_clear: DO_GPIOE_CLEAR,
    #[doc = "0x14c - GPIO output toggle"]
    pub do_gpioe_toggle: DO_GPIOE_TOGGLE,
    #[doc = "0x150 - GPIOF output"]
    pub do_gpiof_value: DO_GPIOF_VALUE,
    #[doc = "0x154 - GPIO output set"]
    pub do_gpiof_set: DO_GPIOF_SET,
    #[doc = "0x158 - GPIO output clear"]
    pub do_gpiof_clear: DO_GPIOF_CLEAR,
    #[doc = "0x15c - GPIO output toggle"]
    pub do_gpiof_toggle: DO_GPIOF_TOGGLE,
    _reserved60: [u8; 0x70],
    #[doc = "0x1d0 - GPIOX output"]
    pub do_gpiox_value: DO_GPIOX_VALUE,
    #[doc = "0x1d4 - GPIO output set"]
    pub do_gpiox_set: DO_GPIOX_SET,
    #[doc = "0x1d8 - GPIO output clear"]
    pub do_gpiox_clear: DO_GPIOX_CLEAR,
    #[doc = "0x1dc - GPIO output toggle"]
    pub do_gpiox_toggle: DO_GPIOX_TOGGLE,
    #[doc = "0x1e0 - GPIOY output"]
    pub do_gpioy_value: DO_GPIOY_VALUE,
    #[doc = "0x1e4 - GPIO output set"]
    pub do_gpioy_set: DO_GPIOY_SET,
    #[doc = "0x1e8 - GPIO output clear"]
    pub do_gpioy_clear: DO_GPIOY_CLEAR,
    #[doc = "0x1ec - GPIO output toggle"]
    pub do_gpioy_toggle: DO_GPIOY_TOGGLE,
    #[doc = "0x1f0 - GPIOZ output"]
    pub do_gpioz_value: DO_GPIOZ_VALUE,
    #[doc = "0x1f4 - GPIO output set"]
    pub do_gpioz_set: DO_GPIOZ_SET,
    #[doc = "0x1f8 - GPIO output clear"]
    pub do_gpioz_clear: DO_GPIOZ_CLEAR,
    #[doc = "0x1fc - GPIO output toggle"]
    pub do_gpioz_toggle: DO_GPIOZ_TOGGLE,
    #[doc = "0x200 - GPIO direction value"]
    pub oe_gpioa_value: OE_GPIOA_VALUE,
    #[doc = "0x204 - GPIO direction set"]
    pub oe_gpioa_set: OE_GPIOA_SET,
    #[doc = "0x208 - GPIO direction clear"]
    pub oe_gpioa_clear: OE_GPIOA_CLEAR,
    #[doc = "0x20c - GPIO direction toggle"]
    pub oe_gpioa_toggle: OE_GPIOA_TOGGLE,
    #[doc = "0x210 - GPIOB direction"]
    pub oe_gpiob_value: OE_GPIOB_VALUE,
    #[doc = "0x214 - GPIO direction set"]
    pub oe_gpiob_set: OE_GPIOB_SET,
    #[doc = "0x218 - GPIO direction clear"]
    pub oe_gpiob_clear: OE_GPIOB_CLEAR,
    #[doc = "0x21c - GPIO direction toggle"]
    pub oe_gpiob_toggle: OE_GPIOB_TOGGLE,
    #[doc = "0x220 - GPIOC direction"]
    pub oe_gpioc_value: OE_GPIOC_VALUE,
    #[doc = "0x224 - GPIO direction set"]
    pub oe_gpioc_set: OE_GPIOC_SET,
    #[doc = "0x228 - GPIO direction clear"]
    pub oe_gpioc_clear: OE_GPIOC_CLEAR,
    #[doc = "0x22c - GPIO direction toggle"]
    pub oe_gpioc_toggle: OE_GPIOC_TOGGLE,
    #[doc = "0x230 - GPIOD direction"]
    pub oe_gpiod_value: OE_GPIOD_VALUE,
    #[doc = "0x234 - GPIO direction set"]
    pub oe_gpiod_set: OE_GPIOD_SET,
    #[doc = "0x238 - GPIO direction clear"]
    pub oe_gpiod_clear: OE_GPIOD_CLEAR,
    #[doc = "0x23c - GPIO direction toggle"]
    pub oe_gpiod_toggle: OE_GPIOD_TOGGLE,
    #[doc = "0x240 - GPIOE direction"]
    pub oe_gpioe_value: OE_GPIOE_VALUE,
    #[doc = "0x244 - GPIO direction set"]
    pub oe_gpioe_set: OE_GPIOE_SET,
    #[doc = "0x248 - GPIO direction clear"]
    pub oe_gpioe_clear: OE_GPIOE_CLEAR,
    #[doc = "0x24c - GPIO direction toggle"]
    pub oe_gpioe_toggle: OE_GPIOE_TOGGLE,
    #[doc = "0x250 - GPIOF direction"]
    pub oe_gpiof_value: OE_GPIOF_VALUE,
    #[doc = "0x254 - GPIO direction set"]
    pub oe_gpiof_set: OE_GPIOF_SET,
    #[doc = "0x258 - GPIO direction clear"]
    pub oe_gpiof_clear: OE_GPIOF_CLEAR,
    #[doc = "0x25c - GPIO direction toggle"]
    pub oe_gpiof_toggle: OE_GPIOF_TOGGLE,
    _reserved96: [u8; 0x70],
    #[doc = "0x2d0 - GPIOX direction"]
    pub oe_gpiox_value: OE_GPIOX_VALUE,
    #[doc = "0x2d4 - GPIO direction set"]
    pub oe_gpiox_set: OE_GPIOX_SET,
    #[doc = "0x2d8 - GPIO direction clear"]
    pub oe_gpiox_clear: OE_GPIOX_CLEAR,
    #[doc = "0x2dc - GPIO direction toggle"]
    pub oe_gpiox_toggle: OE_GPIOX_TOGGLE,
    #[doc = "0x2e0 - GPIOY direction"]
    pub oe_gpioy_value: OE_GPIOY_VALUE,
    #[doc = "0x2e4 - GPIO direction set"]
    pub oe_gpioy_set: OE_GPIOY_SET,
    #[doc = "0x2e8 - GPIO direction clear"]
    pub oe_gpioy_clear: OE_GPIOY_CLEAR,
    #[doc = "0x2ec - GPIO direction toggle"]
    pub oe_gpioy_toggle: OE_GPIOY_TOGGLE,
    #[doc = "0x2f0 - GPIOZ direction"]
    pub oe_gpioz_value: OE_GPIOZ_VALUE,
    #[doc = "0x2f4 - GPIO direction set"]
    pub oe_gpioz_set: OE_GPIOZ_SET,
    #[doc = "0x2f8 - GPIO direction clear"]
    pub oe_gpioz_clear: OE_GPIOZ_CLEAR,
    #[doc = "0x2fc - GPIO direction toggle"]
    pub oe_gpioz_toggle: OE_GPIOZ_TOGGLE,
    #[doc = "0x300 - GPIO interrupt flag value"]
    pub if_gpioa_value: IF_GPIOA_VALUE,
    #[doc = "0x304 - GPIO interrupt flag set"]
    pub if_gpioa_set: IF_GPIOA_SET,
    #[doc = "0x308 - GPIO interrupt flag clear"]
    pub if_gpioa_clear: IF_GPIOA_CLEAR,
    #[doc = "0x30c - GPIO interrupt flag toggle"]
    pub if_gpioa_toggle: IF_GPIOA_TOGGLE,
    #[doc = "0x310 - GPIOB interrupt flag"]
    pub if_gpiob_value: IF_GPIOB_VALUE,
    #[doc = "0x314 - GPIO interrupt flag set"]
    pub if_gpiob_set: IF_GPIOB_SET,
    #[doc = "0x318 - GPIO interrupt flag clear"]
    pub if_gpiob_clear: IF_GPIOB_CLEAR,
    #[doc = "0x31c - GPIO interrupt flag toggle"]
    pub if_gpiob_toggle: IF_GPIOB_TOGGLE,
    #[doc = "0x320 - GPIOC interrupt flag"]
    pub if_gpioc_value: IF_GPIOC_VALUE,
    #[doc = "0x324 - GPIO interrupt flag set"]
    pub if_gpioc_set: IF_GPIOC_SET,
    #[doc = "0x328 - GPIO interrupt flag clear"]
    pub if_gpioc_clear: IF_GPIOC_CLEAR,
    #[doc = "0x32c - GPIO interrupt flag toggle"]
    pub if_gpioc_toggle: IF_GPIOC_TOGGLE,
    #[doc = "0x330 - GPIOD interrupt flag"]
    pub if_gpiod_value: IF_GPIOD_VALUE,
    #[doc = "0x334 - GPIO interrupt flag set"]
    pub if_gpiod_set: IF_GPIOD_SET,
    #[doc = "0x338 - GPIO interrupt flag clear"]
    pub if_gpiod_clear: IF_GPIOD_CLEAR,
    #[doc = "0x33c - GPIO interrupt flag toggle"]
    pub if_gpiod_toggle: IF_GPIOD_TOGGLE,
    #[doc = "0x340 - GPIOE interrupt flag"]
    pub if_gpioe_value: IF_GPIOE_VALUE,
    #[doc = "0x344 - GPIO interrupt flag set"]
    pub if_gpioe_set: IF_GPIOE_SET,
    #[doc = "0x348 - GPIO interrupt flag clear"]
    pub if_gpioe_clear: IF_GPIOE_CLEAR,
    #[doc = "0x34c - GPIO interrupt flag toggle"]
    pub if_gpioe_toggle: IF_GPIOE_TOGGLE,
    #[doc = "0x350 - GPIOF interrupt flag"]
    pub if_gpiof_value: IF_GPIOF_VALUE,
    #[doc = "0x354 - GPIO interrupt flag set"]
    pub if_gpiof_set: IF_GPIOF_SET,
    #[doc = "0x358 - GPIO interrupt flag clear"]
    pub if_gpiof_clear: IF_GPIOF_CLEAR,
    #[doc = "0x35c - GPIO interrupt flag toggle"]
    pub if_gpiof_toggle: IF_GPIOF_TOGGLE,
    _reserved132: [u8; 0x70],
    #[doc = "0x3d0 - GPIOX interrupt flag"]
    pub if_gpiox_value: IF_GPIOX_VALUE,
    #[doc = "0x3d4 - GPIO interrupt flag set"]
    pub if_gpiox_set: IF_GPIOX_SET,
    #[doc = "0x3d8 - GPIO interrupt flag clear"]
    pub if_gpiox_clear: IF_GPIOX_CLEAR,
    #[doc = "0x3dc - GPIO interrupt flag toggle"]
    pub if_gpiox_toggle: IF_GPIOX_TOGGLE,
    #[doc = "0x3e0 - GPIOY interrupt flag"]
    pub if_gpioy_value: IF_GPIOY_VALUE,
    #[doc = "0x3e4 - GPIO interrupt flag set"]
    pub if_gpioy_set: IF_GPIOY_SET,
    #[doc = "0x3e8 - GPIO interrupt flag clear"]
    pub if_gpioy_clear: IF_GPIOY_CLEAR,
    #[doc = "0x3ec - GPIO interrupt flag toggle"]
    pub if_gpioy_toggle: IF_GPIOY_TOGGLE,
    #[doc = "0x3f0 - GPIOZ interrupt flag"]
    pub if_gpioz_value: IF_GPIOZ_VALUE,
    #[doc = "0x3f4 - GPIO interrupt flag set"]
    pub if_gpioz_set: IF_GPIOZ_SET,
    #[doc = "0x3f8 - GPIO interrupt flag clear"]
    pub if_gpioz_clear: IF_GPIOZ_CLEAR,
    #[doc = "0x3fc - GPIO interrupt flag toggle"]
    pub if_gpioz_toggle: IF_GPIOZ_TOGGLE,
    #[doc = "0x400 - GPIO interrupt enable value"]
    pub ie_gpioa_value: IE_GPIOA_VALUE,
    #[doc = "0x404 - GPIO interrupt enable set"]
    pub ie_gpioa_set: IE_GPIOA_SET,
    #[doc = "0x408 - GPIO interrupt enable clear"]
    pub ie_gpioa_clear: IE_GPIOA_CLEAR,
    #[doc = "0x40c - GPIO interrupt enable toggle"]
    pub ie_gpioa_toggle: IE_GPIOA_TOGGLE,
    #[doc = "0x410 - GPIOB interrupt enable"]
    pub ie_gpiob_value: IE_GPIOB_VALUE,
    #[doc = "0x414 - GPIO interrupt enable set"]
    pub ie_gpiob_set: IE_GPIOB_SET,
    #[doc = "0x418 - GPIO interrupt enable clear"]
    pub ie_gpiob_clear: IE_GPIOB_CLEAR,
    #[doc = "0x41c - GPIO interrupt enable toggle"]
    pub ie_gpiob_toggle: IE_GPIOB_TOGGLE,
    #[doc = "0x420 - GPIOC interrupt enable"]
    pub ie_gpioc_value: IE_GPIOC_VALUE,
    #[doc = "0x424 - GPIO interrupt enable set"]
    pub ie_gpioc_set: IE_GPIOC_SET,
    #[doc = "0x428 - GPIO interrupt enable clear"]
    pub ie_gpioc_clear: IE_GPIOC_CLEAR,
    #[doc = "0x42c - GPIO interrupt enable toggle"]
    pub ie_gpioc_toggle: IE_GPIOC_TOGGLE,
    #[doc = "0x430 - GPIOD interrupt enable"]
    pub ie_gpiod_value: IE_GPIOD_VALUE,
    #[doc = "0x434 - GPIO interrupt enable set"]
    pub ie_gpiod_set: IE_GPIOD_SET,
    #[doc = "0x438 - GPIO interrupt enable clear"]
    pub ie_gpiod_clear: IE_GPIOD_CLEAR,
    #[doc = "0x43c - GPIO interrupt enable toggle"]
    pub ie_gpiod_toggle: IE_GPIOD_TOGGLE,
    #[doc = "0x440 - GPIOE interrupt enable"]
    pub ie_gpioe_value: IE_GPIOE_VALUE,
    #[doc = "0x444 - GPIO interrupt enable set"]
    pub ie_gpioe_set: IE_GPIOE_SET,
    #[doc = "0x448 - GPIO interrupt enable clear"]
    pub ie_gpioe_clear: IE_GPIOE_CLEAR,
    #[doc = "0x44c - GPIO interrupt enable toggle"]
    pub ie_gpioe_toggle: IE_GPIOE_TOGGLE,
    #[doc = "0x450 - GPIOF interrupt enable"]
    pub ie_gpiof_value: IE_GPIOF_VALUE,
    #[doc = "0x454 - GPIO interrupt enable set"]
    pub ie_gpiof_set: IE_GPIOF_SET,
    #[doc = "0x458 - GPIO interrupt enable clear"]
    pub ie_gpiof_clear: IE_GPIOF_CLEAR,
    #[doc = "0x45c - GPIO interrupt enable toggle"]
    pub ie_gpiof_toggle: IE_GPIOF_TOGGLE,
    _reserved168: [u8; 0x70],
    #[doc = "0x4d0 - GPIOX interrupt enable"]
    pub ie_gpiox_value: IE_GPIOX_VALUE,
    #[doc = "0x4d4 - GPIO interrupt enable set"]
    pub ie_gpiox_set: IE_GPIOX_SET,
    #[doc = "0x4d8 - GPIO interrupt enable clear"]
    pub ie_gpiox_clear: IE_GPIOX_CLEAR,
    #[doc = "0x4dc - GPIO interrupt enable toggle"]
    pub ie_gpiox_toggle: IE_GPIOX_TOGGLE,
    #[doc = "0x4e0 - GPIOY interrupt enable"]
    pub ie_gpioy_value: IE_GPIOY_VALUE,
    #[doc = "0x4e4 - GPIO interrupt enable set"]
    pub ie_gpioy_set: IE_GPIOY_SET,
    #[doc = "0x4e8 - GPIO interrupt enable clear"]
    pub ie_gpioy_clear: IE_GPIOY_CLEAR,
    #[doc = "0x4ec - GPIO interrupt enable toggle"]
    pub ie_gpioy_toggle: IE_GPIOY_TOGGLE,
    #[doc = "0x4f0 - GPIOZ interrupt enable"]
    pub ie_gpioz_value: IE_GPIOZ_VALUE,
    #[doc = "0x4f4 - GPIO interrupt enable set"]
    pub ie_gpioz_set: IE_GPIOZ_SET,
    #[doc = "0x4f8 - GPIO interrupt enable clear"]
    pub ie_gpioz_clear: IE_GPIOZ_CLEAR,
    #[doc = "0x4fc - GPIO interrupt enable toggle"]
    pub ie_gpioz_toggle: IE_GPIOZ_TOGGLE,
    #[doc = "0x500 - GPIO interrupt polarity value"]
    pub pl_gpioa_value: PL_GPIOA_VALUE,
    #[doc = "0x504 - GPIO interrupt polarity set"]
    pub pl_gpioa_set: PL_GPIOA_SET,
    #[doc = "0x508 - GPIO interrupt polarity clear"]
    pub pl_gpioa_clear: PL_GPIOA_CLEAR,
    #[doc = "0x50c - GPIO interrupt polarity toggle"]
    pub pl_gpioa_toggle: PL_GPIOA_TOGGLE,
    #[doc = "0x510 - GPIOB interrupt polarity"]
    pub pl_gpiob_value: PL_GPIOB_VALUE,
    #[doc = "0x514 - GPIO interrupt polarity set"]
    pub pl_gpiob_set: PL_GPIOB_SET,
    #[doc = "0x518 - GPIO interrupt polarity clear"]
    pub pl_gpiob_clear: PL_GPIOB_CLEAR,
    #[doc = "0x51c - GPIO interrupt polarity toggle"]
    pub pl_gpiob_toggle: PL_GPIOB_TOGGLE,
    #[doc = "0x520 - GPIOC interrupt polarity"]
    pub pl_gpioc_value: PL_GPIOC_VALUE,
    #[doc = "0x524 - GPIO interrupt polarity set"]
    pub pl_gpioc_set: PL_GPIOC_SET,
    #[doc = "0x528 - GPIO interrupt polarity clear"]
    pub pl_gpioc_clear: PL_GPIOC_CLEAR,
    #[doc = "0x52c - GPIO interrupt polarity toggle"]
    pub pl_gpioc_toggle: PL_GPIOC_TOGGLE,
    #[doc = "0x530 - GPIOD interrupt polarity"]
    pub pl_gpiod_value: PL_GPIOD_VALUE,
    #[doc = "0x534 - GPIO interrupt polarity set"]
    pub pl_gpiod_set: PL_GPIOD_SET,
    #[doc = "0x538 - GPIO interrupt polarity clear"]
    pub pl_gpiod_clear: PL_GPIOD_CLEAR,
    #[doc = "0x53c - GPIO interrupt polarity toggle"]
    pub pl_gpiod_toggle: PL_GPIOD_TOGGLE,
    #[doc = "0x540 - GPIOE interrupt polarity"]
    pub pl_gpioe_value: PL_GPIOE_VALUE,
    #[doc = "0x544 - GPIO interrupt polarity set"]
    pub pl_gpioe_set: PL_GPIOE_SET,
    #[doc = "0x548 - GPIO interrupt polarity clear"]
    pub pl_gpioe_clear: PL_GPIOE_CLEAR,
    #[doc = "0x54c - GPIO interrupt polarity toggle"]
    pub pl_gpioe_toggle: PL_GPIOE_TOGGLE,
    #[doc = "0x550 - GPIOF interrupt polarity"]
    pub pl_gpiof_value: PL_GPIOF_VALUE,
    #[doc = "0x554 - GPIO interrupt polarity set"]
    pub pl_gpiof_set: PL_GPIOF_SET,
    #[doc = "0x558 - GPIO interrupt polarity clear"]
    pub pl_gpiof_clear: PL_GPIOF_CLEAR,
    #[doc = "0x55c - GPIO interrupt polarity toggle"]
    pub pl_gpiof_toggle: PL_GPIOF_TOGGLE,
    _reserved204: [u8; 0x70],
    #[doc = "0x5d0 - GPIOX interrupt polarity"]
    pub pl_gpiox_value: PL_GPIOX_VALUE,
    #[doc = "0x5d4 - GPIO interrupt polarity set"]
    pub pl_gpiox_set: PL_GPIOX_SET,
    #[doc = "0x5d8 - GPIO interrupt polarity clear"]
    pub pl_gpiox_clear: PL_GPIOX_CLEAR,
    #[doc = "0x5dc - GPIO interrupt polarity toggle"]
    pub pl_gpiox_toggle: PL_GPIOX_TOGGLE,
    #[doc = "0x5e0 - GPIOY interrupt polarity"]
    pub pl_gpioy_value: PL_GPIOY_VALUE,
    #[doc = "0x5e4 - GPIO interrupt polarity set"]
    pub pl_gpioy_set: PL_GPIOY_SET,
    #[doc = "0x5e8 - GPIO interrupt polarity clear"]
    pub pl_gpioy_clear: PL_GPIOY_CLEAR,
    #[doc = "0x5ec - GPIO interrupt polarity toggle"]
    pub pl_gpioy_toggle: PL_GPIOY_TOGGLE,
    #[doc = "0x5f0 - GPIOZ interrupt polarity"]
    pub pl_gpioz_value: PL_GPIOZ_VALUE,
    #[doc = "0x5f4 - GPIO interrupt polarity set"]
    pub pl_gpioz_set: PL_GPIOZ_SET,
    #[doc = "0x5f8 - GPIO interrupt polarity clear"]
    pub pl_gpioz_clear: PL_GPIOZ_CLEAR,
    #[doc = "0x5fc - GPIO interrupt polarity toggle"]
    pub pl_gpioz_toggle: PL_GPIOZ_TOGGLE,
    #[doc = "0x600 - GPIO interrupt type value"]
    pub tp_gpioa_value: TP_GPIOA_VALUE,
    #[doc = "0x604 - GPIO interrupt type set"]
    pub tp_gpioa_set: TP_GPIOA_SET,
    #[doc = "0x608 - GPIO interrupt type clear"]
    pub tp_gpioa_clear: TP_GPIOA_CLEAR,
    #[doc = "0x60c - GPIO interrupt type toggle"]
    pub tp_gpioa_toggle: TP_GPIOA_TOGGLE,
    #[doc = "0x610 - GPIOB interrupt type"]
    pub tp_gpiob_value: TP_GPIOB_VALUE,
    #[doc = "0x614 - GPIO interrupt type set"]
    pub tp_gpiob_set: TP_GPIOB_SET,
    #[doc = "0x618 - GPIO interrupt type clear"]
    pub tp_gpiob_clear: TP_GPIOB_CLEAR,
    #[doc = "0x61c - GPIO interrupt type toggle"]
    pub tp_gpiob_toggle: TP_GPIOB_TOGGLE,
    #[doc = "0x620 - GPIOC interrupt type"]
    pub tp_gpioc_value: TP_GPIOC_VALUE,
    #[doc = "0x624 - GPIO interrupt type set"]
    pub tp_gpioc_set: TP_GPIOC_SET,
    #[doc = "0x628 - GPIO interrupt type clear"]
    pub tp_gpioc_clear: TP_GPIOC_CLEAR,
    #[doc = "0x62c - GPIO interrupt type toggle"]
    pub tp_gpioc_toggle: TP_GPIOC_TOGGLE,
    #[doc = "0x630 - GPIOD interrupt type"]
    pub tp_gpiod_value: TP_GPIOD_VALUE,
    #[doc = "0x634 - GPIO interrupt type set"]
    pub tp_gpiod_set: TP_GPIOD_SET,
    #[doc = "0x638 - GPIO interrupt type clear"]
    pub tp_gpiod_clear: TP_GPIOD_CLEAR,
    #[doc = "0x63c - GPIO interrupt type toggle"]
    pub tp_gpiod_toggle: TP_GPIOD_TOGGLE,
    #[doc = "0x640 - GPIOE interrupt type"]
    pub tp_gpioe_value: TP_GPIOE_VALUE,
    #[doc = "0x644 - GPIO interrupt type set"]
    pub tp_gpioe_set: TP_GPIOE_SET,
    #[doc = "0x648 - GPIO interrupt type clear"]
    pub tp_gpioe_clear: TP_GPIOE_CLEAR,
    #[doc = "0x64c - GPIO interrupt type toggle"]
    pub tp_gpioe_toggle: TP_GPIOE_TOGGLE,
    #[doc = "0x650 - GPIOF interrupt type"]
    pub tp_gpiof_value: TP_GPIOF_VALUE,
    #[doc = "0x654 - GPIO interrupt type set"]
    pub tp_gpiof_set: TP_GPIOF_SET,
    #[doc = "0x658 - GPIO interrupt type clear"]
    pub tp_gpiof_clear: TP_GPIOF_CLEAR,
    #[doc = "0x65c - GPIO interrupt type toggle"]
    pub tp_gpiof_toggle: TP_GPIOF_TOGGLE,
    _reserved240: [u8; 0x70],
    #[doc = "0x6d0 - GPIOX interrupt type"]
    pub tp_gpiox_value: TP_GPIOX_VALUE,
    #[doc = "0x6d4 - GPIO interrupt type set"]
    pub tp_gpiox_set: TP_GPIOX_SET,
    #[doc = "0x6d8 - GPIO interrupt type clear"]
    pub tp_gpiox_clear: TP_GPIOX_CLEAR,
    #[doc = "0x6dc - GPIO interrupt type toggle"]
    pub tp_gpiox_toggle: TP_GPIOX_TOGGLE,
    #[doc = "0x6e0 - GPIOY interrupt type"]
    pub tp_gpioy_value: TP_GPIOY_VALUE,
    #[doc = "0x6e4 - GPIO interrupt type set"]
    pub tp_gpioy_set: TP_GPIOY_SET,
    #[doc = "0x6e8 - GPIO interrupt type clear"]
    pub tp_gpioy_clear: TP_GPIOY_CLEAR,
    #[doc = "0x6ec - GPIO interrupt type toggle"]
    pub tp_gpioy_toggle: TP_GPIOY_TOGGLE,
    #[doc = "0x6f0 - GPIOZ interrupt type"]
    pub tp_gpioz_value: TP_GPIOZ_VALUE,
    #[doc = "0x6f4 - GPIO interrupt type set"]
    pub tp_gpioz_set: TP_GPIOZ_SET,
    #[doc = "0x6f8 - GPIO interrupt type clear"]
    pub tp_gpioz_clear: TP_GPIOZ_CLEAR,
    #[doc = "0x6fc - GPIO interrupt type toggle"]
    pub tp_gpioz_toggle: TP_GPIOZ_TOGGLE,
    #[doc = "0x700 - GPIO interrupt asynchronous value"]
    pub as_gpioa_value: AS_GPIOA_VALUE,
    #[doc = "0x704 - GPIO interrupt asynchronous set"]
    pub as_gpioa_set: AS_GPIOA_SET,
    #[doc = "0x708 - GPIO interrupt asynchronous clear"]
    pub as_gpioa_clear: AS_GPIOA_CLEAR,
    #[doc = "0x70c - GPIO interrupt asynchronous toggle"]
    pub as_gpioa_toggle: AS_GPIOA_TOGGLE,
    #[doc = "0x710 - GPIOB interrupt asynchronous"]
    pub as_gpiob_value: AS_GPIOB_VALUE,
    #[doc = "0x714 - GPIO interrupt asynchronous set"]
    pub as_gpiob_set: AS_GPIOB_SET,
    #[doc = "0x718 - GPIO interrupt asynchronous clear"]
    pub as_gpiob_clear: AS_GPIOB_CLEAR,
    #[doc = "0x71c - GPIO interrupt asynchronous toggle"]
    pub as_gpiob_toggle: AS_GPIOB_TOGGLE,
    #[doc = "0x720 - GPIOC interrupt asynchronous"]
    pub as_gpioc_value: AS_GPIOC_VALUE,
    #[doc = "0x724 - GPIO interrupt asynchronous set"]
    pub as_gpioc_set: AS_GPIOC_SET,
    #[doc = "0x728 - GPIO interrupt asynchronous clear"]
    pub as_gpioc_clear: AS_GPIOC_CLEAR,
    #[doc = "0x72c - GPIO interrupt asynchronous toggle"]
    pub as_gpioc_toggle: AS_GPIOC_TOGGLE,
    #[doc = "0x730 - GPIOD interrupt asynchronous"]
    pub as_gpiod_value: AS_GPIOD_VALUE,
    #[doc = "0x734 - GPIO interrupt asynchronous set"]
    pub as_gpiod_set: AS_GPIOD_SET,
    #[doc = "0x738 - GPIO interrupt asynchronous clear"]
    pub as_gpiod_clear: AS_GPIOD_CLEAR,
    #[doc = "0x73c - GPIO interrupt asynchronous toggle"]
    pub as_gpiod_toggle: AS_GPIOD_TOGGLE,
    #[doc = "0x740 - GPIOE interrupt asynchronous"]
    pub as_gpioe_value: AS_GPIOE_VALUE,
    #[doc = "0x744 - GPIO interrupt asynchronous set"]
    pub as_gpioe_set: AS_GPIOE_SET,
    #[doc = "0x748 - GPIO interrupt asynchronous clear"]
    pub as_gpioe_clear: AS_GPIOE_CLEAR,
    #[doc = "0x74c - GPIO interrupt asynchronous toggle"]
    pub as_gpioe_toggle: AS_GPIOE_TOGGLE,
    #[doc = "0x750 - GPIOF interrupt asynchronous"]
    pub as_gpiof_value: AS_GPIOF_VALUE,
    #[doc = "0x754 - GPIO interrupt asynchronous set"]
    pub as_gpiof_set: AS_GPIOF_SET,
    #[doc = "0x758 - GPIO interrupt asynchronous clear"]
    pub as_gpiof_clear: AS_GPIOF_CLEAR,
    #[doc = "0x75c - GPIO interrupt asynchronous toggle"]
    pub as_gpiof_toggle: AS_GPIOF_TOGGLE,
    _reserved276: [u8; 0x70],
    #[doc = "0x7d0 - GPIOX interrupt asynchronous"]
    pub as_gpiox_value: AS_GPIOX_VALUE,
    #[doc = "0x7d4 - GPIO interrupt asynchronous set"]
    pub as_gpiox_set: AS_GPIOX_SET,
    #[doc = "0x7d8 - GPIO interrupt asynchronous clear"]
    pub as_gpiox_clear: AS_GPIOX_CLEAR,
    #[doc = "0x7dc - GPIO interrupt asynchronous toggle"]
    pub as_gpiox_toggle: AS_GPIOX_TOGGLE,
    #[doc = "0x7e0 - GPIOY interrupt asynchronous"]
    pub as_gpioy_value: AS_GPIOY_VALUE,
    #[doc = "0x7e4 - GPIO interrupt asynchronous set"]
    pub as_gpioy_set: AS_GPIOY_SET,
    #[doc = "0x7e8 - GPIO interrupt asynchronous clear"]
    pub as_gpioy_clear: AS_GPIOY_CLEAR,
    #[doc = "0x7ec - GPIO interrupt asynchronous toggle"]
    pub as_gpioy_toggle: AS_GPIOY_TOGGLE,
    #[doc = "0x7f0 - GPIOZ interrupt asynchronous"]
    pub as_gpioz_value: AS_GPIOZ_VALUE,
    #[doc = "0x7f4 - GPIO interrupt asynchronous set"]
    pub as_gpioz_set: AS_GPIOZ_SET,
    #[doc = "0x7f8 - GPIO interrupt asynchronous clear"]
    pub as_gpioz_clear: AS_GPIOZ_CLEAR,
    #[doc = "0x7fc - GPIO interrupt asynchronous toggle"]
    pub as_gpioz_toggle: AS_GPIOZ_TOGGLE,
}
#[doc = "DI_GPIOA_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOA_VALUE_SPEC>`"]
pub type DI_GPIOA_VALUE = crate::Reg<di_gpioa_value::DI_GPIOA_VALUE_SPEC>;
#[doc = "GPIO input value"]
pub mod di_gpioa_value;
#[doc = "DI_GPIOA_SET (rw) register accessor: an alias for `Reg<DI_GPIOA_SET_SPEC>`"]
pub type DI_GPIOA_SET = crate::Reg<di_gpioa_set::DI_GPIOA_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpioa_set;
#[doc = "DI_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOA_CLEAR_SPEC>`"]
pub type DI_GPIOA_CLEAR = crate::Reg<di_gpioa_clear::DI_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpioa_clear;
#[doc = "DI_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOA_TOGGLE_SPEC>`"]
pub type DI_GPIOA_TOGGLE = crate::Reg<di_gpioa_toggle::DI_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpioa_toggle;
#[doc = "DI_GPIOB_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOB_VALUE_SPEC>`"]
pub type DI_GPIOB_VALUE = crate::Reg<di_gpiob_value::DI_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB input"]
pub mod di_gpiob_value;
#[doc = "DI_GPIOB_SET (rw) register accessor: an alias for `Reg<DI_GPIOB_SET_SPEC>`"]
pub type DI_GPIOB_SET = crate::Reg<di_gpiob_set::DI_GPIOB_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpiob_set;
#[doc = "DI_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOB_CLEAR_SPEC>`"]
pub type DI_GPIOB_CLEAR = crate::Reg<di_gpiob_clear::DI_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpiob_clear;
#[doc = "DI_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOB_TOGGLE_SPEC>`"]
pub type DI_GPIOB_TOGGLE = crate::Reg<di_gpiob_toggle::DI_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpiob_toggle;
#[doc = "DI_GPIOC_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOC_VALUE_SPEC>`"]
pub type DI_GPIOC_VALUE = crate::Reg<di_gpioc_value::DI_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC input"]
pub mod di_gpioc_value;
#[doc = "DI_GPIOC_SET (rw) register accessor: an alias for `Reg<DI_GPIOC_SET_SPEC>`"]
pub type DI_GPIOC_SET = crate::Reg<di_gpioc_set::DI_GPIOC_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpioc_set;
#[doc = "DI_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOC_CLEAR_SPEC>`"]
pub type DI_GPIOC_CLEAR = crate::Reg<di_gpioc_clear::DI_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpioc_clear;
#[doc = "DI_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOC_TOGGLE_SPEC>`"]
pub type DI_GPIOC_TOGGLE = crate::Reg<di_gpioc_toggle::DI_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpioc_toggle;
#[doc = "DI_GPIOD_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOD_VALUE_SPEC>`"]
pub type DI_GPIOD_VALUE = crate::Reg<di_gpiod_value::DI_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD input"]
pub mod di_gpiod_value;
#[doc = "DI_GPIOD_SET (rw) register accessor: an alias for `Reg<DI_GPIOD_SET_SPEC>`"]
pub type DI_GPIOD_SET = crate::Reg<di_gpiod_set::DI_GPIOD_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpiod_set;
#[doc = "DI_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOD_CLEAR_SPEC>`"]
pub type DI_GPIOD_CLEAR = crate::Reg<di_gpiod_clear::DI_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpiod_clear;
#[doc = "DI_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOD_TOGGLE_SPEC>`"]
pub type DI_GPIOD_TOGGLE = crate::Reg<di_gpiod_toggle::DI_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpiod_toggle;
#[doc = "DI_GPIOE_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOE_VALUE_SPEC>`"]
pub type DI_GPIOE_VALUE = crate::Reg<di_gpioe_value::DI_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE input"]
pub mod di_gpioe_value;
#[doc = "DI_GPIOE_SET (rw) register accessor: an alias for `Reg<DI_GPIOE_SET_SPEC>`"]
pub type DI_GPIOE_SET = crate::Reg<di_gpioe_set::DI_GPIOE_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpioe_set;
#[doc = "DI_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOE_CLEAR_SPEC>`"]
pub type DI_GPIOE_CLEAR = crate::Reg<di_gpioe_clear::DI_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpioe_clear;
#[doc = "DI_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOE_TOGGLE_SPEC>`"]
pub type DI_GPIOE_TOGGLE = crate::Reg<di_gpioe_toggle::DI_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpioe_toggle;
#[doc = "DI_GPIOF_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOF_VALUE_SPEC>`"]
pub type DI_GPIOF_VALUE = crate::Reg<di_gpiof_value::DI_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF input"]
pub mod di_gpiof_value;
#[doc = "DI_GPIOF_SET (rw) register accessor: an alias for `Reg<DI_GPIOF_SET_SPEC>`"]
pub type DI_GPIOF_SET = crate::Reg<di_gpiof_set::DI_GPIOF_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpiof_set;
#[doc = "DI_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOF_CLEAR_SPEC>`"]
pub type DI_GPIOF_CLEAR = crate::Reg<di_gpiof_clear::DI_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpiof_clear;
#[doc = "DI_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOF_TOGGLE_SPEC>`"]
pub type DI_GPIOF_TOGGLE = crate::Reg<di_gpiof_toggle::DI_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpiof_toggle;
#[doc = "DI_GPIOX_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOX_VALUE_SPEC>`"]
pub type DI_GPIOX_VALUE = crate::Reg<di_gpiox_value::DI_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX input"]
pub mod di_gpiox_value;
#[doc = "DI_GPIOX_SET (rw) register accessor: an alias for `Reg<DI_GPIOX_SET_SPEC>`"]
pub type DI_GPIOX_SET = crate::Reg<di_gpiox_set::DI_GPIOX_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpiox_set;
#[doc = "DI_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOX_CLEAR_SPEC>`"]
pub type DI_GPIOX_CLEAR = crate::Reg<di_gpiox_clear::DI_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpiox_clear;
#[doc = "DI_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOX_TOGGLE_SPEC>`"]
pub type DI_GPIOX_TOGGLE = crate::Reg<di_gpiox_toggle::DI_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpiox_toggle;
#[doc = "DI_GPIOY_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOY_VALUE_SPEC>`"]
pub type DI_GPIOY_VALUE = crate::Reg<di_gpioy_value::DI_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY input"]
pub mod di_gpioy_value;
#[doc = "DI_GPIOY_SET (rw) register accessor: an alias for `Reg<DI_GPIOY_SET_SPEC>`"]
pub type DI_GPIOY_SET = crate::Reg<di_gpioy_set::DI_GPIOY_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpioy_set;
#[doc = "DI_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOY_CLEAR_SPEC>`"]
pub type DI_GPIOY_CLEAR = crate::Reg<di_gpioy_clear::DI_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpioy_clear;
#[doc = "DI_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOY_TOGGLE_SPEC>`"]
pub type DI_GPIOY_TOGGLE = crate::Reg<di_gpioy_toggle::DI_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpioy_toggle;
#[doc = "DI_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<DI_GPIOZ_VALUE_SPEC>`"]
pub type DI_GPIOZ_VALUE = crate::Reg<di_gpioz_value::DI_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ input"]
pub mod di_gpioz_value;
#[doc = "DI_GPIOZ_SET (rw) register accessor: an alias for `Reg<DI_GPIOZ_SET_SPEC>`"]
pub type DI_GPIOZ_SET = crate::Reg<di_gpioz_set::DI_GPIOZ_SET_SPEC>;
#[doc = "GPIO input set"]
pub mod di_gpioz_set;
#[doc = "DI_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<DI_GPIOZ_CLEAR_SPEC>`"]
pub type DI_GPIOZ_CLEAR = crate::Reg<di_gpioz_clear::DI_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO input clear"]
pub mod di_gpioz_clear;
#[doc = "DI_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<DI_GPIOZ_TOGGLE_SPEC>`"]
pub type DI_GPIOZ_TOGGLE = crate::Reg<di_gpioz_toggle::DI_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO input toggle"]
pub mod di_gpioz_toggle;
#[doc = "DO_GPIOA_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOA_VALUE_SPEC>`"]
pub type DO_GPIOA_VALUE = crate::Reg<do_gpioa_value::DO_GPIOA_VALUE_SPEC>;
#[doc = "GPIO output value"]
pub mod do_gpioa_value;
#[doc = "DO_GPIOA_SET (rw) register accessor: an alias for `Reg<DO_GPIOA_SET_SPEC>`"]
pub type DO_GPIOA_SET = crate::Reg<do_gpioa_set::DO_GPIOA_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpioa_set;
#[doc = "DO_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOA_CLEAR_SPEC>`"]
pub type DO_GPIOA_CLEAR = crate::Reg<do_gpioa_clear::DO_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpioa_clear;
#[doc = "DO_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOA_TOGGLE_SPEC>`"]
pub type DO_GPIOA_TOGGLE = crate::Reg<do_gpioa_toggle::DO_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpioa_toggle;
#[doc = "DO_GPIOB_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOB_VALUE_SPEC>`"]
pub type DO_GPIOB_VALUE = crate::Reg<do_gpiob_value::DO_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB output"]
pub mod do_gpiob_value;
#[doc = "DO_GPIOB_SET (rw) register accessor: an alias for `Reg<DO_GPIOB_SET_SPEC>`"]
pub type DO_GPIOB_SET = crate::Reg<do_gpiob_set::DO_GPIOB_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpiob_set;
#[doc = "DO_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOB_CLEAR_SPEC>`"]
pub type DO_GPIOB_CLEAR = crate::Reg<do_gpiob_clear::DO_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpiob_clear;
#[doc = "DO_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOB_TOGGLE_SPEC>`"]
pub type DO_GPIOB_TOGGLE = crate::Reg<do_gpiob_toggle::DO_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpiob_toggle;
#[doc = "DO_GPIOC_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOC_VALUE_SPEC>`"]
pub type DO_GPIOC_VALUE = crate::Reg<do_gpioc_value::DO_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC output"]
pub mod do_gpioc_value;
#[doc = "DO_GPIOC_SET (rw) register accessor: an alias for `Reg<DO_GPIOC_SET_SPEC>`"]
pub type DO_GPIOC_SET = crate::Reg<do_gpioc_set::DO_GPIOC_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpioc_set;
#[doc = "DO_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOC_CLEAR_SPEC>`"]
pub type DO_GPIOC_CLEAR = crate::Reg<do_gpioc_clear::DO_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpioc_clear;
#[doc = "DO_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOC_TOGGLE_SPEC>`"]
pub type DO_GPIOC_TOGGLE = crate::Reg<do_gpioc_toggle::DO_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpioc_toggle;
#[doc = "DO_GPIOD_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOD_VALUE_SPEC>`"]
pub type DO_GPIOD_VALUE = crate::Reg<do_gpiod_value::DO_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD output"]
pub mod do_gpiod_value;
#[doc = "DO_GPIOD_SET (rw) register accessor: an alias for `Reg<DO_GPIOD_SET_SPEC>`"]
pub type DO_GPIOD_SET = crate::Reg<do_gpiod_set::DO_GPIOD_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpiod_set;
#[doc = "DO_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOD_CLEAR_SPEC>`"]
pub type DO_GPIOD_CLEAR = crate::Reg<do_gpiod_clear::DO_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpiod_clear;
#[doc = "DO_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOD_TOGGLE_SPEC>`"]
pub type DO_GPIOD_TOGGLE = crate::Reg<do_gpiod_toggle::DO_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpiod_toggle;
#[doc = "DO_GPIOE_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOE_VALUE_SPEC>`"]
pub type DO_GPIOE_VALUE = crate::Reg<do_gpioe_value::DO_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE output"]
pub mod do_gpioe_value;
#[doc = "DO_GPIOE_SET (rw) register accessor: an alias for `Reg<DO_GPIOE_SET_SPEC>`"]
pub type DO_GPIOE_SET = crate::Reg<do_gpioe_set::DO_GPIOE_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpioe_set;
#[doc = "DO_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOE_CLEAR_SPEC>`"]
pub type DO_GPIOE_CLEAR = crate::Reg<do_gpioe_clear::DO_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpioe_clear;
#[doc = "DO_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOE_TOGGLE_SPEC>`"]
pub type DO_GPIOE_TOGGLE = crate::Reg<do_gpioe_toggle::DO_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpioe_toggle;
#[doc = "DO_GPIOF_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOF_VALUE_SPEC>`"]
pub type DO_GPIOF_VALUE = crate::Reg<do_gpiof_value::DO_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF output"]
pub mod do_gpiof_value;
#[doc = "DO_GPIOF_SET (rw) register accessor: an alias for `Reg<DO_GPIOF_SET_SPEC>`"]
pub type DO_GPIOF_SET = crate::Reg<do_gpiof_set::DO_GPIOF_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpiof_set;
#[doc = "DO_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOF_CLEAR_SPEC>`"]
pub type DO_GPIOF_CLEAR = crate::Reg<do_gpiof_clear::DO_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpiof_clear;
#[doc = "DO_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOF_TOGGLE_SPEC>`"]
pub type DO_GPIOF_TOGGLE = crate::Reg<do_gpiof_toggle::DO_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpiof_toggle;
#[doc = "DO_GPIOX_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOX_VALUE_SPEC>`"]
pub type DO_GPIOX_VALUE = crate::Reg<do_gpiox_value::DO_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX output"]
pub mod do_gpiox_value;
#[doc = "DO_GPIOX_SET (rw) register accessor: an alias for `Reg<DO_GPIOX_SET_SPEC>`"]
pub type DO_GPIOX_SET = crate::Reg<do_gpiox_set::DO_GPIOX_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpiox_set;
#[doc = "DO_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOX_CLEAR_SPEC>`"]
pub type DO_GPIOX_CLEAR = crate::Reg<do_gpiox_clear::DO_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpiox_clear;
#[doc = "DO_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOX_TOGGLE_SPEC>`"]
pub type DO_GPIOX_TOGGLE = crate::Reg<do_gpiox_toggle::DO_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpiox_toggle;
#[doc = "DO_GPIOY_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOY_VALUE_SPEC>`"]
pub type DO_GPIOY_VALUE = crate::Reg<do_gpioy_value::DO_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY output"]
pub mod do_gpioy_value;
#[doc = "DO_GPIOY_SET (rw) register accessor: an alias for `Reg<DO_GPIOY_SET_SPEC>`"]
pub type DO_GPIOY_SET = crate::Reg<do_gpioy_set::DO_GPIOY_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpioy_set;
#[doc = "DO_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOY_CLEAR_SPEC>`"]
pub type DO_GPIOY_CLEAR = crate::Reg<do_gpioy_clear::DO_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpioy_clear;
#[doc = "DO_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOY_TOGGLE_SPEC>`"]
pub type DO_GPIOY_TOGGLE = crate::Reg<do_gpioy_toggle::DO_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpioy_toggle;
#[doc = "DO_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<DO_GPIOZ_VALUE_SPEC>`"]
pub type DO_GPIOZ_VALUE = crate::Reg<do_gpioz_value::DO_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ output"]
pub mod do_gpioz_value;
#[doc = "DO_GPIOZ_SET (rw) register accessor: an alias for `Reg<DO_GPIOZ_SET_SPEC>`"]
pub type DO_GPIOZ_SET = crate::Reg<do_gpioz_set::DO_GPIOZ_SET_SPEC>;
#[doc = "GPIO output set"]
pub mod do_gpioz_set;
#[doc = "DO_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<DO_GPIOZ_CLEAR_SPEC>`"]
pub type DO_GPIOZ_CLEAR = crate::Reg<do_gpioz_clear::DO_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO output clear"]
pub mod do_gpioz_clear;
#[doc = "DO_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<DO_GPIOZ_TOGGLE_SPEC>`"]
pub type DO_GPIOZ_TOGGLE = crate::Reg<do_gpioz_toggle::DO_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO output toggle"]
pub mod do_gpioz_toggle;
#[doc = "OE_GPIOA_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOA_VALUE_SPEC>`"]
pub type OE_GPIOA_VALUE = crate::Reg<oe_gpioa_value::OE_GPIOA_VALUE_SPEC>;
#[doc = "GPIO direction value"]
pub mod oe_gpioa_value;
#[doc = "OE_GPIOA_SET (rw) register accessor: an alias for `Reg<OE_GPIOA_SET_SPEC>`"]
pub type OE_GPIOA_SET = crate::Reg<oe_gpioa_set::OE_GPIOA_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpioa_set;
#[doc = "OE_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOA_CLEAR_SPEC>`"]
pub type OE_GPIOA_CLEAR = crate::Reg<oe_gpioa_clear::OE_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpioa_clear;
#[doc = "OE_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOA_TOGGLE_SPEC>`"]
pub type OE_GPIOA_TOGGLE = crate::Reg<oe_gpioa_toggle::OE_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpioa_toggle;
#[doc = "OE_GPIOB_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOB_VALUE_SPEC>`"]
pub type OE_GPIOB_VALUE = crate::Reg<oe_gpiob_value::OE_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB direction"]
pub mod oe_gpiob_value;
#[doc = "OE_GPIOB_SET (rw) register accessor: an alias for `Reg<OE_GPIOB_SET_SPEC>`"]
pub type OE_GPIOB_SET = crate::Reg<oe_gpiob_set::OE_GPIOB_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpiob_set;
#[doc = "OE_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOB_CLEAR_SPEC>`"]
pub type OE_GPIOB_CLEAR = crate::Reg<oe_gpiob_clear::OE_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpiob_clear;
#[doc = "OE_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOB_TOGGLE_SPEC>`"]
pub type OE_GPIOB_TOGGLE = crate::Reg<oe_gpiob_toggle::OE_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpiob_toggle;
#[doc = "OE_GPIOC_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOC_VALUE_SPEC>`"]
pub type OE_GPIOC_VALUE = crate::Reg<oe_gpioc_value::OE_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC direction"]
pub mod oe_gpioc_value;
#[doc = "OE_GPIOC_SET (rw) register accessor: an alias for `Reg<OE_GPIOC_SET_SPEC>`"]
pub type OE_GPIOC_SET = crate::Reg<oe_gpioc_set::OE_GPIOC_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpioc_set;
#[doc = "OE_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOC_CLEAR_SPEC>`"]
pub type OE_GPIOC_CLEAR = crate::Reg<oe_gpioc_clear::OE_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpioc_clear;
#[doc = "OE_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOC_TOGGLE_SPEC>`"]
pub type OE_GPIOC_TOGGLE = crate::Reg<oe_gpioc_toggle::OE_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpioc_toggle;
#[doc = "OE_GPIOD_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOD_VALUE_SPEC>`"]
pub type OE_GPIOD_VALUE = crate::Reg<oe_gpiod_value::OE_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD direction"]
pub mod oe_gpiod_value;
#[doc = "OE_GPIOD_SET (rw) register accessor: an alias for `Reg<OE_GPIOD_SET_SPEC>`"]
pub type OE_GPIOD_SET = crate::Reg<oe_gpiod_set::OE_GPIOD_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpiod_set;
#[doc = "OE_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOD_CLEAR_SPEC>`"]
pub type OE_GPIOD_CLEAR = crate::Reg<oe_gpiod_clear::OE_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpiod_clear;
#[doc = "OE_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOD_TOGGLE_SPEC>`"]
pub type OE_GPIOD_TOGGLE = crate::Reg<oe_gpiod_toggle::OE_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpiod_toggle;
#[doc = "OE_GPIOE_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOE_VALUE_SPEC>`"]
pub type OE_GPIOE_VALUE = crate::Reg<oe_gpioe_value::OE_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE direction"]
pub mod oe_gpioe_value;
#[doc = "OE_GPIOE_SET (rw) register accessor: an alias for `Reg<OE_GPIOE_SET_SPEC>`"]
pub type OE_GPIOE_SET = crate::Reg<oe_gpioe_set::OE_GPIOE_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpioe_set;
#[doc = "OE_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOE_CLEAR_SPEC>`"]
pub type OE_GPIOE_CLEAR = crate::Reg<oe_gpioe_clear::OE_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpioe_clear;
#[doc = "OE_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOE_TOGGLE_SPEC>`"]
pub type OE_GPIOE_TOGGLE = crate::Reg<oe_gpioe_toggle::OE_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpioe_toggle;
#[doc = "OE_GPIOF_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOF_VALUE_SPEC>`"]
pub type OE_GPIOF_VALUE = crate::Reg<oe_gpiof_value::OE_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF direction"]
pub mod oe_gpiof_value;
#[doc = "OE_GPIOF_SET (rw) register accessor: an alias for `Reg<OE_GPIOF_SET_SPEC>`"]
pub type OE_GPIOF_SET = crate::Reg<oe_gpiof_set::OE_GPIOF_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpiof_set;
#[doc = "OE_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOF_CLEAR_SPEC>`"]
pub type OE_GPIOF_CLEAR = crate::Reg<oe_gpiof_clear::OE_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpiof_clear;
#[doc = "OE_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOF_TOGGLE_SPEC>`"]
pub type OE_GPIOF_TOGGLE = crate::Reg<oe_gpiof_toggle::OE_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpiof_toggle;
#[doc = "OE_GPIOX_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOX_VALUE_SPEC>`"]
pub type OE_GPIOX_VALUE = crate::Reg<oe_gpiox_value::OE_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX direction"]
pub mod oe_gpiox_value;
#[doc = "OE_GPIOX_SET (rw) register accessor: an alias for `Reg<OE_GPIOX_SET_SPEC>`"]
pub type OE_GPIOX_SET = crate::Reg<oe_gpiox_set::OE_GPIOX_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpiox_set;
#[doc = "OE_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOX_CLEAR_SPEC>`"]
pub type OE_GPIOX_CLEAR = crate::Reg<oe_gpiox_clear::OE_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpiox_clear;
#[doc = "OE_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOX_TOGGLE_SPEC>`"]
pub type OE_GPIOX_TOGGLE = crate::Reg<oe_gpiox_toggle::OE_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpiox_toggle;
#[doc = "OE_GPIOY_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOY_VALUE_SPEC>`"]
pub type OE_GPIOY_VALUE = crate::Reg<oe_gpioy_value::OE_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY direction"]
pub mod oe_gpioy_value;
#[doc = "OE_GPIOY_SET (rw) register accessor: an alias for `Reg<OE_GPIOY_SET_SPEC>`"]
pub type OE_GPIOY_SET = crate::Reg<oe_gpioy_set::OE_GPIOY_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpioy_set;
#[doc = "OE_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOY_CLEAR_SPEC>`"]
pub type OE_GPIOY_CLEAR = crate::Reg<oe_gpioy_clear::OE_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpioy_clear;
#[doc = "OE_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOY_TOGGLE_SPEC>`"]
pub type OE_GPIOY_TOGGLE = crate::Reg<oe_gpioy_toggle::OE_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpioy_toggle;
#[doc = "OE_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<OE_GPIOZ_VALUE_SPEC>`"]
pub type OE_GPIOZ_VALUE = crate::Reg<oe_gpioz_value::OE_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ direction"]
pub mod oe_gpioz_value;
#[doc = "OE_GPIOZ_SET (rw) register accessor: an alias for `Reg<OE_GPIOZ_SET_SPEC>`"]
pub type OE_GPIOZ_SET = crate::Reg<oe_gpioz_set::OE_GPIOZ_SET_SPEC>;
#[doc = "GPIO direction set"]
pub mod oe_gpioz_set;
#[doc = "OE_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<OE_GPIOZ_CLEAR_SPEC>`"]
pub type OE_GPIOZ_CLEAR = crate::Reg<oe_gpioz_clear::OE_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO direction clear"]
pub mod oe_gpioz_clear;
#[doc = "OE_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<OE_GPIOZ_TOGGLE_SPEC>`"]
pub type OE_GPIOZ_TOGGLE = crate::Reg<oe_gpioz_toggle::OE_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO direction toggle"]
pub mod oe_gpioz_toggle;
#[doc = "IF_GPIOA_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOA_VALUE_SPEC>`"]
pub type IF_GPIOA_VALUE = crate::Reg<if_gpioa_value::IF_GPIOA_VALUE_SPEC>;
#[doc = "GPIO interrupt flag value"]
pub mod if_gpioa_value;
#[doc = "IF_GPIOA_SET (rw) register accessor: an alias for `Reg<IF_GPIOA_SET_SPEC>`"]
pub type IF_GPIOA_SET = crate::Reg<if_gpioa_set::IF_GPIOA_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpioa_set;
#[doc = "IF_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOA_CLEAR_SPEC>`"]
pub type IF_GPIOA_CLEAR = crate::Reg<if_gpioa_clear::IF_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpioa_clear;
#[doc = "IF_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOA_TOGGLE_SPEC>`"]
pub type IF_GPIOA_TOGGLE = crate::Reg<if_gpioa_toggle::IF_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpioa_toggle;
#[doc = "IF_GPIOB_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOB_VALUE_SPEC>`"]
pub type IF_GPIOB_VALUE = crate::Reg<if_gpiob_value::IF_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB interrupt flag"]
pub mod if_gpiob_value;
#[doc = "IF_GPIOB_SET (rw) register accessor: an alias for `Reg<IF_GPIOB_SET_SPEC>`"]
pub type IF_GPIOB_SET = crate::Reg<if_gpiob_set::IF_GPIOB_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpiob_set;
#[doc = "IF_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOB_CLEAR_SPEC>`"]
pub type IF_GPIOB_CLEAR = crate::Reg<if_gpiob_clear::IF_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpiob_clear;
#[doc = "IF_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOB_TOGGLE_SPEC>`"]
pub type IF_GPIOB_TOGGLE = crate::Reg<if_gpiob_toggle::IF_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpiob_toggle;
#[doc = "IF_GPIOC_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOC_VALUE_SPEC>`"]
pub type IF_GPIOC_VALUE = crate::Reg<if_gpioc_value::IF_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC interrupt flag"]
pub mod if_gpioc_value;
#[doc = "IF_GPIOC_SET (rw) register accessor: an alias for `Reg<IF_GPIOC_SET_SPEC>`"]
pub type IF_GPIOC_SET = crate::Reg<if_gpioc_set::IF_GPIOC_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpioc_set;
#[doc = "IF_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOC_CLEAR_SPEC>`"]
pub type IF_GPIOC_CLEAR = crate::Reg<if_gpioc_clear::IF_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpioc_clear;
#[doc = "IF_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOC_TOGGLE_SPEC>`"]
pub type IF_GPIOC_TOGGLE = crate::Reg<if_gpioc_toggle::IF_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpioc_toggle;
#[doc = "IF_GPIOD_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOD_VALUE_SPEC>`"]
pub type IF_GPIOD_VALUE = crate::Reg<if_gpiod_value::IF_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD interrupt flag"]
pub mod if_gpiod_value;
#[doc = "IF_GPIOD_SET (rw) register accessor: an alias for `Reg<IF_GPIOD_SET_SPEC>`"]
pub type IF_GPIOD_SET = crate::Reg<if_gpiod_set::IF_GPIOD_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpiod_set;
#[doc = "IF_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOD_CLEAR_SPEC>`"]
pub type IF_GPIOD_CLEAR = crate::Reg<if_gpiod_clear::IF_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpiod_clear;
#[doc = "IF_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOD_TOGGLE_SPEC>`"]
pub type IF_GPIOD_TOGGLE = crate::Reg<if_gpiod_toggle::IF_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpiod_toggle;
#[doc = "IF_GPIOE_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOE_VALUE_SPEC>`"]
pub type IF_GPIOE_VALUE = crate::Reg<if_gpioe_value::IF_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE interrupt flag"]
pub mod if_gpioe_value;
#[doc = "IF_GPIOE_SET (rw) register accessor: an alias for `Reg<IF_GPIOE_SET_SPEC>`"]
pub type IF_GPIOE_SET = crate::Reg<if_gpioe_set::IF_GPIOE_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpioe_set;
#[doc = "IF_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOE_CLEAR_SPEC>`"]
pub type IF_GPIOE_CLEAR = crate::Reg<if_gpioe_clear::IF_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpioe_clear;
#[doc = "IF_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOE_TOGGLE_SPEC>`"]
pub type IF_GPIOE_TOGGLE = crate::Reg<if_gpioe_toggle::IF_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpioe_toggle;
#[doc = "IF_GPIOF_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOF_VALUE_SPEC>`"]
pub type IF_GPIOF_VALUE = crate::Reg<if_gpiof_value::IF_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF interrupt flag"]
pub mod if_gpiof_value;
#[doc = "IF_GPIOF_SET (rw) register accessor: an alias for `Reg<IF_GPIOF_SET_SPEC>`"]
pub type IF_GPIOF_SET = crate::Reg<if_gpiof_set::IF_GPIOF_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpiof_set;
#[doc = "IF_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOF_CLEAR_SPEC>`"]
pub type IF_GPIOF_CLEAR = crate::Reg<if_gpiof_clear::IF_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpiof_clear;
#[doc = "IF_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOF_TOGGLE_SPEC>`"]
pub type IF_GPIOF_TOGGLE = crate::Reg<if_gpiof_toggle::IF_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpiof_toggle;
#[doc = "IF_GPIOX_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOX_VALUE_SPEC>`"]
pub type IF_GPIOX_VALUE = crate::Reg<if_gpiox_value::IF_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX interrupt flag"]
pub mod if_gpiox_value;
#[doc = "IF_GPIOX_SET (rw) register accessor: an alias for `Reg<IF_GPIOX_SET_SPEC>`"]
pub type IF_GPIOX_SET = crate::Reg<if_gpiox_set::IF_GPIOX_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpiox_set;
#[doc = "IF_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOX_CLEAR_SPEC>`"]
pub type IF_GPIOX_CLEAR = crate::Reg<if_gpiox_clear::IF_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpiox_clear;
#[doc = "IF_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOX_TOGGLE_SPEC>`"]
pub type IF_GPIOX_TOGGLE = crate::Reg<if_gpiox_toggle::IF_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpiox_toggle;
#[doc = "IF_GPIOY_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOY_VALUE_SPEC>`"]
pub type IF_GPIOY_VALUE = crate::Reg<if_gpioy_value::IF_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY interrupt flag"]
pub mod if_gpioy_value;
#[doc = "IF_GPIOY_SET (rw) register accessor: an alias for `Reg<IF_GPIOY_SET_SPEC>`"]
pub type IF_GPIOY_SET = crate::Reg<if_gpioy_set::IF_GPIOY_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpioy_set;
#[doc = "IF_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOY_CLEAR_SPEC>`"]
pub type IF_GPIOY_CLEAR = crate::Reg<if_gpioy_clear::IF_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpioy_clear;
#[doc = "IF_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOY_TOGGLE_SPEC>`"]
pub type IF_GPIOY_TOGGLE = crate::Reg<if_gpioy_toggle::IF_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpioy_toggle;
#[doc = "IF_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<IF_GPIOZ_VALUE_SPEC>`"]
pub type IF_GPIOZ_VALUE = crate::Reg<if_gpioz_value::IF_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ interrupt flag"]
pub mod if_gpioz_value;
#[doc = "IF_GPIOZ_SET (rw) register accessor: an alias for `Reg<IF_GPIOZ_SET_SPEC>`"]
pub type IF_GPIOZ_SET = crate::Reg<if_gpioz_set::IF_GPIOZ_SET_SPEC>;
#[doc = "GPIO interrupt flag set"]
pub mod if_gpioz_set;
#[doc = "IF_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<IF_GPIOZ_CLEAR_SPEC>`"]
pub type IF_GPIOZ_CLEAR = crate::Reg<if_gpioz_clear::IF_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO interrupt flag clear"]
pub mod if_gpioz_clear;
#[doc = "IF_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<IF_GPIOZ_TOGGLE_SPEC>`"]
pub type IF_GPIOZ_TOGGLE = crate::Reg<if_gpioz_toggle::IF_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO interrupt flag toggle"]
pub mod if_gpioz_toggle;
#[doc = "IE_GPIOA_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOA_VALUE_SPEC>`"]
pub type IE_GPIOA_VALUE = crate::Reg<ie_gpioa_value::IE_GPIOA_VALUE_SPEC>;
#[doc = "GPIO interrupt enable value"]
pub mod ie_gpioa_value;
#[doc = "IE_GPIOA_SET (rw) register accessor: an alias for `Reg<IE_GPIOA_SET_SPEC>`"]
pub type IE_GPIOA_SET = crate::Reg<ie_gpioa_set::IE_GPIOA_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpioa_set;
#[doc = "IE_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOA_CLEAR_SPEC>`"]
pub type IE_GPIOA_CLEAR = crate::Reg<ie_gpioa_clear::IE_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpioa_clear;
#[doc = "IE_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOA_TOGGLE_SPEC>`"]
pub type IE_GPIOA_TOGGLE = crate::Reg<ie_gpioa_toggle::IE_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpioa_toggle;
#[doc = "IE_GPIOB_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOB_VALUE_SPEC>`"]
pub type IE_GPIOB_VALUE = crate::Reg<ie_gpiob_value::IE_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB interrupt enable"]
pub mod ie_gpiob_value;
#[doc = "IE_GPIOB_SET (rw) register accessor: an alias for `Reg<IE_GPIOB_SET_SPEC>`"]
pub type IE_GPIOB_SET = crate::Reg<ie_gpiob_set::IE_GPIOB_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpiob_set;
#[doc = "IE_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOB_CLEAR_SPEC>`"]
pub type IE_GPIOB_CLEAR = crate::Reg<ie_gpiob_clear::IE_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpiob_clear;
#[doc = "IE_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOB_TOGGLE_SPEC>`"]
pub type IE_GPIOB_TOGGLE = crate::Reg<ie_gpiob_toggle::IE_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpiob_toggle;
#[doc = "IE_GPIOC_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOC_VALUE_SPEC>`"]
pub type IE_GPIOC_VALUE = crate::Reg<ie_gpioc_value::IE_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC interrupt enable"]
pub mod ie_gpioc_value;
#[doc = "IE_GPIOC_SET (rw) register accessor: an alias for `Reg<IE_GPIOC_SET_SPEC>`"]
pub type IE_GPIOC_SET = crate::Reg<ie_gpioc_set::IE_GPIOC_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpioc_set;
#[doc = "IE_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOC_CLEAR_SPEC>`"]
pub type IE_GPIOC_CLEAR = crate::Reg<ie_gpioc_clear::IE_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpioc_clear;
#[doc = "IE_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOC_TOGGLE_SPEC>`"]
pub type IE_GPIOC_TOGGLE = crate::Reg<ie_gpioc_toggle::IE_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpioc_toggle;
#[doc = "IE_GPIOD_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOD_VALUE_SPEC>`"]
pub type IE_GPIOD_VALUE = crate::Reg<ie_gpiod_value::IE_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD interrupt enable"]
pub mod ie_gpiod_value;
#[doc = "IE_GPIOD_SET (rw) register accessor: an alias for `Reg<IE_GPIOD_SET_SPEC>`"]
pub type IE_GPIOD_SET = crate::Reg<ie_gpiod_set::IE_GPIOD_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpiod_set;
#[doc = "IE_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOD_CLEAR_SPEC>`"]
pub type IE_GPIOD_CLEAR = crate::Reg<ie_gpiod_clear::IE_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpiod_clear;
#[doc = "IE_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOD_TOGGLE_SPEC>`"]
pub type IE_GPIOD_TOGGLE = crate::Reg<ie_gpiod_toggle::IE_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpiod_toggle;
#[doc = "IE_GPIOE_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOE_VALUE_SPEC>`"]
pub type IE_GPIOE_VALUE = crate::Reg<ie_gpioe_value::IE_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE interrupt enable"]
pub mod ie_gpioe_value;
#[doc = "IE_GPIOE_SET (rw) register accessor: an alias for `Reg<IE_GPIOE_SET_SPEC>`"]
pub type IE_GPIOE_SET = crate::Reg<ie_gpioe_set::IE_GPIOE_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpioe_set;
#[doc = "IE_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOE_CLEAR_SPEC>`"]
pub type IE_GPIOE_CLEAR = crate::Reg<ie_gpioe_clear::IE_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpioe_clear;
#[doc = "IE_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOE_TOGGLE_SPEC>`"]
pub type IE_GPIOE_TOGGLE = crate::Reg<ie_gpioe_toggle::IE_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpioe_toggle;
#[doc = "IE_GPIOF_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOF_VALUE_SPEC>`"]
pub type IE_GPIOF_VALUE = crate::Reg<ie_gpiof_value::IE_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF interrupt enable"]
pub mod ie_gpiof_value;
#[doc = "IE_GPIOF_SET (rw) register accessor: an alias for `Reg<IE_GPIOF_SET_SPEC>`"]
pub type IE_GPIOF_SET = crate::Reg<ie_gpiof_set::IE_GPIOF_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpiof_set;
#[doc = "IE_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOF_CLEAR_SPEC>`"]
pub type IE_GPIOF_CLEAR = crate::Reg<ie_gpiof_clear::IE_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpiof_clear;
#[doc = "IE_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOF_TOGGLE_SPEC>`"]
pub type IE_GPIOF_TOGGLE = crate::Reg<ie_gpiof_toggle::IE_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpiof_toggle;
#[doc = "IE_GPIOX_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOX_VALUE_SPEC>`"]
pub type IE_GPIOX_VALUE = crate::Reg<ie_gpiox_value::IE_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX interrupt enable"]
pub mod ie_gpiox_value;
#[doc = "IE_GPIOX_SET (rw) register accessor: an alias for `Reg<IE_GPIOX_SET_SPEC>`"]
pub type IE_GPIOX_SET = crate::Reg<ie_gpiox_set::IE_GPIOX_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpiox_set;
#[doc = "IE_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOX_CLEAR_SPEC>`"]
pub type IE_GPIOX_CLEAR = crate::Reg<ie_gpiox_clear::IE_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpiox_clear;
#[doc = "IE_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOX_TOGGLE_SPEC>`"]
pub type IE_GPIOX_TOGGLE = crate::Reg<ie_gpiox_toggle::IE_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpiox_toggle;
#[doc = "IE_GPIOY_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOY_VALUE_SPEC>`"]
pub type IE_GPIOY_VALUE = crate::Reg<ie_gpioy_value::IE_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY interrupt enable"]
pub mod ie_gpioy_value;
#[doc = "IE_GPIOY_SET (rw) register accessor: an alias for `Reg<IE_GPIOY_SET_SPEC>`"]
pub type IE_GPIOY_SET = crate::Reg<ie_gpioy_set::IE_GPIOY_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpioy_set;
#[doc = "IE_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOY_CLEAR_SPEC>`"]
pub type IE_GPIOY_CLEAR = crate::Reg<ie_gpioy_clear::IE_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpioy_clear;
#[doc = "IE_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOY_TOGGLE_SPEC>`"]
pub type IE_GPIOY_TOGGLE = crate::Reg<ie_gpioy_toggle::IE_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpioy_toggle;
#[doc = "IE_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<IE_GPIOZ_VALUE_SPEC>`"]
pub type IE_GPIOZ_VALUE = crate::Reg<ie_gpioz_value::IE_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ interrupt enable"]
pub mod ie_gpioz_value;
#[doc = "IE_GPIOZ_SET (rw) register accessor: an alias for `Reg<IE_GPIOZ_SET_SPEC>`"]
pub type IE_GPIOZ_SET = crate::Reg<ie_gpioz_set::IE_GPIOZ_SET_SPEC>;
#[doc = "GPIO interrupt enable set"]
pub mod ie_gpioz_set;
#[doc = "IE_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<IE_GPIOZ_CLEAR_SPEC>`"]
pub type IE_GPIOZ_CLEAR = crate::Reg<ie_gpioz_clear::IE_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO interrupt enable clear"]
pub mod ie_gpioz_clear;
#[doc = "IE_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<IE_GPIOZ_TOGGLE_SPEC>`"]
pub type IE_GPIOZ_TOGGLE = crate::Reg<ie_gpioz_toggle::IE_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO interrupt enable toggle"]
pub mod ie_gpioz_toggle;
#[doc = "PL_GPIOA_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOA_VALUE_SPEC>`"]
pub type PL_GPIOA_VALUE = crate::Reg<pl_gpioa_value::PL_GPIOA_VALUE_SPEC>;
#[doc = "GPIO interrupt polarity value"]
pub mod pl_gpioa_value;
#[doc = "PL_GPIOA_SET (rw) register accessor: an alias for `Reg<PL_GPIOA_SET_SPEC>`"]
pub type PL_GPIOA_SET = crate::Reg<pl_gpioa_set::PL_GPIOA_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpioa_set;
#[doc = "PL_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOA_CLEAR_SPEC>`"]
pub type PL_GPIOA_CLEAR = crate::Reg<pl_gpioa_clear::PL_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpioa_clear;
#[doc = "PL_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOA_TOGGLE_SPEC>`"]
pub type PL_GPIOA_TOGGLE = crate::Reg<pl_gpioa_toggle::PL_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpioa_toggle;
#[doc = "PL_GPIOB_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOB_VALUE_SPEC>`"]
pub type PL_GPIOB_VALUE = crate::Reg<pl_gpiob_value::PL_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB interrupt polarity"]
pub mod pl_gpiob_value;
#[doc = "PL_GPIOB_SET (rw) register accessor: an alias for `Reg<PL_GPIOB_SET_SPEC>`"]
pub type PL_GPIOB_SET = crate::Reg<pl_gpiob_set::PL_GPIOB_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpiob_set;
#[doc = "PL_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOB_CLEAR_SPEC>`"]
pub type PL_GPIOB_CLEAR = crate::Reg<pl_gpiob_clear::PL_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpiob_clear;
#[doc = "PL_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOB_TOGGLE_SPEC>`"]
pub type PL_GPIOB_TOGGLE = crate::Reg<pl_gpiob_toggle::PL_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpiob_toggle;
#[doc = "PL_GPIOC_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOC_VALUE_SPEC>`"]
pub type PL_GPIOC_VALUE = crate::Reg<pl_gpioc_value::PL_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC interrupt polarity"]
pub mod pl_gpioc_value;
#[doc = "PL_GPIOC_SET (rw) register accessor: an alias for `Reg<PL_GPIOC_SET_SPEC>`"]
pub type PL_GPIOC_SET = crate::Reg<pl_gpioc_set::PL_GPIOC_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpioc_set;
#[doc = "PL_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOC_CLEAR_SPEC>`"]
pub type PL_GPIOC_CLEAR = crate::Reg<pl_gpioc_clear::PL_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpioc_clear;
#[doc = "PL_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOC_TOGGLE_SPEC>`"]
pub type PL_GPIOC_TOGGLE = crate::Reg<pl_gpioc_toggle::PL_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpioc_toggle;
#[doc = "PL_GPIOD_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOD_VALUE_SPEC>`"]
pub type PL_GPIOD_VALUE = crate::Reg<pl_gpiod_value::PL_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD interrupt polarity"]
pub mod pl_gpiod_value;
#[doc = "PL_GPIOD_SET (rw) register accessor: an alias for `Reg<PL_GPIOD_SET_SPEC>`"]
pub type PL_GPIOD_SET = crate::Reg<pl_gpiod_set::PL_GPIOD_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpiod_set;
#[doc = "PL_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOD_CLEAR_SPEC>`"]
pub type PL_GPIOD_CLEAR = crate::Reg<pl_gpiod_clear::PL_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpiod_clear;
#[doc = "PL_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOD_TOGGLE_SPEC>`"]
pub type PL_GPIOD_TOGGLE = crate::Reg<pl_gpiod_toggle::PL_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpiod_toggle;
#[doc = "PL_GPIOE_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOE_VALUE_SPEC>`"]
pub type PL_GPIOE_VALUE = crate::Reg<pl_gpioe_value::PL_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE interrupt polarity"]
pub mod pl_gpioe_value;
#[doc = "PL_GPIOE_SET (rw) register accessor: an alias for `Reg<PL_GPIOE_SET_SPEC>`"]
pub type PL_GPIOE_SET = crate::Reg<pl_gpioe_set::PL_GPIOE_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpioe_set;
#[doc = "PL_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOE_CLEAR_SPEC>`"]
pub type PL_GPIOE_CLEAR = crate::Reg<pl_gpioe_clear::PL_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpioe_clear;
#[doc = "PL_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOE_TOGGLE_SPEC>`"]
pub type PL_GPIOE_TOGGLE = crate::Reg<pl_gpioe_toggle::PL_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpioe_toggle;
#[doc = "PL_GPIOF_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOF_VALUE_SPEC>`"]
pub type PL_GPIOF_VALUE = crate::Reg<pl_gpiof_value::PL_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF interrupt polarity"]
pub mod pl_gpiof_value;
#[doc = "PL_GPIOF_SET (rw) register accessor: an alias for `Reg<PL_GPIOF_SET_SPEC>`"]
pub type PL_GPIOF_SET = crate::Reg<pl_gpiof_set::PL_GPIOF_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpiof_set;
#[doc = "PL_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOF_CLEAR_SPEC>`"]
pub type PL_GPIOF_CLEAR = crate::Reg<pl_gpiof_clear::PL_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpiof_clear;
#[doc = "PL_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOF_TOGGLE_SPEC>`"]
pub type PL_GPIOF_TOGGLE = crate::Reg<pl_gpiof_toggle::PL_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpiof_toggle;
#[doc = "PL_GPIOX_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOX_VALUE_SPEC>`"]
pub type PL_GPIOX_VALUE = crate::Reg<pl_gpiox_value::PL_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX interrupt polarity"]
pub mod pl_gpiox_value;
#[doc = "PL_GPIOX_SET (rw) register accessor: an alias for `Reg<PL_GPIOX_SET_SPEC>`"]
pub type PL_GPIOX_SET = crate::Reg<pl_gpiox_set::PL_GPIOX_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpiox_set;
#[doc = "PL_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOX_CLEAR_SPEC>`"]
pub type PL_GPIOX_CLEAR = crate::Reg<pl_gpiox_clear::PL_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpiox_clear;
#[doc = "PL_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOX_TOGGLE_SPEC>`"]
pub type PL_GPIOX_TOGGLE = crate::Reg<pl_gpiox_toggle::PL_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpiox_toggle;
#[doc = "PL_GPIOY_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOY_VALUE_SPEC>`"]
pub type PL_GPIOY_VALUE = crate::Reg<pl_gpioy_value::PL_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY interrupt polarity"]
pub mod pl_gpioy_value;
#[doc = "PL_GPIOY_SET (rw) register accessor: an alias for `Reg<PL_GPIOY_SET_SPEC>`"]
pub type PL_GPIOY_SET = crate::Reg<pl_gpioy_set::PL_GPIOY_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpioy_set;
#[doc = "PL_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOY_CLEAR_SPEC>`"]
pub type PL_GPIOY_CLEAR = crate::Reg<pl_gpioy_clear::PL_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpioy_clear;
#[doc = "PL_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOY_TOGGLE_SPEC>`"]
pub type PL_GPIOY_TOGGLE = crate::Reg<pl_gpioy_toggle::PL_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpioy_toggle;
#[doc = "PL_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<PL_GPIOZ_VALUE_SPEC>`"]
pub type PL_GPIOZ_VALUE = crate::Reg<pl_gpioz_value::PL_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ interrupt polarity"]
pub mod pl_gpioz_value;
#[doc = "PL_GPIOZ_SET (rw) register accessor: an alias for `Reg<PL_GPIOZ_SET_SPEC>`"]
pub type PL_GPIOZ_SET = crate::Reg<pl_gpioz_set::PL_GPIOZ_SET_SPEC>;
#[doc = "GPIO interrupt polarity set"]
pub mod pl_gpioz_set;
#[doc = "PL_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<PL_GPIOZ_CLEAR_SPEC>`"]
pub type PL_GPIOZ_CLEAR = crate::Reg<pl_gpioz_clear::PL_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO interrupt polarity clear"]
pub mod pl_gpioz_clear;
#[doc = "PL_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<PL_GPIOZ_TOGGLE_SPEC>`"]
pub type PL_GPIOZ_TOGGLE = crate::Reg<pl_gpioz_toggle::PL_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO interrupt polarity toggle"]
pub mod pl_gpioz_toggle;
#[doc = "TP_GPIOA_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOA_VALUE_SPEC>`"]
pub type TP_GPIOA_VALUE = crate::Reg<tp_gpioa_value::TP_GPIOA_VALUE_SPEC>;
#[doc = "GPIO interrupt type value"]
pub mod tp_gpioa_value;
#[doc = "TP_GPIOA_SET (rw) register accessor: an alias for `Reg<TP_GPIOA_SET_SPEC>`"]
pub type TP_GPIOA_SET = crate::Reg<tp_gpioa_set::TP_GPIOA_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpioa_set;
#[doc = "TP_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOA_CLEAR_SPEC>`"]
pub type TP_GPIOA_CLEAR = crate::Reg<tp_gpioa_clear::TP_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpioa_clear;
#[doc = "TP_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOA_TOGGLE_SPEC>`"]
pub type TP_GPIOA_TOGGLE = crate::Reg<tp_gpioa_toggle::TP_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpioa_toggle;
#[doc = "TP_GPIOB_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOB_VALUE_SPEC>`"]
pub type TP_GPIOB_VALUE = crate::Reg<tp_gpiob_value::TP_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB interrupt type"]
pub mod tp_gpiob_value;
#[doc = "TP_GPIOB_SET (rw) register accessor: an alias for `Reg<TP_GPIOB_SET_SPEC>`"]
pub type TP_GPIOB_SET = crate::Reg<tp_gpiob_set::TP_GPIOB_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpiob_set;
#[doc = "TP_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOB_CLEAR_SPEC>`"]
pub type TP_GPIOB_CLEAR = crate::Reg<tp_gpiob_clear::TP_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpiob_clear;
#[doc = "TP_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOB_TOGGLE_SPEC>`"]
pub type TP_GPIOB_TOGGLE = crate::Reg<tp_gpiob_toggle::TP_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpiob_toggle;
#[doc = "TP_GPIOC_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOC_VALUE_SPEC>`"]
pub type TP_GPIOC_VALUE = crate::Reg<tp_gpioc_value::TP_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC interrupt type"]
pub mod tp_gpioc_value;
#[doc = "TP_GPIOC_SET (rw) register accessor: an alias for `Reg<TP_GPIOC_SET_SPEC>`"]
pub type TP_GPIOC_SET = crate::Reg<tp_gpioc_set::TP_GPIOC_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpioc_set;
#[doc = "TP_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOC_CLEAR_SPEC>`"]
pub type TP_GPIOC_CLEAR = crate::Reg<tp_gpioc_clear::TP_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpioc_clear;
#[doc = "TP_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOC_TOGGLE_SPEC>`"]
pub type TP_GPIOC_TOGGLE = crate::Reg<tp_gpioc_toggle::TP_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpioc_toggle;
#[doc = "TP_GPIOD_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOD_VALUE_SPEC>`"]
pub type TP_GPIOD_VALUE = crate::Reg<tp_gpiod_value::TP_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD interrupt type"]
pub mod tp_gpiod_value;
#[doc = "TP_GPIOD_SET (rw) register accessor: an alias for `Reg<TP_GPIOD_SET_SPEC>`"]
pub type TP_GPIOD_SET = crate::Reg<tp_gpiod_set::TP_GPIOD_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpiod_set;
#[doc = "TP_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOD_CLEAR_SPEC>`"]
pub type TP_GPIOD_CLEAR = crate::Reg<tp_gpiod_clear::TP_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpiod_clear;
#[doc = "TP_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOD_TOGGLE_SPEC>`"]
pub type TP_GPIOD_TOGGLE = crate::Reg<tp_gpiod_toggle::TP_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpiod_toggle;
#[doc = "TP_GPIOE_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOE_VALUE_SPEC>`"]
pub type TP_GPIOE_VALUE = crate::Reg<tp_gpioe_value::TP_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE interrupt type"]
pub mod tp_gpioe_value;
#[doc = "TP_GPIOE_SET (rw) register accessor: an alias for `Reg<TP_GPIOE_SET_SPEC>`"]
pub type TP_GPIOE_SET = crate::Reg<tp_gpioe_set::TP_GPIOE_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpioe_set;
#[doc = "TP_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOE_CLEAR_SPEC>`"]
pub type TP_GPIOE_CLEAR = crate::Reg<tp_gpioe_clear::TP_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpioe_clear;
#[doc = "TP_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOE_TOGGLE_SPEC>`"]
pub type TP_GPIOE_TOGGLE = crate::Reg<tp_gpioe_toggle::TP_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpioe_toggle;
#[doc = "TP_GPIOF_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOF_VALUE_SPEC>`"]
pub type TP_GPIOF_VALUE = crate::Reg<tp_gpiof_value::TP_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF interrupt type"]
pub mod tp_gpiof_value;
#[doc = "TP_GPIOF_SET (rw) register accessor: an alias for `Reg<TP_GPIOF_SET_SPEC>`"]
pub type TP_GPIOF_SET = crate::Reg<tp_gpiof_set::TP_GPIOF_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpiof_set;
#[doc = "TP_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOF_CLEAR_SPEC>`"]
pub type TP_GPIOF_CLEAR = crate::Reg<tp_gpiof_clear::TP_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpiof_clear;
#[doc = "TP_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOF_TOGGLE_SPEC>`"]
pub type TP_GPIOF_TOGGLE = crate::Reg<tp_gpiof_toggle::TP_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpiof_toggle;
#[doc = "TP_GPIOX_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOX_VALUE_SPEC>`"]
pub type TP_GPIOX_VALUE = crate::Reg<tp_gpiox_value::TP_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX interrupt type"]
pub mod tp_gpiox_value;
#[doc = "TP_GPIOX_SET (rw) register accessor: an alias for `Reg<TP_GPIOX_SET_SPEC>`"]
pub type TP_GPIOX_SET = crate::Reg<tp_gpiox_set::TP_GPIOX_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpiox_set;
#[doc = "TP_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOX_CLEAR_SPEC>`"]
pub type TP_GPIOX_CLEAR = crate::Reg<tp_gpiox_clear::TP_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpiox_clear;
#[doc = "TP_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOX_TOGGLE_SPEC>`"]
pub type TP_GPIOX_TOGGLE = crate::Reg<tp_gpiox_toggle::TP_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpiox_toggle;
#[doc = "TP_GPIOY_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOY_VALUE_SPEC>`"]
pub type TP_GPIOY_VALUE = crate::Reg<tp_gpioy_value::TP_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY interrupt type"]
pub mod tp_gpioy_value;
#[doc = "TP_GPIOY_SET (rw) register accessor: an alias for `Reg<TP_GPIOY_SET_SPEC>`"]
pub type TP_GPIOY_SET = crate::Reg<tp_gpioy_set::TP_GPIOY_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpioy_set;
#[doc = "TP_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOY_CLEAR_SPEC>`"]
pub type TP_GPIOY_CLEAR = crate::Reg<tp_gpioy_clear::TP_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpioy_clear;
#[doc = "TP_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOY_TOGGLE_SPEC>`"]
pub type TP_GPIOY_TOGGLE = crate::Reg<tp_gpioy_toggle::TP_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpioy_toggle;
#[doc = "TP_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<TP_GPIOZ_VALUE_SPEC>`"]
pub type TP_GPIOZ_VALUE = crate::Reg<tp_gpioz_value::TP_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ interrupt type"]
pub mod tp_gpioz_value;
#[doc = "TP_GPIOZ_SET (rw) register accessor: an alias for `Reg<TP_GPIOZ_SET_SPEC>`"]
pub type TP_GPIOZ_SET = crate::Reg<tp_gpioz_set::TP_GPIOZ_SET_SPEC>;
#[doc = "GPIO interrupt type set"]
pub mod tp_gpioz_set;
#[doc = "TP_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<TP_GPIOZ_CLEAR_SPEC>`"]
pub type TP_GPIOZ_CLEAR = crate::Reg<tp_gpioz_clear::TP_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO interrupt type clear"]
pub mod tp_gpioz_clear;
#[doc = "TP_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<TP_GPIOZ_TOGGLE_SPEC>`"]
pub type TP_GPIOZ_TOGGLE = crate::Reg<tp_gpioz_toggle::TP_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO interrupt type toggle"]
pub mod tp_gpioz_toggle;
#[doc = "AS_GPIOA_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOA_VALUE_SPEC>`"]
pub type AS_GPIOA_VALUE = crate::Reg<as_gpioa_value::AS_GPIOA_VALUE_SPEC>;
#[doc = "GPIO interrupt asynchronous value"]
pub mod as_gpioa_value;
#[doc = "AS_GPIOA_SET (rw) register accessor: an alias for `Reg<AS_GPIOA_SET_SPEC>`"]
pub type AS_GPIOA_SET = crate::Reg<as_gpioa_set::AS_GPIOA_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpioa_set;
#[doc = "AS_GPIOA_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOA_CLEAR_SPEC>`"]
pub type AS_GPIOA_CLEAR = crate::Reg<as_gpioa_clear::AS_GPIOA_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpioa_clear;
#[doc = "AS_GPIOA_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOA_TOGGLE_SPEC>`"]
pub type AS_GPIOA_TOGGLE = crate::Reg<as_gpioa_toggle::AS_GPIOA_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpioa_toggle;
#[doc = "AS_GPIOB_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOB_VALUE_SPEC>`"]
pub type AS_GPIOB_VALUE = crate::Reg<as_gpiob_value::AS_GPIOB_VALUE_SPEC>;
#[doc = "GPIOB interrupt asynchronous"]
pub mod as_gpiob_value;
#[doc = "AS_GPIOB_SET (rw) register accessor: an alias for `Reg<AS_GPIOB_SET_SPEC>`"]
pub type AS_GPIOB_SET = crate::Reg<as_gpiob_set::AS_GPIOB_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpiob_set;
#[doc = "AS_GPIOB_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOB_CLEAR_SPEC>`"]
pub type AS_GPIOB_CLEAR = crate::Reg<as_gpiob_clear::AS_GPIOB_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpiob_clear;
#[doc = "AS_GPIOB_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOB_TOGGLE_SPEC>`"]
pub type AS_GPIOB_TOGGLE = crate::Reg<as_gpiob_toggle::AS_GPIOB_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpiob_toggle;
#[doc = "AS_GPIOC_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOC_VALUE_SPEC>`"]
pub type AS_GPIOC_VALUE = crate::Reg<as_gpioc_value::AS_GPIOC_VALUE_SPEC>;
#[doc = "GPIOC interrupt asynchronous"]
pub mod as_gpioc_value;
#[doc = "AS_GPIOC_SET (rw) register accessor: an alias for `Reg<AS_GPIOC_SET_SPEC>`"]
pub type AS_GPIOC_SET = crate::Reg<as_gpioc_set::AS_GPIOC_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpioc_set;
#[doc = "AS_GPIOC_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOC_CLEAR_SPEC>`"]
pub type AS_GPIOC_CLEAR = crate::Reg<as_gpioc_clear::AS_GPIOC_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpioc_clear;
#[doc = "AS_GPIOC_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOC_TOGGLE_SPEC>`"]
pub type AS_GPIOC_TOGGLE = crate::Reg<as_gpioc_toggle::AS_GPIOC_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpioc_toggle;
#[doc = "AS_GPIOD_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOD_VALUE_SPEC>`"]
pub type AS_GPIOD_VALUE = crate::Reg<as_gpiod_value::AS_GPIOD_VALUE_SPEC>;
#[doc = "GPIOD interrupt asynchronous"]
pub mod as_gpiod_value;
#[doc = "AS_GPIOD_SET (rw) register accessor: an alias for `Reg<AS_GPIOD_SET_SPEC>`"]
pub type AS_GPIOD_SET = crate::Reg<as_gpiod_set::AS_GPIOD_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpiod_set;
#[doc = "AS_GPIOD_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOD_CLEAR_SPEC>`"]
pub type AS_GPIOD_CLEAR = crate::Reg<as_gpiod_clear::AS_GPIOD_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpiod_clear;
#[doc = "AS_GPIOD_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOD_TOGGLE_SPEC>`"]
pub type AS_GPIOD_TOGGLE = crate::Reg<as_gpiod_toggle::AS_GPIOD_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpiod_toggle;
#[doc = "AS_GPIOE_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOE_VALUE_SPEC>`"]
pub type AS_GPIOE_VALUE = crate::Reg<as_gpioe_value::AS_GPIOE_VALUE_SPEC>;
#[doc = "GPIOE interrupt asynchronous"]
pub mod as_gpioe_value;
#[doc = "AS_GPIOE_SET (rw) register accessor: an alias for `Reg<AS_GPIOE_SET_SPEC>`"]
pub type AS_GPIOE_SET = crate::Reg<as_gpioe_set::AS_GPIOE_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpioe_set;
#[doc = "AS_GPIOE_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOE_CLEAR_SPEC>`"]
pub type AS_GPIOE_CLEAR = crate::Reg<as_gpioe_clear::AS_GPIOE_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpioe_clear;
#[doc = "AS_GPIOE_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOE_TOGGLE_SPEC>`"]
pub type AS_GPIOE_TOGGLE = crate::Reg<as_gpioe_toggle::AS_GPIOE_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpioe_toggle;
#[doc = "AS_GPIOF_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOF_VALUE_SPEC>`"]
pub type AS_GPIOF_VALUE = crate::Reg<as_gpiof_value::AS_GPIOF_VALUE_SPEC>;
#[doc = "GPIOF interrupt asynchronous"]
pub mod as_gpiof_value;
#[doc = "AS_GPIOF_SET (rw) register accessor: an alias for `Reg<AS_GPIOF_SET_SPEC>`"]
pub type AS_GPIOF_SET = crate::Reg<as_gpiof_set::AS_GPIOF_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpiof_set;
#[doc = "AS_GPIOF_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOF_CLEAR_SPEC>`"]
pub type AS_GPIOF_CLEAR = crate::Reg<as_gpiof_clear::AS_GPIOF_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpiof_clear;
#[doc = "AS_GPIOF_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOF_TOGGLE_SPEC>`"]
pub type AS_GPIOF_TOGGLE = crate::Reg<as_gpiof_toggle::AS_GPIOF_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpiof_toggle;
#[doc = "AS_GPIOX_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOX_VALUE_SPEC>`"]
pub type AS_GPIOX_VALUE = crate::Reg<as_gpiox_value::AS_GPIOX_VALUE_SPEC>;
#[doc = "GPIOX interrupt asynchronous"]
pub mod as_gpiox_value;
#[doc = "AS_GPIOX_SET (rw) register accessor: an alias for `Reg<AS_GPIOX_SET_SPEC>`"]
pub type AS_GPIOX_SET = crate::Reg<as_gpiox_set::AS_GPIOX_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpiox_set;
#[doc = "AS_GPIOX_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOX_CLEAR_SPEC>`"]
pub type AS_GPIOX_CLEAR = crate::Reg<as_gpiox_clear::AS_GPIOX_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpiox_clear;
#[doc = "AS_GPIOX_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOX_TOGGLE_SPEC>`"]
pub type AS_GPIOX_TOGGLE = crate::Reg<as_gpiox_toggle::AS_GPIOX_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpiox_toggle;
#[doc = "AS_GPIOY_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOY_VALUE_SPEC>`"]
pub type AS_GPIOY_VALUE = crate::Reg<as_gpioy_value::AS_GPIOY_VALUE_SPEC>;
#[doc = "GPIOY interrupt asynchronous"]
pub mod as_gpioy_value;
#[doc = "AS_GPIOY_SET (rw) register accessor: an alias for `Reg<AS_GPIOY_SET_SPEC>`"]
pub type AS_GPIOY_SET = crate::Reg<as_gpioy_set::AS_GPIOY_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpioy_set;
#[doc = "AS_GPIOY_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOY_CLEAR_SPEC>`"]
pub type AS_GPIOY_CLEAR = crate::Reg<as_gpioy_clear::AS_GPIOY_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpioy_clear;
#[doc = "AS_GPIOY_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOY_TOGGLE_SPEC>`"]
pub type AS_GPIOY_TOGGLE = crate::Reg<as_gpioy_toggle::AS_GPIOY_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpioy_toggle;
#[doc = "AS_GPIOZ_VALUE (rw) register accessor: an alias for `Reg<AS_GPIOZ_VALUE_SPEC>`"]
pub type AS_GPIOZ_VALUE = crate::Reg<as_gpioz_value::AS_GPIOZ_VALUE_SPEC>;
#[doc = "GPIOZ interrupt asynchronous"]
pub mod as_gpioz_value;
#[doc = "AS_GPIOZ_SET (rw) register accessor: an alias for `Reg<AS_GPIOZ_SET_SPEC>`"]
pub type AS_GPIOZ_SET = crate::Reg<as_gpioz_set::AS_GPIOZ_SET_SPEC>;
#[doc = "GPIO interrupt asynchronous set"]
pub mod as_gpioz_set;
#[doc = "AS_GPIOZ_CLEAR (rw) register accessor: an alias for `Reg<AS_GPIOZ_CLEAR_SPEC>`"]
pub type AS_GPIOZ_CLEAR = crate::Reg<as_gpioz_clear::AS_GPIOZ_CLEAR_SPEC>;
#[doc = "GPIO interrupt asynchronous clear"]
pub mod as_gpioz_clear;
#[doc = "AS_GPIOZ_TOGGLE (rw) register accessor: an alias for `Reg<AS_GPIOZ_TOGGLE_SPEC>`"]
pub type AS_GPIOZ_TOGGLE = crate::Reg<as_gpioz_toggle::AS_GPIOZ_TOGGLE_SPEC>;
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod as_gpioz_toggle;
