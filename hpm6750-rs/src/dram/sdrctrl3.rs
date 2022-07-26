#[doc = "Register `SDRCTRL3` reader"]
pub struct R(crate::R<SDRCTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRCTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRCTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRCTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRCTRL3` writer"]
pub struct W(crate::W<SDRCTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRCTRL3_SPEC>;
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
impl From<crate::W<SDRCTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRCTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UT` reader - Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period"]
pub type UT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UT` writer - Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period"]
pub type UT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL3_SPEC, u8, u8, 8, O>;
#[doc = "Field `RT` reader - Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period"]
pub type RT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RT` writer - Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period"]
pub type RT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRESCALE` reader - Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles"]
pub type PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALE` writer - Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles"]
pub type PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL3_SPEC, u8, u8, 8, O>;
#[doc = "Field `REBL` reader - Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8"]
pub type REBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REBL` writer - Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8"]
pub type REBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL3_SPEC, u8, u8, 3, O>;
#[doc = "Field `REN` reader - Refresh enable"]
pub type REN_R = crate::BitReader<bool>;
#[doc = "Field `REN` writer - Refresh enable"]
pub type REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRCTRL3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:31 - Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period"]
    #[inline(always)]
    pub fn ut(&self) -> UT_R {
        UT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 1:3 - Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8"]
    #[inline(always)]
    pub fn rebl(&self) -> REBL_R {
        REBL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 0 - Refresh enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period"]
    #[inline(always)]
    pub fn ut(&mut self) -> UT_W<24> {
        UT_W::new(self)
    }
    #[doc = "Bits 16:23 - Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W<16> {
        RT_W::new(self)
    }
    #[doc = "Bits 8:15 - Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W<8> {
        PRESCALE_W::new(self)
    }
    #[doc = "Bits 1:3 - Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8"]
    #[inline(always)]
    pub fn rebl(&mut self) -> REBL_W<1> {
        REBL_W::new(self)
    }
    #[doc = "Bit 0 - Refresh enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<0> {
        REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrctrl3](index.html) module"]
pub struct SDRCTRL3_SPEC;
impl crate::RegisterSpec for SDRCTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrctrl3::R](R) reader structure"]
impl crate::Readable for SDRCTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrctrl3::W](W) writer structure"]
impl crate::Writable for SDRCTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRCTRL3 to value 0"]
impl crate::Resettable for SDRCTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
