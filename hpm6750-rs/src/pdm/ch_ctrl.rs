#[doc = "Register `CH_CTRL` reader"]
pub struct R(crate::R<CH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_CTRL` writer"]
pub struct W(crate::W<CH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CTRL_SPEC>;
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
impl From<crate::W<CH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_EN` reader - Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
pub type CH_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH_EN` writer - Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
pub type CH_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CTRL_SPEC, u16, u16, 10, O>;
#[doc = "Field `CH_POL` reader - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
pub type CH_POL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_POL` writer - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
pub type CH_POL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:9 - Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
    #[inline(always)]
    pub fn ch_en(&self) -> CH_EN_R {
        CH_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
    #[inline(always)]
    pub fn ch_pol(&self) -> CH_POL_R {
        CH_POL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
    #[inline(always)]
    #[must_use]
    pub fn ch_en(&mut self) -> CH_EN_W<0> {
        CH_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
    #[inline(always)]
    #[must_use]
    pub fn ch_pol(&mut self) -> CH_POL_W<16> {
        CH_POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ctrl](index.html) module"]
pub struct CH_CTRL_SPEC;
impl crate::RegisterSpec for CH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_ctrl::R](R) reader structure"]
impl crate::Readable for CH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_ctrl::W](W) writer structure"]
impl crate::Writable for CH_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_CTRL to value 0"]
impl crate::Resettable for CH_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
