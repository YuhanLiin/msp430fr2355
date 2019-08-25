#[doc = "Reader of register SFRIFG1"]
pub type R = crate::R<u16, super::SFRIFG1>;
#[doc = "Writer for register SFRIFG1"]
pub type W = crate::W<u16, super::SFRIFG1>;
#[doc = "Register SFRIFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SFRIFG1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFIFG_A {
    #[doc = "0: No interrupt pending"]
    OFIFG_0,
    #[doc = "1: Interrupt pending"]
    OFIFG_1,
}
impl From<OFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: OFIFG_A) -> Self {
        match variant {
            OFIFG_A::OFIFG_0 => false,
            OFIFG_A::OFIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `OFIFG`"]
pub type OFIFG_R = crate::R<bool, OFIFG_A>;
impl OFIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFIFG_A {
        match self.bits {
            false => OFIFG_A::OFIFG_0,
            true => OFIFG_A::OFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `OFIFG_0`"]
    #[inline(always)]
    pub fn is_ofifg_0(&self) -> bool {
        *self == OFIFG_A::OFIFG_0
    }
    #[doc = "Checks if the value of the field is `OFIFG_1`"]
    #[inline(always)]
    pub fn is_ofifg_1(&self) -> bool {
        *self == OFIFG_A::OFIFG_1
    }
}
#[doc = "Write proxy for field `OFIFG`"]
pub struct OFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OFIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ofifg_0(self) -> &'a mut W {
        self.variant(OFIFG_A::OFIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ofifg_1(self) -> &'a mut W {
        self.variant(OFIFG_A::OFIFG_1)
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
#[doc = "Vacant memory access interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VMAIFG_A {
    #[doc = "0: No interrupt pending"]
    VMAIFG_0,
    #[doc = "1: Interrupt pending"]
    VMAIFG_1,
}
impl From<VMAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: VMAIFG_A) -> Self {
        match variant {
            VMAIFG_A::VMAIFG_0 => false,
            VMAIFG_A::VMAIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `VMAIFG`"]
pub type VMAIFG_R = crate::R<bool, VMAIFG_A>;
impl VMAIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMAIFG_A {
        match self.bits {
            false => VMAIFG_A::VMAIFG_0,
            true => VMAIFG_A::VMAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `VMAIFG_0`"]
    #[inline(always)]
    pub fn is_vmaifg_0(&self) -> bool {
        *self == VMAIFG_A::VMAIFG_0
    }
    #[doc = "Checks if the value of the field is `VMAIFG_1`"]
    #[inline(always)]
    pub fn is_vmaifg_1(&self) -> bool {
        *self == VMAIFG_A::VMAIFG_1
    }
}
#[doc = "Write proxy for field `VMAIFG`"]
pub struct VMAIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VMAIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VMAIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn vmaifg_0(self) -> &'a mut W {
        self.variant(VMAIFG_A::VMAIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn vmaifg_1(self) -> &'a mut W {
        self.variant(VMAIFG_A::VMAIFG_1)
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
#[doc = "NMI pin interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIIFG_A {
    #[doc = "0: No interrupt pending"]
    NMIIFG_0,
    #[doc = "1: Interrupt pending"]
    NMIIFG_1,
}
impl From<NMIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: NMIIFG_A) -> Self {
        match variant {
            NMIIFG_A::NMIIFG_0 => false,
            NMIIFG_A::NMIIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `NMIIFG`"]
pub type NMIIFG_R = crate::R<bool, NMIIFG_A>;
impl NMIIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIIFG_A {
        match self.bits {
            false => NMIIFG_A::NMIIFG_0,
            true => NMIIFG_A::NMIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `NMIIFG_0`"]
    #[inline(always)]
    pub fn is_nmiifg_0(&self) -> bool {
        *self == NMIIFG_A::NMIIFG_0
    }
    #[doc = "Checks if the value of the field is `NMIIFG_1`"]
    #[inline(always)]
    pub fn is_nmiifg_1(&self) -> bool {
        *self == NMIIFG_A::NMIIFG_1
    }
}
#[doc = "Write proxy for field `NMIIFG`"]
pub struct NMIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn nmiifg_0(self) -> &'a mut W {
        self.variant(NMIIFG_A::NMIIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn nmiifg_1(self) -> &'a mut W {
        self.variant(NMIIFG_A::NMIIFG_1)
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
#[doc = "Watchdog timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTIFG_A {
    #[doc = "0: No interrupt pending"]
    WDTIFG_0,
    #[doc = "1: Interrupt pending"]
    WDTIFG_1,
}
impl From<WDTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: WDTIFG_A) -> Self {
        match variant {
            WDTIFG_A::WDTIFG_0 => false,
            WDTIFG_A::WDTIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `WDTIFG`"]
pub type WDTIFG_R = crate::R<bool, WDTIFG_A>;
impl WDTIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIFG_A {
        match self.bits {
            false => WDTIFG_A::WDTIFG_0,
            true => WDTIFG_A::WDTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDTIFG_0`"]
    #[inline(always)]
    pub fn is_wdtifg_0(&self) -> bool {
        *self == WDTIFG_A::WDTIFG_0
    }
    #[doc = "Checks if the value of the field is `WDTIFG_1`"]
    #[inline(always)]
    pub fn is_wdtifg_1(&self) -> bool {
        *self == WDTIFG_A::WDTIFG_1
    }
}
#[doc = "Write proxy for field `WDTIFG`"]
pub struct WDTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn wdtifg_0(self) -> &'a mut W {
        self.variant(WDTIFG_A::WDTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn wdtifg_1(self) -> &'a mut W {
        self.variant(WDTIFG_A::WDTIFG_1)
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
#[doc = "JTAG mailbox input interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBINIFG_A {
    #[doc = "0: No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    JMBINIFG_0,
    #[doc = "1: Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    JMBINIFG_1,
}
impl From<JMBINIFG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBINIFG_A) -> Self {
        match variant {
            JMBINIFG_A::JMBINIFG_0 => false,
            JMBINIFG_A::JMBINIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `JMBINIFG`"]
pub type JMBINIFG_R = crate::R<bool, JMBINIFG_A>;
impl JMBINIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBINIFG_A {
        match self.bits {
            false => JMBINIFG_A::JMBINIFG_0,
            true => JMBINIFG_A::JMBINIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBINIFG_0`"]
    #[inline(always)]
    pub fn is_jmbinifg_0(&self) -> bool {
        *self == JMBINIFG_A::JMBINIFG_0
    }
    #[doc = "Checks if the value of the field is `JMBINIFG_1`"]
    #[inline(always)]
    pub fn is_jmbinifg_1(&self) -> bool {
        *self == JMBINIFG_A::JMBINIFG_1
    }
}
#[doc = "Write proxy for field `JMBINIFG`"]
pub struct JMBINIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBINIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBINIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBI0 is read by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBI0 and JMBI1 have been read by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read"]
    #[inline(always)]
    pub fn jmbinifg_0(self) -> &'a mut W {
        self.variant(JMBINIFG_A::JMBINIFG_0)
    }
    #[doc = "Interrupt pending. A message is waiting in the JMBIN registers. In 16-bit mode (JMBMODE = 0) when JMBI0 has been written by the JTAG module. In 32-bit mode (JMBMODE = 1) when JMBI0 and JMBI1 have been written by the JTAG module."]
    #[inline(always)]
    pub fn jmbinifg_1(self) -> &'a mut W {
        self.variant(JMBINIFG_A::JMBINIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "JTAG mailbox output interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBOUTIFG_A {
    #[doc = "0: No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    JMBOUTIFG_0,
    #[doc = "1: Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    JMBOUTIFG_1,
}
impl From<JMBOUTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: JMBOUTIFG_A) -> Self {
        match variant {
            JMBOUTIFG_A::JMBOUTIFG_0 => false,
            JMBOUTIFG_A::JMBOUTIFG_1 => true,
        }
    }
}
#[doc = "Reader of field `JMBOUTIFG`"]
pub type JMBOUTIFG_R = crate::R<bool, JMBOUTIFG_A>;
impl JMBOUTIFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JMBOUTIFG_A {
        match self.bits {
            false => JMBOUTIFG_A::JMBOUTIFG_0,
            true => JMBOUTIFG_A::JMBOUTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JMBOUTIFG_0`"]
    #[inline(always)]
    pub fn is_jmboutifg_0(&self) -> bool {
        *self == JMBOUTIFG_A::JMBOUTIFG_0
    }
    #[doc = "Checks if the value of the field is `JMBOUTIFG_1`"]
    #[inline(always)]
    pub fn is_jmboutifg_1(&self) -> bool {
        *self == JMBOUTIFG_A::JMBOUTIFG_1
    }
}
#[doc = "Write proxy for field `JMBOUTIFG`"]
pub struct JMBOUTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> JMBOUTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JMBOUTIFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending. When in 16-bit mode (JMBMODE = 0), this bit is cleared automatically when JMBO0 has been written with a new message to the JTAG module by the CPU. When in 32-bit mode (JMBMODE = 1), this bit is cleared automatically when both JMBO0 and JMBO1 have been written with new messages to the JTAG module by the CPU. This bit is also cleared when the associated vector in SYSUNIV has been read."]
    #[inline(always)]
    pub fn jmboutifg_0(self) -> &'a mut W {
        self.variant(JMBOUTIFG_A::JMBOUTIFG_0)
    }
    #[doc = "Interrupt pending. JMBO registers are ready for new messages. In 16-bit mode (JMBMODE = 0), JMBO0 has been received by the JTAG module and is ready for a new message from the CPU. In 32-bit mode (JMBMODE = 1), JMBO0 and JMBO1 have been received by the JTAG module and are ready for new messages from the CPU."]
    #[inline(always)]
    pub fn jmboutifg_1(self) -> &'a mut W {
        self.variant(JMBOUTIFG_A::JMBOUTIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn ofifg(&self) -> OFIFG_R {
        OFIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vacant memory access interrupt flag"]
    #[inline(always)]
    pub fn vmaifg(&self) -> VMAIFG_R {
        VMAIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI pin interrupt flag"]
    #[inline(always)]
    pub fn nmiifg(&self) -> NMIIFG_R {
        NMIIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Watchdog timer interrupt flag"]
    #[inline(always)]
    pub fn wdtifg(&self) -> WDTIFG_R {
        WDTIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt flag"]
    #[inline(always)]
    pub fn jmbinifg(&self) -> JMBINIFG_R {
        JMBINIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt flag"]
    #[inline(always)]
    pub fn jmboutifg(&self) -> JMBOUTIFG_R {
        JMBOUTIFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn ofifg(&mut self) -> OFIFG_W {
        OFIFG_W { w: self }
    }
    #[doc = "Bit 3 - Vacant memory access interrupt flag"]
    #[inline(always)]
    pub fn vmaifg(&mut self) -> VMAIFG_W {
        VMAIFG_W { w: self }
    }
    #[doc = "Bit 4 - NMI pin interrupt flag"]
    #[inline(always)]
    pub fn nmiifg(&mut self) -> NMIIFG_W {
        NMIIFG_W { w: self }
    }
    #[doc = "Bit 0 - Watchdog timer interrupt flag"]
    #[inline(always)]
    pub fn wdtifg(&mut self) -> WDTIFG_W {
        WDTIFG_W { w: self }
    }
    #[doc = "Bit 6 - JTAG mailbox input interrupt flag"]
    #[inline(always)]
    pub fn jmbinifg(&mut self) -> JMBINIFG_W {
        JMBINIFG_W { w: self }
    }
    #[doc = "Bit 7 - JTAG mailbox output interrupt flag"]
    #[inline(always)]
    pub fn jmboutifg(&mut self) -> JMBOUTIFG_W {
        JMBOUTIFG_W { w: self }
    }
}
