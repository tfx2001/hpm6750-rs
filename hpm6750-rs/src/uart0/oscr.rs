#[doc = "Register `OSCR` reader"]
pub struct R(crate::R<OSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCR` writer"]
pub struct W(crate::W<OSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCR_SPEC>;
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
impl From<crate::W<OSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC` reader - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC"]
pub type OSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC` writer - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC"]
pub type OSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W<0> {
        OSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Over Sample Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscr](index.html) module"]
pub struct OSCR_SPEC;
impl crate::RegisterSpec for OSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscr::R](R) reader structure"]
impl crate::Readable for OSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscr::W](W) writer structure"]
impl crate::Writable for OSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCR to value 0x10"]
impl crate::Resettable for OSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
