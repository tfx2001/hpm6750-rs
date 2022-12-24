#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable or disable the watchdog timer 0: Disable 1: Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable or disable the watchdog timer 0: Disable 1: Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CLKSEL` reader - Clock source of timer: 0: EXTCLK 1: PCLK"]
pub type CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CLKSEL` writer - Clock source of timer: 0: EXTCLK 1: PCLK"]
pub type CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INTEN` reader - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `INTEN` writer - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RSTEN` reader - Enable or disable the watchdog reset 0: Disable 1: Enable"]
pub type RSTEN_R = crate::BitReader<bool>;
#[doc = "Field `RSTEN` writer - Enable or disable the watchdog reset 0: Disable 1: Enable"]
pub type RSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INTTIME` reader - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
pub type INTTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTTIME` writer - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
pub type INTTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSTTIME` reader - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
pub type RSTTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTTIME` writer - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
pub type RSTTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable or disable the watchdog timer 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock source of timer: 0: EXTCLK 1: PCLK"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable the watchdog reset 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn rsten(&self) -> RSTEN_R {
        RSTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
    #[inline(always)]
    pub fn inttime(&self) -> INTTIME_R {
        INTTIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
    #[inline(always)]
    pub fn rsttime(&self) -> RSTTIME_R {
        RSTTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable the watchdog timer 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Clock source of timer: 0: EXTCLK 1: PCLK"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<1> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<2> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable the watchdog reset 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rsten(&mut self) -> RSTEN_W<3> {
        RSTEN_W::new(self)
    }
    #[doc = "Bits 4:7 - The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
    #[inline(always)]
    #[must_use]
    pub fn inttime(&mut self) -> INTTIME_W<4> {
        INTTIME_W::new(self)
    }
    #[doc = "Bits 8:10 - The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
    #[inline(always)]
    #[must_use]
    pub fn rsttime(&mut self) -> RSTTIME_W<8> {
        RSTTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
