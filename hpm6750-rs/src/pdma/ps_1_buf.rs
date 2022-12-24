#[doc = "Register `PS_1_BUF` reader"]
pub struct R(crate::R<PS_1_BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_1_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_1_BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_1_BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_1_BUF` writer"]
pub struct W(crate::W<PS_1_BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_1_BUF_SPEC>;
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
impl From<crate::W<PS_1_BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_1_BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Address pointer for the PS RGB or Y (luma) input buffer."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Address pointer for the PS RGB or Y (luma) input buffer."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_BUF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address pointer for the PS RGB or Y (luma) input buffer."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address pointer for the PS RGB or Y (luma) input buffer."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer data buffer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_1_buf](index.html) module"]
pub struct PS_1_BUF_SPEC;
impl crate::RegisterSpec for PS_1_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_1_buf::R](R) reader structure"]
impl crate::Readable for PS_1_BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_1_buf::W](W) writer structure"]
impl crate::Writable for PS_1_BUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PS_1_BUF to value 0"]
impl crate::Resettable for PS_1_BUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
