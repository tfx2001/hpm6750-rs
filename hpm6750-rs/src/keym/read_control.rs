#[doc = "Register `READ_CONTROL` reader"]
pub struct R(crate::R<READ_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READ_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READ_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READ_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READ_CONTROL` writer"]
pub struct W(crate::W<READ_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READ_CONTROL_SPEC>;
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
impl From<crate::W<READ_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READ_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_SMK_READ` reader - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_SMK_READ_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK_SMK_READ` writer - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_SMK_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, READ_CONTROL_SPEC, bool, O>;
#[doc = "Field `BLOCK_PK_READ` reader - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_PK_READ_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK_PK_READ` writer - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
pub type BLOCK_PK_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, READ_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    pub fn block_smk_read(&self) -> BLOCK_SMK_READ_R {
        BLOCK_SMK_READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    pub fn block_pk_read(&self) -> BLOCK_PK_READ_R {
        BLOCK_PK_READ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    #[must_use]
    pub fn block_smk_read(&mut self) -> BLOCK_SMK_READ_W<0> {
        BLOCK_SMK_READ_W::new(self)
    }
    #[doc = "Bit 16 - asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    #[inline(always)]
    #[must_use]
    pub fn block_pk_read(&mut self) -> BLOCK_PK_READ_W<16> {
        BLOCK_PK_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key read out control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [read_control](index.html) module"]
pub struct READ_CONTROL_SPEC;
impl crate::RegisterSpec for READ_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [read_control::R](R) reader structure"]
impl crate::Readable for READ_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [read_control::W](W) writer structure"]
impl crate::Writable for READ_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READ_CONTROL to value 0"]
impl crate::Resettable for READ_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
