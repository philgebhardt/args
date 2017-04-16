macro_rules! args {
    () => {{
        let mut args = Args::new("program", "Run this program");
        args.flag("f", "flag", "Flag");
        args
    }};
    ( $occur:expr, $default:expr ) => {{
        let mut args = Args::new("program", "Run this program");
        args.option("o", "option", "Option", "OPT", $occur, $default);
        args
    }};
}

mod has_options {
    mod has_none {
        use Args;

        #[test]
        fn it_returns_false() {
            let args = Args::new("program", "Run this program.");

            assert!(!args.has_options());
        }
    }

    mod has_flag {
        use Args;

        #[test]
        fn it_returns_true() {
            let args = args!();

            assert!(args.has_options());
        }
    }

    mod has_option {
        use Args;
        use getopts::Occur;

        #[test]
        fn it_returns_true() {
            let args = args!(Occur::Optional, None);

            assert!(args.has_options());
        }
    }
}

mod has_value {
    mod absent {
        use Args;
        use getopts::Occur;

        #[test]
        #[allow(unused_must_use)]
        fn returns_false() {
            let mut args = args!(Occur::Optional, None);
            args.parse(&vec!(""));

            assert!(!args.has_value("option"));
        }
    }

    mod present {
        use Args;
        use getopts::Occur;

        #[test]
        #[allow(unused_must_use)]
        fn returns_true() {
            let mut args = args!(Occur::Optional, None);
            args.parse(&vec!("-o", "option"));

            assert!(args.has_value("option"));
        }
    }
}

mod parse {
    mod unrecognized_option {
        use Args;

        #[test]
        fn returns_err() {
            let mut args = args!();
            assert!(args.parse(&vec!("-i")).is_err());
        }
    }

    mod flag {
        mod absent {
            use Args;

            #[test]
            #[allow(unused_must_use)]
            fn returns_false() {
                let mut args = args!();
                args.parse(&vec!(""));

                assert!(!args.value_of::<bool>("flag").unwrap());
            }
        }

        mod present {
            use Args;

            #[test]
            #[allow(unused_must_use)]
            fn returns_true() {
                let mut args = args!();
                args.parse(&vec!("-f"));

                assert!(args.value_of::<bool>("flag").unwrap());
            }
        }
    }

    mod single {
        mod argument_missing {
            use Args;
            use getopts::Occur;

            #[test]
            fn returns_err() {
                let mut args = args!(Occur::Optional, None);
                assert!(args.parse(&vec!("-o")).is_err());
            }
        }

        mod optional {
            mod absent {
                mod defaulted {
                    use Args;
                    use getopts::Occur;

                    #[test]
                    #[allow(unused_must_use)]
                    fn returns_default() {
                        let default = "default";
                        let mut args = args!(Occur::Optional, Some(default.to_string()));
                        args.parse(&vec!(""));

                        assert_eq!(default.to_string(), args.value_of::<String>("option").unwrap());
                    }
                }

                mod not_defaulted {
                    use Args;
                    use getopts::Occur;

                    #[test]
                    #[allow(unused_must_use)]
                    fn returns_err() {
                        let mut args = args!(Occur::Optional, None);
                        args.parse(&vec!(""));

                        assert!(args.value_of::<String>("option").is_err());
                    }
                }
            }

            mod present {
                use Args;
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_value() {
                    let value = "value";
                    let mut args = args!(Occur::Optional, None);
                    args.parse(&vec!("-o", value));

                    assert_eq!(value.to_string(), args.value_of::<String>("option").unwrap());
                }
            }
        }

        mod required {
            mod absent {
                mod defaulted {
                    use Args;
                    use getopts::Occur;

                    #[test]
                    #[allow(unused_must_use)]
                    fn returns_default() {
                        let default = "default";
                        let mut args = args!(Occur::Req, Some(default.to_string()));
                        args.parse(&vec!(""));

                        assert_eq!(default.to_string(), args.value_of::<String>("option").unwrap());
                    }
                }

                mod not_defaulted {
                    use Args;
                    use getopts::Occur;

                    #[test]
                    fn returns_err() {
                        let mut args = args!(Occur::Req, None);
                        assert!(args.parse(&vec!("")).is_err());
                    }
                }
            }

            mod present {
                use Args;
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_value() {
                    let value = "value";
                    let mut args = args!(Occur::Req, None);
                    args.parse(&vec!("-o", value));

                    assert_eq!(value.to_string(), args.value_of::<String>("option").unwrap());
                }
            }
        }
    }

    mod multi {
        mod absent {
            use Args;
            use getopts::Occur;

            #[test]
            fn returns_ok() {
                let raw_args: Vec<String> = Vec::new();
                let mut args = args!(Occur::Multi, None);
                let parse = args.parse(&raw_args);
                assert!(parse.is_ok(), parse.unwrap_err().to_string());
            }
        }

        mod argument_missing {
            use Args;
            use getopts::Occur;

            #[test]
            fn returns_err() {
                let mut args = args!(Occur::Multi, None);
                assert!(args.parse(&vec!("-o")).is_err());
            }
        }

        mod present {
            mod single_arg {
                use Args;
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_value() {
                    let value = "value";
                    let mut args = args!(Occur::Multi, None);
                    args.parse(&vec!("-o", value));

                    let result = args.values_of::<String>("option");
                    assert!(result.is_ok());
                    let results = result.unwrap();
                    assert_eq!(1, results.len());
                    assert_eq!(value, results[0]);
                }
            }

