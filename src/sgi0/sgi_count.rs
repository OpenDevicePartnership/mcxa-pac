#[doc = "Register `sgi_count` reader"]
pub type R = crate::R<SgiCountSpec>;
#[doc = "Register `sgi_count` writer"]
pub type W = crate::W<SgiCountSpec>;
#[doc = "Field `count` reader - Calculation counter, incremented with"]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `count` writer - Calculation counter, incremented with"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `count_rsvd` reader - reserved"]
pub type CountRsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Calculation counter, incremented with"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn count_rsvd(&self) -> CountRsvdR {
        CountRsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sgi_count")
            .field("count", &self.count())
            .field("count_rsvd", &self.count_rsvd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Calculation counter, incremented with"]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, SgiCountSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Calculation counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sgi_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgi_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgiCountSpec;
impl crate::RegisterSpec for SgiCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgi_count::R`](R) reader structure"]
impl crate::Readable for SgiCountSpec {}
#[doc = "`write(|w| ..)` method takes [`sgi_count::W`](W) writer structure"]
impl crate::Writable for SgiCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sgi_count to value 0"]
impl crate::Resettable for SgiCountSpec {}
