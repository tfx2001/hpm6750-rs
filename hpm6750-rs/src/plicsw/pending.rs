#[doc = "Register `PENDING` reader"]
pub struct R(crate::R<PENDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PENDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PENDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PENDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PENDING` writer"]
pub struct W(crate::W<PENDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PENDING_SPEC>;
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
impl From<crate::W<PENDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PENDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT` reader - writing 1 to trigger software interrupt"]
pub type INTERRUPT_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT` writer - writing 1 to trigger software interrupt"]
pub type INTERRUPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PENDING_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - writing 1 to trigger software interrupt"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - writing 1 to trigger software interrupt"]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<1> {
        INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pending](index.html) module"]
pub struct PENDING_SPEC;
impl crate::RegisterSpec for PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pending::R](R) reader structure"]
impl crate::Readable for PENDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pending::W](W) writer structure"]
impl crate::Writable for PENDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PENDING to value 0"]
impl crate::Resettable for PENDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
