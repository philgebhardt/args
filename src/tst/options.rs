macro_rules! opt {
    ( $has_arg:expr, $occur:expr, $default:expr ) => {
        Opt::new("o",
            "option",
            "Option",
            "OPT",
            $has_arg,
            $occur,
            $default
        )
    };
}

mod opt {
    mod is_required {
        mod when_optional {
            use getopts::{HasArg,Occur};
            use options::Opt;

            #[test]
            fn returns_false() {
                let opt = opt!(HasArg::Yes, Occur::Optional, None);

                assert!(!opt.is_required());
            }
        }

        mod when_required {
            mod with_default {
                use getopts::{HasArg,Occur};
                use options::Opt;

                #[test]
                fn returns_false() {
                    let opt = opt!(HasArg::Yes, Occur::Req, Some("default".to_string()));

                    assert!(!opt.is_required());
                }
            }

            mod without_default {
                use getopts::{HasArg,Occur};
                use options::Opt;

                #[test]
                fn returns_true() {
                    let opt = opt!(HasArg::Yes, Occur::Req, None);

                    assert!(opt.is_required());
                }
            }
        }
    }

    mod parse {
        mod flag {
            mod absent {
                use getopts::{HasArg,Occur,Options};
                use options::Opt;

                #[test]
                fn returns_some_false() {
                    let mut options = Options::new();
                    let opt = opt!(HasArg::No, Occur::Optional, None);
                    opt.register_option(&mut options);
                    let matches = options.parse(vec!("")).unwrap();

                    let parsed = opt.parse(&matches);
                    assert!(parsed.is_some());
                    assert_eq!("false".to_string(), parsed.unwrap());
                }
            }

            mod present {
                use getopts::{HasArg,Occur,Options};
                use options::Opt;

                #[test]
                fn returns_some_true() {
                    let mut options = Options::new();
                    let opt = opt!(HasArg::No, Occur::Optional, None);
                    opt.register_option(&mut options);
                    let matches = options.parse(vec!(&format!("--{}", opt.name()))).unwrap();

                    let parsed = opt.parse(&matches);
                    assert!(parsed.is_some());
                    assert_eq!("true".to_string(), parsed.unwrap());
                }
            }
        }

        mod option {
            mod absent {
                mod without_default {
                    use getopts::{HasArg,Occur,Options};
                    use options::Opt;

                    #[test]
                    fn returns_none() {
                        let mut options = Options::new();
                        let opt = opt!(HasArg::Yes, Occur::Optional, None);
                        opt.register_option(&mut options);
                        let matches = options.parse(vec!("")).unwrap();

                        let parsed = opt.parse(&matches);
                        assert!(parsed.is_none());
                    }
                }

                mod with_default {
                    use getopts::{HasArg,Occur,Options};
                    use options::Opt;

                    #[test]
                    fn returns_some_default() {
                        let mut options = Options::new();
                        let default = "default";
                        let opt = opt!(HasArg::Yes, Occur::Optional, Some(default.to_string()));
                        opt.register_option(&mut options);
                        let matches = options.parse(vec!("")).unwrap();

                        let parsed = opt.parse(&matches);
                        assert!(parsed.is_some());
                        assert_eq!(default.to_string(), parsed.unwrap());
                    }
                }
            }

            mod present {
                use getopts::{HasArg,Occur,Options};
                use options::Opt;

                #[test]
                fn returns_some_value() {
                    let mut options = Options::new();
                    let opt = opt!(HasArg::Yes, Occur::Optional, None);
                    opt.register_option(&mut options);
                    let value = "value";
                    let matches = options.parse(vec!(&format!("--{}", opt.name()), &value.to_string())).unwrap();

                    let parsed = opt.parse(&matches);
                    assert!(parsed.is_some());
                    assert_eq!(value.to_string(), parsed.unwrap());
                }
            }
        }
    }
}

