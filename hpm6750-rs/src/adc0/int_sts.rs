#[doc = "Register `INT_STS` reader"]
pub struct R(crate::R<INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STS` writer"]
pub struct W(crate::W<INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG_CMPT` reader - interrupt for one trigger conversion complete if enabled"]
pub type TRIG_CMPT_R = crate::BitReader<bool>;
#[doc = "Field `TRIG_CMPT` writer - interrupt for one trigger conversion complete if enabled"]
pub type TRIG_CMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `TRIG_SW_CFLCT` reader - No description avaiable"]
pub type TRIG_SW_CFLCT_R = crate::BitReader<bool>;
#[doc = "Field `TRIG_SW_CFLCT` writer - No description avaiable"]
pub type TRIG_SW_CFLCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `TRIG_HW_CFLCT` reader - No description avaiable"]
pub type TRIG_HW_CFLCT_R = crate::BitReader<bool>;
#[doc = "Field `TRIG_HW_CFLCT` writer - No description avaiable"]
pub type TRIG_HW_CFLCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `READ_CFLCT` reader - read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
pub type READ_CFLCT_R = crate::BitReader<bool>;
#[doc = "Field `READ_CFLCT` writer - read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
pub type READ_CFLCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `SEQ_SW_CFLCT` reader - sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
pub type SEQ_SW_CFLCT_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_SW_CFLCT` writer - sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
pub type SEQ_SW_CFLCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `SEQ_HW_CFLCT` reader - No description avaiable"]
pub type SEQ_HW_CFLCT_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_HW_CFLCT` writer - No description avaiable"]
pub type SEQ_HW_CFLCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `SEQ_DMAABT` reader - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
pub type SEQ_DMAABT_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_DMAABT` writer - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
pub type SEQ_DMAABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `SEQ_CMPT` reader - the whole sequence complete interrupt"]
pub type SEQ_CMPT_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_CMPT` writer - the whole sequence complete interrupt"]
pub type SEQ_CMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `SEQ_CVC` reader - one conversion complete in seq_queue if related seq_int_en is set"]
pub type SEQ_CVC_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_CVC` writer - one conversion complete in seq_queue if related seq_int_en is set"]
pub type SEQ_CVC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `DMA_FIFO_FULL` reader - No description avaiable"]
pub type DMA_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `DMA_FIFO_FULL` writer - No description avaiable"]
pub type DMA_FIFO_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `AHB_ERR` reader - set if got hresp=1"]
pub type AHB_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AHB_ERR` writer - set if got hresp=1"]
pub type AHB_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STS_SPEC, bool, O>;
#[doc = "Field `WDOG` reader - set if one chanel watch dog event triggered"]
pub type WDOG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDOG` writer - set if one chanel watch dog event triggered"]
pub type WDOG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT_STS_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bit 31 - interrupt for one trigger conversion complete if enabled"]
    #[inline(always)]
    pub fn trig_cmpt(&self) -> TRIG_CMPT_R {
        TRIG_CMPT_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - No description avaiable"]
    #[inline(always)]
    pub fn trig_sw_cflct(&self) -> TRIG_SW_CFLCT_R {
        TRIG_SW_CFLCT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - No description avaiable"]
    #[inline(always)]
    pub fn trig_hw_cflct(&self) -> TRIG_HW_CFLCT_R {
        TRIG_HW_CFLCT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
    #[inline(always)]
    pub fn read_cflct(&self) -> READ_CFLCT_R {
        READ_CFLCT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
    #[inline(always)]
    pub fn seq_sw_cflct(&self) -> SEQ_SW_CFLCT_R {
        SEQ_SW_CFLCT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - No description avaiable"]
    #[inline(always)]
    pub fn seq_hw_cflct(&self) -> SEQ_HW_CFLCT_R {
        SEQ_HW_CFLCT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
    #[inline(always)]
    pub fn seq_dmaabt(&self) -> SEQ_DMAABT_R {
        SEQ_DMAABT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - the whole sequence complete interrupt"]
    #[inline(always)]
    pub fn seq_cmpt(&self) -> SEQ_CMPT_R {
        SEQ_CMPT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - one conversion complete in seq_queue if related seq_int_en is set"]
    #[inline(always)]
    pub fn seq_cvc(&self) -> SEQ_CVC_R {
        SEQ_CVC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - No description avaiable"]
    #[inline(always)]
    pub fn dma_fifo_full(&self) -> DMA_FIFO_FULL_R {
        DMA_FIFO_FULL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - set if got hresp=1"]
    #[inline(always)]
    pub fn ahb_err(&self) -> AHB_ERR_R {
        AHB_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 0:18 - set if one chanel watch dog event triggered"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - interrupt for one trigger conversion complete if enabled"]
    #[inline(always)]
    pub fn trig_cmpt(&mut self) -> TRIG_CMPT_W<31> {
        TRIG_CMPT_W::new(self)
    }
    #[doc = "Bit 30 - No description avaiable"]
    #[inline(always)]
    pub fn trig_sw_cflct(&mut self) -> TRIG_SW_CFLCT_W<30> {
        TRIG_SW_CFLCT_W::new(self)
    }
    #[doc = "Bit 29 - No description avaiable"]
    #[inline(always)]
    pub fn trig_hw_cflct(&mut self) -> TRIG_HW_CFLCT_W<29> {
        TRIG_HW_CFLCT_W::new(self)
    }
    #[doc = "Bit 28 - read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
    #[inline(always)]
    pub fn read_cflct(&mut self) -> READ_CFLCT_W<28> {
        READ_CFLCT_W::new(self)
    }
    #[doc = "Bit 27 - sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
    #[inline(always)]
    pub fn seq_sw_cflct(&mut self) -> SEQ_SW_CFLCT_W<27> {
        SEQ_SW_CFLCT_W::new(self)
    }
    #[doc = "Bit 26 - No description avaiable"]
    #[inline(always)]
    pub fn seq_hw_cflct(&mut self) -> SEQ_HW_CFLCT_W<26> {
        SEQ_HW_CFLCT_W::new(self)
    }
    #[doc = "Bit 25 - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
    #[inline(always)]
    pub fn seq_dmaabt(&mut self) -> SEQ_DMAABT_W<25> {
        SEQ_DMAABT_W::new(self)
    }
    #[doc = "Bit 24 - the whole sequence complete interrupt"]
    #[inline(always)]
    pub fn seq_cmpt(&mut self) -> SEQ_CMPT_W<24> {
        SEQ_CMPT_W::new(self)
    }
    #[doc = "Bit 23 - one conversion complete in seq_queue if related seq_int_en is set"]
    #[inline(always)]
    pub fn seq_cvc(&mut self) -> SEQ_CVC_W<23> {
        SEQ_CVC_W::new(self)
    }
    #[doc = "Bit 22 - No description avaiable"]
    #[inline(always)]
    pub fn dma_fifo_full(&mut self) -> DMA_FIFO_FULL_W<22> {
        DMA_FIFO_FULL_W::new(self)
    }
    #[doc = "Bit 21 - set if got hresp=1"]
    #[inline(always)]
    pub fn ahb_err(&mut self) -> AHB_ERR_W<21> {
        AHB_ERR_W::new(self)
    }
    #[doc = "Bits 0:18 - set if one chanel watch dog event triggered"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WDOG_W<0> {
        WDOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_sts](index.html) module"]
pub struct INT_STS_SPEC;
impl crate::RegisterSpec for INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_sts::R](R) reader structure"]
impl crate::Readable for INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_sts::W](W) writer structure"]
impl crate::Writable for INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_STS to value 0"]
impl crate::Resettable for INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
