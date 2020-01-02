extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").unwrap();

    let mut lmdb: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("mdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    let mut config = cc::Build::new();
    config.file(lmdb.join("mdb.c"))
          .file(lmdb.join("midl.c"));
    config.opt_level(2);

    if target.contains("dragonfly") {
        config.flag("-DMDB_DSYNC=O_SYNC");
        config.flag("-DMDB_FDATASYNC=fsync");
    }

    if target.contains("android") {
        config.define("ANDROID", "1");
    } else if target.contains("apple") {
        config.define("MDB_USE_POSIX_MUTEX", "1");
        config.define("MDB_USE_ROBUST", "0");
    }

    config.compile("liblmdb.a");
}
