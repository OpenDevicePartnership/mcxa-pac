#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `FILTERS` reader - Filter Number"]
pub type FiltersR = crate::FieldReader;
#[doc = "Field `DMAS` reader - DMA Number"]
pub type DmasR = crate::FieldReader;
#[doc = "Field `MODULES` reader - Module Number"]
pub type ModulesR = crate::FieldReader;
#[doc = "Field `PINS` reader - Pin Number"]
pub type PinsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Filter Number"]
    #[inline(always)]
    pub fn filters(&self) -> FiltersR {
        FiltersR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA Number"]
    #[inline(always)]
    pub fn dmas(&self) -> DmasR {
        DmasR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Module Number"]
    #[inline(always)]
    pub fn modules(&self) -> ModulesR {
        ModulesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Pin Number"]
    #[inline(always)]
    pub fn pins(&self) -> PinsR {
        PinsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("filters", &self.filters())
            .field("dmas", &self.dmas())
            .field("modules", &self.modules())
            .field("pins", &self.pins())
            .finish()
    }
}
#[doc = "Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0x2020_2002"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0x2020_2002;
}
