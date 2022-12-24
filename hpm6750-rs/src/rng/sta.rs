#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - when 1, means the RNG engine is busy for seeding or random number generation, self test and so on."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `IDLE` reader - Idle, the RNG is in the idle mode, and internal clocks are disabled, in this mode, access to the FIFO is allowed. Once the FIFO is empty, the RNGB fills the FIFO and then enters idle mode again."]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `RSDREQ` reader - Reseed needed Indicates that the RNG needs to be reseeded. This is done by setting the CMD\\[GS\\], or automatically if the CTRL\\[ARS\\]
is set."]
pub type RSDREQ_R = crate::BitReader<bool>;
#[doc = "Field `SCDN` reader - Self Check Done Indicates whether Self Test is done or not. Can be cleared by the hardware reset or a new self test is initiated by setting the CMD\\[ST\\]. 0 Self test not completed 1 Completed a self test since the last reset."]
pub type SCDN_R = crate::BitReader<bool>;
#[doc = "Field `FSDDN` reader - 1st Seed done When \"1\", Indicates that the RNG generated the first seed."]
pub type FSDDN_R = crate::BitReader<bool>;
#[doc = "Field `NSDDN` reader - New seed done."]
pub type NSDDN_R = crate::BitReader<bool>;
#[doc = "Field `FRNNU` reader - Fifo Level, Indicates the number of random words currently in the output FIFO"]
pub type FRNNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSIZE` reader - Fifo Size, it is 5 in this design."]
pub type FSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNCERR` reader - Error was detected, check ESR register for details"]
pub type FUNCERR_R = crate::BitReader<bool>;
#[doc = "Field `SCPF` reader - Self Check Pass Fail"]
pub type SCPF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 1 - when 1, means the RNG engine is busy for seeding or random number generation, self test and so on."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle, the RNG is in the idle mode, and internal clocks are disabled, in this mode, access to the FIFO is allowed. Once the FIFO is empty, the RNGB fills the FIFO and then enters idle mode again."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reseed needed Indicates that the RNG needs to be reseeded. This is done by setting the CMD\\[GS\\], or automatically if the CTRL\\[ARS\\]
is set."]
    #[inline(always)]
    pub fn rsdreq(&self) -> RSDREQ_R {
        RSDREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Self Check Done Indicates whether Self Test is done or not. Can be cleared by the hardware reset or a new self test is initiated by setting the CMD\\[ST\\]. 0 Self test not completed 1 Completed a self test since the last reset."]
    #[inline(always)]
    pub fn scdn(&self) -> SCDN_R {
        SCDN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1st Seed done When \"1\", Indicates that the RNG generated the first seed."]
    #[inline(always)]
    pub fn fsddn(&self) -> FSDDN_R {
        FSDDN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New seed done."]
    #[inline(always)]
    pub fn nsddn(&self) -> NSDDN_R {
        NSDDN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Fifo Level, Indicates the number of random words currently in the output FIFO"]
    #[inline(always)]
    pub fn frnnu(&self) -> FRNNU_R {
        FRNNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Fifo Size, it is 5 in this design."]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Error was detected, check ESR register for details"]
    #[inline(always)]
    pub fn funcerr(&self) -> FUNCERR_R {
        FUNCERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Self Check Pass Fail"]
    #[inline(always)]
    pub fn scpf(&self) -> SCPF_R {
        SCPF_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
