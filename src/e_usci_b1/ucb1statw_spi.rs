#[doc = "Register `UCB1STATW_SPI` reader"]
pub struct R(crate::R<UCB1STATW_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1STATW_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1STATW_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1STATW_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1STATW_SPI` writer"]
pub struct W(crate::W<UCB1STATW_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1STATW_SPI_SPEC>;
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
impl From<crate::W<UCB1STATW_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1STATW_SPI_SPEC>) -> Self {
        W(writer)
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
pub type UCOE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1STATW_SPI_SPEC, UCOE_A, O>;
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
    #[doc = "1: Bus conflict occurred"]
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
pub type UCFE_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1STATW_SPI_SPEC, UCFE_A, O>;
impl<'a, const O: u8> UCFE_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucfe_0(self) -> &'a mut W {
        self.variant(UCFE_A::UCFE_0)
    }
    #[doc = "Bus conflict occurred"]
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
    #[doc = "1: Enabled. UCBxTXD is internally fed back to the receiver"]
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
pub type UCLISTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1STATW_SPI_SPEC, UCLISTEN_A, O>;
impl<'a, const O: u8> UCLISTEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn uclisten_0(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_0)
    }
    #[doc = "Enabled. UCBxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn uclisten_1(self) -> &'a mut W {
        self.variant(UCLISTEN_A::UCLISTEN_1)
    }
}
impl R {
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
#[doc = "UCB1STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1statw_spi](index.html) module"]
pub struct UCB1STATW_SPI_SPEC;
impl crate::RegisterSpec for UCB1STATW_SPI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1statw_spi::R](R) reader structure"]
impl crate::Readable for UCB1STATW_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1statw_spi::W](W) writer structure"]
impl crate::Writable for UCB1STATW_SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1STATW_SPI to value 0"]
impl crate::Resettable for UCB1STATW_SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
