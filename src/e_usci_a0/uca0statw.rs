#[doc = "Register `UCA0STATW` reader"]
pub struct R(crate::R<UCA0STATW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0STATW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0STATW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0STATW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0STATW` writer"]
pub struct W(crate::W<UCA0STATW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0STATW_SPEC>;
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
impl From<crate::W<UCA0STATW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0STATW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBUSY` reader - eUSCI_A busy"]
pub type UCBUSY_R = crate::BitReader<UCBUSY_A>;
#[doc = "eUSCI_A busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBUSY_A {
    #[doc = "0: eUSCI_A inactive"]
    IDLE = 0,
    #[doc = "1: eUSCI_A transmitting or receiving"]
    BUSY = 1,
}
impl From<UCBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: UCBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBUSY_A {
        match self.bits {
            false => UCBUSY_A::IDLE,
            true => UCBUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == UCBUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == UCBUSY_A::BUSY
    }
}
#[doc = "Field `UCADDR_UCIDLE` reader - Address received / Idle line detected"]
pub type UCADDR_UCIDLE_R = crate::BitReader<UCADDR_UCIDLE_A>;
#[doc = "Address received / Idle line detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCADDR_UCIDLE_A {
    #[doc = "0: UCADDR: Received character is data. UCIDLE: No idle line detected"]
    UCADDR_UCIDLE_0 = 0,
    #[doc = "1: UCADDR: Received character is an address. UCIDLE: Idle line detected"]
    UCADDR_UCIDLE_1 = 1,
}
impl From<UCADDR_UCIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: UCADDR_UCIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCADDR_UCIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCADDR_UCIDLE_A {
        match self.bits {
            false => UCADDR_UCIDLE_A::UCADDR_UCIDLE_0,
            true => UCADDR_UCIDLE_A::UCADDR_UCIDLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCADDR_UCIDLE_0`"]
    #[inline(always)]
    pub fn is_ucaddr_ucidle_0(&self) -> bool {
        *self == UCADDR_UCIDLE_A::UCADDR_UCIDLE_0
    }
    #[doc = "Checks if the value of the field is `UCADDR_UCIDLE_1`"]
    #[inline(always)]
    pub fn is_ucaddr_ucidle_1(&self) -> bool {
        *self == UCADDR_UCIDLE_A::UCADDR_UCIDLE_1
    }
}
#[doc = "Field `UCADDR_UCIDLE` writer - Address received / Idle line detected"]
pub type UCADDR_UCIDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCADDR_UCIDLE_A, O>;
impl<'a, const O: u8> UCADDR_UCIDLE_W<'a, O> {
    #[doc = "UCADDR: Received character is data. UCIDLE: No idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle_0(self) -> &'a mut W {
        self.variant(UCADDR_UCIDLE_A::UCADDR_UCIDLE_0)
    }
    #[doc = "UCADDR: Received character is an address. UCIDLE: Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle_1(self) -> &'a mut W {
        self.variant(UCADDR_UCIDLE_A::UCADDR_UCIDLE_1)
    }
}
#[doc = "Field `UCRXERR` reader - Receive error flag"]
pub type UCRXERR_R = crate::BitReader<UCRXERR_A>;
#[doc = "Receive error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCRXERR_A {
    #[doc = "0: No receive errors detected"]
    UCRXERR_0 = 0,
    #[doc = "1: Receive error detected"]
    UCRXERR_1 = 1,
}
impl From<UCRXERR_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXERR_A) -> Self {
        variant as u8 != 0
    }
}
impl UCRXERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXERR_A {
        match self.bits {
            false => UCRXERR_A::UCRXERR_0,
            true => UCRXERR_A::UCRXERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXERR_0`"]
    #[inline(always)]
    pub fn is_ucrxerr_0(&self) -> bool {
        *self == UCRXERR_A::UCRXERR_0
    }
    #[doc = "Checks if the value of the field is `UCRXERR_1`"]
    #[inline(always)]
    pub fn is_ucrxerr_1(&self) -> bool {
        *self == UCRXERR_A::UCRXERR_1
    }
}
#[doc = "Field `UCRXERR` writer - Receive error flag"]
pub type UCRXERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCRXERR_A, O>;
impl<'a, const O: u8> UCRXERR_W<'a, O> {
    #[doc = "No receive errors detected"]
    #[inline(always)]
    pub fn ucrxerr_0(self) -> &'a mut W {
        self.variant(UCRXERR_A::UCRXERR_0)
    }
    #[doc = "Receive error detected"]
    #[inline(always)]
    pub fn ucrxerr_1(self) -> &'a mut W {
        self.variant(UCRXERR_A::UCRXERR_1)
    }
}
#[doc = "Field `UCBRK` reader - Break detect flag"]
pub type UCBRK_R = crate::BitReader<UCBRK_A>;
#[doc = "Break detect flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBRK_A {
    #[doc = "0: No break condition"]
    UCBRK_0 = 0,
    #[doc = "1: Break condition occurred"]
    UCBRK_1 = 1,
}
impl From<UCBRK_A> for bool {
    #[inline(always)]
    fn from(variant: UCBRK_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBRK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRK_A {
        match self.bits {
            false => UCBRK_A::UCBRK_0,
            true => UCBRK_A::UCBRK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBRK_0`"]
    #[inline(always)]
    pub fn is_ucbrk_0(&self) -> bool {
        *self == UCBRK_A::UCBRK_0
    }
    #[doc = "Checks if the value of the field is `UCBRK_1`"]
    #[inline(always)]
    pub fn is_ucbrk_1(&self) -> bool {
        *self == UCBRK_A::UCBRK_1
    }
}
#[doc = "Field `UCBRK` writer - Break detect flag"]
pub type UCBRK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCBRK_A, O>;
impl<'a, const O: u8> UCBRK_W<'a, O> {
    #[doc = "No break condition"]
    #[inline(always)]
    pub fn ucbrk_0(self) -> &'a mut W {
        self.variant(UCBRK_A::UCBRK_0)
    }
    #[doc = "Break condition occurred"]
    #[inline(always)]
    pub fn ucbrk_1(self) -> &'a mut W {
        self.variant(UCBRK_A::UCBRK_1)
    }
}
#[doc = "Field `UCPE` reader - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
pub type UCPE_R = crate::BitReader<UCPE_A>;
#[doc = "Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCPE_A {
    #[doc = "0: No error"]
    UCPE_0 = 0,
    #[doc = "1: Character received with parity error"]
    UCPE_1 = 1,
}
impl From<UCPE_A> for bool {
    #[inline(always)]
    fn from(variant: UCPE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPE_A {
        match self.bits {
            false => UCPE_A::UCPE_0,
            true => UCPE_A::UCPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCPE_0`"]
    #[inline(always)]
    pub fn is_ucpe_0(&self) -> bool {
        *self == UCPE_A::UCPE_0
    }
    #[doc = "Checks if the value of the field is `UCPE_1`"]
    #[inline(always)]
    pub fn is_ucpe_1(&self) -> bool {
        *self == UCPE_A::UCPE_1
    }
}
#[doc = "Field `UCPE` writer - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
pub type UCPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCPE_A, O>;
impl<'a, const O: u8> UCPE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucpe_0(self) -> &'a mut W {
        self.variant(UCPE_A::UCPE_0)
    }
    #[doc = "Character received with parity error"]
    #[inline(always)]
    pub fn ucpe_1(self) -> &'a mut W {
        self.variant(UCPE_A::UCPE_1)
    }
}
#[doc = "Field `UCOE` reader - Overrun error flag"]
pub type UCOE_R = crate::BitReader<UCOE_A>;
#[doc = "Overrun error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCOE_A {
    #[doc = "0: No error"]
    UCOE_0 = 0,
    #[doc = "1: Overrun error occurred"]
    UCOE_1 = 1,
}
impl From<UCOE_A> for bool {
    #[inline(always)]
    fn from(variant: UCOE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOE_A {
        match self.bits {
            false => UCOE_A::UCOE_0,
            true => UCOE_A::UCOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOE_0`"]
    #[inline(always)]
    pub fn is_ucoe_0(&self) -> bool {
        *self == UCOE_A::UCOE_0
    }
    #[doc = "Checks if the value of the field is `UCOE_1`"]
    #[inline(always)]
    pub fn is_ucoe_1(&self) -> bool {
        *self == UCOE_A::UCOE_1
    }
}
#[doc = "Field `UCOE` writer - Overrun error flag"]
pub type UCOE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCOE_A, O>;
impl<'a, const O: u8> UCOE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucoe_0(self) -> &'a mut W {
        self.variant(UCOE_A::UCOE_0)
    }
    #[doc = "Overrun error occurred"]
    #[inline(always)]
    pub fn ucoe_1(self) -> &'a mut W {
        self.variant(UCOE_A::UCOE_1)
    }
}
#[doc = "Field `UCFE` reader - Framing error flag"]
pub type UCFE_R = crate::BitReader<UCFE_A>;
#[doc = "Framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCFE_A {
    #[doc = "0: No error"]
    UCFE_0 = 0,
    #[doc = "1: Character received with low stop bit"]
    UCFE_1 = 1,
}
impl From<UCFE_A> for bool {
    #[inline(always)]
    fn from(variant: UCFE_A) -> Self {
        variant as u8 != 0
    }
}
impl UCFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCFE_A {
        match self.bits {
            false => UCFE_A::UCFE_0,
            true => UCFE_A::UCFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCFE_0`"]
    #[inline(always)]
    pub fn is_ucfe_0(&self) -> bool {
        *self == UCFE_A::UCFE_0
    }
    #[doc = "Checks if the value of the field is `UCFE_1`"]
    #[inline(always)]
    pub fn is_ucfe_1(&self) -> bool {
        *self == UCFE_A::UCFE_1
    }
}
#[doc = "Field `UCFE` writer - Framing error flag"]
pub type UCFE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCFE_A, O>;
impl<'a, const O: u8> UCFE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucfe_0(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_0)
    }
    #[doc = "Character received with low stop bit"]
    #[inline(always)]
    pub fn ucfe_1(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_1)
    }
}
#[doc = "Field `UCLISTEN` reader - Listen enable"]
pub type UCLISTEN_R = crate::BitReader<UCLISTEN_A>;
#[doc = "Listen enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCLISTEN_A {
    #[doc = "0: Disabled"]
    UCLISTEN_0 = 0,
    #[doc = "1: Enabled. UCAxTXD is internally fed back to the receiver"]
    UCLISTEN_1 = 1,
}
impl From<UCLISTEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCLISTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl UCLISTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCLISTEN_A {
        match self.bits {
            false => UCLISTEN_A::UCLISTEN_0,
            true => UCLISTEN_A::UCLISTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCLISTEN_0`"]
    #[inline(always)]
    pub fn is_uclisten_0(&self) -> bool {
        *self == UCLISTEN_A::UCLISTEN_0
    }
    #[doc = "Checks if the value of the field is `UCLISTEN_1`"]
    #[inline(always)]
    pub fn is_uclisten_1(&self) -> bool {
        *self == UCLISTEN_A::UCLISTEN_1
    }
}
#[doc = "Field `UCLISTEN` writer - Listen enable"]
pub type UCLISTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCA0STATW_SPEC, UCLISTEN_A, O>;
impl<'a, const O: u8> UCLISTEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn uclisten_0(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_0)
    }
    #[doc = "Enabled. UCAxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn uclisten_1(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_A busy"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UCBUSY_R {
        UCBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address received / Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle(&self) -> UCADDR_UCIDLE_R {
        UCADDR_UCIDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive error flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UCRXERR_R {
        UCRXERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Break detect flag"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UCBRK_R {
        UCBRK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
    #[inline(always)]
    pub fn ucpe(&self) -> UCPE_R {
        UCPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UCOE_R {
        UCOE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UCFE_R {
        UCFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&self) -> UCLISTEN_R {
        UCLISTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Address received / Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle(&mut self) -> UCADDR_UCIDLE_W<1> {
        UCADDR_UCIDLE_W::new(self)
    }
    #[doc = "Bit 2 - Receive error flag"]
    #[inline(always)]
    pub fn ucrxerr(&mut self) -> UCRXERR_W<2> {
        UCRXERR_W::new(self)
    }
    #[doc = "Bit 3 - Break detect flag"]
    #[inline(always)]
    pub fn ucbrk(&mut self) -> UCBRK_W<3> {
        UCBRK_W::new(self)
    }
    #[doc = "Bit 4 - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
    #[inline(always)]
    pub fn ucpe(&mut self) -> UCPE_W<4> {
        UCPE_W::new(self)
    }
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UCOE_W<5> {
        UCOE_W::new(self)
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UCFE_W<6> {
        UCFE_W::new(self)
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UCLISTEN_W<7> {
        UCLISTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0statw](index.html) module"]
pub struct UCA0STATW_SPEC;
impl crate::RegisterSpec for UCA0STATW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uca0statw::R](R) reader structure"]
impl crate::Readable for UCA0STATW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0statw::W](W) writer structure"]
impl crate::Writable for UCA0STATW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0STATW to value 0"]
impl crate::Resettable for UCA0STATW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
