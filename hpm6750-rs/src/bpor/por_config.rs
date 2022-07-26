#[doc = "Register `POR_CONFIG` reader"]
pub struct R(crate::R<POR_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POR_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POR_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POR_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POR_CONFIG` writer"]
pub struct W(crate::W<POR_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POR_CONFIG_SPEC>;
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
impl From<crate::W<POR_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POR_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION` reader - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
pub type RETENTION_R = crate::BitReader<bool>;
#[doc = "Field `RETENTION` writer - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
pub type RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POR_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
    #[inline(always)]
    pub fn retention(&self) -> RETENTION_R {
        RETENTION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
    #[inline(always)]
    pub fn retention(&mut self) -> RETENTION_W<0> {
        RETENTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power on reset config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_config](index.html) module"]
pub struct POR_CONFIG_SPEC;
impl crate::RegisterSpec for POR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [por_config::R](R) reader structure"]
impl crate::Readable for POR_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [por_config::W](W) writer structure"]
impl crate::Writable for POR_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POR_CONFIG to value 0"]
impl crate::Resettable for POR_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
