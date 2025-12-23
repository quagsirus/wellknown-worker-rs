use core::panic;
use std::fs::{File, copy};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};

use pgp::composed::{Deserializable, SignedPublicKey};
use sha1::{Digest, Sha1};

fn main() {
    println!("cargo:rerun-if-changed=pgp_keys");

    let out_dir = &env::var("OUT_DIR").unwrap();
    let out_path = Path::new(out_dir);
    let mut file: BufWriter<File> =
        BufWriter::new(File::create(&out_path.join("codegen.rs")).unwrap());

    let mut map_builder = phf_codegen::Map::new();

    let key_out_path = out_path.join("pgp_keys");
    fs::create_dir_all(&key_out_path).unwrap();

    let mut map_keys = vec![];

    for search_result in fs::read_dir("pgp_keys").unwrap() {
        let dir_entry = search_result.unwrap();
        let path = dir_entry.path();

        if path.is_file() {
            if path.file_stem().is_some() {
                let public_key = SignedPublicKey::from_file(&path).unwrap();
                let new_key_path = key_out_path.join(path.file_name().unwrap());

                for user in public_key.details.users {
                    let id = user.id.as_str().unwrap();
                    let start = id.find('<').unwrap() + 1;
                    let end = id[start..].find('@').unwrap() + start;
                    let email_localpart = id.get(start..end).unwrap().to_ascii_lowercase();
                    let localpart_sha1 = Sha1::digest(email_localpart.as_bytes());
                    let localpart_zbase32 = zbase32::encode_full_bytes(&localpart_sha1[..]);
                    if map_keys.contains(&localpart_zbase32) {
                        panic!("Duplicate localpart detected: {}", email_localpart);
                    }
                    map_keys.push(localpart_zbase32.clone());
                    map_builder.entry(
                        localpart_zbase32,
                        format!("include_bytes!(\"{}\")", new_key_path.to_str().unwrap()),
                    );
                }
                copy(path, new_key_path).unwrap();
            }
        }
    }

    write!(
        &mut file,
        "static WEB_KEY_DIRECTORY_MAP: phf::Map<&'static str, &'static [u8]> = {}",
        map_builder.build()
    )
    .unwrap();
    write!(&mut file, ";\n").unwrap();
}
