#[doc = "Reader of register UCA0CTLW0"]
pub type R = crate::R<u16, super::UCA0CTLW0>;
#[doc = "Writer for register UCA0CTLW0"]
pub type W = crate::W<u16, super::UCA0CTLW0>;
#[doc = "Register UCA0CTLW0 `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA0CTLW0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSWRST_A {
    #[doc = "0: Disabled. eUSCI_A reset released for operation"]
    DISABLE,
    #[doc = "1: Enabled. eUSCI_A logic held in reset state"]
    ENABLE,
}
impl From<UCSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: UCSWRST_A) -> Self {
        match variant {
            UCSWRST_A::DISABLE => false,
            UCSWRST_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `UCSWRST`"]
pub type UCSWRST_R = crate::R<bool, UCSWRST_A>;
impl UCSWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSWRST_A {
        match self.bits {
            false => UCSWRST_A::DISABLE,
            true => UCSWRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UCSWRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == UCSWRST_A::ENABLE
    }
}
#[doc = "Write proxy for field `UCSWRST`"]
pub struct UCSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. eUSCI_A reset released for operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UCSWRST_A::DISABLE)
    }
    #[doc = "Enabled. eUSCI_A logic held in reset state"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UCSWRST_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Transmit break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXBRK_A {
    #[doc = "0: Next frame transmitted is not a break"]
    UCTXBRK_0,
    #[doc = "1: Next frame transmitted is a break or a break/synch"]
    UCTXBRK_1,
}
impl From<UCTXBRK_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXBRK_A) -> Self {
        match variant {
            UCTXBRK_A::UCTXBRK_0 => false,
            UCTXBRK_A::UCTXBRK_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXBRK`"]
pub type UCTXBRK_R = crate::R<bool, UCTXBRK_A>;
impl UCTXBRK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXBRK_A {
        match self.bits {
            false => UCTXBRK_A::UCTXBRK_0,
            true => UCTXBRK_A::UCTXBRK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXBRK_0`"]
    #[inline(always)]
    pub fn is_uctxbrk_0(&self) -> bool {
        *self == UCTXBRK_A::UCTXBRK_0
    }
    #[doc = "Checks if the value of the field is `UCTXBRK_1`"]
    #[inline(always)]
    pub fn is_uctxbrk_1(&self) -> bool {
        *self == UCTXBRK_A::UCTXBRK_1
    }
}
#[doc = "Write proxy for field `UCTXBRK`"]
pub struct UCTXBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXBRK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXBRK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Next frame transmitted is not a break"]
    #[inline(always)]
    pub fn uctxbrk_0(self) -> &'a mut W {
        self.variant(UCTXBRK_A::UCTXBRK_0)
    }
    #[doc = "Next frame transmitted is a break or a break/synch"]
    #[inline(always)]
    pub fn uctxbrk_1(self) -> &'a mut W {
        self.variant(UCTXBRK_A::UCTXBRK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Transmit address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXADDR_A {
    #[doc = "0: Next frame transmitted is data"]
    UCTXADDR_0,
    #[doc = "1: Next frame transmitted is an address"]
    UCTXADDR_1,
}
impl From<UCTXADDR_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXADDR_A) -> Self {
        match variant {
            UCTXADDR_A::UCTXADDR_0 => false,
            UCTXADDR_A::UCTXADDR_1 => true,
        }
    }
}
#[doc = "Reader of field `UCTXADDR`"]
pub type UCTXADDR_R = crate::R<bool, UCTXADDR_A>;
impl UCTXADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXADDR_A {
        match self.bits {
            false => UCTXADDR_A::UCTXADDR_0,
            true => UCTXADDR_A::UCTXADDR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXADDR_0`"]
    #[inline(always)]
    pub fn is_uctxaddr_0(&self) -> bool {
        *self == UCTXADDR_A::UCTXADDR_0
    }
    #[doc = "Checks if the value of the field is `UCTXADDR_1`"]
    #[inline(always)]
    pub fn is_uctxaddr_1(&self) -> bool {
        *self == UCTXADDR_A::UCTXADDR_1
    }
}
#[doc = "Write proxy for field `UCTXADDR`"]
pub struct UCTXADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Next frame transmitted is data"]
    #[inline(always)]
    pub fn uctxaddr_0(self) -> &'a mut W {
        self.variant(UCTXADDR_A::UCTXADDR_0)
    }
    #[doc = "Next frame transmitted is an address"]
    #[inline(always)]
    pub fn uctxaddr_1(self) -> &'a mut W {
        self.variant(UCTXADDR_A::UCTXADDR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Dormant\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCDORM_A {
    #[doc = "0: Not dormant. All received characters set UCRXIFG."]
    UCDORM_0,
    #[doc = "1: Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG."]
    UCDORM_1,
}
impl From<UCDORM_A> for bool {
    #[inline(always)]
    fn from(variant: UCDORM_A) -> Self {
        match variant {
            UCDORM_A::UCDORM_0 => false,
            UCDORM_A::UCDORM_1 => true,
        }
    }
}
#[doc = "Reader of field `UCDORM`"]
pub type UCDORM_R = crate::R<bool, UCDORM_A>;
impl UCDORM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCDORM_A {
        match self.bits {
            false => UCDORM_A::UCDORM_0,
            true => UCDORM_A::UCDORM_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCDORM_0`"]
    #[inline(always)]
    pub fn is_ucdorm_0(&self) -> bool {
        *self == UCDORM_A::UCDORM_0
    }
    #[doc = "Checks if the value of the field is `UCDORM_1`"]
    #[inline(always)]
    pub fn is_ucdorm_1(&self) -> bool {
        *self == UCDORM_A::UCDORM_1
    }
}
#[doc = "Write proxy for field `UCDORM`"]
pub struct UCDORM_W<'a> {
    w: &'a mut W,
}
impl<'a> UCDORM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCDORM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not dormant. All received characters set UCRXIFG."]
    #[inline(always)]
    pub fn ucdorm_0(self) -> &'a mut W {
        self.variant(UCDORM_A::UCDORM_0)
    }
    #[doc = "Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG."]
    #[inline(always)]
    pub fn ucdorm_1(self) -> &'a mut W {
        self.variant(UCDORM_A::UCDORM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Receive break character interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRKIE_A {
    #[doc = "0: Received break characters do not set UCRXIFG"]
    UCBRKIE_0,
    #[doc = "1: Received break characters set UCRXIFG"]
    UCBRKIE_1,
}
impl From<UCBRKIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCBRKIE_A) -> Self {
        match variant {
            UCBRKIE_A::UCBRKIE_0 => false,
            UCBRKIE_A::UCBRKIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCBRKIE`"]
pub type UCBRKIE_R = crate::R<bool, UCBRKIE_A>;
impl UCBRKIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBRKIE_A {
        match self.bits {
            false => UCBRKIE_A::UCBRKIE_0,
            true => UCBRKIE_A::UCBRKIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBRKIE_0`"]
    #[inline(always)]
    pub fn is_ucbrkie_0(&self) -> bool {
        *self == UCBRKIE_A::UCBRKIE_0
    }
    #[doc = "Checks if the value of the field is `UCBRKIE_1`"]
    #[inline(always)]
    pub fn is_ucbrkie_1(&self) -> bool {
        *self == UCBRKIE_A::UCBRKIE_1
    }
}
#[doc = "Write proxy for field `UCBRKIE`"]
pub struct UCBRKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRKIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBRKIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Received break characters do not set UCRXIFG"]
    #[inline(always)]
    pub fn ucbrkie_0(self) -> &'a mut W {
        self.variant(UCBRKIE_A::UCBRKIE_0)
    }
    #[doc = "Received break characters set UCRXIFG"]
    #[inline(always)]
    pub fn ucbrkie_1(self) -> &'a mut W {
        self.variant(UCBRKIE_A::UCBRKIE_1)
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
#[doc = "Receive erroneous-character interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXEIE_A {
    #[doc = "0: Erroneous characters rejected and UCRXIFG is not set"]
    UCRXEIE_0,
    #[doc = "1: Erroneous characters received set UCRXIFG"]
    UCRXEIE_1,
}
impl From<UCRXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXEIE_A) -> Self {
        match variant {
            UCRXEIE_A::UCRXEIE_0 => false,
            UCRXEIE_A::UCRXEIE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCRXEIE`"]
pub type UCRXEIE_R = crate::R<bool, UCRXEIE_A>;
impl UCRXEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXEIE_A {
        match self.bits {
            false => UCRXEIE_A::UCRXEIE_0,
            true => UCRXEIE_A::UCRXEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXEIE_0`"]
    #[inline(always)]
    pub fn is_ucrxeie_0(&self) -> bool {
        *self == UCRXEIE_A::UCRXEIE_0
    }
    #[doc = "Checks if the value of the field is `UCRXEIE_1`"]
    #[inline(always)]
    pub fn is_ucrxeie_1(&self) -> bool {
        *self == UCRXEIE_A::UCRXEIE_1
    }
}
#[doc = "Write proxy for field `UCRXEIE`"]
pub struct UCRXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Erroneous characters rejected and UCRXIFG is not set"]
    #[inline(always)]
    pub fn ucrxeie_0(self) -> &'a mut W {
        self.variant(UCRXEIE_A::UCRXEIE_0)
    }
    #[doc = "Erroneous characters received set UCRXIFG"]
    #[inline(always)]
    pub fn ucrxeie_1(self) -> &'a mut W {
        self.variant(UCRXEIE_A::UCRXEIE_1)
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
#[doc = "eUSCI_A clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSSEL_A {
    #[doc = "0: UCLK"]
    UCLK,
    #[doc = "1: ACLK"]
    ACLK,
    #[doc = "2: SMCLK"]
    SMCLK,
    #[doc = "3: SMCLK"]
    UCSSEL_3,
}
impl From<UCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCSSEL_A) -> Self {
        match variant {
            UCSSEL_A::UCLK => 0,
            UCSSEL_A::ACLK => 1,
            UCSSEL_A::SMCLK => 2,
            UCSSEL_A::UCSSEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `UCSSEL`"]
pub type UCSSEL_R = crate::R<u8, UCSSEL_A>;
impl UCSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSSEL_A {
        match self.bits {
            0 => UCSSEL_A::UCLK,
            1 => UCSSEL_A::ACLK,
            2 => UCSSEL_A::SMCLK,
            3 => UCSSEL_A::UCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCLK`"]
    #[inline(always)]
    pub fn is_uclk(&self) -> bool {
        *self == UCSSEL_A::UCLK
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == UCSSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == UCSSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `UCSSEL_3`"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == UCSSEL_A::UCSSEL_3
    }
}
#[doc = "Write proxy for field `UCSSEL`"]
pub struct UCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "UCLK"]
    #[inline(always)]
    pub fn uclk(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCLK)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(UCSSEL_A::ACLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(UCSSEL_A::SMCLK)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut W {
        self.variant(UCSSEL_A::UCSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "Synchronous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSYNC_A {
    #[doc = "0: Asynchronous mode"]
    ASYNC,
    #[doc = "1: Synchronous mode"]
    SYNC,
}
impl From<UCSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: UCSYNC_A) -> Self {
        match variant {
            UCSYNC_A::ASYNC => false,
            UCSYNC_A::SYNC => true,
        }
    }
}
#[doc = "Reader of field `UCSYNC`"]
pub type UCSYNC_R = crate::R<bool, UCSYNC_A>;
impl UCSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSYNC_A {
        match self.bits {
            false => UCSYNC_A::ASYNC,
            true => UCSYNC_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == UCSYNC_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == UCSYNC_A::SYNC
    }
}
#[doc = "Write proxy for field `UCSYNC`"]
pub struct UCSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn async(self) -> &'a mut W {
        self.variant(UCSYNC_A::ASYNC)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(UCSYNC_A::SYNC)
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
#[doc = "eUSCI_A mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMODE_A {
    #[doc = "0: UART mode"]
    UCMODE_0,
    #[doc = "1: Idle-line multiprocessor mode"]
    UCMODE_1,
    #[doc = "2: Address-bit multiprocessor mode"]
    UCMODE_2,
    #[doc = "3: UART mode with automatic baud-rate detection"]
    UCMODE_3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        match variant {
            UCMODE_A::UCMODE_0 => 0,
            UCMODE_A::UCMODE_1 => 1,
            UCMODE_A::UCMODE_2 => 2,
            UCMODE_A::UCMODE_3 => 3,
        }
    }
}
#[doc = "Reader of field `UCMODE`"]
pub type UCMODE_R = crate::R<u8, UCMODE_A>;
impl UCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Write proxy for field `UCMODE`"]
pub struct UCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "UART mode"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Idle-line multiprocessor mode"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Address-bit multiprocessor mode"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "UART mode with automatic baud-rate detection"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u16) & 0x03) << 9);
        self.w
    }
}
#[doc = "Stop bit select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSPB_A {
    #[doc = "0: One stop bit"]
    UCSPB_0,
    #[doc = "1: Two stop bits"]
    UCSPB_1,
}
impl From<UCSPB_A> for bool {
    #[inline(always)]
    fn from(variant: UCSPB_A) -> Self {
        match variant {
            UCSPB_A::UCSPB_0 => false,
            UCSPB_A::UCSPB_1 => true,
        }
    }
}
#[doc = "Reader of field `UCSPB`"]
pub type UCSPB_R = crate::R<bool, UCSPB_A>;
impl UCSPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSPB_A {
        match self.bits {
            false => UCSPB_A::UCSPB_0,
            true => UCSPB_A::UCSPB_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSPB_0`"]
    #[inline(always)]
    pub fn is_ucspb_0(&self) -> bool {
        *self == UCSPB_A::UCSPB_0
    }
    #[doc = "Checks if the value of the field is `UCSPB_1`"]
    #[inline(always)]
    pub fn is_ucspb_1(&self) -> bool {
        *self == UCSPB_A::UCSPB_1
    }
}
#[doc = "Write proxy for field `UCSPB`"]
pub struct UCSPB_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn ucspb_0(self) -> &'a mut W {
        self.variant(UCSPB_A::UCSPB_0)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn ucspb_1(self) -> &'a mut W {
        self.variant(UCSPB_A::UCSPB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Character length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UC7BIT_A {
    #[doc = "0: 8-bit data"]
    _8BIT,
    #[doc = "1: 7-bit data"]
    _7BIT,
}
impl From<UC7BIT_A> for bool {
    #[inline(always)]
    fn from(variant: UC7BIT_A) -> Self {
        match variant {
            UC7BIT_A::_8BIT => false,
            UC7BIT_A::_7BIT => true,
        }
    }
}
#[doc = "Reader of field `UC7BIT`"]
pub type UC7BIT_R = crate::R<bool, UC7BIT_A>;
impl UC7BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UC7BIT_A {
        match self.bits {
            false => UC7BIT_A::_8BIT,
            true => UC7BIT_A::_7BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == UC7BIT_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_7BIT`"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == UC7BIT_A::_7BIT
    }
}
#[doc = "Write proxy for field `UC7BIT`"]
pub struct UC7BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UC7BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UC7BIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(UC7BIT_A::_8BIT)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut W {
        self.variant(UC7BIT_A::_7BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "MSB first select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCMSB_A {
    #[doc = "0: LSB first"]
    UCMSB_0,
    #[doc = "1: MSB first"]
    UCMSB_1,
}
impl From<UCMSB_A> for bool {
    #[inline(always)]
    fn from(variant: UCMSB_A) -> Self {
        match variant {
            UCMSB_A::UCMSB_0 => false,
            UCMSB_A::UCMSB_1 => true,
        }
    }
}
#[doc = "Reader of field `UCMSB`"]
pub type UCMSB_R = crate::R<bool, UCMSB_A>;
impl UCMSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMSB_A {
        match self.bits {
            false => UCMSB_A::UCMSB_0,
            true => UCMSB_A::UCMSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCMSB_0`"]
    #[inline(always)]
    pub fn is_ucmsb_0(&self) -> bool {
        *self == UCMSB_A::UCMSB_0
    }
    #[doc = "Checks if the value of the field is `UCMSB_1`"]
    #[inline(always)]
    pub fn is_ucmsb_1(&self) -> bool {
        *self == UCMSB_A::UCMSB_1
    }
}
#[doc = "Write proxy for field `UCMSB`"]
pub struct UCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> UCMSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCMSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn ucmsb_0(self) -> &'a mut W {
        self.variant(UCMSB_A::UCMSB_0)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn ucmsb_1(self) -> &'a mut W {
        self.variant(UCMSB_A::UCMSB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Parity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCPAR_A {
    #[doc = "0: Odd parity"]
    ODD,
    #[doc = "1: Even parity"]
    EVEN,
}
impl From<UCPAR_A> for bool {
    #[inline(always)]
    fn from(variant: UCPAR_A) -> Self {
        match variant {
            UCPAR_A::ODD => false,
            UCPAR_A::EVEN => true,
        }
    }
}
#[doc = "Reader of field `UCPAR`"]
pub type UCPAR_R = crate::R<bool, UCPAR_A>;
impl UCPAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPAR_A {
        match self.bits {
            false => UCPAR_A::ODD,
            true => UCPAR_A::EVEN,
        }
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == UCPAR_A::ODD
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == UCPAR_A::EVEN
    }
}
#[doc = "Write proxy for field `UCPAR`"]
pub struct UCPAR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCPAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(UCPAR_A::ODD)
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(UCPAR_A::EVEN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCPEN_A {
    #[doc = "0: Parity disabled"]
    UCPEN_0,
    #[doc = "1: Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation."]
    UCPEN_1,
}
impl From<UCPEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCPEN_A) -> Self {
        match variant {
            UCPEN_A::UCPEN_0 => false,
            UCPEN_A::UCPEN_1 => true,
        }
    }
}
#[doc = "Reader of field `UCPEN`"]
pub type UCPEN_R = crate::R<bool, UCPEN_A>;
impl UCPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCPEN_A {
        match self.bits {
            false => UCPEN_A::UCPEN_0,
            true => UCPEN_A::UCPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCPEN_0`"]
    #[inline(always)]
    pub fn is_ucpen_0(&self) -> bool {
        *self == UCPEN_A::UCPEN_0
    }
    #[doc = "Checks if the value of the field is `UCPEN_1`"]
    #[inline(always)]
    pub fn is_ucpen_1(&self) -> bool {
        *self == UCPEN_A::UCPEN_1
    }
}
#[doc = "Write proxy for field `UCPEN`"]
pub struct UCPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Parity disabled"]
    #[inline(always)]
    pub fn ucpen_0(self) -> &'a mut W {
        self.variant(UCPEN_A::UCPEN_0)
    }
    #[doc = "Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation."]
    #[inline(always)]
    pub fn ucpen_1(self) -> &'a mut W {
        self.variant(UCPEN_A::UCPEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UCSWRST_R {
        UCSWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&self) -> UCTXBRK_R {
        UCTXBRK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit address"]
    #[inline(always)]
    pub fn uctxaddr(&self) -> UCTXADDR_R {
        UCTXADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dormant"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UCDORM_R {
        UCDORM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UCBRKIE_R {
        UCBRKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive erroneous-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UCRXEIE_R {
        UCRXEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UCSSEL_R {
        UCSSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Stop bit select"]
    #[inline(always)]
    pub fn ucspb(&self) -> UCSPB_R {
        UCSPB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&self) -> UCPAR_R {
        UCPAR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UCPEN_R {
        UCPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UCSWRST_W {
        UCSWRST_W { w: self }
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&mut self) -> UCTXBRK_W {
        UCTXBRK_W { w: self }
    }
    #[doc = "Bit 2 - Transmit address"]
    #[inline(always)]
    pub fn uctxaddr(&mut self) -> UCTXADDR_W {
        UCTXADDR_W { w: self }
    }
    #[doc = "Bit 3 - Dormant"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UCDORM_W {
        UCDORM_W { w: self }
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UCBRKIE_W {
        UCBRKIE_W { w: self }
    }
    #[doc = "Bit 5 - Receive erroneous-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UCRXEIE_W {
        UCRXEIE_W { w: self }
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UCSSEL_W {
        UCSSEL_W { w: self }
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UCSYNC_W {
        UCSYNC_W { w: self }
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UCMODE_W {
        UCMODE_W { w: self }
    }
    #[doc = "Bit 11 - Stop bit select"]
    #[inline(always)]
    pub fn ucspb(&mut self) -> UCSPB_W {
        UCSPB_W { w: self }
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> UC7BIT_W {
        UC7BIT_W { w: self }
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UCMSB_W {
        UCMSB_W { w: self }
    }
    #[doc = "Bit 14 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&mut self) -> UCPAR_W {
        UCPAR_W { w: self }
    }
    #[doc = "Bit 15 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&mut self) -> UCPEN_W {
        UCPEN_W { w: self }
    }
}
