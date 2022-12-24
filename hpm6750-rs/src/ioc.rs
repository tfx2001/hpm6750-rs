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
#[doc = "pad_a\\[%s\\]"]
pub use self::pad_a::PAD_A;
#[doc = r"Cluster"]
#[doc = "pad_a\\[%s\\]"]
pub mod pad_a;
pub use self::pad_a as pad_b;
pub use self::pad_a as pad_c;
pub use self::pad_a as pad_d;
pub use self::pad_a as pad_e;
pub use self::pad_a as pad_f;
pub use self::pad_a as pad_x;
pub use self::pad_a as pad_y;
pub use self::pad_a as pad_z;
#[doc = "pad_b\\[%s\\]"]
pub use self::PAD_A as PAD_B;
#[doc = "pad_c\\[%s\\]"]
pub use self::PAD_A as PAD_C;
#[doc = "pad_d\\[%s\\]"]
pub use self::PAD_A as PAD_D;
#[doc = "pad_e\\[%s\\]"]
pub use self::PAD_A as PAD_E;
#[doc = "pad_f\\[%s\\]"]
pub use self::PAD_A as PAD_F;
#[doc = "pad_x\\[%s\\]"]
pub use self::PAD_A as PAD_X;
#[doc = "pad_y\\[%s\\]"]
pub use self::PAD_A as PAD_Y;
#[doc = "pad_z\\[%s\\]"]
pub use self::PAD_A as PAD_Z;
