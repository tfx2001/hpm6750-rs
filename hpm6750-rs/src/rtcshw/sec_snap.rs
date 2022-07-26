#[doc = "Register `SEC_SNAP` reader"]
pub struct R(crate::R<SEC_SNAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_SNAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_SNAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_SNAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_SNAP` writer"]
pub struct W(crate::W<SEC_SNAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_SNAP_SPEC>;
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
impl From<crate::W<SEC_SNAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_SNAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_SNAP` reader - second snap shot, write to take snap shot"]
pub type SEC_SNAP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEC_SNAP` writer - second snap shot, write to take snap shot"]
pub type SEC_SNAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEC_SNAP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - second snap shot, write to take snap shot"]
    #[inline(always)]
    pub fn sec_snap(&self) -> SEC_SNAP_R {
        SEC_SNAP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - second snap shot, write to take snap shot"]
    #[inline(always)]
    pub fn sec_snap(&mut self) -> SEC_SNAP_W<0> {
        SEC_SNAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second counter snap shot\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_snap](index.html) module"]
pub struct SEC_SNAP_SPEC;
impl crate::RegisterSpec for SEC_SNAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_snap::R](R) reader structure"]
impl crate::Readable for SEC_SNAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_snap::W](W) writer structure"]
impl crate::Writable for SEC_SNAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_SNAP to value 0"]
impl crate::Resettable for SEC_SNAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
