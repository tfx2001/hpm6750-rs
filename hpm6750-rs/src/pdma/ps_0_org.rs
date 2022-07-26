#[doc = "Register `PS_0_ORG` reader"]
pub struct R(crate::R<PS_0_ORG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_0_ORG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_0_ORG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_0_ORG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_0_ORG` writer"]
pub struct W(crate::W<PS_0_ORG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_0_ORG_SPEC>;
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
impl From<crate::W<PS_0_ORG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_0_ORG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIGHT` reader - The number of vertical pixels of the original frame (not -1)"]
pub type HIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIGHT` writer - The number of vertical pixels of the original frame (not -1)"]
pub type HIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_0_ORG_SPEC, u16, u16, 14, O>;
#[doc = "Field `WIDTH` reader - The number of horizontal pixels of the original frame (not -1)"]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIDTH` writer - The number of horizontal pixels of the original frame (not -1)"]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_0_ORG_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 16:29 - The number of vertical pixels of the original frame (not -1)"]
    #[inline(always)]
    pub fn hight(&self) -> HIGHT_R {
        HIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 0:13 - The number of horizontal pixels of the original frame (not -1)"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:29 - The number of vertical pixels of the original frame (not -1)"]
    #[inline(always)]
    pub fn hight(&mut self) -> HIGHT_W<16> {
        HIGHT_W::new(self)
    }
    #[doc = "Bits 0:13 - The number of horizontal pixels of the original frame (not -1)"]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer original size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_0_org](index.html) module"]
pub struct PS_0_ORG_SPEC;
impl crate::RegisterSpec for PS_0_ORG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_0_org::R](R) reader structure"]
impl crate::Readable for PS_0_ORG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_0_org::W](W) writer structure"]
impl crate::Writable for PS_0_ORG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PS_0_ORG to value 0"]
impl crate::Resettable for PS_0_ORG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
