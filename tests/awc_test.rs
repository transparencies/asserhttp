#[macro_export]
macro_rules! awc_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[actix_web::test]
            #[stubr::mock($stub)]
            async fn [<awc_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+;)+
            }
            #[actix_web::test]
            #[stubr::mock($stub)]
            async fn [<awc_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $error:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_result_ $fn_name>]() {
                #[allow(unused_imports)]
                use asserhttp::*;
                $(assert_eq!(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+.unwrap_err(), $error);)+
            }
        }
    };
}
