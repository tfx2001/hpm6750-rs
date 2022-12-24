#[doc = "Register `CHANNEL_CHN3_IRQEN` reader"]
pub struct R(crate::R<CHANNEL_CHN3_IRQEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CHN3_IRQEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CHN3_IRQEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CHN3_IRQEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CHN3_IRQEN` writer"]
pub struct W(crate::W<CHANNEL_CHN3_IRQEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CHN3_IRQEN_SPEC>;
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
impl From<crate::W<CHANNEL_CHN3_IRQEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CHN3_IRQEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDGEN` reader - Output rising edge flag interrupt enable bit."]
pub type REDGEN_R = crate::BitReader<bool>;
#[doc = "Field `REDGEN` writer - Output rising edge flag interrupt enable bit."]
pub type REDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN3_IRQEN_SPEC, bool, O>;
#[doc = "Field `FEDGEN` reader - Output falling edge flag interrupt enable bit."]
pub type FEDGEN_R = crate::BitReader<bool>;
#[doc = "Field `FEDGEN` writer - Output falling edge flag interrupt enable bit."]
pub type FEDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN3_IRQEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Output rising edge flag interrupt enable bit."]
    #[inline(always)]
    pub fn redgen(&self) -> REDGEN_R {
        REDGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output falling edge flag interrupt enable bit."]
    #[inline(always)]
    pub fn fedgen(&self) -> FEDGEN_R {
        FEDGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output rising edge flag interrupt enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn redgen(&mut self) -> REDGEN_W<0> {
        REDGEN_W::new(self)
    }
    #[doc = "Bit 1 - Output falling edge flag interrupt enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn fedgen(&mut self) -> FEDGEN_W<1> {
        FEDGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_chn3_irqen](index.html) module"]
pub struct CHANNEL_CHN3_IRQEN_SPEC;
impl crate::RegisterSpec for CHANNEL_CHN3_IRQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_chn3_irqen::R](R) reader structure"]
impl crate::Readable for CHANNEL_CHN3_IRQEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_chn3_irqen::W](W) writer structure"]
impl crate::Writable for CHANNEL_CHN3_IRQEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL_CHN3_IRQEN to value 0"]
impl crate::Resettable for CHANNEL_CHN3_IRQEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
