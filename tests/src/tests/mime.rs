use spore_utils::MIME;

#[test]
fn test_mime_basic() {
    assert!(MIME::str_parse("image/png").is_ok());
    assert!(MIME::str_parse("image/png;immortal=true").is_ok());
    assert!(MIME::str_parse("image/").is_err());
    assert!(MIME::str_parse("image/;").is_err());
    assert!(MIME::str_parse("/;").is_err());
    assert!(MIME::str_parse(";").is_err());
    assert!(MIME::str_parse("").is_err());

    let content_type = "image/png;immortal=true;mutant[]=c219351b150b900e50a7039f1e448b844110927e5fd9bd30425806cb8ddff1fd,9c87faf08de5c15c727d5350399115431bf4f0226fbc4abd400e63492faac3d2";
    let mime = MIME::str_parse(content_type)
        .map_err(|err| format!("mutant str_parse: {}", err as u8))
        .unwrap();
    let expected_value = b"c219351b150b900e50a7039f1e448b844110927e5fd9bd30425806cb8ddff1fd,9c87faf08de5c15c727d5350399115431bf4f0226fbc4abd400e63492faac3d2";
    let value_range = mime
        .get_param(content_type.as_bytes(), "mutant[]")
        .map_err(|err| format!("mutant get_param: {}", err as u8))
        .unwrap()
        .expect("empty range");
    assert!(content_type.as_bytes()[value_range] == expected_value[..]);

    let expected_value = b"true";
    let value_range = mime
        .get_param(content_type.as_bytes(), "immortal")
        .map_err(|err| format!("mutant get_param: {}", err as u8))
        .unwrap()
        .expect("empty range");
    assert!(content_type.as_bytes()[value_range] == expected_value[..]);
}
