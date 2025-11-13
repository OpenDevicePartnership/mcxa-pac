#[doc = "Register `MWMSG_DDR_CONTROL` writer"]
pub type W = crate::W<MwmsgDdrControlSpec>;
#[doc = "Field `ADDRCMD` writer - Address Command"]
pub type AddrcmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MwmsgDdrControlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Address Command"]
    #[inline(always)]
    pub fn addrcmd(&mut self) -> AddrcmdW<'_, MwmsgDdrControlSpec> {
        AddrcmdW::new(self, 0)
    }
}
#[doc = "Controller Write Message in DDR mode: First Control Word\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr_control::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwmsgDdrControlSpec;
impl crate::RegisterSpec for MwmsgDdrControlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwmsg_ddr_control::W`](W) writer structure"]
impl crate::Writable for MwmsgDdrControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MWMSG_DDR_CONTROL to value 0"]
impl crate::Resettable for MwmsgDdrControlSpec {}
