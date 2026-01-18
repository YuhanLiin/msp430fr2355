#[doc = "Register `CSCTL4` reader"]
pub type R = crate::R<Csctl4Spec>;
#[doc = "Register `CSCTL4` writer"]
pub type W = crate::W<Csctl4Spec>;
#[doc = "Selects the MCLK and SMCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selms {
    #[doc = "0: DCOCLKDIV"]
    Dcoclkdiv = 0,
    #[doc = "1: REFOCLK"]
    Refoclk = 1,
    #[doc = "2: XT1CLK"]
    Xt1clk = 2,
    #[doc = "3: VLOCLK"]
    Vloclk = 3,
    #[doc = "4: Reserved for future use"]
    Selms4 = 4,
    #[doc = "5: Reserved for future use"]
    Selms5 = 5,
    #[doc = "6: Reserved for future use"]
    Selms6 = 6,
    #[doc = "7: Reserved for future use"]
    Selms7 = 7,
}
impl From<Selms> for u8 {
    #[inline(always)]
    fn from(variant: Selms) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selms {
    type Ux = u8;
}
impl crate::IsEnum for Selms {}
#[doc = "Field `SELMS` reader - Selects the MCLK and SMCLK source"]
pub type SelmsR = crate::FieldReader<Selms>;
impl SelmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selms {
        match self.bits {
            0 => Selms::Dcoclkdiv,
            1 => Selms::Refoclk,
            2 => Selms::Xt1clk,
            3 => Selms::Vloclk,
            4 => Selms::Selms4,
            5 => Selms::Selms5,
            6 => Selms::Selms6,
            7 => Selms::Selms7,
            _ => unreachable!(),
        }
    }
    #[doc = "DCOCLKDIV"]
    #[inline(always)]
    pub fn is_dcoclkdiv(&self) -> bool {
        *self == Selms::Dcoclkdiv
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == Selms::Refoclk
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Selms::Xt1clk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Selms::Vloclk
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_selms_4(&self) -> bool {
        *self == Selms::Selms4
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_selms_5(&self) -> bool {
        *self == Selms::Selms5
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_selms_6(&self) -> bool {
        *self == Selms::Selms6
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_selms_7(&self) -> bool {
        *self == Selms::Selms7
    }
}
#[doc = "Field `SELMS` writer - Selects the MCLK and SMCLK source"]
pub type SelmsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selms, crate::Safe>;
impl<'a, REG> SelmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCOCLKDIV"]
    #[inline(always)]
    pub fn dcoclkdiv(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Dcoclkdiv)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Refoclk)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Xt1clk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Vloclk)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_4(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms4)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_5(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms5)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_6(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms6)
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn selms_7(self) -> &'a mut crate::W<REG> {
        self.variant(Selms::Selms7)
    }
}
#[doc = "Selects the ACLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sela {
    #[doc = "0: XT1CLK with divider (must be no more than 40 kHz)"]
    Xt1clk = 0,
    #[doc = "1: REFO (internal 32-kHz clock source)"]
    Refoclk = 1,
    #[doc = "2: VLO (internal 10-kHz clock source)"]
    Vloclk = 2,
}
impl From<Sela> for u8 {
    #[inline(always)]
    fn from(variant: Sela) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sela {
    type Ux = u8;
}
impl crate::IsEnum for Sela {}
#[doc = "Field `SELA` reader - Selects the ACLK source"]
pub type SelaR = crate::FieldReader<Sela>;
impl SelaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sela {
        match self.bits {
            0 => Sela::Xt1clk,
            1 => Sela::Refoclk,
            2 => Sela::Vloclk,
            _ => unreachable!(),
        }
    }
    #[doc = "XT1CLK with divider (must be no more than 40 kHz)"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Sela::Xt1clk
    }
    #[doc = "REFO (internal 32-kHz clock source)"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == Sela::Refoclk
    }
    #[doc = "VLO (internal 10-kHz clock source)"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Sela::Vloclk
    }
}
#[doc = "Field `SELA` writer - Selects the ACLK source"]
pub type SelaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sela>;
impl<'a, REG> SelaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "XT1CLK with divider (must be no more than 40 kHz)"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Xt1clk)
    }
    #[doc = "REFO (internal 32-kHz clock source)"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Refoclk)
    }
    #[doc = "VLO (internal 10-kHz clock source)"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sela::Vloclk)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects the MCLK and SMCLK source"]
    #[inline(always)]
    pub fn selms(&self) -> SelmsR {
        SelmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&self) -> SelaR {
        SelaR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the MCLK and SMCLK source"]
    #[inline(always)]
    pub fn selms(&mut self) -> SelmsW<'_, Csctl4Spec> {
        SelmsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Selects the ACLK source"]
    #[inline(always)]
    pub fn sela(&mut self) -> SelaW<'_, Csctl4Spec> {
        SelaW::new(self, 8)
    }
}
#[doc = "Clock System Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl4Spec;
impl crate::RegisterSpec for Csctl4Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl4::R`](R) reader structure"]
impl crate::Readable for Csctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl4::W`](W) writer structure"]
impl crate::Writable for Csctl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for Csctl4Spec {}
