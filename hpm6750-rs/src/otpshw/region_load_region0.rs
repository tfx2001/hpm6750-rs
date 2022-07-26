#[doc = "Register `REGION_LOAD_REGION0` reader"]
pub struct R(crate::R<REGION_LOAD_REGION0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION_LOAD_REGION0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION_LOAD_REGION0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION_LOAD_REGION0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION_LOAD_REGION0` writer"]
pub struct W(crate::W<REGION_LOAD_REGION0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION_LOAD_REGION0_SPEC>;
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
impl From<crate::W<REGION_LOAD_REGION0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION_LOAD_REGION0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
pub type STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP` writer - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
pub type STOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REGION_LOAD_REGION0_SPEC, u8, u8, 7, O>;
#[doc = "Field `START` reader - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
pub type START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `START` writer - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
pub type START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REGION_LOAD_REGION0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 8:14 - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - stop address of load region, fuse word at end address will NOT be reloaded region0: fixed at 8 region1: fixed at 16 region2: fixed at 0, region3: usrer configurable"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<8> {
        STOP_W::new(self)
    }
    #[doc = "Bits 0:6 - start address of load region, fuse word at start address will be reloaded region0: fixed at 0 region1: fixed at 8 region2: fixed at 16, region3: usrer configurable"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOAD region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_load_region0](index.html) module"]
pub struct REGION_LOAD_REGION0_SPEC;
impl crate::RegisterSpec for REGION_LOAD_REGION0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region_load_region0::R](R) reader structure"]
impl crate::Readable for REGION_LOAD_REGION0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region_load_region0::W](W) writer structure"]
impl crate::Writable for REGION_LOAD_REGION0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGION_LOAD_REGION0 to value 0x0800"]
impl crate::Resettable for REGION_LOAD_REGION0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
