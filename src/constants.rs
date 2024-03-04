pub struct NavLink<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub url: &'a str,
    pub only_mobile: bool,
}
impl<'a> NavLink<'a> {
    pub fn iter() -> Vec<NavLink<'a>> {
        vec![
            NavLink {
                id: "0",
                title: "Features",
                url: "#features",
                only_mobile: false,
            },
            NavLink {
                id: "1",
                title: "Pricing",
                url: "#pricing",
                only_mobile: false,
            },
            NavLink {
                id: "2",
                title: "How to use",
                url: "#how-to-use",
                only_mobile: false,
            },
            NavLink {
                id: "3",
                title: "Roadmap",
                url: "#roadmap",
                only_mobile: false,
            },
            NavLink {
                id: "4",
                title: "New account",
                url: "#signup",
                only_mobile: true,
            },
            NavLink {
                id: "5",
                title: "Sign in",
                url: "#login",
                only_mobile: true,
            },
        ]
    }
}
