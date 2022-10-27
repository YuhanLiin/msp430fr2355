#[doc = "Register `UCB1CTLW1` reader"]
pub struct R(crate::R<UCB1CTLW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB1CTLW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB1CTLW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB1CTLW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB1CTLW1` writer"]
pub struct W(crate::W<UCB1CTLW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB1CTLW1_SPEC>;
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
impl From<crate::W<UCB1CTLW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB1CTLW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCGLIT` reader - Deglitch time"]
pub type UCGLIT_R = crate::FieldReader<u8, UCGLIT_A>;
#[doc = "Deglitch time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCGLIT_A {
    #[doc = "0: 50 ns"]
    UCGLIT_0 = 0,
    #[doc = "1: 25 ns"]
    UCGLIT_1 = 1,
    #[doc = "2: 12.5 ns"]
    UCGLIT_2 = 2,
    #[doc = "3: 6.25 ns"]
    UCGLIT_3 = 3,
}
impl From<UCGLIT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCGLIT_A) -> Self {
        variant as _
    }
}
impl UCGLIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGLIT_A {
        match self.bits {
            0 => UCGLIT_A::UCGLIT_0,
            1 => UCGLIT_A::UCGLIT_1,
            2 => UCGLIT_A::UCGLIT_2,
            3 => UCGLIT_A::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLIT_A::UCGLIT_3
    }
}
#[doc = "Field `UCGLIT` writer - Deglitch time"]
pub type UCGLIT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB1CTLW1_SPEC, u8, UCGLIT_A, 2, O>;
impl<'a, const O: u8> UCGLIT_W<'a, O> {
    #[doc = "50 ns"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_0)
    }
    #[doc = "25 ns"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_1)
    }
    #[doc = "12.5 ns"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_2)
    }
    #[doc = "6.25 ns"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_3)
    }
}
#[doc = "Field `UCASTP` reader - Automatic STOP condition generation"]
pub type UCASTP_R = crate::FieldReader<u8, UCASTP_A>;
#[doc = "Automatic STOP condition generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCASTP_A {
    #[doc = "0: No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care."]
    UCASTP_0 = 0,
    #[doc = "1: UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT"]
    UCASTP_1 = 1,
    #[doc = "2: A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold"]
    UCASTP_2 = 2,
    #[doc = "3: Reserved"]
    UCASTP_3 = 3,
}
impl From<UCASTP_A> for u8 {
    #[inline(always)]
    fn from(variant: UCASTP_A) -> Self {
        variant as _
    }
}
impl UCASTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCASTP_A {
        match self.bits {
            0 => UCASTP_A::UCASTP_0,
            1 => UCASTP_A::UCASTP_1,
            2 => UCASTP_A::UCASTP_2,
            3 => UCASTP_A::UCASTP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCASTP_0`"]
    #[inline(always)]
    pub fn is_ucastp_0(&self) -> bool {
        *self == UCASTP_A::UCASTP_0
    }
    #[doc = "Checks if the value of the field is `UCASTP_1`"]
    #[inline(always)]
    pub fn is_ucastp_1(&self) -> bool {
        *self == UCASTP_A::UCASTP_1
    }
    #[doc = "Checks if the value of the field is `UCASTP_2`"]
    #[inline(always)]
    pub fn is_ucastp_2(&self) -> bool {
        *self == UCASTP_A::UCASTP_2
    }
    #[doc = "Checks if the value of the field is `UCASTP_3`"]
    #[inline(always)]
    pub fn is_ucastp_3(&self) -> bool {
        *self == UCASTP_A::UCASTP_3
    }
}
#[doc = "Field `UCASTP` writer - Automatic STOP condition generation"]
pub type UCASTP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB1CTLW1_SPEC, u8, UCASTP_A, 2, O>;
impl<'a, const O: u8> UCASTP_W<'a, O> {
    #[doc = "No automatic STOP generation. The STOP condition is generated after the user sets the UCTXSTP bit. The value in UCBxTBCNT is a don't care."]
    #[inline(always)]
    pub fn ucastp_0(self) -> &'a mut W {
        self.variant(UCASTP_A::UCASTP_0)
    }
    #[doc = "UCBCNTIFG is set with the byte counter reaches the threshold defined in UCBxTBCNT"]
    #[inline(always)]
    pub fn ucastp_1(self) -> &'a mut W {
        self.variant(UCASTP_A::UCASTP_1)
    }
    #[doc = "A STOP condition is generated automatically after the byte counter value reached UCBxTBCNT. UCBCNTIFG is set with the byte counter reaching the threshold"]
    #[inline(always)]
    pub fn ucastp_2(self) -> &'a mut W {
        self.variant(UCASTP_A::UCASTP_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn ucastp_3(self) -> &'a mut W {
        self.variant(UCASTP_A::UCASTP_3)
    }
}
#[doc = "Field `UCSWACK` reader - SW or HW ACK control"]
pub type UCSWACK_R = crate::BitReader<UCSWACK_A>;
#[doc = "SW or HW ACK control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSWACK_A {
    #[doc = "0: The address acknowledge of the slave is controlled by the eUSCI_B module"]
    UCSWACK_0 = 0,
    #[doc = "1: The user needs to trigger the sending of the address ACK by issuing UCTXACK"]
    UCSWACK_1 = 1,
}
impl From<UCSWACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCSWACK_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSWACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSWACK_A {
        match self.bits {
            false => UCSWACK_A::UCSWACK_0,
            true => UCSWACK_A::UCSWACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSWACK_0`"]
    #[inline(always)]
    pub fn is_ucswack_0(&self) -> bool {
        *self == UCSWACK_A::UCSWACK_0
    }
    #[doc = "Checks if the value of the field is `UCSWACK_1`"]
    #[inline(always)]
    pub fn is_ucswack_1(&self) -> bool {
        *self == UCSWACK_A::UCSWACK_1
    }
}
#[doc = "Field `UCSWACK` writer - SW or HW ACK control"]
pub type UCSWACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1CTLW1_SPEC, UCSWACK_A, O>;
impl<'a, const O: u8> UCSWACK_W<'a, O> {
    #[doc = "The address acknowledge of the slave is controlled by the eUSCI_B module"]
    #[inline(always)]
    pub fn ucswack_0(self) -> &'a mut W {
        self.variant(UCSWACK_A::UCSWACK_0)
    }
    #[doc = "The user needs to trigger the sending of the address ACK by issuing UCTXACK"]
    #[inline(always)]
    pub fn ucswack_1(self) -> &'a mut W {
        self.variant(UCSWACK_A::UCSWACK_1)
    }
}
#[doc = "Field `UCSTPNACK` reader - ACK all master bytes"]
pub type UCSTPNACK_R = crate::BitReader<UCSTPNACK_A>;
#[doc = "ACK all master bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSTPNACK_A {
    #[doc = "0: Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)"]
    UCSTPNACK_0 = 0,
    #[doc = "1: All bytes are acknowledged by the eUSCI_B when configured as master receiver"]
    UCSTPNACK_1 = 1,
}
impl From<UCSTPNACK_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTPNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSTPNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTPNACK_A {
        match self.bits {
            false => UCSTPNACK_A::UCSTPNACK_0,
            true => UCSTPNACK_A::UCSTPNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTPNACK_0`"]
    #[inline(always)]
    pub fn is_ucstpnack_0(&self) -> bool {
        *self == UCSTPNACK_A::UCSTPNACK_0
    }
    #[doc = "Checks if the value of the field is `UCSTPNACK_1`"]
    #[inline(always)]
    pub fn is_ucstpnack_1(&self) -> bool {
        *self == UCSTPNACK_A::UCSTPNACK_1
    }
}
#[doc = "Field `UCSTPNACK` writer - ACK all master bytes"]
pub type UCSTPNACK_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1CTLW1_SPEC, UCSTPNACK_A, O>;
impl<'a, const O: u8> UCSTPNACK_W<'a, O> {
    #[doc = "Send a non-acknowledge before the STOP condition as a master receiver (conform to I2C standard)"]
    #[inline(always)]
    pub fn ucstpnack_0(self) -> &'a mut W {
        self.variant(UCSTPNACK_A::UCSTPNACK_0)
    }
    #[doc = "All bytes are acknowledged by the eUSCI_B when configured as master receiver"]
    #[inline(always)]
    pub fn ucstpnack_1(self) -> &'a mut W {
        self.variant(UCSTPNACK_A::UCSTPNACK_1)
    }
}
#[doc = "Field `UCCLTO` reader - Clock low timeout select"]
pub type UCCLTO_R = crate::FieldReader<u8, UCCLTO_A>;
#[doc = "Clock low timeout select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCCLTO_A {
    #[doc = "0: Disable clock low timeout counter"]
    UCCLTO_0 = 0,
    #[doc = "1: 135 000 SYSCLK cycles (approximately 28 ms)"]
    UCCLTO_1 = 1,
    #[doc = "2: 150 000 SYSCLK cycles (approximately 31 ms)"]
    UCCLTO_2 = 2,
    #[doc = "3: 165 000 SYSCLK cycles (approximately 34 ms)"]
    UCCLTO_3 = 3,
}
impl From<UCCLTO_A> for u8 {
    #[inline(always)]
    fn from(variant: UCCLTO_A) -> Self {
        variant as _
    }
}
impl UCCLTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCLTO_A {
        match self.bits {
            0 => UCCLTO_A::UCCLTO_0,
            1 => UCCLTO_A::UCCLTO_1,
            2 => UCCLTO_A::UCCLTO_2,
            3 => UCCLTO_A::UCCLTO_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCCLTO_0`"]
    #[inline(always)]
    pub fn is_ucclto_0(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_0
    }
    #[doc = "Checks if the value of the field is `UCCLTO_1`"]
    #[inline(always)]
    pub fn is_ucclto_1(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_1
    }
    #[doc = "Checks if the value of the field is `UCCLTO_2`"]
    #[inline(always)]
    pub fn is_ucclto_2(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_2
    }
    #[doc = "Checks if the value of the field is `UCCLTO_3`"]
    #[inline(always)]
    pub fn is_ucclto_3(&self) -> bool {
        *self == UCCLTO_A::UCCLTO_3
    }
}
#[doc = "Field `UCCLTO` writer - Clock low timeout select"]
pub type UCCLTO_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, UCB1CTLW1_SPEC, u8, UCCLTO_A, 2, O>;
impl<'a, const O: u8> UCCLTO_W<'a, O> {
    #[doc = "Disable clock low timeout counter"]
    #[inline(always)]
    pub fn ucclto_0(self) -> &'a mut W {
        self.variant(UCCLTO_A::UCCLTO_0)
    }
    #[doc = "135 000 SYSCLK cycles (approximately 28 ms)"]
    #[inline(always)]
    pub fn ucclto_1(self) -> &'a mut W {
        self.variant(UCCLTO_A::UCCLTO_1)
    }
    #[doc = "150 000 SYSCLK cycles (approximately 31 ms)"]
    #[inline(always)]
    pub fn ucclto_2(self) -> &'a mut W {
        self.variant(UCCLTO_A::UCCLTO_2)
    }
    #[doc = "165 000 SYSCLK cycles (approximately 34 ms)"]
    #[inline(always)]
    pub fn ucclto_3(self) -> &'a mut W {
        self.variant(UCCLTO_A::UCCLTO_3)
    }
}
#[doc = "Field `UCETXINT` reader - Early UCTXIFG0"]
pub type UCETXINT_R = crate::BitReader<UCETXINT_A>;
#[doc = "Early UCTXIFG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCETXINT_A {
    #[doc = "0: UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit"]
    UCETXINT_0 = 0,
    #[doc = "1: UCTXIFG0 is set for each START condition"]
    UCETXINT_1 = 1,
}
impl From<UCETXINT_A> for bool {
    #[inline(always)]
    fn from(variant: UCETXINT_A) -> Self {
        variant as u8 != 0
    }
}
impl UCETXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCETXINT_A {
        match self.bits {
            false => UCETXINT_A::UCETXINT_0,
            true => UCETXINT_A::UCETXINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCETXINT_0`"]
    #[inline(always)]
    pub fn is_ucetxint_0(&self) -> bool {
        *self == UCETXINT_A::UCETXINT_0
    }
    #[doc = "Checks if the value of the field is `UCETXINT_1`"]
    #[inline(always)]
    pub fn is_ucetxint_1(&self) -> bool {
        *self == UCETXINT_A::UCETXINT_1
    }
}
#[doc = "Field `UCETXINT` writer - Early UCTXIFG0"]
pub type UCETXINT_W<'a, const O: u8> = crate::BitWriter<'a, u16, UCB1CTLW1_SPEC, UCETXINT_A, O>;
impl<'a, const O: u8> UCETXINT_W<'a, O> {
    #[doc = "UCTXIFGx is set after an address match with UCxI2COAx and the direction bit indicating slave transmit"]
    #[inline(always)]
    pub fn ucetxint_0(self) -> &'a mut W {
        self.variant(UCETXINT_A::UCETXINT_0)
    }
    #[doc = "UCTXIFG0 is set for each START condition"]
    #[inline(always)]
    pub fn ucetxint_1(self) -> &'a mut W {
        self.variant(UCETXINT_A::UCETXINT_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Automatic STOP condition generation"]
    #[inline(always)]
    pub fn ucastp(&self) -> UCASTP_R {
        UCASTP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SW or HW ACK control"]
    #[inline(always)]
    pub fn ucswack(&self) -> UCSWACK_R {
        UCSWACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK all master bytes"]
    #[inline(always)]
    pub fn ucstpnack(&self) -> UCSTPNACK_R {
        UCSTPNACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Clock low timeout select"]
    #[inline(always)]
    pub fn ucclto(&self) -> UCCLTO_R {
        UCCLTO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&self) -> UCETXINT_R {
        UCETXINT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UCGLIT_W<0> {
        UCGLIT_W::new(self)
    }
    #[doc = "Bits 2:3 - Automatic STOP condition generation"]
    #[inline(always)]
    pub fn ucastp(&mut self) -> UCASTP_W<2> {
        UCASTP_W::new(self)
    }
    #[doc = "Bit 4 - SW or HW ACK control"]
    #[inline(always)]
    pub fn ucswack(&mut self) -> UCSWACK_W<4> {
        UCSWACK_W::new(self)
    }
    #[doc = "Bit 5 - ACK all master bytes"]
    #[inline(always)]
    pub fn ucstpnack(&mut self) -> UCSTPNACK_W<5> {
        UCSTPNACK_W::new(self)
    }
    #[doc = "Bits 6:7 - Clock low timeout select"]
    #[inline(always)]
    pub fn ucclto(&mut self) -> UCCLTO_W<6> {
        UCCLTO_W::new(self)
    }
    #[doc = "Bit 8 - Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&mut self) -> UCETXINT_W<8> {
        UCETXINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctlw1](index.html) module"]
pub struct UCB1CTLW1_SPEC;
impl crate::RegisterSpec for UCB1CTLW1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb1ctlw1::R](R) reader structure"]
impl crate::Readable for UCB1CTLW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb1ctlw1::W](W) writer structure"]
impl crate::Writable for UCB1CTLW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB1CTLW1 to value 0"]
impl crate::Resettable for UCB1CTLW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
