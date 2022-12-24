#[doc = "Register `CHCTRL_CH0_LLPOINTER` reader"]
pub struct R(crate::R<CHCTRL_CH0_LLPOINTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRL_CH0_LLPOINTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRL_CH0_LLPOINTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRL_CH0_LLPOINTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRL_CH0_LLPOINTER` writer"]
pub struct W(crate::W<CHCTRL_CH0_LLPOINTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRL_CH0_LLPOINTER_SPEC>;
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
impl From<crate::W<CHCTRL_CH0_LLPOINTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRL_CH0_LLPOINTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLDBUSINFIDX` reader - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
pub type LLDBUSINFIDX_R = crate::BitReader<bool>;
#[doc = "Field `LLDBUSINFIDX` writer - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
pub type LLDBUSINFIDX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CHCTRL_CH0_LLPOINTER_SPEC, bool, O>;
#[doc = "Field `LLPOINTERL` reader - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
pub type LLPOINTERL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LLPOINTERL` writer - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
pub type LLPOINTERL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRL_CH0_LLPOINTER_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    #[inline(always)]
    pub fn lldbusinfidx(&self) -> LLDBUSINFIDX_R {
        LLDBUSINFIDX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:31 - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    #[inline(always)]
    pub fn llpointerl(&self) -> LLPOINTERL_R {
        LLPOINTERL_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    #[inline(always)]
    #[must_use]
    pub fn lldbusinfidx(&mut self) -> LLDBUSINFIDX_W<0> {
        LLDBUSINFIDX_W::new(self)
    }
    #[doc = "Bits 3:31 - Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    #[inline(always)]
    #[must_use]
    pub fn llpointerl(&mut self) -> LLPOINTERL_W<3> {
        LLPOINTERL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrl_ch0_llpointer](index.html) module"]
pub struct CHCTRL_CH0_LLPOINTER_SPEC;
impl crate::RegisterSpec for CHCTRL_CH0_LLPOINTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrl_ch0_llpointer::R](R) reader structure"]
impl crate::Readable for CHCTRL_CH0_LLPOINTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrl_ch0_llpointer::W](W) writer structure"]
impl crate::Writable for CHCTRL_CH0_LLPOINTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRL_CH0_LLPOINTER to value 0"]
impl crate::Resettable for CHCTRL_CH0_LLPOINTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
