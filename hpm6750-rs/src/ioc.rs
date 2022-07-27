#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x100 - pad_a\\[%s\\]"]
    pub pad_a: [PAD_A; 32],
    #[doc = "0x100..0x200 - pad_b\\[%s\\]"]
    pub pad_b: [PAD_B; 32],
    #[doc = "0x200..0x300 - pad_c\\[%s\\]"]
    pub pad_c: [PAD_C; 32],
    #[doc = "0x300..0x400 - pad_d\\[%s\\]"]
    pub pad_d: [PAD_D; 32],
    #[doc = "0x400..0x500 - pad_e\\[%s\\]"]
    pub pad_e: [PAD_E; 32],
    #[doc = "0x500..0x558 - pad_f\\[%s\\]"]
    pub pad_f: [PAD_F; 11],
    _reserved6: [u8; 0x07a8],
    #[doc = "0xd00..0xd60 - pad_x\\[%s\\]"]
    pub pad_x: [PAD_X; 12],
    _reserved7: [u8; 0xa0],
    #[doc = "0xe00..0xe60 - pad_y\\[%s\\]"]
    pub pad_y: [PAD_Y; 12],
    _reserved8: [u8; 0xa0],
    #[doc = "0xf00..0xf60 - pad_z\\[%s\\]"]
    pub pad_z: [PAD_Z; 12],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_A {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_a::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_a::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_a\\[%s\\]"]
pub mod pad_a;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_B {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_b::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_b::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_b\\[%s\\]"]
pub mod pad_b;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_C {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_c::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_c::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_c\\[%s\\]"]
pub mod pad_c;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_D {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_d::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_d::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_d\\[%s\\]"]
pub mod pad_d;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_E {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_e::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_e::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_e\\[%s\\]"]
pub mod pad_e;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_F {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_f::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_f::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_f\\[%s\\]"]
pub mod pad_f;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_X {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_x::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_x::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_x\\[%s\\]"]
pub mod pad_x;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_Y {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_y::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_y::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_y\\[%s\\]"]
pub mod pad_y;
#[doc = r"Register block"]
#[repr(C)]
pub struct PAD_Z {
    #[doc = "0x00 - Select function for this pad"]
    pub function: crate::Reg<self::pad_z::function::FUNCTION_SPEC>,
    #[doc = "0x04 - Configurate pad settings"]
    pub pad: crate::Reg<self::pad_z::pad::PAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "pad_z\\[%s\\]"]
pub mod pad_z;
