#[allow(dead_code)]
pub mod filedata {
    use std::fs;
    use std::io::Read;

    struct FileData {
        fp: fs::File,
        fm: fs::Metadata,
    }

    impl FileData {
        pub fn open<P: AsRef<std::path::Path>>(p: P) -> Result<FileData, String> {
            let _fp = fs::File::open(p);
            let _file: fs::File;
            match _fp {
                Ok(f) => _file = f,
                Err(e) => return Err(e.to_string())
            };

            let _fm = _file.metadata();
            let _meta: fs::Metadata;
            match _fm {
                Ok(m) => _meta = m,
                Err(e) => return Err(e.to_string())
            }

            Ok(
                FileData {
                    fp: _file,
                    fm: _meta,
                }
            )
        }

        pub fn to_vec(mut self) -> Result<Vec<u8>, String> {
            let mut vec = vec![0; self.fm.len() as usize];
            let _res = self.fp.read(&mut vec);
            match _res {
                Ok(r) => if vec.len() == r {
                    Ok(vec)
                } else {
                    Err("error reading from file".to_string())
                },
                Err(e) => Err(e.to_string()),
            }
        }

        pub fn file(self) -> fs::File {
            self.fp
        }

        pub fn metadata(self) -> fs::Metadata {
            self.fm
        }
    }
}