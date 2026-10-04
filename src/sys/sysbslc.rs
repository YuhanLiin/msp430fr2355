#[doc = "Register `SYSBSLC` reader"]
pub type R = crate::R<SysbslcSpec>;
#[doc = "Register `SYSBSLC` writer"]
pub type W = crate::W<SysbslcSpec>;
#[doc = "RAM assigned to BSL\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysbslr {
    #[doc = "0: No RAM assigned to BSL area"]
    Noram = 0,
    #[doc = "1: Lowest 16 bytes of RAM assigned to BSL"]
    Ram = 1,
}
impl From<Sysbslr> for bool {
    #[inline(always)]
    fn from(variant: Sysbslr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBSLR` reader - RAM assigned to BSL"]
pub type SysbslrR = crate::BitReader<Sysbslr>;
impl SysbslrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysbslr {
        match self.bits {
            false => Sysbslr::Noram,
            true => Sysbslr::Ram,
        }
    }
    #[doc = "No RAM assigned to BSL area"]
    #[inline(always)]
    pub fn is_noram(&self) -> bool {
        *self == Sysbslr::Noram
    }
    #[doc = "Lowest 16 bytes of RAM assigned to BSL"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == Sysbslr::Ram
    }
}
#[doc = "Field `SYSBSLR` writer - RAM assigned to BSL"]
pub type SysbslrW<'a, REG> = crate::BitWriter<'a, REG, Sysbslr>;
impl<'a, REG> SysbslrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RAM assigned to BSL area"]
    #[inline(always)]
    pub fn noram(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbslr::Noram)
    }
    #[doc = "Lowest 16 bytes of RAM assigned to BSL"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbslr::Ram)
    }
}
#[doc = "Bootstrap loader memory disable for the size covered in SYSBSLSIZE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysbsloff {
    #[doc = "0: BSL memory is addressed when this area is read."]
    On = 0,
    #[doc = "1: BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    Off = 1,
}
impl From<Sysbsloff> for bool {
    #[inline(always)]
    fn from(variant: Sysbsloff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBSLOFF` reader - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
pub type SysbsloffR = crate::BitReader<Sysbsloff>;
impl SysbsloffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysbsloff {
        match self.bits {
            false => Sysbsloff::On,
            true => Sysbsloff::Off,
        }
    }
    #[doc = "BSL memory is addressed when this area is read."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Sysbsloff::On
    }
    #[doc = "BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Sysbsloff::Off
    }
}
#[doc = "Field `SYSBSLOFF` writer - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
pub type SysbsloffW<'a, REG> = crate::BitWriter<'a, REG, Sysbsloff>;
impl<'a, REG> SysbsloffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BSL memory is addressed when this area is read."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbsloff::On)
    }
    #[doc = "BSL memory behaves like vacant memory. Reads cause 3FFFh to be read. Fetches cause JMP $ to be executed."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbsloff::Off)
    }
}
#[doc = "Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysbslpe {
    #[doc = "0: Area not protected. Read, program, and erase of BSL memory is possible."]
    Notprot = 0,
    #[doc = "1: Area protected"]
    Prot = 1,
}
impl From<Sysbslpe> for bool {
    #[inline(always)]
    fn from(variant: Sysbslpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSBSLPE` reader - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
pub type SysbslpeR = crate::BitReader<Sysbslpe>;
impl SysbslpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysbslpe {
        match self.bits {
            false => Sysbslpe::Notprot,
            true => Sysbslpe::Prot,
        }
    }
    #[doc = "Area not protected. Read, program, and erase of BSL memory is possible."]
    #[inline(always)]
    pub fn is_notprot(&self) -> bool {
        *self == Sysbslpe::Notprot
    }
    #[doc = "Area protected"]
    #[inline(always)]
    pub fn is_prot(&self) -> bool {
        *self == Sysbslpe::Prot
    }
}
#[doc = "Field `SYSBSLPE` writer - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
pub type SysbslpeW<'a, REG> = crate::BitWriter<'a, REG, Sysbslpe>;
impl<'a, REG> SysbslpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Area not protected. Read, program, and erase of BSL memory is possible."]
    #[inline(always)]
    pub fn notprot(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbslpe::Notprot)
    }
    #[doc = "Area protected"]
    #[inline(always)]
    pub fn prot(self) -> &'a mut crate::W<REG> {
        self.variant(Sysbslpe::Prot)
    }
}
impl R {
    #[doc = "Bit 2 - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&self) -> SysbslrR {
        SysbslrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 14 - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
    #[inline(always)]
    pub fn sysbsloff(&self) -> SysbsloffR {
        SysbsloffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
    #[inline(always)]
    pub fn sysbslpe(&self) -> SysbslpeR {
        SysbslpeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RAM assigned to BSL"]
    #[inline(always)]
    pub fn sysbslr(&mut self) -> SysbslrW<'_, SysbslcSpec> {
        SysbslrW::new(self, 2)
    }
    #[doc = "Bit 14 - Bootstrap loader memory disable for the size covered in SYSBSLSIZE"]
    #[inline(always)]
    pub fn sysbsloff(&mut self) -> SysbsloffW<'_, SysbslcSpec> {
        SysbsloffW::new(self, 14)
    }
    #[doc = "Bit 15 - Bootstrap loader memory protection enable for the size covered in SYSBSLSIZE. By default, this bit is cleared by hardware with a BOR event (as indicated above); however, the boot code that checks for an available BSL may set this bit in software to protect the BSL. Because devices normally come with a TI BSL preprogrammed and protected, the boot code sets this bit."]
    #[inline(always)]
    pub fn sysbslpe(&mut self) -> SysbslpeW<'_, SysbslcSpec> {
        SysbslpeW::new(self, 15)
    }
}
#[doc = "Bootstrap Loader Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysbslc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysbslc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysbslcSpec;
impl crate::RegisterSpec for SysbslcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysbslc::R`](R) reader structure"]
impl crate::Readable for SysbslcSpec {}
#[doc = "`write(|w| ..)` method takes [`sysbslc::W`](W) writer structure"]
impl crate::Writable for SysbslcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSBSLC to value 0"]
impl crate::Resettable for SysbslcSpec {}
