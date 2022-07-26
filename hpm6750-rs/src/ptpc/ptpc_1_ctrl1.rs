#[doc = "Register `PTPC_1_CTRL1` reader"]
pub struct R(crate::R<PTPC_1_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_1_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_1_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_1_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPC_1_CTRL1` writer"]
pub struct W(crate::W<PTPC_1_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPC_1_CTRL1_SPEC>;
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
impl From<crate::W<PTPC_1_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPC_1_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS_INCR` reader - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
pub type SS_INCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SS_INCR` writer - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
pub type SS_INCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPC_1_CTRL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
    #[inline(always)]
    pub fn ss_incr(&self) -> SS_INCR_R {
        SS_INCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
    #[inline(always)]
    pub fn ss_incr(&mut self) -> SS_INCR_W<0> {
        SS_INCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_1_ctrl1](index.html) module"]
pub struct PTPC_1_CTRL1_SPEC;
impl crate::RegisterSpec for PTPC_1_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_1_ctrl1::R](R) reader structure"]
impl crate::Readable for PTPC_1_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpc_1_ctrl1::W](W) writer structure"]
impl crate::Writable for PTPC_1_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPC_1_CTRL1 to value 0"]
impl crate::Resettable for PTPC_1_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
