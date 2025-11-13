#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `IRQNUM` reader - Interrupt Number"]
pub type IrqnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Interrupt Number"]
    #[inline(always)]
    pub fn irqnum(&self) -> IrqnumR {
        IrqnumR::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM").field("irqnum", &self.irqnum()).finish()
    }
}
#[doc = "Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0x01"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0x01;
}
