pub use designer;
pub use processor;
pub mod executor {
    pub use exec_compiler::Compiler;
    pub use exec_interpreter::Interpreter;
}

#[macro_export]
macro_rules! build_dsl {
    ($dsl:expr) => {{
        use std::path::Path;
        use std::rc::Rc;
        use std::{env, fs};

        use $crate::designer::codegen::rust;

        let dsl_code = rust($dsl).unwrap();
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dst_path = Path::new(&out_dir).join("DSL.rs");
        fs::write(&dst_path, dsl_code).unwrap();

        println!("cargo:rerun-if-changed=build.rs");
    }};

    ($dsl:expr, $genfunc:ident, $file:expr) => {{
        use std::path::Path;
        use std::{env, fs};

        let dsl_code = $genfunc($dsl).unwrap();
        let out_dir = env::current_dir().unwrap();
        let dst_path = Path::new(&out_dir).join(concat!(stringify!($file)));
        fs::write(&dst_path, dsl_code).unwrap();

        println!("cargo:rerun-if-changed=build.rs");
    }};
}

#[macro_export]
macro_rules! load_dsl {
    () => {
        include!(concat!(env!("OUT_DIR"), "/DSL.rs"));
    };
}
