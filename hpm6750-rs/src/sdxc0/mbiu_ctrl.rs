#[doc = "Register `MBIU_CTRL` reader"]
pub struct R(crate::R<MBIU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBIU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBIU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBIU_CTRL` writer"]
pub struct W(crate::W<MBIU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIU_CTRL_SPEC>;
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
impl From<crate::W<MBIU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBIU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BURST_INCR16_EN` reader - No description avaiable"]
pub type BURST_INCR16_EN_R = crate::BitReader<bool>;
#[doc = "Field `BURST_INCR16_EN` writer - No description avaiable"]
pub type BURST_INCR16_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIU_CTRL_SPEC, bool, O>;
#[doc = "Field `BURST_INCR8_EN` reader - No description avaiable"]
pub type BURST_INCR8_EN_R = crate::BitReader<bool>;
#[doc = "Field `BURST_INCR8_EN` writer - No description avaiable"]
pub type BURST_INCR8_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIU_CTRL_SPEC, bool, O>;
#[doc = "Field `BUSRT_INCR4_EN` reader - No description avaiable"]
pub type BUSRT_INCR4_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUSRT_INCR4_EN` writer - No description avaiable"]
pub type BUSRT_INCR4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIU_CTRL_SPEC, bool, O>;
#[doc = "Field `UNDEFL_INCR_EN` reader - No description avaiable"]
pub type UNDEFL_INCR_EN_R = crate::BitReader<bool>;
#[doc = "Field `UNDEFL_INCR_EN` writer - No description avaiable"]
pub type UNDEFL_INCR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIU_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn burst_incr16_en(&self) -> BURST_INCR16_EN_R {
        BURST_INCR16_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn burst_incr8_en(&self) -> BURST_INCR8_EN_R {
        BURST_INCR8_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn busrt_incr4_en(&self) -> BUSRT_INCR4_EN_R {
        BUSRT_INCR4_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn undefl_incr_en(&self) -> UNDEFL_INCR_EN_R {
        UNDEFL_INCR_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn burst_incr16_en(&mut self) -> BURST_INCR16_EN_W<3> {
        BURST_INCR16_EN_W::new(self)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn burst_incr8_en(&mut self) -> BURST_INCR8_EN_W<2> {
        BURST_INCR8_EN_W::new(self)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn busrt_incr4_en(&mut self) -> BUSRT_INCR4_EN_W<1> {
        BUSRT_INCR4_EN_W::new(self)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn undefl_incr_en(&mut self) -> UNDEFL_INCR_EN_W<0> {
        UNDEFL_INCR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Y\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbiu_ctrl](index.html) module"]
pub struct MBIU_CTRL_SPEC;
impl crate::RegisterSpec for MBIU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbiu_ctrl::R](R) reader structure"]
impl crate::Readable for MBIU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbiu_ctrl::W](W) writer structure"]
impl crate::Writable for MBIU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBIU_CTRL to value 0"]
impl crate::Resettable for MBIU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
