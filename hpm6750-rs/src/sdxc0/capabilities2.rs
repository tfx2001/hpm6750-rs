#[doc = "Register `CAPABILITIES2` reader"]
pub struct R(crate::R<CAPABILITIES2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPABILITIES2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPABILITIES2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPABILITIES2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SDR50_SUPPORT` reader - SDR50 Support (UHS-I only) This bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not. Values: 0x0 (FALSE): SDR50 is not supported 0x1 (TRUE): SDR50 is supported"]
pub type SDR50_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `SDR104_SUPPORT` reader - SDR104 Support (UHS-I only) This bit mentions that SDR104 requires tuning. Values: 0x0 (FALSE): SDR104 is not supported 0x1 (TRUE): SDR104 is supported"]
pub type SDR104_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `DDR50_SUPPORT` reader - DDR50 Support (UHS-I only) Values: 0x0 (FALSE): DDR50 is not supported 0x1 (TRUE): DDR50 is supported"]
pub type DDR50_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `UHS2_SUPPORT` reader - UHS-II Support (UHS-II only) This bit indicates whether Host Controller supports UHS-II. Values: 0x0 (FALSE): UHS-II is not supported 0x1 (TRUE): UHS-II is supported"]
pub type UHS2_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `DRV_TYPEA` reader - Driver Type A Support (UHS-I only) This bit indicates support of Driver Type A for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type A is not supported 0x1 (TRUE): Driver Type A is supported"]
pub type DRV_TYPEA_R = crate::BitReader<bool>;
#[doc = "Field `DRV_TYPEC` reader - Driver Type C Support (UHS-I only) This bit indicates support of Driver Type C for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type C is not supported 0x1 (TRUE): Driver Type C is supported"]
pub type DRV_TYPEC_R = crate::BitReader<bool>;
#[doc = "Field `DRV_TYPED` reader - Driver Type D Support (UHS-I only) This bit indicates support of Driver Type D for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type D is not supported 0x1 (TRUE): Driver Type D is supported"]
pub type DRV_TYPED_R = crate::BitReader<bool>;
#[doc = "Field `RETUNE_CNT` reader - Timer Count for Re-Tuning (UHS-I only) 0x0: Re-Tuning Timer disabled 0x1: 1 seconds 0x2: 2 seconds 0x3: 4 seconds ........ 0xB: 1024 seconds 0xC: Reserved 0xD: Reserved 0xE: Reserved 0xF: Get information from other source"]
pub type RETUNE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USE_TUNING_SDR50` reader - Use Tuning for SDR50 (UHS-I only) Values: 0x0 (ZERO): SDR50 does not require tuning 0x1 (ONE): SDR50 requires tuning"]
pub type USE_TUNING_SDR50_R = crate::BitReader<bool>;
#[doc = "Field `RE_TUNING_MODES` reader - Re-Tuning Modes (UHS-I only) These bits select the re-tuning method and limit the maximum data length. Values: 0x0 (MODE1): Timer 0x1 (MODE2): Timer and Re-Tuning Request (Not supported) 0x2 (MODE3): Auto Re-Tuning (for transfer) 0x3 (RSVD_MODE): Reserved"]
pub type RE_TUNING_MODES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_MUL` reader - Clock Multiplier These bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator. 0x0: Clock Multiplier is not Supported 0x1: Clock Multiplier M = 2 0x2: Clock Multiplier M = 3 ......... 0xFF: Clock Multiplier M = 256"]
pub type CLK_MUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMA3_SUPPORT` reader - ADMA3 Support This bit indicates whether the Host Controller is capable of using ADMA3. Values: 0x0 (FALSE): ADMA3 not Supported 0x1 (TRUE): ADMA3 Supported"]
pub type ADMA3_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `VDD2_18V_SUPPORT` reader - 1.8V VDD2 Support This bit indicates support of VDD2 for the Host System. 0x0 (FALSE): 1.8V VDD2 is not Supported 0x1 (TRUE): 1.8V VDD2 is Supported"]
pub type VDD2_18V_SUPPORT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SDR50 Support (UHS-I only) This bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not. Values: 0x0 (FALSE): SDR50 is not supported 0x1 (TRUE): SDR50 is supported"]
    #[inline(always)]
    pub fn sdr50_support(&self) -> SDR50_SUPPORT_R {
        SDR50_SUPPORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support (UHS-I only) This bit mentions that SDR104 requires tuning. Values: 0x0 (FALSE): SDR104 is not supported 0x1 (TRUE): SDR104 is supported"]
    #[inline(always)]
    pub fn sdr104_support(&self) -> SDR104_SUPPORT_R {
        SDR104_SUPPORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support (UHS-I only) Values: 0x0 (FALSE): DDR50 is not supported 0x1 (TRUE): DDR50 is supported"]
    #[inline(always)]
    pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
        DDR50_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UHS-II Support (UHS-II only) This bit indicates whether Host Controller supports UHS-II. Values: 0x0 (FALSE): UHS-II is not supported 0x1 (TRUE): UHS-II is supported"]
    #[inline(always)]
    pub fn uhs2_support(&self) -> UHS2_SUPPORT_R {
        UHS2_SUPPORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support (UHS-I only) This bit indicates support of Driver Type A for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type A is not supported 0x1 (TRUE): Driver Type A is supported"]
    #[inline(always)]
    pub fn drv_typea(&self) -> DRV_TYPEA_R {
        DRV_TYPEA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support (UHS-I only) This bit indicates support of Driver Type C for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type C is not supported 0x1 (TRUE): Driver Type C is supported"]
    #[inline(always)]
    pub fn drv_typec(&self) -> DRV_TYPEC_R {
        DRV_TYPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support (UHS-I only) This bit indicates support of Driver Type D for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type D is not supported 0x1 (TRUE): Driver Type D is supported"]
    #[inline(always)]
    pub fn drv_typed(&self) -> DRV_TYPED_R {
        DRV_TYPED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning (UHS-I only) 0x0: Re-Tuning Timer disabled 0x1: 1 seconds 0x2: 2 seconds 0x3: 4 seconds ........ 0xB: 1024 seconds 0xC: Reserved 0xD: Reserved 0xE: Reserved 0xF: Get information from other source"]
    #[inline(always)]
    pub fn retune_cnt(&self) -> RETUNE_CNT_R {
        RETUNE_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50 (UHS-I only) Values: 0x0 (ZERO): SDR50 does not require tuning 0x1 (ONE): SDR50 requires tuning"]
    #[inline(always)]
    pub fn use_tuning_sdr50(&self) -> USE_TUNING_SDR50_R {
        USE_TUNING_SDR50_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Re-Tuning Modes (UHS-I only) These bits select the re-tuning method and limit the maximum data length. Values: 0x0 (MODE1): Timer 0x1 (MODE2): Timer and Re-Tuning Request (Not supported) 0x2 (MODE3): Auto Re-Tuning (for transfer) 0x3 (RSVD_MODE): Reserved"]
    #[inline(always)]
    pub fn re_tuning_modes(&self) -> RE_TUNING_MODES_R {
        RE_TUNING_MODES_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier These bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator. 0x0: Clock Multiplier is not Supported 0x1: Clock Multiplier M = 2 0x2: Clock Multiplier M = 3 ......... 0xFF: Clock Multiplier M = 256"]
    #[inline(always)]
    pub fn clk_mul(&self) -> CLK_MUL_R {
        CLK_MUL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 27 - ADMA3 Support This bit indicates whether the Host Controller is capable of using ADMA3. Values: 0x0 (FALSE): ADMA3 not Supported 0x1 (TRUE): ADMA3 Supported"]
    #[inline(always)]
    pub fn adma3_support(&self) -> ADMA3_SUPPORT_R {
        ADMA3_SUPPORT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1.8V VDD2 Support This bit indicates support of VDD2 for the Host System. 0x0 (FALSE): 1.8V VDD2 is not Supported 0x1 (TRUE): 1.8V VDD2 is Supported"]
    #[inline(always)]
    pub fn vdd2_18v_support(&self) -> VDD2_18V_SUPPORT_R {
        VDD2_18V_SUPPORT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capabilities2](index.html) module"]
pub struct CAPABILITIES2_SPEC;
impl crate::RegisterSpec for CAPABILITIES2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capabilities2::R](R) reader structure"]
impl crate::Readable for CAPABILITIES2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPABILITIES2 to value 0"]
impl crate::Resettable for CAPABILITIES2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
