#[doc = "Register `ICCSC` reader"]
pub type R = crate::R<IccscSpec>;
#[doc = "Register `ICCSC` writer"]
pub type W = crate::W<IccscSpec>;
#[doc = "Field `ICMC` reader - Current Interrupt Compare Mask of virtual stack specifies the current ICM at the top of virtual stack If ICM\\[1:0\\] is less than the priority level (ILSRx\\[1:0\\]) of the new interrupt, the corresponding source is sent to the CPU. Note that the ICMC is the element stack that the stack pointer is pointing to."]
pub type IcmcR = crate::FieldReader;
#[doc = "Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsfflg {
    #[doc = "0: ICCMVS register is not full"]
    Vsfflg0 = 0,
    #[doc = "1: ICCMVS register is full"]
    Vsfflg1 = 1,
}
impl From<Vsfflg> for bool {
    #[inline(always)]
    fn from(variant: Vsfflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSFFLG` reader - Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped."]
pub type VsfflgR = crate::BitReader<Vsfflg>;
impl VsfflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsfflg {
        match self.bits {
            false => Vsfflg::Vsfflg0,
            true => Vsfflg::Vsfflg1,
        }
    }
    #[doc = "ICCMVS register is not full"]
    #[inline(always)]
    pub fn is_vsfflg_0(&self) -> bool {
        *self == Vsfflg::Vsfflg0
    }
    #[doc = "ICCMVS register is full"]
    #[inline(always)]
    pub fn is_vsfflg_1(&self) -> bool {
        *self == Vsfflg::Vsfflg1
    }
}
#[doc = "Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vseflg {
    #[doc = "0: Stack has valid data"]
    Vseflg0 = 0,
    #[doc = "1: Stack has no valid data"]
    Vseflg1 = 1,
}
impl From<Vseflg> for bool {
    #[inline(always)]
    fn from(variant: Vseflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSEFLG` reader - Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped."]
pub type VseflgR = crate::BitReader<Vseflg>;
impl VseflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vseflg {
        match self.bits {
            false => Vseflg::Vseflg0,
            true => Vseflg::Vseflg1,
        }
    }
    #[doc = "Stack has valid data"]
    #[inline(always)]
    pub fn is_vseflg_0(&self) -> bool {
        *self == Vseflg::Vseflg0
    }
    #[doc = "Stack has no valid data"]
    #[inline(always)]
    pub fn is_vseflg_1(&self) -> bool {
        *self == Vseflg::Vseflg1
    }
}
#[doc = "ICC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iccen {
    #[doc = "0: ICC module disabled"]
    Iccen0 = 0,
    #[doc = "1: ICC module enabled"]
    Iccen1 = 1,
}
impl From<Iccen> for bool {
    #[inline(always)]
    fn from(variant: Iccen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICCEN` reader - ICC enable"]
pub type IccenR = crate::BitReader<Iccen>;
impl IccenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iccen {
        match self.bits {
            false => Iccen::Iccen0,
            true => Iccen::Iccen1,
        }
    }
    #[doc = "ICC module disabled"]
    #[inline(always)]
    pub fn is_iccen_0(&self) -> bool {
        *self == Iccen::Iccen0
    }
    #[doc = "ICC module enabled"]
    #[inline(always)]
    pub fn is_iccen_1(&self) -> bool {
        *self == Iccen::Iccen1
    }
}
#[doc = "Field `ICCEN` writer - ICC enable"]
pub type IccenW<'a, REG> = crate::BitWriter<'a, REG, Iccen>;
impl<'a, REG> IccenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICC module disabled"]
    #[inline(always)]
    pub fn iccen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Iccen::Iccen0)
    }
    #[doc = "ICC module enabled"]
    #[inline(always)]
    pub fn iccen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Iccen::Iccen1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Current Interrupt Compare Mask of virtual stack specifies the current ICM at the top of virtual stack If ICM\\[1:0\\] is less than the priority level (ILSRx\\[1:0\\]) of the new interrupt, the corresponding source is sent to the CPU. Note that the ICMC is the element stack that the stack pointer is pointing to."]
    #[inline(always)]
    pub fn icmc(&self) -> IcmcR {
        IcmcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped."]
    #[inline(always)]
    pub fn vsfflg(&self) -> VsfflgR {
        VsfflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped."]
    #[inline(always)]
    pub fn vseflg(&self) -> VseflgR {
        VseflgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ICC enable"]
    #[inline(always)]
    pub fn iccen(&self) -> IccenR {
        IccenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ICC enable"]
    #[inline(always)]
    pub fn iccen(&mut self) -> IccenW<'_, IccscSpec> {
        IccenW::new(self, 7)
    }
}
#[doc = "ICCSC\n\nYou can [`read`](crate::Reg::read) this register and get [`iccsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IccscSpec;
impl crate::RegisterSpec for IccscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccsc::R`](R) reader structure"]
impl crate::Readable for IccscSpec {}
#[doc = "`write(|w| ..)` method takes [`iccsc::W`](W) writer structure"]
impl crate::Writable for IccscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCSC to value 0"]
impl crate::Resettable for IccscSpec {}
