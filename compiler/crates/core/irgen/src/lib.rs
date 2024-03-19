mod r#impl;

use vfs::VfsPath;

use designer::design::DSLGeneratable;

pub use r#impl::rust::rust;

pub fn irgen<T, F>(genfunc: &F, dsl: T, vfs: VfsPath) -> anyhow::Result<VfsPath>
where
    T: DSLGeneratable,
    F: Fn(T, VfsPath) -> anyhow::Result<VfsPath> + 'static,
{
    genfunc(dsl, vfs)
}

#[cfg(test)]
mod test {
    use vfs::MemoryFS;

    use designer::design::syntax::RuleSet;
    use designer::design::DSLGeneratable;

    use crate::{irgen, rust};

    struct TmpDSL;

    impl DSLGeneratable for TmpDSL {
        fn name(&self) -> &'static str {
            "tmp"
        }

        fn start(&self) -> &'static str {
            "start"
        }

        fn design(&self) -> RuleSet {
            ("tmp", vec![]).into()
        }
    }

    #[test]
    fn gen_rust() {
        let fs = MemoryFS::new();
        irgen(&rust, TmpDSL {}, fs.into()).unwrap();
    }
}
