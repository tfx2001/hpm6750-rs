#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AXIBUSERR` reader - AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read"]
pub type AXIBUSERR_R = crate::BitReader<bool>;
#[doc = "Field `AXIBUSERR` writer - AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read"]
pub type AXIBUSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `AXICMDERR` reader - AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
pub type AXICMDERR_R = crate::BitReader<bool>;
#[doc = "Field `AXICMDERR` writer - AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
pub type AXICMDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `IPCMDERR` reader - IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command"]
pub type IPCMDERR_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDERR` writer - IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command"]
pub type IPCMDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `IPCMDDONE` reader - IP command normal done interrupt"]
pub type IPCMDDONE_R = crate::BitReader<bool>;
#[doc = "Field `IPCMDDONE` writer - IP command normal done interrupt"]
pub type IPCMDDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read"]
    #[inline(always)]
    pub fn axibuserr(&self) -> AXIBUSERR_R {
        AXIBUSERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
    #[inline(always)]
    pub fn axicmderr(&self) -> AXICMDERR_R {
        AXICMDERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command"]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IPCMDERR_R {
        IPCMDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IP command normal done interrupt"]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IPCMDDONE_R {
        IPCMDDONE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read"]
    #[inline(always)]
    pub fn axibuserr(&mut self) -> AXIBUSERR_W<3> {
        AXIBUSERR_W::new(self)
    }
    #[doc = "Bit 2 - AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
    #[inline(always)]
    pub fn axicmderr(&mut self) -> AXICMDERR_W<2> {
        AXICMDERR_W::new(self)
    }
    #[doc = "Bit 1 - IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command"]
    #[inline(always)]
    pub fn ipcmderr(&mut self) -> IPCMDERR_W<1> {
        IPCMDERR_W::new(self)
    }
    #[doc = "Bit 0 - IP command normal done interrupt"]
    #[inline(always)]
    pub fn ipcmddone(&mut self) -> IPCMDDONE_W<0> {
        IPCMDDONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
