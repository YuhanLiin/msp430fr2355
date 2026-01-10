#[doc = "Register `CSCTL3` reader"]
pub type R = crate::R<Csctl3Spec>;
#[doc = "Register `CSCTL3` writer"]
pub type W = crate::W<Csctl3Spec>;
#[doc = "FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fllrefdiv {
    #[doc = "0: fFLLREFCLK / 1"]
    _1 = 0,
    #[doc = "1: fFLLREFCLK / 32"]
    _32 = 1,
    #[doc = "2: fFLLREFCLK / 64"]
    _64 = 2,
    #[doc = "3: fFLLREFCLK / 128"]
    _128 = 3,
    #[doc = "4: fFLLREFCLK / 256"]
    _256 = 4,
    #[doc = "5: fFLLREFCLK / 512"]
    _512 = 5,
    #[doc = "6: fFLLREFCLK / 640 (only available in 24MHz clock system)"]
    Fllrefdiv6 = 6,
    #[doc = "7: fFLLREFCLK / 768(only available in 24MHz clock system)"]
    Fllrefdiv7 = 7,
}
impl From<Fllrefdiv> for u8 {
    #[inline(always)]
    fn from(variant: Fllrefdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fllrefdiv {
    type Ux = u8;
}
impl crate::IsEnum for Fllrefdiv {}
#[doc = "Field `FLLREFDIV` reader - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
pub type FllrefdivR = crate::FieldReader<Fllrefdiv>;
impl FllrefdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fllrefdiv {
        match self.bits {
            0 => Fllrefdiv::_1,
            1 => Fllrefdiv::_32,
            2 => Fllrefdiv::_64,
            3 => Fllrefdiv::_128,
            4 => Fllrefdiv::_256,
            5 => Fllrefdiv::_512,
            6 => Fllrefdiv::Fllrefdiv6,
            7 => Fllrefdiv::Fllrefdiv7,
            _ => unreachable!(),
        }
    }
    #[doc = "fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fllrefdiv::_1
    }
    #[doc = "fFLLREFCLK / 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Fllrefdiv::_32
    }
    #[doc = "fFLLREFCLK / 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Fllrefdiv::_64
    }
    #[doc = "fFLLREFCLK / 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Fllrefdiv::_128
    }
    #[doc = "fFLLREFCLK / 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Fllrefdiv::_256
    }
    #[doc = "fFLLREFCLK / 512"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Fllrefdiv::_512
    }
    #[doc = "fFLLREFCLK / 640 (only available in 24MHz clock system)"]
    #[inline(always)]
    pub fn is_fllrefdiv_6(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv6
    }
    #[doc = "fFLLREFCLK / 768(only available in 24MHz clock system)"]
    #[inline(always)]
    pub fn is_fllrefdiv_7(&self) -> bool {
        *self == Fllrefdiv::Fllrefdiv7
    }
}
#[doc = "Field `FLLREFDIV` writer - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
pub type FllrefdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fllrefdiv, crate::Safe>;
impl<'a, REG> FllrefdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::_1)
    }
    #[doc = "fFLLREFCLK / 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::_32)
    }
    #[doc = "fFLLREFCLK / 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::_64)
    }
    #[doc = "fFLLREFCLK / 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::_128)
    }
    #[doc = "fFLLREFCLK / 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::_256)
    }
    #[doc = "fFLLREFCLK / 512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::_512)
    }
    #[doc = "fFLLREFCLK / 640 (only available in 24MHz clock system)"]
    #[inline(always)]
    pub fn fllrefdiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv6)
    }
    #[doc = "fFLLREFCLK / 768(only available in 24MHz clock system)"]
    #[inline(always)]
    pub fn fllrefdiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Fllrefdiv::Fllrefdiv7)
    }
}
#[doc = "FLL reference select. These bits select the FLL reference clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selref {
    #[doc = "0: XT1CLK"]
    Xt1clk = 0,
    #[doc = "1: REFOCLK"]
    Refoclk = 1,
    #[doc = "2: served for future use"]
    Selref2 = 2,
    #[doc = "3: served for future use"]
    Selref3 = 3,
}
impl From<Selref> for u8 {
    #[inline(always)]
    fn from(variant: Selref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selref {
    type Ux = u8;
}
impl crate::IsEnum for Selref {}
#[doc = "Field `SELREF` reader - FLL reference select. These bits select the FLL reference clock source."]
pub type SelrefR = crate::FieldReader<Selref>;
impl SelrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selref {
        match self.bits {
            0 => Selref::Xt1clk,
            1 => Selref::Refoclk,
            2 => Selref::Selref2,
            3 => Selref::Selref3,
            _ => unreachable!(),
        }
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Selref::Xt1clk
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == Selref::Refoclk
    }
    #[doc = "served for future use"]
    #[inline(always)]
    pub fn is_selref_2(&self) -> bool {
        *self == Selref::Selref2
    }
    #[doc = "served for future use"]
    #[inline(always)]
    pub fn is_selref_3(&self) -> bool {
        *self == Selref::Selref3
    }
}
#[doc = "Field `SELREF` writer - FLL reference select. These bits select the FLL reference clock source."]
pub type SelrefW<'a, REG> = crate::FieldWriter<'a, REG, 2, Selref, crate::Safe>;
impl<'a, REG> SelrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Xt1clk)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Refoclk)
    }
    #[doc = "served for future use"]
    #[inline(always)]
    pub fn selref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Selref2)
    }
    #[doc = "served for future use"]
    #[inline(always)]
    pub fn selref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Selref::Selref3)
    }
}
#[doc = "REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refolp {
    #[doc = "0: REFO Low Power Disabled (High Power Mode)"]
    Refolp0 = 0,
    #[doc = "1: REFO Low Power Enabled"]
    Refolp1 = 1,
}
impl From<Refolp> for bool {
    #[inline(always)]
    fn from(variant: Refolp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOLP` reader - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
pub type RefolpR = crate::BitReader<Refolp>;
impl RefolpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refolp {
        match self.bits {
            false => Refolp::Refolp0,
            true => Refolp::Refolp1,
        }
    }
    #[doc = "REFO Low Power Disabled (High Power Mode)"]
    #[inline(always)]
    pub fn is_refolp_0(&self) -> bool {
        *self == Refolp::Refolp0
    }
    #[doc = "REFO Low Power Enabled"]
    #[inline(always)]
    pub fn is_refolp_1(&self) -> bool {
        *self == Refolp::Refolp1
    }
}
#[doc = "Field `REFOLP` writer - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
pub type RefolpW<'a, REG> = crate::BitWriter<'a, REG, Refolp>;
impl<'a, REG> RefolpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "REFO Low Power Disabled (High Power Mode)"]
    #[inline(always)]
    pub fn refolp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Refolp::Refolp0)
    }
    #[doc = "REFO Low Power Enabled"]
    #[inline(always)]
    pub fn refolp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Refolp::Refolp1)
    }
}
impl R {
    #[doc = "Bits 0:2 - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FllrefdivR {
        FllrefdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - FLL reference select. These bits select the FLL reference clock source."]
    #[inline(always)]
    pub fn selref(&self) -> SelrefR {
        SelrefR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
    #[inline(always)]
    pub fn refolp(&self) -> RefolpR {
        RefolpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FLL reference divider. These bits define the divide factor for f(FLLREFCLK). If XT1 supports high frequency input higher than 32 kHz, the divided frequency is used as the FLL reference frequency. If XT1 only supports 32-kHz clock, FLLREFDIV is always read and written as zero, 000b = fFLLREFCLK / 1"]
    #[inline(always)]
    pub fn fllrefdiv(&mut self) -> FllrefdivW<'_, Csctl3Spec> {
        FllrefdivW::new(self, 0)
    }
    #[doc = "Bits 4:5 - FLL reference select. These bits select the FLL reference clock source."]
    #[inline(always)]
    pub fn selref(&mut self) -> SelrefW<'_, Csctl3Spec> {
        SelrefW::new(self, 4)
    }
    #[doc = "Bit 7 - REFO Low Power Enable. This bit turns on REFO low-power mode. During switch, the low-power mode will be invalid until REFOREADY is set."]
    #[inline(always)]
    pub fn refolp(&mut self) -> RefolpW<'_, Csctl3Spec> {
        RefolpW::new(self, 7)
    }
}
#[doc = "Clock System Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl3Spec;
impl crate::RegisterSpec for Csctl3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl3::R`](R) reader structure"]
impl crate::Readable for Csctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl3::W`](W) writer structure"]
impl crate::Writable for Csctl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL3 to value 0"]
impl crate::Resettable for Csctl3Spec {}
