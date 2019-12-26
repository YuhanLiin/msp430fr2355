#[doc = "Reader of register UCB0CTLW1"]
pub type R = crate::R<u16, super::UCB0CTLW1>;
#[doc = "Writer for register UCB0CTLW1"]
pub type W = crate::W<u16, super::UCB0CTLW1>;
#[doc = "Register UCB0CTLW1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0CTLW1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Deglitch time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `UCGLIT`"]
pub type UCGLIT_R = crate::R<u8, UCGLIT_A>;
impl UCGLIT_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCGLIT`"]
pub struct UCGLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGLIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCGLIT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Automatic STOP condition generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `UCASTP`"]
pub type UCASTP_R = crate::R<u8, UCASTP_A>;
impl UCASTP_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCASTP`"]
pub struct UCASTP_W<'a> {
    w: &'a mut W,
}
impl<'a> UCASTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCASTP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "SW or HW ACK control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `UCSWACK`"]
pub type UCSWACK_R = crate::R<bool, UCSWACK_A>;
impl UCSWACK_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCSWACK`"]
pub struct UCSWACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSWACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSWACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "ACK all master bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `UCSTPNACK`"]
pub type UCSTPNACK_R = crate::R<bool, UCSTPNACK_A>;
impl UCSTPNACK_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCSTPNACK`"]
pub struct UCSTPNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTPNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Clock low timeout select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `UCCLTO`"]
pub type UCCLTO_R = crate::R<u8, UCCLTO_A>;
impl UCCLTO_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCCLTO`"]
pub struct UCCLTO_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCLTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCCLTO_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Early UCTXIFG0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `UCETXINT`"]
pub type UCETXINT_R = crate::R<bool, UCETXINT_A>;
impl UCETXINT_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `UCETXINT`"]
pub struct UCETXINT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCETXINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCETXINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Automatic STOP condition generation"]
    #[inline(always)]
    pub fn ucastp(&self) -> UCASTP_R {
        UCASTP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - SW or HW ACK control"]
    #[inline(always)]
    pub fn ucswack(&self) -> UCSWACK_R {
        UCSWACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACK all master bytes"]
    #[inline(always)]
    pub fn ucstpnack(&self) -> UCSTPNACK_R {
        UCSTPNACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Clock low timeout select"]
    #[inline(always)]
    pub fn ucclto(&self) -> UCCLTO_R {
        UCCLTO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&self) -> UCETXINT_R {
        UCETXINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UCGLIT_W {
        UCGLIT_W { w: self }
    }
    #[doc = "Bits 2:3 - Automatic STOP condition generation"]
    #[inline(always)]
    pub fn ucastp(&mut self) -> UCASTP_W {
        UCASTP_W { w: self }
    }
    #[doc = "Bit 4 - SW or HW ACK control"]
    #[inline(always)]
    pub fn ucswack(&mut self) -> UCSWACK_W {
        UCSWACK_W { w: self }
    }
    #[doc = "Bit 5 - ACK all master bytes"]
    #[inline(always)]
    pub fn ucstpnack(&mut self) -> UCSTPNACK_W {
        UCSTPNACK_W { w: self }
    }
    #[doc = "Bits 6:7 - Clock low timeout select"]
    #[inline(always)]
    pub fn ucclto(&mut self) -> UCCLTO_W {
        UCCLTO_W { w: self }
    }
    #[doc = "Bit 8 - Early UCTXIFG0"]
    #[inline(always)]
    pub fn ucetxint(&mut self) -> UCETXINT_W {
        UCETXINT_W { w: self }
    }
}
