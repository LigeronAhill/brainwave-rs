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
pub const HERO_ICONS: [&str; 4] = [
    "/public/home-smile.svg",
    "/public/file-02.svg",
    "/public/search-md.svg",
    "/public/plus-square.svg",
];
pub const NOTIFICATION_IMAGES: [&str; 3] = [
    "/public/notification/image-4.png",
    "/public/notification/image-3.png",
    "/public/notification/image-2.png",
];
pub const COMPANY_LOGOS: [&str; 4] = [
    "/public/yourlogo.svg",
    "/public/yourlogo.svg",
    "/public/yourlogo.svg",
    "/public/yourlogo.svg",
];
pub const BRAINWAVE_SERVICES: [&str; 3] =
    ["Photo generating", "Photo enhance", "Seamless Integration"];
pub const BRAINWAVE_SERVICES_ICONS: [&str; 5] = [
    "/public/recording-03.svg",
    "/public/recording-01.svg",
    "/public/disc-02.svg",
    "/public/chrome-cast.svg",
    "/public/sliders-04.svg",
];
pub struct Roadmap<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub text: &'a str,
    pub date: &'a str,
    pub status: &'a str,
    pub image_url: &'a str,
    pub colorful: bool,
}
pub const ROADMAP: [Roadmap; 4] = [
  Roadmap {
    id: "0",
    title: "Voice recognition",
    text: "Enable the chatbot to understand and respond to voice commands, making it easier for users to interact with the app hands-free.",
    date: "May 2023",
    status: "done",
    image_url: "/public/roadmap/image-1.png",
    colorful: true,
  },
  Roadmap {
    id: "1",
    title: "Gamification",
    text: "Add game-like elements, such as badges or leaderboards, to incentivize users to engage with the chatbot more frequently.",
    date: "May 2023",
    status: "progress",
    image_url: "/public/roadmap/image-2.png",
    colorful: false,
  },
  Roadmap {
    id: "2",
    title: "Chatbot customization",
    text: "Allow users to customize the chatbot's appearance and behavior, making it more engaging and fun to interact with.",
    date: "May 2023",
    status: "done",
    image_url: "/public/roadmap/image-3.png",
    colorful: false,
  },
  Roadmap {
    id: "3",
    title: "Integration with APIs",
    text: "Allow the chatbot to access external data sources, such as weather APIs or news APIs, to provide more relevant recommendations.",
    date: "May 2023",
    status: "progress",
    image_url: "/public/roadmap/image-4.png",
    colorful: false,
  },
];
pub const COLLABTEXT: &str = "With smart automation and top-notch security, it's the perfect solution for teams looking to work smarter.";
