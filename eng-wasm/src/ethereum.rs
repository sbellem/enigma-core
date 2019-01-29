use internal_std::Vec;
use internal_std::String;
use internal_std::SliceConcatExt;
use tiny_keccak::Keccak;
use ethabi::param_type::{Writer, ParamType};
use byteorder::{BigEndian, ByteOrder};


pub fn short_signature(name: &str, params: &[ParamType]) -> u32/*[u8; 4] */{
    let mut result = [0u8; 4];
    fill_signature(name, params, &mut result);
    BigEndian::read_u32(&result.as_ref()[0..4])
}

fn fill_signature(name: &str, params: &[ParamType], result: &mut [u8]) {
    let types = params.iter()
        .map(Writer::write)
        .collect::<Vec<String>>()
        .join(",");

    let data: Vec<u8> = From::from(eformat!("{}({})", name, types).as_str());

    let mut sponge = Keccak::new_keccak256();
    sponge.update(&data);
    sponge.finalize(result);
}