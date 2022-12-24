#[doc = "Register `RWKFRMFILT` reader"]
pub struct R(crate::R<RWKFRMFILT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWKFRMFILT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWKFRMFILT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWKFRMFILT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWKFRMFILT` writer"]
pub struct W(crate::W<RWKFRMFILT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWKFRMFILT_SPEC>;
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
impl From<crate::W<RWKFRMFILT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWKFRMFILT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPFRMFILT` reader - This is the address through which the application writes or reads the remote wake-up frame filter registers (wkupfmfilter_reg). The wkupfmfilter_reg register is a pointer to eight wkupfmfilter_reg registers. The wkupfmfilter_reg register is loaded by sequentially loading the eight register values. Eight sequential writes to this address (0x0028) write all wkupfmfilter_reg registers. Similarly, eight sequential reads from this address (0x0028) read all wkupfmfilter_reg registers"]
pub type WKUPFRMFILT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WKUPFRMFILT` writer - This is the address through which the application writes or reads the remote wake-up frame filter registers (wkupfmfilter_reg). The wkupfmfilter_reg register is a pointer to eight wkupfmfilter_reg registers. The wkupfmfilter_reg register is loaded by sequentially loading the eight register values. Eight sequential writes to this address (0x0028) write all wkupfmfilter_reg registers. Similarly, eight sequential reads from this address (0x0028) read all wkupfmfilter_reg registers"]
pub type WKUPFRMFILT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RWKFRMFILT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the address through which the application writes or reads the remote wake-up frame filter registers (wkupfmfilter_reg). The wkupfmfilter_reg register is a pointer to eight wkupfmfilter_reg registers. The wkupfmfilter_reg register is loaded by sequentially loading the eight register values. Eight sequential writes to this address (0x0028) write all wkupfmfilter_reg registers. Similarly, eight sequential reads from this address (0x0028) read all wkupfmfilter_reg registers"]
    #[inline(always)]
    pub fn wkupfrmfilt(&self) -> WKUPFRMFILT_R {
        WKUPFRMFILT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the address through which the application writes or reads the remote wake-up frame filter registers (wkupfmfilter_reg). The wkupfmfilter_reg register is a pointer to eight wkupfmfilter_reg registers. The wkupfmfilter_reg register is loaded by sequentially loading the eight register values. Eight sequential writes to this address (0x0028) write all wkupfmfilter_reg registers. Similarly, eight sequential reads from this address (0x0028) read all wkupfmfilter_reg registers"]
    #[inline(always)]
    #[must_use]
    pub fn wkupfrmfilt(&mut self) -> WKUPFRMFILT_W<0> {
        WKUPFRMFILT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remote Wake-Up Frame Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwkfrmfilt](index.html) module"]
pub struct RWKFRMFILT_SPEC;
impl crate::RegisterSpec for RWKFRMFILT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rwkfrmfilt::R](R) reader structure"]
impl crate::Readable for RWKFRMFILT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwkfrmfilt::W](W) writer structure"]
impl crate::Writable for RWKFRMFILT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWKFRMFILT to value 0"]
impl crate::Resettable for RWKFRMFILT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
