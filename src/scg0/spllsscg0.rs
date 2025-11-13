#[doc = "Register `SPLLSSCG0` reader"]
pub type R = crate::R<Spllsscg0Spec>;
#[doc = "Register `SPLLSSCG0` writer"]
pub type W = crate::W<Spllsscg0Spec>;
#[doc = "Field `SS_MDIV_LSB` reader - SS_MDIV\\[31:0\\]"]
pub type SsMdivLsbR = crate::FieldReader<u32>;
#[doc = "Field `SS_MDIV_LSB` writer - SS_MDIV\\[31:0\\]"]
pub type SsMdivLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SS_MDIV\\[31:0\\]"]
    #[inline(always)]
    pub fn ss_mdiv_lsb(&self) -> SsMdivLsbR {
        SsMdivLsbR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPLLSSCG0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - SS_MDIV\\[31:0\\]"]
    #[inline(always)]
    pub fn ss_mdiv_lsb(&mut self) -> SsMdivLsbW<'_, Spllsscg0Spec> {
        SsMdivLsbW::new(self, 0)
    }
}
#[doc = "SPLL Spread Spectrum Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spllsscg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spllsscg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spllsscg0Spec;
impl crate::RegisterSpec for Spllsscg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spllsscg0::R`](R) reader structure"]
impl crate::Readable for Spllsscg0Spec {}
#[doc = "`write(|w| ..)` method takes [`spllsscg0::W`](W) writer structure"]
impl crate::Writable for Spllsscg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPLLSSCG0 to value 0"]
impl crate::Resettable for Spllsscg0Spec {}
