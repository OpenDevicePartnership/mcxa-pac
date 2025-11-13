#[doc = "Register `MWMSG_DDR_DATA` writer"]
pub type W = crate::W<MwmsgDdrDataSpec>;
#[doc = "Field `DATA16B` writer - Data"]
pub type Data16bW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MwmsgDdrDataSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    pub fn data16b(&mut self) -> Data16bW<'_, MwmsgDdrDataSpec> {
        Data16bW::new(self, 0)
    }
}
#[doc = "Controller Write Message Data in DDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwmsgDdrDataSpec;
impl crate::RegisterSpec for MwmsgDdrDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwmsg_ddr_data::W`](W) writer structure"]
impl crate::Writable for MwmsgDdrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MWMSG_DDR_DATA to value 0"]
impl crate::Resettable for MwmsgDdrDataSpec {}
