#[doc = "Reader of register SYSBSLC"]
pub type R = crate::R<u16, super::SYSBSLC>;
#[doc = "Writer for register SYSBSLC"]
pub type W = crate::W<u16, super::SYSBSLC>;
#[doc = "Register SYSBSLC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSBSLC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RAM assigned to BSL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSBSLR_A {
    #[doc = "0: No RAM assigned to BSL area"]
    SYSBSLR_0 = 0,
    #[doc = "1: Lowest 16 bytes of RAM assigned to BSL"]
    SYSBSLR_1 = 1,
}
impl From<SYSBSLR_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSBSLR`"]
pub type SYSBSLR_R = crate::R<bool, SYSBSLR_A>;
impl SYSBSLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLR_A {
        match self.bits {
            false => SYSBSLR_A::SYSBSLR_0,
            true => SYSBSLR_A::SYSBSLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSBSLR_0`"]
    #[inline(always)]
    pub fn is_sysbslr_0(&self) -> bool {
        *self == SYSBSLR_A::SYSBSLR_0
    }
    #[doc = "Checks if the value of the field is `SYSBSLR_1`"]
    #[inline(always)]
    pub fn is_sysbslr_1(&self) -> bool {
        *self == SYSBSLR_A::SYSBSLR_1
    }
}
#[doc = "Write proxy for field `SYSBSLR`"]
pub struct SYSBSLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSBSLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No RAM assigned to BSL area"]
    #[inline(always)]
    pub fn sysbslr_0(self) -> &'a mut W {
        self.variant(SYSBSLR_A::SYSBSLR_0)
    }
    #[doc = "Lowest 16 bytes of RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr_1(self) -> &'a mut W {
        self.variant(SYSBSLR_A::SYSBSLR_1)
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
#[doc = "Bootstrap loader memory disable for the size covered in SYSBSLSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSBSLOFF_A {
    #[doc = "0: BSL memory is addressed when this area is read."]
    SYSBSLOFF_0 = 0,
    #[doc = "1: BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    SYSBSLOFF_1 = 1,
}
impl From<SYSBSLOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSBSLOFF`"]
pub type SYSBSLOFF_R = crate::R<bool, SYSBSLOFF_A>;
impl SYSBSLOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLOFF_A {
        match self.bits {
            false => SYSBSLOFF_A::SYSBSLOFF_0,
            true => SYSBSLOFF_A::SYSBSLOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSBSLOFF_0`"]
    #[inline(always)]
    pub fn is_sysbsloff_0(&self) -> bool {
        *self == SYSBSLOFF_A::SYSBSLOFF_0
    }
    #[doc = "Checks if the value of the field is `SYSBSLOFF_1`"]
    #[inline(always)]
    pub fn is_sysbsloff_1(&self) -> bool {
        *self == SYSBSLOFF_A::SYSBSLOFF_1
    }
}
#[doc = "Write proxy for field `SYSBSLOFF`"]
pub struct SYSBSLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSBSLOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BSL memory is addressed when this area is read."]
    #[inline(always)]
    pub fn sysbsloff_0(self) -> &'a mut W {
        self.variant(SYSBSLOFF_A::SYSBSLOFF_0)
    }
    #[doc = "BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    #[inline(always)]
    pub fn sysbsloff_1(self) -> &'a mut W {
        self.variant(SYSBSLOFF_A::SYSBSLOFF_1)
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
#[doc = "Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSBSLPE_A {
    #[doc = "0: Area not protected. Read, program, and erase of BSL memory is possible."]
    SYSBSLPE_0 = 0,
    #[doc = "1: Area protected"]
    SYSBSLPE_1 = 1,
}
impl From<SYSBSLPE_A> for bool {
    #[inline(always)]
    fn from(variant: SYSBSLPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSBSLPE`"]
pub type SYSBSLPE_R = crate::R<bool, SYSBSLPE_A>;
impl SYSBSLPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSBSLPE_A {
        match self.bits {
            false => SYSBSLPE_A::SYSBSLPE_0,
            true => SYSBSLPE_A::SYSBSLPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSBSLPE_0`"]
    #[inline(always)]
    pub fn is_sysbslpe_0(&self) -> bool {
        *self == SYSBSLPE_A::SYSBSLPE_0
    }
    #[doc = "Checks if the value of the field is `SYSBSLPE_1`"]
    #[inline(always)]
    pub fn is_sysbslpe_1(&self) -> bool {
        *self == SYSBSLPE_A::SYSBSLPE_1
    }
}
#[doc = "Write proxy for field `SYSBSLPE`"]
pub struct SYSBSLPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBSLPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSBSLPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Area not protected. Read, program, and erase of BSL memory is possible."]
    #[inline(always)]
    pub fn sysbslpe_0(self) -> &'a mut W {
        self.variant(SYSBSLPE_A::SYSBSLPE_0)
    }
    #[doc = "Area protected"]
    #[inline(always)]
    pub fn sysbslpe_1(self) -> &'a mut W {
        self.variant(SYSBSLPE_A::SYSBSLPE_1)
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
    #[doc = "Bit 2 - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SYSBSLR_R {
        SYSBSLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SYSBSLOFF_R {
        SYSBSLOFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SYSBSLPE_R {
        SYSBSLPE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&mut self) -> SYSBSLR_W {
        SYSBSLR_W { w: self }
    }
    #[doc = "Bit 14 - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
    #[inline(always)]
    pub fn sysbsloff(&mut self) -> SYSBSLOFF_W {
        SYSBSLOFF_W { w: self }
    }
    #[doc = "Bit 15 - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
    #[inline(always)]
    pub fn sysbslpe(&mut self) -> SYSBSLPE_W {
        SYSBSLPE_W { w: self }
    }
}
