#[cfg(test)]
use crate::*;
use actix_web::{
    http::{self},
    test,
};


#[actix_web::test]
async fn test_index_ok() {
    let _req = test::TestRequest::default();
}

