#[doc = "Register `SYSCFG0` reader"]
pub struct R(crate::R<SYSCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG0` writer"]
pub struct W(crate::W<SYSCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG0_SPEC>;
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
impl From<crate::W<SYSCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFWP` reader - Program FRAM write protection"]
pub type PFWP_R = crate::BitReader<PFWP_A>;
#[doc = "Program FRAM write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFWP_A {
    #[doc = "0: Program FRAM write enable"]
    PFWP_0 = 0,
    #[doc = "1: Program FRAM write protected (not writable)"]
    PFWP_1 = 1,
}
impl From<PFWP_A> for bool {
    #[inline(always)]
    fn from(variant: PFWP_A) -> Self {
        variant as u8 != 0
    }
}
impl PFWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFWP_A {
        match self.bits {
            false => PFWP_A::PFWP_0,
            true => PFWP_A::PFWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFWP_0`"]
    #[inline(always)]
    pub fn is_pfwp_0(&self) -> bool {
        *self == PFWP_A::PFWP_0
    }
    #[doc = "Checks if the value of the field is `PFWP_1`"]
    #[inline(always)]
    pub fn is_pfwp_1(&self) -> bool {
        *self == PFWP_A::PFWP_1
    }
}
#[doc = "Field `PFWP` writer - Program FRAM write protection"]
pub type PFWP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG0_SPEC, PFWP_A, O>;
impl<'a, const O: u8> PFWP_W<'a, O> {
    #[doc = "Program FRAM write enable"]
    #[inline(always)]
    pub fn pfwp_0(self) -> &'a mut W {
        self.variant(PFWP_A::PFWP_0)
    }
    #[doc = "Program FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn pfwp_1(self) -> &'a mut W {
        self.variant(PFWP_A::PFWP_1)
    }
}
#[doc = "Field `DFWP` reader - Data FRAM write protection"]
pub type DFWP_R = crate::BitReader<DFWP_A>;
#[doc = "Data FRAM write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFWP_A {
    #[doc = "0: Data FRAM write enable"]
    DFWP_0 = 0,
    #[doc = "1: Data FRAM write protected (not writable)"]
    DFWP_1 = 1,
}
impl From<DFWP_A> for bool {
    #[inline(always)]
    fn from(variant: DFWP_A) -> Self {
        variant as u8 != 0
    }
}
impl DFWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFWP_A {
        match self.bits {
            false => DFWP_A::DFWP_0,
            true => DFWP_A::DFWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DFWP_0`"]
    #[inline(always)]
    pub fn is_dfwp_0(&self) -> bool {
        *self == DFWP_A::DFWP_0
    }
    #[doc = "Checks if the value of the field is `DFWP_1`"]
    #[inline(always)]
    pub fn is_dfwp_1(&self) -> bool {
        *self == DFWP_A::DFWP_1
    }
}
#[doc = "Field `DFWP` writer - Data FRAM write protection"]
pub type DFWP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG0_SPEC, DFWP_A, O>;
impl<'a, const O: u8> DFWP_W<'a, O> {
    #[doc = "Data FRAM write enable"]
    #[inline(always)]
    pub fn dfwp_0(self) -> &'a mut W {
        self.variant(DFWP_A::DFWP_0)
    }
    #[doc = "Data FRAM write protected (not writable)"]
    #[inline(always)]
    pub fn dfwp_1(self) -> &'a mut W {
        self.variant(DFWP_A::DFWP_1)
    }
}
#[doc = "Field `FRWPOA` reader - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
pub type FRWPOA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRWPOA` writer - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
pub type FRWPOA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SYSCFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `FRWPPW` reader - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
pub type FRWPPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRWPPW` writer - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
pub type FRWPPW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SYSCFG0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Program FRAM write protection"]
    #[inline(always)]
    pub fn pfwp(&self) -> PFWP_R {
        PFWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FRAM write protection"]
    #[inline(always)]
    pub fn dfwp(&self) -> DFWP_R {
        DFWP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
    #[inline(always)]
    pub fn frwpoa(&self) -> FRWPOA_R {
        FRWPOA_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
    #[inline(always)]
    pub fn frwppw(&self) -> FRWPPW_R {
        FRWPPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Program FRAM write protection"]
    #[inline(always)]
    pub fn pfwp(&mut self) -> PFWP_W<0> {
        PFWP_W::new(self)
    }
    #[doc = "Bit 1 - Data FRAM write protection"]
    #[inline(always)]
    pub fn dfwp(&mut self) -> DFWP_W<1> {
        DFWP_W::new(self)
    }
    #[doc = "Bits 2:7 - Program FRAM write protection offset address from the beginning of Program FRAM. The offset increases by 1KB resolution"]
    #[inline(always)]
    pub fn frwpoa(&mut self) -> FRWPOA_W<2> {
        FRWPOA_W::new(self)
    }
    #[doc = "Bits 8:15 - FRAM protection password, FRAM protection password. Write with 0A5h to unlock the FRAM protection registers. Always reads as 096h"]
    #[inline(always)]
    pub fn frwppw(&mut self) -> FRWPPW_W<8> {
        FRWPPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg0](index.html) module"]
pub struct SYSCFG0_SPEC;
impl crate::RegisterSpec for SYSCFG0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg0::R](R) reader structure"]
impl crate::Readable for SYSCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg0::W](W) writer structure"]
impl crate::Writable for SYSCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG0 to value 0"]
impl crate::Resettable for SYSCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
