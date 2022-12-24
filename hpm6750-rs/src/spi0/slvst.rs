#[doc = "Register `SLVST` reader"]
pub struct R(crate::R<SLVST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVST` writer"]
pub struct W(crate::W<SLVST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVST_SPEC>;
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
impl From<crate::W<SLVST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_STATUS` reader - User defined status flags"]
pub type USR_STATUS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USR_STATUS` writer - User defined status flags"]
pub type USR_STATUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLVST_SPEC, u16, u16, 16, O>;
#[doc = "Field `READY` reader - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `READY` writer - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVST_SPEC, bool, O>;
#[doc = "Field `OVERRUN` reader - Data overrun occurs in the last transaction"]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - Data overrun occurs in the last transaction"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVST_SPEC, bool, O>;
#[doc = "Field `UNDERRUN` reader - Data underrun occurs in the last transaction"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN` writer - Data underrun occurs in the last transaction"]
pub type UNDERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - User defined status flags"]
    #[inline(always)]
    pub fn usr_status(&self) -> USR_STATUS_R {
        USR_STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data overrun occurs in the last transaction"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data underrun occurs in the last transaction"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined status flags"]
    #[inline(always)]
    #[must_use]
    pub fn usr_status(&mut self) -> USR_STATUS_W<0> {
        USR_STATUS_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to indicate that the ATCSPI200 is ready for data transaction. When an SPI transaction other than slave status-reading command ends, this bit will be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<16> {
        READY_W::new(self)
    }
    #[doc = "Bit 17 - Data overrun occurs in the last transaction"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<17> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 18 - Data underrun occurs in the last transaction"]
    #[inline(always)]
    #[must_use]
    pub fn underrun(&mut self) -> UNDERRUN_W<18> {
        UNDERRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvst](index.html) module"]
pub struct SLVST_SPEC;
impl crate::RegisterSpec for SLVST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvst::R](R) reader structure"]
impl crate::Readable for SLVST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvst::W](W) writer structure"]
impl crate::Writable for SLVST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLVST to value 0"]
impl crate::Resettable for SLVST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
