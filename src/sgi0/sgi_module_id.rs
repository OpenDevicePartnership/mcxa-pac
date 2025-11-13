#[doc = "Register `sgi_module_id` reader"]
pub type R = crate::R<SgiModuleIdSpec>;
#[doc = "Field `placeholder` reader - Module ID"]
pub type PlaceholderR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Module ID"]
    #[inline(always)]
    pub fn placeholder(&self) -> PlaceholderR {
        PlaceholderR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sgi_module_id")
            .field("placeholder", &self.placeholder())
            .finish()
    }
}
#[doc = "Module ID\n\nYou can [`read`](crate::Reg::read) this register and get [`sgi_module_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgiModuleIdSpec;
impl crate::RegisterSpec for SgiModuleIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgi_module_id::R`](R) reader structure"]
impl crate::Readable for SgiModuleIdSpec {}
#[doc = "`reset()` method sets sgi_module_id to value 0"]
impl crate::Resettable for SgiModuleIdSpec {}