            mod multiple_args {
                use Args;
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_values() {
                    let values = vec!["test", "value"];
                    let mut args = args!(Occur::Multi, None);
                    args.parse(&vec!("-o", values[0], "-o", values[1]));

                    let result = args.values_of::<String>("option");
                    assert!(result.is_ok());
                    let results = result.unwrap();
                    assert_eq!(2, values.len());
                    assert_eq!(values[0], results[0]);
                    assert_eq!(values[1], results[1]);
                }
            }
        }
    }
}

mod validated_value_of {
    mod opt_absent {
        use Args;

        #[test]
        fn returns_err() {
            assert!(args!().validated_value_of::<i32>("", &[]).is_err());
        }
    }

    mod opt_present {
        mod cannot_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_err() {
                let value = "value";
                let mut args = args!(Occur::Req, None);
                args.parse(&vec!("-o", value));

                assert!(args.validated_value_of::<i32>("option", &[]).is_err());
            }
        }

        mod can_be_cast {
            mod validation_fails {
                use Args;
                use validations::{Order,OrderValidation};
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_err() {
                    let value = "0";
                    let mut args = args!(Occur::Req, None);
                    args.parse(&vec!("-o", value));

                    let validation = Box::new(OrderValidation::new(Order::GreaterThan, 0i32));
                    assert!(args.validated_value_of::<i32>("option", &[validation]).is_err());
                }
            }

            mod validation_passes {
                use Args;
                use validations::{Order,OrderValidation};
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_err() {
                    let value = "0";
                    let mut args = args!(Occur::Req, None);
                    args.parse(&vec!("-o", value));

                    let validation = Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 0i32));
                    let result = args.validated_value_of::<i32>("option", &[validation]);
                    assert!(result.is_ok());
                    assert_eq!(0i32, result.unwrap());
                }
            }
        }
    }
}

mod optional_value_of {
    mod opt_absent {
        use Args;

        #[test]
        fn returns_none() {
            assert!(!args!().optional_value_of::<i32>("").unwrap().is_some())
        }
    }

    mod opt_present {
        mod cannot_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_err() {
                let value = "value";
                let mut args = args!(Occur::Req, None);
                args.parse(&vec!("-o", value));

                assert!(args.optional_value_of::<i32>("option").is_err());
            }
        }

        mod can_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_ok_value() {
                let value = "0";
                let mut args = args!(Occur::Req, None);
                args.parse(&vec!("-o", value));

                let result = args.optional_value_of::<i32>("option");
                assert!(result.is_ok());
                let optional = result.unwrap();
                assert!(optional.is_some());
                assert_eq!(0i32, optional.unwrap());
            }
        }
    }
}

mod optional_validated_value_of {
    mod opt_absent {
        use Args;

        #[test]
        fn returns_none() {
            assert!(args!().optional_validated_value_of::<i32>("", &[]).unwrap().is_none());
        }
    }

    mod opt_present {
        mod cannot_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_err() {
                let value = "value";
                let mut args = args!(Occur::Req, None);
                args.parse(&vec!("-o", value));

                assert!(args.optional_validated_value_of::<i32>("option", &[]).is_err());
            }
        }

        mod can_be_cast {
            mod validation_fails {
                use Args;
                use validations::{Order,OrderValidation};
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_err() {
                    let value = "0";
                    let mut args = args!(Occur::Req, None);
                    args.parse(&vec!("-o", value));

                    let validation = Box::new(OrderValidation::new(Order::GreaterThan, 0i32));
                    assert!(args.optional_validated_value_of::<i32>("option", &[validation]).is_err());
                }
            }

            mod validation_passes {
                use Args;
                use validations::{Order,OrderValidation};
                use getopts::Occur;

                #[test]
                #[allow(unused_must_use)]
                fn returns_err() {
                    let value = "0";
                    let mut args = args!(Occur::Req, None);
                    args.parse(&vec!("-o", value));

                    let validation = Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 0i32));
                    let result = args.optional_validated_value_of::<i32>("option", &[validation]);
                    assert!(result.is_ok());
                    let optional = result.unwrap();
                    assert!(optional.is_some());
                    assert_eq!(0i32, optional.unwrap());
                }
            }
        }
    }
}

mod value_of {
    mod opt_absent {
        use Args;

        #[test]
        fn returns_err() {
            assert!(args!().value_of::<i32>("").is_err());
        }
    }

    mod opt_present {
        mod cannot_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_err() {
                let value = "value";
                let mut args = args!(Occur::Req, None);
                args.parse(&vec!("-o", value));

                assert!(args.value_of::<i32>("option").is_err());
            }
        }

        mod can_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_ok_value() {
                let value = "0";
                let mut args = args!(Occur::Req, None);
                args.parse(&vec!("-o", value));

                let result = args.value_of::<i32>("option");
                assert!(result.is_ok());
                assert_eq!(0i32, result.unwrap());
            }
        }
    }
}

mod values_of {
    mod opt_absent {
        use Args;

        #[test]
        fn returns_err() {
            assert!(args!().values_of::<i32>("").is_err());
        }
    }

    mod opt_present {
        mod cannot_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_err() {
                let value = "value";
                let mut args = args!(Occur::Multi, None);
                args.parse(&vec!("-o", "0", "-o", value));

                assert!(args.values_of::<i32>("option").is_err());
            }
        }

        mod can_be_cast {
            use Args;
            use getopts::Occur;

            #[test]
            #[allow(unused_must_use)]
            fn returns_ok_value() {
                let values = vec!["0", "0"];
                let mut args = args!(Occur::Multi, None);
                args.parse(&vec!("-o", values[0], "-o", values[1]));

                let result = args.values_of::<i32>("option");
                assert!(result.is_ok());
                let results = result.unwrap();
                assert_eq!(0i32, results[0]);
                assert_eq!(0i32, results[1]);
            }
        }
    }
}
