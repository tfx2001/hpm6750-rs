#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMSI` reader - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
pub type EMSI_R = crate::BitReader<bool>;
#[doc = "Field `EMSI` writer - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
pub type EMSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ELSI` reader - Enable receiver line status interrupt"]
pub type ELSI_R = crate::BitReader<bool>;
#[doc = "Field `ELSI` writer - Enable receiver line status interrupt"]
pub type ELSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ETHEI` reader - Enable transmitter holding register interrupt"]
pub type ETHEI_R = crate::BitReader<bool>;
#[doc = "Field `ETHEI` writer - Enable transmitter holding register interrupt"]
pub type ETHEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ERBI` reader - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
pub type ERBI_R = crate::BitReader<bool>;
#[doc = "Field `ERBI` writer - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
pub type ERBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
    #[inline(always)]
    pub fn emsi(&self) -> EMSI_R {
        EMSI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable receiver line status interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ELSI_R {
        ELSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmitter holding register interrupt"]
    #[inline(always)]
    pub fn ethei(&self) -> ETHEI_R {
        ETHEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn erbi(&self) -> ERBI_R {
        ERBI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
    #[inline(always)]
    pub fn emsi(&mut self) -> EMSI_W<3> {
        EMSI_W::new(self)
    }
    #[doc = "Bit 2 - Enable receiver line status interrupt"]
    #[inline(always)]
    pub fn elsi(&mut self) -> ELSI_W<2> {
        ELSI_W::new(self)
    }
    #[doc = "Bit 1 - Enable transmitter holding register interrupt"]
    #[inline(always)]
    pub fn ethei(&mut self) -> ETHEI_W<1> {
        ETHEI_W::new(self)
    }
    #[doc = "Bit 0 - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn erbi(&mut self) -> ERBI_W<0> {
        ERBI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register (when DLAB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
