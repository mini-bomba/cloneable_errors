/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/
#![cfg(feature = "bincode")]

use bincode::{decode_from_slice, encode_into_slice};
use cloneable_errors::SharedString;

#[test]
fn shared_string_decoding_static() {
    let config = bincode::config::standard();
    // static shared string
    let string = SharedString::Static("helo");
    let mut buf = [0u8; 5];

    encode_into_slice(string, &mut buf, config).unwrap();
    let decoded: SharedString = decode_from_slice(&buf, config).unwrap().0;

    #[allow(clippy::match_wildcard_for_single_variants)]
    match decoded {
        SharedString::Arc(s) => assert_eq!(&*s, "helo"),
        x => panic!("Expected SharedString::Arc(\"helo\"), got {x:?}")
    }
}

#[test]
fn shared_string_decoding_arc() {
    let config = bincode::config::standard();
    // static shared string
    let string = SharedString::Arc("helo".into());
    let mut buf = [0u8; 5];

    encode_into_slice(string, &mut buf, config).unwrap();
    let decoded: SharedString = decode_from_slice(&buf, config).unwrap().0;

    #[allow(clippy::match_wildcard_for_single_variants)]
    match decoded {
        SharedString::Arc(s) => assert_eq!(&*s, "helo"),
        x => panic!("Expected SharedString::Arc(\"helo\"), got {x:?}")
    }
}
