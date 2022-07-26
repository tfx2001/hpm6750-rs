#[doc = "Register `OFIFO_CTRL` reader"]
pub struct R(crate::R<OFIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFIFO_CTRL` writer"]
pub struct W(crate::W<OFIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFIFO_CTRL_SPEC>;
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
impl From<crate::W<OFIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Asserted to enable OFIFO"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Asserted to enable OFIFO"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFIFO_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Asserted to enable OFIFO"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asserted to enable OFIFO"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofifo_ctrl](index.html) module"]
pub struct OFIFO_CTRL_SPEC;
impl crate::RegisterSpec for OFIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofifo_ctrl::R](R) reader structure"]
impl crate::Readable for OFIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofifo_ctrl::W](W) writer structure"]
impl crate::Writable for OFIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFIFO_CTRL to value 0"]
impl crate::Resettable for OFIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
