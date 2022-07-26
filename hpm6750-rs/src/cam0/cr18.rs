#[doc = "Register `CR18` reader"]
pub struct R(crate::R<CR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR18` writer"]
pub struct W(crate::W<CR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR18_SPEC>;
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
impl From<crate::W<CR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAM_ENABLE` reader - CAM global enable signal. Only when this bit is 1, CAM can start to receive the data and store to memory."]
pub type CAM_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CAM_ENABLE` writer - CAM global enable signal. Only when this bit is 1, CAM can start to receive the data and store to memory."]
pub type CAM_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR18_SPEC, bool, O>;
#[doc = "Field `AWQOS` reader - AWQOS for bus fabric arbitration"]
pub type AWQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQOS` writer - AWQOS for bus fabric arbitration"]
pub type AWQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR18_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 31 - CAM global enable signal. Only when this bit is 1, CAM can start to receive the data and store to memory."]
    #[inline(always)]
    pub fn cam_enable(&self) -> CAM_ENABLE_R {
        CAM_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 7:10 - AWQOS for bus fabric arbitration"]
    #[inline(always)]
    pub fn awqos(&self) -> AWQOS_R {
        AWQOS_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - CAM global enable signal. Only when this bit is 1, CAM can start to receive the data and store to memory."]
    #[inline(always)]
    pub fn cam_enable(&mut self) -> CAM_ENABLE_W<31> {
        CAM_ENABLE_W::new(self)
    }
    #[doc = "Bits 7:10 - AWQOS for bus fabric arbitration"]
    #[inline(always)]
    pub fn awqos(&mut self) -> AWQOS_W<7> {
        AWQOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control CR18 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr18](index.html) module"]
pub struct CR18_SPEC;
impl crate::RegisterSpec for CR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr18::R](R) reader structure"]
impl crate::Readable for CR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr18::W](W) writer structure"]
impl crate::Writable for CR18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR18 to value 0"]
impl crate::Resettable for CR18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
