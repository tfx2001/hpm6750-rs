#[doc = "Register `PTPC_0_PPS_CTRL` reader"]
pub struct R(crate::R<PTPC_0_PPS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_0_PPS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_0_PPS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_0_PPS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPC_0_PPS_CTRL` writer"]
pub struct W(crate::W<PTPC_0_PPS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPC_0_PPS_CTRL_SPEC>;
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
impl From<crate::W<PTPC_0_PPS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPC_0_PPS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPS_CTRL` reader - No description avaiable"]
pub type PPS_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPS_CTRL` writer - No description avaiable"]
pub type PPS_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PTPC_0_PPS_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - No description avaiable"]
    #[inline(always)]
    pub fn pps_ctrl(&self) -> PPS_CTRL_R {
        PPS_CTRL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn pps_ctrl(&mut self) -> PPS_CTRL_W<0> {
        PPS_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_0_pps_ctrl](index.html) module"]
pub struct PTPC_0_PPS_CTRL_SPEC;
impl crate::RegisterSpec for PTPC_0_PPS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_0_pps_ctrl::R](R) reader structure"]
impl crate::Readable for PTPC_0_PPS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpc_0_pps_ctrl::W](W) writer structure"]
impl crate::Writable for PTPC_0_PPS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPC_0_PPS_CTRL to value 0"]
impl crate::Resettable for PTPC_0_PPS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
