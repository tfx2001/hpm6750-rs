#[doc = "Register `PTPC_0_TS_UPDTL` reader"]
pub struct R(crate::R<PTPC_0_TS_UPDTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_0_TS_UPDTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_0_TS_UPDTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_0_TS_UPDTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPC_0_TS_UPDTL` writer"]
pub struct W(crate::W<PTPC_0_TS_UPDTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPC_0_TS_UPDTL_SPEC>;
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
impl From<crate::W<PTPC_0_TS_UPDTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPC_0_TS_UPDTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD_SUB` reader - 1 for sub; 0 for add, used only at update"]
pub type ADD_SUB_R = crate::BitReader<bool>;
#[doc = "Field `ADD_SUB` writer - 1 for sub; 0 for add, used only at update"]
pub type ADD_SUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_0_TS_UPDTL_SPEC, bool, O>;
#[doc = "Field `NS_UPDATE` reader - No description avaiable"]
pub type NS_UPDATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NS_UPDATE` writer - No description avaiable"]
pub type NS_UPDATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PTPC_0_TS_UPDTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 31 - 1 for sub; 0 for add, used only at update"]
    #[inline(always)]
    pub fn add_sub(&self) -> ADD_SUB_R {
        ADD_SUB_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 0:30 - No description avaiable"]
    #[inline(always)]
    pub fn ns_update(&self) -> NS_UPDATE_R {
        NS_UPDATE_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 1 for sub; 0 for add, used only at update"]
    #[inline(always)]
    pub fn add_sub(&mut self) -> ADD_SUB_W<31> {
        ADD_SUB_W::new(self)
    }
    #[doc = "Bits 0:30 - No description avaiable"]
    #[inline(always)]
    pub fn ns_update(&mut self) -> NS_UPDATE_W<0> {
        NS_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "timestamp update low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_0_ts_updtl](index.html) module"]
pub struct PTPC_0_TS_UPDTL_SPEC;
impl crate::RegisterSpec for PTPC_0_TS_UPDTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_0_ts_updtl::R](R) reader structure"]
impl crate::Readable for PTPC_0_TS_UPDTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpc_0_ts_updtl::W](W) writer structure"]
impl crate::Writable for PTPC_0_TS_UPDTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPC_0_TS_UPDTL to value 0"]
impl crate::Resettable for PTPC_0_TS_UPDTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
