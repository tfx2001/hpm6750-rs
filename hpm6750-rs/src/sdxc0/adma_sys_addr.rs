#[doc = "Register `ADMA_SYS_ADDR` reader"]
pub struct R(crate::R<ADMA_SYS_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMA_SYS_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMA_SYS_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMA_SYS_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADMA_SYS_ADDR` writer"]
pub struct W(crate::W<ADMA_SYS_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADMA_SYS_ADDR_SPEC>;
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
impl From<crate::W<ADMA_SYS_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADMA_SYS_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADMA_SA` reader - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location ADMA2: This register stores the byte address of the executing command of the descriptor table ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
pub type ADMA_SA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADMA_SA` writer - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location ADMA2: This register stores the byte address of the executing command of the descriptor table ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
pub type ADMA_SA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADMA_SYS_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location ADMA2: This register stores the byte address of the executing command of the descriptor table ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    #[inline(always)]
    pub fn adma_sa(&self) -> ADMA_SA_R {
        ADMA_SA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location ADMA2: This register stores the byte address of the executing command of the descriptor table ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    #[inline(always)]
    pub fn adma_sa(&mut self) -> ADMA_SA_W<0> {
        ADMA_SA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_sys_addr](index.html) module"]
pub struct ADMA_SYS_ADDR_SPEC;
impl crate::RegisterSpec for ADMA_SYS_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adma_sys_addr::R](R) reader structure"]
impl crate::Readable for ADMA_SYS_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adma_sys_addr::W](W) writer structure"]
impl crate::Writable for ADMA_SYS_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADMA_SYS_ADDR to value 0"]
impl crate::Resettable for ADMA_SYS_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
