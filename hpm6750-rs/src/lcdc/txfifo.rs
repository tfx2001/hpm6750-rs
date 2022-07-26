#[doc = "Register `TXFIFO` reader"]
pub struct R(crate::R<TXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFIFO` writer"]
pub struct W(crate::W<TXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFIFO_SPEC>;
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
impl From<crate::W<TXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRSH` reader - Threshold to start the lcd raster (0--0x7F)"]
pub type THRSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRSH` writer - Threshold to start the lcd raster (0--0x7F)"]
pub type THRSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFIFO_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Threshold to start the lcd raster (0--0x7F)"]
    #[inline(always)]
    pub fn thrsh(&self) -> THRSH_R {
        THRSH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold to start the lcd raster (0--0x7F)"]
    #[inline(always)]
    pub fn thrsh(&mut self) -> THRSH_W<0> {
        THRSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo](index.html) module"]
pub struct TXFIFO_SPEC;
impl crate::RegisterSpec for TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfifo::R](R) reader structure"]
impl crate::Readable for TXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfifo::W](W) writer structure"]
impl crate::Writable for TXFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TXFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
