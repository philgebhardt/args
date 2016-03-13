mod order {
    mod compare {
        mod greater_than {
            mod when_greater_than {
                use validations::Order;

                #[test]
                fn returns_true() {
                    assert!(Order::GreaterThan.compare::<i32>(&0i32, &1i32));
                }
            }

            mod when_equal_to {
                use validations::Order;

                #[test]
                fn returns_false() {
                    assert!(!Order::GreaterThan.compare::<i32>(&0i32, &0i32));
                }
            }

            mod when_less_than {
                use validations::Order;

                #[test]
                fn returns_false() {
                    assert!(!Order::GreaterThan.compare::<i32>(&0i32, &-1i32));
                }
            }
        }

        mod greater_than_or_equal {
            mod when_greater_than {
                use validations::Order;

                #[test]
                fn returns_true() {
                    assert!(Order::GreaterThanOrEqual.compare::<i32>(&0i32, &1i32));
                }
            }

            mod when_equal_to {
                use validations::Order;

                #[test]
                fn returns_true() {
                    assert!(Order::GreaterThanOrEqual.compare::<i32>(&0i32, &0i32));
                }
            }

            mod when_less_than {
                use validations::Order;

                #[test]
                fn returns_false() {
                    assert!(!Order::GreaterThanOrEqual.compare::<i32>(&0i32, &-1i32));
                }
            }
        }

        mod less_than {
            mod when_greater_than {
                use validations::Order;

                #[test]
                fn returns_false() {
                    assert!(!Order::LessThan.compare::<i32>(&0i32, &1i32));
                }
            }

            mod when_equal_to {
                use validations::Order;

                #[test]
                fn returns_false() {
                    assert!(!Order::LessThan.compare::<i32>(&0i32, &0i32));
                }
            }

            mod when_less_than {
                use validations::Order;

                #[test]
                fn returns_true() {
                    assert!(Order::LessThan.compare::<i32>(&0i32, &-1i32));
                }
            }
        }

        mod less_than_or_equal {
            mod when_greater_than {
                use validations::Order;

                #[test]
                fn returns_false() {
                    assert!(!Order::LessThanOrEqual.compare::<i32>(&0i32, &1i32));
                }
            }

            mod when_equal_to {
                use validations::Order;

                #[test]
                fn returns_true() {
                    assert!(Order::LessThanOrEqual.compare::<i32>(&0i32, &0i32));
                }
            }

            mod when_less_than {
                use validations::Order;

                #[test]
                fn returns_true() {
                    assert!(Order::LessThanOrEqual.compare::<i32>(&0i32, &-1i32));
                }
            }
        }
    }
}

