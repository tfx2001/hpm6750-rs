#[doc = "Register `UVWCFG` reader"]
pub struct R(crate::R<UVWCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UVWCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UVWCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UVWCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UVWCFG` writer"]
pub struct W(crate::W<UVWCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UVWCFG_SPEC>;
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
impl From<crate::W<UVWCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UVWCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRECNT` reader - the clock cycle number which the pre flag will set before the next uvw transition"]
pub type PRECNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRECNT` writer - the clock cycle number which the pre flag will set before the next uvw transition"]
pub type PRECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UVWCFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - the clock cycle number which the pre flag will set before the next uvw transition"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - the clock cycle number which the pre flag will set before the next uvw transition"]
    #[inline(always)]
    pub fn precnt(&mut self) -> PRECNT_W<0> {
        PRECNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U,V,W configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uvwcfg](index.html) module"]
pub struct UVWCFG_SPEC;
impl crate::RegisterSpec for UVWCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uvwcfg::R](R) reader structure"]
impl crate::Readable for UVWCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uvwcfg::W](W) writer structure"]
impl crate::Writable for UVWCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UVWCFG to value 0"]
impl crate::Resettable for UVWCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
