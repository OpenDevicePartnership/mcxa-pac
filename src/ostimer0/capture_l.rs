#[doc = "Register `CAPTURE_L` reader"]
pub type R = crate::R<CaptureLSpec>;
#[doc = "Field `CAPTURE_VALUE` reader - EVTimer Capture Value"]
pub type CaptureValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - EVTimer Capture Value"]
    #[inline(always)]
    pub fn capture_value(&self) -> CaptureValueR {
        CaptureValueR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPTURE_L")
            .field("capture_value", &self.capture_value())
            .finish()
    }
}
#[doc = "Local Capture Low for CPU\n\nYou can [`read`](crate::Reg::read) this register and get [`capture_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaptureLSpec;
impl crate::RegisterSpec for CaptureLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capture_l::R`](R) reader structure"]
impl crate::Readable for CaptureLSpec {}
#[doc = "`reset()` method sets CAPTURE_L to value 0"]
impl crate::Resettable for CaptureLSpec {}
