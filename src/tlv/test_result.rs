#[doc = "Register `TEST_RESULT` reader"]
pub type R = crate::R<TestResultSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Test result\n\nYou can [`read`](crate::Reg::read) this register and get [`test_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestResultSpec;
impl crate::RegisterSpec for TestResultSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`test_result::R`](R) reader structure"]
impl crate::Readable for TestResultSpec {}
#[doc = "`reset()` method sets TEST_RESULT to value 0"]
impl crate::Resettable for TestResultSpec {}
