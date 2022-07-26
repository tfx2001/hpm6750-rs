#[doc = "Register `CQCAP` reader"]
pub struct R(crate::R<CQCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRYPTO_SUPPORT` reader - Crypto Support This bit indicates whether the Host Controller supports cryptographic operations. Values: 0x0 (FALSE): Crypto not Supported 0x1 (TRUE): Crypto Supported"]
pub type CRYPTO_SUPPORT_R = crate::BitReader<bool>;
#[doc = "Field `ITCFMUL` reader - Internal Timer Clock Frequency Multiplier (ITCFMUL) This field indicates the frequency of the clock used for interrupt coalescing timer and for determining the SQS polling period. See ITCFVAL definition for details. Values 0x5 to 0xF are reserved. Values: 0x0 (CLK_1KHz): 1KHz clock 0x1 (CLK_10KHz): 10KHz clock 0x2 (CLK_100KHz): 100KHz clock 0x3 (CLK_1MHz): 1MHz clock 0x4 (CLK_10MHz): 10MHz clock"]
pub type ITCFMUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITCFVAL` reader - Internal Timer Clock Frequency Value (ITCFVAL) This field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
pub type ITCFVAL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 28 - Crypto Support This bit indicates whether the Host Controller supports cryptographic operations. Values: 0x0 (FALSE): Crypto not Supported 0x1 (TRUE): Crypto Supported"]
    #[inline(always)]
    pub fn crypto_support(&self) -> CRYPTO_SUPPORT_R {
        CRYPTO_SUPPORT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Internal Timer Clock Frequency Multiplier (ITCFMUL) This field indicates the frequency of the clock used for interrupt coalescing timer and for determining the SQS polling period. See ITCFVAL definition for details. Values 0x5 to 0xF are reserved. Values: 0x0 (CLK_1KHz): 1KHz clock 0x1 (CLK_10KHz): 10KHz clock 0x2 (CLK_100KHz): 100KHz clock 0x3 (CLK_1MHz): 1MHz clock 0x4 (CLK_10MHz): 10MHz clock"]
    #[inline(always)]
    pub fn itcfmul(&self) -> ITCFMUL_R {
        ITCFMUL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - Internal Timer Clock Frequency Value (ITCFVAL) This field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
    #[inline(always)]
    pub fn itcfval(&self) -> ITCFVAL_R {
        ITCFVAL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcap](index.html) module"]
pub struct CQCAP_SPEC;
impl crate::RegisterSpec for CQCAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcap::R](R) reader structure"]
impl crate::Readable for CQCAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CQCAP to value 0"]
impl crate::Resettable for CQCAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
