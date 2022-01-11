pub mod hosting;
/* mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn take_order() {}
        
        fn take_payment() {}
        fn cook_order() {}
    }
    fn serve_order() {}
    
    pub fn eat_at_restaurant() {
        let mut meal = back_of_house::Breakfast::summer("호밀빵");
        meal.toast = String::from("밀빵");
        println!("{} 토스트주세요", meal.toast);
    }

    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            fruit: String
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    fruit: String::from("복숭아")
                }
            }
        }

        fn fix_incorrect_order() {
            //cook_order();
            super::serve_order();
        }
    }
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
} */