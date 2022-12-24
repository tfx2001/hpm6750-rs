#[doc = "Register `INT_EN` reader"]
pub struct R(crate::R<INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN` writer"]
pub struct W(crate::W<INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SPEC>;
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
impl From<crate::W<INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
pub type LOAD_R = crate::BitReader<bool>;
#[doc = "Field `LOAD` writer - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `READ` reader - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
pub type READ_R = crate::BitReader<bool>;
#[doc = "Field `READ` writer - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `WRITE` reader - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - fuse load interrupt enable 0: fuse load interrupt is not enable 1: fuse load interrupt is enable"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 1 - fuse read interrupt enable 0: fuse read interrupt is not enable 1: fuse read interrupt is enable"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<1> {
        READ_W::new(self)
    }
    #[doc = "Bit 2 - fuse write interrupt enable 0: fuse write interrupt is not enable 1: fuse write interrupt is enable"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<2> {
        WRITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](index.html) module"]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en::R](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en::W](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
