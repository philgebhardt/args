mod options;
mod validations;

macro_rules! args {
    () => {{
        let mut args = Args::new("program");
        args.flag("f", "flag", "Flag");
        args
    }};
    ( $occur:expr, $default:expr ) => {{
        let mut args = Args::new("program");
        args.option("o", "option", "Option", "OPT", $occur, $default);
        args
    }};
}

mod has_value {
    mod absent {
        use Args;
        use getopts::Occur;

        #[test]
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
        fn returns_true() {
            let mut args = args!(Occur::Optional, None);
            args.parse(&vec!("-o", "option"));

            assert!(args.has_value("option"));
        }
    }
}

mod parse {
    mod flag {
        mod absent {
            use Args;

            #[test]
            fn returns_false() {
                let mut args = args!();
                args.parse(&vec!(""));

                assert!(!args.value_of::<bool>("flag").unwrap());
            }
        }

        mod present {
            use Args;

            #[test]
            fn returns_true() {
                let mut args = args!();
                args.parse(&vec!("-f"));

                assert!(args.value_of::<bool>("flag").unwrap());
            }
        }
    }

    mod option {
        mod optional {
            mod absent {
                mod defaulted {
                    use Args;
                    use getopts::Occur;

                    #[test]
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
                    #[should_panic]
                    fn panics() {
                        let mut args = args!(Occur::Req, None);
                        args.parse(&vec!(""));
                    }
                }
            }

            mod present {
                use Args;
                use getopts::Occur;

                #[test]
                fn returns_value() {
                    let value = "value";
                    let mut args = args!(Occur::Req, None);
                    args.parse(&vec!("-o", value));

                    assert_eq!(value.to_string(), args.value_of::<String>("option").unwrap());
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

