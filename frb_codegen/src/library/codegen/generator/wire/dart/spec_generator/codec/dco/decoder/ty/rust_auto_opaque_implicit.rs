use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::rust_opaque::generalized_rust_opaque_generate_impl_decode_body;

impl WireDartCodecDcoGeneratorDecoderTrait for RustAutoOpaqueImplicitWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        generalized_rust_opaque_generate_impl_decode_body(self.mir.clone().into(), self.context)
    }
}
