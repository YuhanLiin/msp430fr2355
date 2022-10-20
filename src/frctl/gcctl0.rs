#[doc = "Register `GCCTL0` reader"]
pub struct R(crate::R<GCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTL0` writer"]
pub struct W(crate::W<GCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTL0_SPEC>;
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
impl From<crate::W<GCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRLPMPWR` reader - Enables FRAM auto power up after LPM"]
pub type FRLPMPWR_R = crate::BitReader<FRLPMPWR_A>;
#[doc = "Enables FRAM auto power up after LPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRLPMPWR_A {
    #[doc = "0: FRAM startup is delayed to the first FRAM access after exit from LPM"]
    FRLPMPWR_0 = 0,
    #[doc = "1: FRAM is powered up immediately on exit from LPM"]
    FRLPMPWR_1 = 1,
}
impl From<FRLPMPWR_A> for bool {
    #[inline(always)]
    fn from(variant: FRLPMPWR_A) -> Self {
        variant as u8 != 0
    }
}
impl FRLPMPWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRLPMPWR_A {
        match self.bits {
            false => FRLPMPWR_A::FRLPMPWR_0,
            true => FRLPMPWR_A::FRLPMPWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRLPMPWR_0`"]
    #[inline(always)]
    pub fn is_frlpmpwr_0(&self) -> bool {
        *self == FRLPMPWR_A::FRLPMPWR_0
    }
    #[doc = "Checks if the value of the field is `FRLPMPWR_1`"]
    #[inline(always)]
    pub fn is_frlpmpwr_1(&self) -> bool {
        *self == FRLPMPWR_A::FRLPMPWR_1
    }
}
#[doc = "Field `FRLPMPWR` writer - Enables FRAM auto power up after LPM"]
pub type FRLPMPWR_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, FRLPMPWR_A, O>;
impl<'a, const O: u8> FRLPMPWR_W<'a, O> {
    #[doc = "FRAM startup is delayed to the first FRAM access after exit from LPM"]
    #[inline(always)]
    pub fn frlpmpwr_0(self) -> &'a mut W {
        self.variant(FRLPMPWR_A::FRLPMPWR_0)
    }
    #[doc = "FRAM is powered up immediately on exit from LPM"]
    #[inline(always)]
    pub fn frlpmpwr_1(self) -> &'a mut W {
        self.variant(FRLPMPWR_A::FRLPMPWR_1)
    }
}
#[doc = "Field `FRPWR` reader - FRAM Memory Power Control Request"]
pub type FRPWR_R = crate::BitReader<FRPWR_A>;
#[doc = "FRAM Memory Power Control Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRPWR_A {
    #[doc = "0: Enable INACTIVE mode."]
    FRPWR_0 = 0,
    #[doc = "1: Enable ACTIVE mode."]
    FRPWR_1 = 1,
}
impl From<FRPWR_A> for bool {
    #[inline(always)]
    fn from(variant: FRPWR_A) -> Self {
        variant as u8 != 0
    }
}
impl FRPWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRPWR_A {
        match self.bits {
            false => FRPWR_A::FRPWR_0,
            true => FRPWR_A::FRPWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRPWR_0`"]
    #[inline(always)]
    pub fn is_frpwr_0(&self) -> bool {
        *self == FRPWR_A::FRPWR_0
    }
    #[doc = "Checks if the value of the field is `FRPWR_1`"]
    #[inline(always)]
    pub fn is_frpwr_1(&self) -> bool {
        *self == FRPWR_A::FRPWR_1
    }
}
#[doc = "Field `FRPWR` writer - FRAM Memory Power Control Request"]
pub type FRPWR_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, FRPWR_A, O>;
impl<'a, const O: u8> FRPWR_W<'a, O> {
    #[doc = "Enable INACTIVE mode."]
    #[inline(always)]
    pub fn frpwr_0(self) -> &'a mut W {
        self.variant(FRPWR_A::FRPWR_0)
    }
    #[doc = "Enable ACTIVE mode."]
    #[inline(always)]
    pub fn frpwr_1(self) -> &'a mut W {
        self.variant(FRPWR_A::FRPWR_1)
    }
}
#[doc = "Field `CBDIE` reader - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
pub type CBDIE_R = crate::BitReader<CBDIE_A>;
#[doc = "Enable NMI event for the correctable bit error detection flag (CBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBDIE_A {
    #[doc = "0: Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    CBDIE_0 = 0,
    #[doc = "1: Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    CBDIE_1 = 1,
}
impl From<CBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: CBDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBDIE_A {
        match self.bits {
            false => CBDIE_A::CBDIE_0,
            true => CBDIE_A::CBDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CBDIE_0`"]
    #[inline(always)]
    pub fn is_cbdie_0(&self) -> bool {
        *self == CBDIE_A::CBDIE_0
    }
    #[doc = "Checks if the value of the field is `CBDIE_1`"]
    #[inline(always)]
    pub fn is_cbdie_1(&self) -> bool {
        *self == CBDIE_A::CBDIE_1
    }
}
#[doc = "Field `CBDIE` writer - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
pub type CBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, CBDIE_A, O>;
impl<'a, const O: u8> CBDIE_W<'a, O> {
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG)."]
    #[inline(always)]
    pub fn cbdie_0(self) -> &'a mut W {
        self.variant(CBDIE_A::CBDIE_0)
    }
    #[doc = "Disable NMI for the correctable bit error detection flag (CBDIFG). Generates vector in SYSSNIV."]
    #[inline(always)]
    pub fn cbdie_1(self) -> &'a mut W {
        self.variant(CBDIE_A::CBDIE_1)
    }
}
#[doc = "Field `UBDIE` reader - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UBDIE_R = crate::BitReader<UBDIE_A>;
#[doc = "Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UBDIE_A {
    #[doc = "0: Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    UBDIE_0 = 0,
    #[doc = "1: Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    UBDIE_1 = 1,
}
impl From<UBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: UBDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UBDIE_A {
        match self.bits {
            false => UBDIE_A::UBDIE_0,
            true => UBDIE_A::UBDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UBDIE_0`"]
    #[inline(always)]
    pub fn is_ubdie_0(&self) -> bool {
        *self == UBDIE_A::UBDIE_0
    }
    #[doc = "Checks if the value of the field is `UBDIE_1`"]
    #[inline(always)]
    pub fn is_ubdie_1(&self) -> bool {
        *self == UBDIE_A::UBDIE_1
    }
}
#[doc = "Field `UBDIE` writer - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, UBDIE_A, O>;
impl<'a, const O: u8> UBDIE_W<'a, O> {
    #[doc = "Disable NMI for the uncorrectable bit error detection flag (UBDIFG)."]
    #[inline(always)]
    pub fn ubdie_0(self) -> &'a mut W {
        self.variant(UBDIE_A::UBDIE_0)
    }
    #[doc = "Enable NMI for the uncorrectable bit error detection flag (UBDIFG). Generates vector in SYSSNIV. Clear the UBDRSTEN bit."]
    #[inline(always)]
    pub fn ubdie_1(self) -> &'a mut W {
        self.variant(UBDIE_A::UBDIE_1)
    }
}
#[doc = "Field `UBDRSTEN` reader - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UBDRSTEN_R = crate::BitReader<UBDRSTEN_A>;
#[doc = "Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UBDRSTEN_A {
    #[doc = "0: PUC not initiated on uncorrectable bit error detection flag."]
    UBDRSTEN_0 = 0,
    #[doc = "1: PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    UBDRSTEN_1 = 1,
}
impl From<UBDRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UBDRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UBDRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UBDRSTEN_A {
        match self.bits {
            false => UBDRSTEN_A::UBDRSTEN_0,
            true => UBDRSTEN_A::UBDRSTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UBDRSTEN_0`"]
    #[inline(always)]
    pub fn is_ubdrsten_0(&self) -> bool {
        *self == UBDRSTEN_A::UBDRSTEN_0
    }
    #[doc = "Checks if the value of the field is `UBDRSTEN_1`"]
    #[inline(always)]
    pub fn is_ubdrsten_1(&self) -> bool {
        *self == UBDRSTEN_A::UBDRSTEN_1
    }
}
#[doc = "Field `UBDRSTEN` writer - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
pub type UBDRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, GCCTL0_SPEC, UBDRSTEN_A, O>;
impl<'a, const O: u8> UBDRSTEN_W<'a, O> {
    #[doc = "PUC not initiated on uncorrectable bit error detection flag."]
    #[inline(always)]
    pub fn ubdrsten_0(self) -> &'a mut W {
        self.variant(UBDRSTEN_A::UBDRSTEN_0)
    }
    #[doc = "PUC initiated on uncorrectable bit error detection flag. Generates vector in SYSRSTIV. Clear the UBDIE bit."]
    #[inline(always)]
    pub fn ubdrsten_1(self) -> &'a mut W {
        self.variant(UBDRSTEN_A::UBDRSTEN_1)
    }
}
impl R {
    #[doc = "Bit 1 - Enables FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&self) -> FRLPMPWR_R {
        FRLPMPWR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FRAM Memory Power Control Request"]
    #[inline(always)]
    pub fn frpwr(&self) -> FRPWR_R {
        FRPWR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
    #[inline(always)]
    pub fn cbdie(&self) -> CBDIE_R {
        CBDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdie(&self) -> UBDIE_R {
        UBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdrsten(&self) -> UBDRSTEN_R {
        UBDRSTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables FRAM auto power up after LPM"]
    #[inline(always)]
    pub fn frlpmpwr(&mut self) -> FRLPMPWR_W<1> {
        FRLPMPWR_W::new(self)
    }
    #[doc = "Bit 2 - FRAM Memory Power Control Request"]
    #[inline(always)]
    pub fn frpwr(&mut self) -> FRPWR_W<2> {
        FRPWR_W::new(self)
    }
    #[doc = "Bit 5 - Enable NMI event for the correctable bit error detection flag (CBDIFG)"]
    #[inline(always)]
    pub fn cbdie(&mut self) -> CBDIE_W<5> {
        CBDIE_W::new(self)
    }
    #[doc = "Bit 6 - Enable NMI event for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdie(&mut self) -> UBDIE_W<6> {
        UBDIE_W::new(self)
    }
    #[doc = "Bit 7 - Enable Power Up Clear (PUC) reset for the uncorrectable bit error detection flag (UBDIFG)"]
    #[inline(always)]
    pub fn ubdrsten(&mut self) -> UBDRSTEN_W<7> {
        UBDRSTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctl0](index.html) module"]
pub struct GCCTL0_SPEC;
impl crate::RegisterSpec for GCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gcctl0::R](R) reader structure"]
impl crate::Readable for GCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctl0::W](W) writer structure"]
impl crate::Writable for GCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTL0 to value 0"]
impl crate::Resettable for GCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
