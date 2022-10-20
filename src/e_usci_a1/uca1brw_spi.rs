#[doc = "Register `UCA1BRW_SPI` reader"]
pub struct R(crate::R<UCA1BRW_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1BRW_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1BRW_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1BRW_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1BRW_SPI` writer"]
pub struct W(crate::W<UCA1BRW_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1BRW_SPI_SPEC>;
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
impl From<crate::W<UCA1BRW_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1BRW_SPI_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1brw_spi](index.html) module"]
pub struct UCA1BRW_SPI_SPEC;
impl crate::RegisterSpec for UCA1BRW_SPI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca1brw_spi::R](R) reader structure"]
impl crate::Readable for UCA1BRW_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1brw_spi::W](W) writer structure"]
impl crate::Writable for UCA1BRW_SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA1BRW_SPI to value 0"]
impl crate::Resettable for UCA1BRW_SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
