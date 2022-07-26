#[doc = "Register `BGND_CL` reader"]
pub struct R(crate::R<BGND_CL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGND_CL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGND_CL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGND_CL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGND_CL` writer"]
pub struct W(crate::W<BGND_CL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGND_CL_SPEC>;
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
impl From<crate::W<BGND_CL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGND_CL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R` reader - Red component of the default color displayed in the sectors where no layer is active."]
pub type R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `R` writer - Red component of the default color displayed in the sectors where no layer is active."]
pub type R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGND_CL_SPEC, u8, u8, 8, O>;
#[doc = "Field `G` reader - Green component of the default color displayed in the sectors where no layer is active."]
pub type G_R = crate::FieldReader<u8, u8>;
#[doc = "Field `G` writer - Green component of the default color displayed in the sectors where no layer is active."]
pub type G_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGND_CL_SPEC, u8, u8, 8, O>;
#[doc = "Field `B` reader - Blue component of the default color displayed in the sectors where no layer is active."]
pub type B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B` writer - Blue component of the default color displayed in the sectors where no layer is active."]
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGND_CL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - Red component of the default color displayed in the sectors where no layer is active."]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green component of the default color displayed in the sectors where no layer is active."]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Blue component of the default color displayed in the sectors where no layer is active."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Red component of the default color displayed in the sectors where no layer is active."]
    #[inline(always)]
    pub fn r(&mut self) -> R_W<16> {
        R_W::new(self)
    }
    #[doc = "Bits 8:15 - Green component of the default color displayed in the sectors where no layer is active."]
    #[inline(always)]
    pub fn g(&mut self) -> G_W<8> {
        G_W::new(self)
    }
    #[doc = "Bits 0:7 - Blue component of the default color displayed in the sectors where no layer is active."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<0> {
        B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Color Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgnd_cl](index.html) module"]
pub struct BGND_CL_SPEC;
impl crate::RegisterSpec for BGND_CL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgnd_cl::R](R) reader structure"]
impl crate::Readable for BGND_CL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgnd_cl::W](W) writer structure"]
impl crate::Writable for BGND_CL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGND_CL to value 0"]
impl crate::Resettable for BGND_CL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
