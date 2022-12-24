#[doc = "Register `SUB_SNAP` reader"]
pub struct R(crate::R<SUB_SNAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUB_SNAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUB_SNAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUB_SNAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUB_SNAP` writer"]
pub struct W(crate::W<SUB_SNAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUB_SNAP_SPEC>;
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
impl From<crate::W<SUB_SNAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUB_SNAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUB_SNAP` reader - sub second snap shot, write to take snap shot"]
pub type SUB_SNAP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SUB_SNAP` writer - sub second snap shot, write to take snap shot"]
pub type SUB_SNAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUB_SNAP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - sub second snap shot, write to take snap shot"]
    #[inline(always)]
    pub fn sub_snap(&self) -> SUB_SNAP_R {
        SUB_SNAP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sub second snap shot, write to take snap shot"]
    #[inline(always)]
    #[must_use]
    pub fn sub_snap(&mut self) -> SUB_SNAP_W<0> {
        SUB_SNAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-second counter snap shot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sub_snap](index.html) module"]
pub struct SUB_SNAP_SPEC;
impl crate::RegisterSpec for SUB_SNAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sub_snap::R](R) reader structure"]
impl crate::Readable for SUB_SNAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sub_snap::W](W) writer structure"]
impl crate::Writable for SUB_SNAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUB_SNAP to value 0"]
impl crate::Resettable for SUB_SNAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
