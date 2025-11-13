#[doc = "Register `TCD_CITER_ELINKNO` reader"]
pub type R = crate::R<TcdCiterElinknoSpec>;
#[doc = "Register `TCD_CITER_ELINKNO` writer"]
pub type W = crate::W<TcdCiterElinknoSpec>;
#[doc = "Field `CITER` reader - Current Major Iteration Count"]
pub type CiterR = crate::FieldReader<u16>;
#[doc = "Field `CITER` writer - Current Major Iteration Count"]
pub type CiterW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Enable Link\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elink {
    #[doc = "0: Channel-to-channel linking disabled"]
    Disable = 0,
    #[doc = "1: Channel-to-channel linking enabled"]
    Enable = 1,
}
impl From<Elink> for bool {
    #[inline(always)]
    fn from(variant: Elink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELINK` reader - Enable Link"]
pub type ElinkR = crate::BitReader<Elink>;
impl ElinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Elink {
        match self.bits {
            false => Elink::Disable,
            true => Elink::Enable,
        }
    }
    #[doc = "Channel-to-channel linking disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Elink::Disable
    }
    #[doc = "Channel-to-channel linking enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Elink::Enable
    }
}
#[doc = "Field `ELINK` writer - Enable Link"]
pub type ElinkW<'a, REG> = crate::BitWriter<'a, REG, Elink>;
impl<'a, REG> ElinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel-to-channel linking disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Elink::Disable)
    }
    #[doc = "Channel-to-channel linking enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Elink::Enable)
    }
}
impl R {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&self) -> CiterR {
        CiterR::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - Enable Link"]
    #[inline(always)]
    pub fn elink(&self) -> ElinkR {
        ElinkR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCD_CITER_ELINKNO")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&mut self) -> CiterW<'_, TcdCiterElinknoSpec> {
        CiterW::new(self, 0)
    }
    #[doc = "Bit 15 - Enable Link"]
    #[inline(always)]
    pub fn elink(&mut self) -> ElinkW<'_, TcdCiterElinknoSpec> {
        ElinkW::new(self, 15)
    }
}
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`tcd_citer_elinkno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcd_citer_elinkno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdCiterElinknoSpec;
impl crate::RegisterSpec for TcdCiterElinknoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tcd_citer_elinkno::R`](R) reader structure"]
impl crate::Readable for TcdCiterElinknoSpec {}
#[doc = "`write(|w| ..)` method takes [`tcd_citer_elinkno::W`](W) writer structure"]
impl crate::Writable for TcdCiterElinknoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCD_CITER_ELINKNO to value 0"]
impl crate::Resettable for TcdCiterElinknoSpec {}
