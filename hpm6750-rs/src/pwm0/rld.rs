#[doc = "Register `RLD` reader"]
pub struct R(crate::R<RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLD` writer"]
pub struct W(crate::W<RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLD_SPEC>;
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
impl From<crate::W<RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XRLD` reader - timeout counter extended reload point, counter will reload to xsta after reach this point"]
pub type XRLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XRLD` writer - timeout counter extended reload point, counter will reload to xsta after reach this point"]
pub type XRLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLD_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLD` reader - pwm timer counter reload value"]
pub type RLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLD` writer - pwm timer counter reload value"]
pub type RLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLD_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 28:31 - timeout counter extended reload point, counter will reload to xsta after reach this point"]
    #[inline(always)]
    pub fn xrld(&self) -> XRLD_R {
        XRLD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 4:27 - pwm timer counter reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new(((self.bits >> 4) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 28:31 - timeout counter extended reload point, counter will reload to xsta after reach this point"]
    #[inline(always)]
    pub fn xrld(&mut self) -> XRLD_W<28> {
        XRLD_W::new(self)
    }
    #[doc = "Bits 4:27 - pwm timer counter reload value"]
    #[inline(always)]
    pub fn rld(&mut self) -> RLD_W<4> {
        RLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rld](index.html) module"]
pub struct RLD_SPEC;
impl crate::RegisterSpec for RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rld::R](R) reader structure"]
impl crate::Readable for RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rld::W](W) writer structure"]
impl crate::Writable for RLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RLD to value 0"]
impl crate::Resettable for RLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
