#[doc = "Register `INFO_LENGTH` reader"]
pub type R = crate::R<InfoLengthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Info length\n\nYou can [`read`](crate::Reg::read) this register and get [`info_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoLengthSpec;
impl crate::RegisterSpec for InfoLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`info_length::R`](R) reader structure"]
impl crate::Readable for InfoLengthSpec {}
#[doc = "`reset()` method sets INFO_LENGTH to value 0"]
impl crate::Resettable for InfoLengthSpec {}
