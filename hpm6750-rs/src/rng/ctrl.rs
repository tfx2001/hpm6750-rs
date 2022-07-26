#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIRQERR` reader - Mask Interrupt Request for Error"]
pub type MIRQERR_R = crate::BitReader<bool>;
#[doc = "Field `MIRQERR` writer - Mask Interrupt Request for Error"]
pub type MIRQERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MIRQDN` reader - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
pub type MIRQDN_R = crate::BitReader<bool>;
#[doc = "Field `MIRQDN` writer - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
pub type MIRQDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `AUTRSD` reader - Auto Reseed"]
pub type AUTRSD_R = crate::BitReader<bool>;
#[doc = "Field `AUTRSD` writer - Auto Reseed"]
pub type AUTRSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FUFMOD` reader - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
pub type FUFMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUFMOD` writer - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
pub type FUFMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 6 - Mask Interrupt Request for Error"]
    #[inline(always)]
    pub fn mirqerr(&self) -> MIRQERR_R {
        MIRQERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
    #[inline(always)]
    pub fn mirqdn(&self) -> MIRQDN_R {
        MIRQDN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto Reseed"]
    #[inline(always)]
    pub fn autrsd(&self) -> AUTRSD_R {
        AUTRSD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:1 - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
    #[inline(always)]
    pub fn fufmod(&self) -> FUFMOD_R {
        FUFMOD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - Mask Interrupt Request for Error"]
    #[inline(always)]
    pub fn mirqerr(&mut self) -> MIRQERR_W<6> {
        MIRQERR_W::new(self)
    }
    #[doc = "Bit 5 - Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
    #[inline(always)]
    pub fn mirqdn(&mut self) -> MIRQDN_W<5> {
        MIRQDN_W::new(self)
    }
    #[doc = "Bit 4 - Auto Reseed"]
    #[inline(always)]
    pub fn autrsd(&mut self) -> AUTRSD_W<4> {
        AUTRSD_W::new(self)
    }
    #[doc = "Bits 0:1 - FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
    #[inline(always)]
    pub fn fufmod(&mut self) -> FUFMOD_W<0> {
        FUFMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
