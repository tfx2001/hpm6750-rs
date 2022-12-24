#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCMDDONE` reader - IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type IPCMDDONE_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDDONE` writer - IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type IPCMDDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `IPCMDERR` reader - IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type IPCMDERR_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDERR` writer - IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type IPCMDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `AXICMDERR` reader - AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type AXICMDERR_R = crate::BitReader<bool>;
#[doc = "Field `AXICMDERR` writer - AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type AXICMDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `AXIBUSERR` reader - AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type AXIBUSERR_R = crate::BitReader<bool>;
#[doc = "Field `AXIBUSERR` writer - AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
pub type AXIBUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IPCMDDONE_R {
        IPCMDDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IPCMDERR_R {
        IPCMDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    pub fn axicmderr(&self) -> AXICMDERR_R {
        AXICMDERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    pub fn axibuserr(&self) -> AXIBUSERR_R {
        AXIBUSERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddone(&mut self) -> IPCMDDONE_W<0> {
        IPCMDDONE_W::new(self)
    }
    #[doc = "Bit 1 - IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderr(&mut self) -> IPCMDERR_W<1> {
        IPCMDERR_W::new(self)
    }
    #[doc = "Bit 2 - AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn axicmderr(&mut self) -> AXICMDERR_W<2> {
        AXICMDERR_W::new(self)
    }
    #[doc = "Bit 3 - AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn axibuserr(&mut self) -> AXIBUSERR_W<3> {
        AXIBUSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
