#[doc = "Register `SYSUNIV` reader"]
pub type R = crate::R<SysunivSpec>;
#[doc = "Register `SYSUNIV` writer"]
pub type W = crate::W<SysunivSpec>;
#[doc = "User NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sysuniv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: NMIIFG NMI pin"]
    Nmiifg = 2,
    #[doc = "4: OFIFG oscillator fault"]
    Ofifg = 4,
}
impl From<Sysuniv> for u16 {
    #[inline(always)]
    fn from(variant: Sysuniv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysuniv {
    type Ux = u16;
}
impl crate::IsEnum for Sysuniv {}
#[doc = "Field `SYSUNIV` reader - User NMI vector"]
pub type SysunivR = crate::FieldReader<Sysuniv>;
impl SysunivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysuniv> {
        match self.bits {
            0 => Some(Sysuniv::None),
            2 => Some(Sysuniv::Nmiifg),
            4 => Some(Sysuniv::Ofifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sysuniv::None
    }
    #[doc = "NMIIFG NMI pin"]
    #[inline(always)]
    pub fn is_nmiifg(&self) -> bool {
        *self == Sysuniv::Nmiifg
    }
    #[doc = "OFIFG oscillator fault"]
    #[inline(always)]
    pub fn is_ofifg(&self) -> bool {
        *self == Sysuniv::Ofifg
    }
}
impl R {
    #[doc = "Bits 0:15 - User NMI vector"]
    #[inline(always)]
    pub fn sysuniv(&self) -> SysunivR {
        SysunivR::new(self.bits)
    }
}
impl W {}
#[doc = "User NMI Vector Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysuniv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysuniv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysunivSpec;
impl crate::RegisterSpec for SysunivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysuniv::R`](R) reader structure"]
impl crate::Readable for SysunivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysuniv::W`](W) writer structure"]
impl crate::Writable for SysunivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSUNIV to value 0"]
impl crate::Resettable for SysunivSpec {}
