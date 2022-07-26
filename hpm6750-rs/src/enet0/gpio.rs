#[doc = "Register `GPIO` reader"]
pub struct R(crate::R<GPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO` writer"]
pub struct W(crate::W<GPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_SPEC>;
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
impl From<crate::W<GPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIT` reader - No description avaiable"]
pub type GPIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIT` writer - No description avaiable"]
pub type GPIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_SPEC, u8, u8, 4, O>;
#[doc = "Field `GPIE` reader - No description avaiable"]
pub type GPIE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIE` writer - No description avaiable"]
pub type GPIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_SPEC, u8, u8, 4, O>;
#[doc = "Field `GPO` reader - No description avaiable"]
pub type GPO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPO` writer - No description avaiable"]
pub type GPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_SPEC, u8, u8, 4, O>;
#[doc = "Field `GPIS` reader - No description avaiable"]
pub type GPIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIS` writer - No description avaiable"]
pub type GPIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 24:27 - No description avaiable"]
    #[inline(always)]
    pub fn gpit(&self) -> GPIT_R {
        GPIT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - No description avaiable"]
    #[inline(always)]
    pub fn gpie(&self) -> GPIE_R {
        GPIE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - No description avaiable"]
    #[inline(always)]
    pub fn gpo(&self) -> GPO_R {
        GPO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - No description avaiable"]
    #[inline(always)]
    pub fn gpis(&self) -> GPIS_R {
        GPIS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - No description avaiable"]
    #[inline(always)]
    pub fn gpit(&mut self) -> GPIT_W<24> {
        GPIT_W::new(self)
    }
    #[doc = "Bits 16:19 - No description avaiable"]
    #[inline(always)]
    pub fn gpie(&mut self) -> GPIE_W<16> {
        GPIE_W::new(self)
    }
    #[doc = "Bits 8:11 - No description avaiable"]
    #[inline(always)]
    pub fn gpo(&mut self) -> GPO_W<8> {
        GPO_W::new(self)
    }
    #[doc = "Bits 0:3 - No description avaiable"]
    #[inline(always)]
    pub fn gpis(&mut self) -> GPIS_W<0> {
        GPIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose IO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio](index.html) module"]
pub struct GPIO_SPEC;
impl crate::RegisterSpec for GPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio::R](R) reader structure"]
impl crate::Readable for GPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio::W](W) writer structure"]
impl crate::Writable for GPIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO to value 0"]
impl crate::Resettable for GPIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
