#[doc = "Reader of register ICCSC"]
pub type R = crate::R<u16, super::ICCSC>;
#[doc = "Writer for register ICCSC"]
pub type W = crate::W<u16, super::ICCSC>;
#[doc = "Register ICCSC `reset()`'s with value 0"]
impl crate::ResetValue for super::ICCSC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICMC`"]
pub type ICMC_R = crate::R<u8, u8>;
#[doc = "Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSFFLG_A {
    #[doc = "0: ICCMVS register is not full"]
    VSFFLG_0 = 0,
    #[doc = "1: ICCMVS register is full"]
    VSFFLG_1 = 1,
}
impl From<VSFFLG_A> for bool {
    #[inline(always)]
    fn from(variant: VSFFLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSFFLG`"]
pub type VSFFLG_R = crate::R<bool, VSFFLG_A>;
impl VSFFLG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSFFLG_A {
        match self.bits {
            false => VSFFLG_A::VSFFLG_0,
            true => VSFFLG_A::VSFFLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSFFLG_0`"]
    #[inline(always)]
    pub fn is_vsfflg_0(&self) -> bool {
        *self == VSFFLG_A::VSFFLG_0
    }
    #[doc = "Checks if the value of the field is `VSFFLG_1`"]
    #[inline(always)]
    pub fn is_vsfflg_1(&self) -> bool {
        *self == VSFFLG_A::VSFFLG_1
    }
}
#[doc = "Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSEFLG_A {
    #[doc = "0: Stack has valid data"]
    VSEFLG_0 = 0,
    #[doc = "1: Stack has no valid data"]
    VSEFLG_1 = 1,
}
impl From<VSEFLG_A> for bool {
    #[inline(always)]
    fn from(variant: VSEFLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSEFLG`"]
pub type VSEFLG_R = crate::R<bool, VSEFLG_A>;
impl VSEFLG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSEFLG_A {
        match self.bits {
            false => VSEFLG_A::VSEFLG_0,
            true => VSEFLG_A::VSEFLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSEFLG_0`"]
    #[inline(always)]
    pub fn is_vseflg_0(&self) -> bool {
        *self == VSEFLG_A::VSEFLG_0
    }
    #[doc = "Checks if the value of the field is `VSEFLG_1`"]
    #[inline(always)]
    pub fn is_vseflg_1(&self) -> bool {
        *self == VSEFLG_A::VSEFLG_1
    }
}
#[doc = "ICC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICCEN_A {
    #[doc = "0: ICC module disabled"]
    ICCEN_0 = 0,
    #[doc = "1: ICC module enabled"]
    ICCEN_1 = 1,
}
impl From<ICCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ICCEN`"]
pub type ICCEN_R = crate::R<bool, ICCEN_A>;
impl ICCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICCEN_A {
        match self.bits {
            false => ICCEN_A::ICCEN_0,
            true => ICCEN_A::ICCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICCEN_0`"]
    #[inline(always)]
    pub fn is_iccen_0(&self) -> bool {
        *self == ICCEN_A::ICCEN_0
    }
    #[doc = "Checks if the value of the field is `ICCEN_1`"]
    #[inline(always)]
    pub fn is_iccen_1(&self) -> bool {
        *self == ICCEN_A::ICCEN_1
    }
}
#[doc = "Write proxy for field `ICCEN`"]
pub struct ICCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ICC module disabled"]
    #[inline(always)]
    pub fn iccen_0(self) -> &'a mut W {
        self.variant(ICCEN_A::ICCEN_0)
    }
    #[doc = "ICC module enabled"]
    #[inline(always)]
    pub fn iccen_1(self) -> &'a mut W {
        self.variant(ICCEN_A::ICCEN_1)
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
    #[doc = "Bits 0:1 - Current Interrupt Compare Mask of virtual stack specifies the current ICM at the top of virtual stack If ICM\\[1:0\\] is less than the priority level (ILSRx\\[1:0\\]) of the new interrupt, the corresponding source is sent to the CPU. Note that the ICMC is the element stack that the stack pointer is pointing to."]
    #[inline(always)]
    pub fn icmc(&self) -> ICMC_R {
        ICMC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped."]
    #[inline(always)]
    pub fn vsfflg(&self) -> VSFFLG_R {
        VSFFLG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped."]
    #[inline(always)]
    pub fn vseflg(&self) -> VSEFLG_R {
        VSEFLG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ICC enable"]
    #[inline(always)]
    pub fn iccen(&self) -> ICCEN_R {
        ICCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ICC enable"]
    #[inline(always)]
    pub fn iccen(&mut self) -> ICCEN_W {
        ICCEN_W { w: self }
    }
}
