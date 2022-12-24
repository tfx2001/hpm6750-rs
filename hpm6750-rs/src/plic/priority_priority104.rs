#[doc = "Register `PRIORITY_PRIORITY104` reader"]
pub struct R(crate::R<PRIORITY_PRIORITY104_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIORITY_PRIORITY104_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIORITY_PRIORITY104_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIORITY_PRIORITY104_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIORITY_PRIORITY104` writer"]
pub struct W(crate::W<PRIORITY_PRIORITY104_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIORITY_PRIORITY104_SPEC>;
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
impl From<crate::W<PRIORITY_PRIORITY104_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIORITY_PRIORITY104_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY` reader - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
pub type PRIORITY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRIORITY` writer - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
pub type PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRIORITY_PRIORITY104_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt source priority. The valid range of this field is 0-7. 0: Never interrupt 1-7: Interrupt source priority. The larger the value, the higher the priority."]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PRIORITY_W<0> {
        PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priority_priority104](index.html) module"]
pub struct PRIORITY_PRIORITY104_SPEC;
impl crate::RegisterSpec for PRIORITY_PRIORITY104_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priority_priority104::R](R) reader structure"]
impl crate::Readable for PRIORITY_PRIORITY104_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priority_priority104::W](W) writer structure"]
impl crate::Writable for PRIORITY_PRIORITY104_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIORITY_PRIORITY104 to value 0x01"]
impl crate::Resettable for PRIORITY_PRIORITY104_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
