//! PoC of vectorization execution through JNI to Rust.
#![deny(warnings)]
#![deny(missing_docs)]

pub mod jni_api;

pub mod datatype;
pub mod expression;
pub mod functions;
pub mod operators;

// Include generated modules from .proto files.
#[allow(missing_docs)]
pub mod spark_expression {
    include!(concat!("generated", "/spark.expression.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
