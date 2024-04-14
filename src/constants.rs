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

pub struct CollabContent<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub text: &'a str,
}

pub const COLLAB_CONTENT: [CollabContent; 3] = [
    CollabContent {
        id: "0",
        title: "Seamless Intergration",
        text: COLLABTEXT,
    },
    CollabContent {
        id: "1",
        title: "Smart Automation",
        text: "",
    },
    CollabContent {
        id: "2",
        title: "Top-notch Security",
        text: "",
    },
];

pub struct CollabApp<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub icon: &'a str,
    pub width: i32,
    pub height: i32,
}

pub const COLLAB_APPS: [CollabApp; 8] = [
    CollabApp {
        id: "0",
        title: "Figma",
        icon: "/public/collaboration/figma.png",
        width: 26,
        height: 36,
    },
    CollabApp {
        id: "1",
        title: "Notion",
        icon: "/public/collaboration/notion.png",
        width: 34,
        height: 36,
    },
    CollabApp {
        id: "2",
        title: "Discord",
        icon: "/public/collaboration/discord.png",
        width: 36,
        height: 28,
    },
    CollabApp {
        id: "3",
        title: "Slack",
        icon: "/public/collaboration/slack.png",
        width: 34,
        height: 35,
    },
    CollabApp {
        id: "4",
        title: "Photoshop",
        icon: "/public/collaboration/photoshop.png",
        width: 34,
        height: 34,
    },
    CollabApp {
        id: "5",
        title: "Protopie",
        icon: "/public/collaboration/protopie.png",
        width: 34,
        height: 34,
    },
    CollabApp {
        id: "6",
        title: "Framer",
        icon: "/public/collaboration/framer.png",
        width: 26,
        height: 34,
    },
    CollabApp {
        id: "7",
        title: "Raindrop",
        icon: "/public/collaboration/raindrop.png",
        width: 38,
        height: 32,
    },
];

pub struct BenefitsList<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub text: &'a str,
    pub background_url: &'a str,
    pub icon_url: &'a str,
    pub image_url: &'a str,
    pub light: bool,
}
pub const BENEFITS: [BenefitsList; 6] = [
  BenefitsList {
    id: "0",
    title: "Ask anything",
    text: "Lets users quickly find answers to their questions without having to search through multiple sources.",
    background_url: "/public/benefits/card-1.svg",
    icon_url: "/public/benefits/icon-1.svg",
    image_url: "/public/benefits/image-2.png",
    light: false,
  },
  BenefitsList {
    id: "1",
    title: "Improve everyday",
    text: "The app uses natural language processing to understand user queries and provide accurate and relevant responses.",
    background_url: "/public/benefits/card-2.svg",
    icon_url: "/public/benefits/icon-2.svg",
    image_url: "/public/benefits/image-2.png",
    light: true,
  },
  BenefitsList {
    id: "2",
    title: "Connect everywhere",
    text: "Connect with the AI chatbot from anywhere, on any device, making it more accessible and convenient.",
    background_url: "/public/benefits/card-3.svg",
    icon_url: "/public/benefits/icon-3.svg",
    image_url: "/public/benefits/image-2.png",
    light: false,
  },
  BenefitsList {
    id: "3",
    title: "Fast responding",
    text: "Lets users quickly find answers to their questions without having to search through multiple sources.",
    background_url: "/public/benefits/card-4.svg",
    icon_url: "/public/benefits/icon-4.svg",
    image_url: "/public/benefits/image-2.png",
    light: true,
  },
  BenefitsList {
    id: "4",
    title: "Ask anything",
    text: "Lets users quickly find answers to their questions without having to search through multiple sources.",
    background_url: "/public/benefits/card-5.svg",
    icon_url: "/public/benefits/icon-1.svg",
    image_url: "/public/benefits/image-2.png",
    light: false,
  },
  BenefitsList {
    id: "5",
    title: "Improve everyday",
    text: "The app uses natural language processing to understand user queries and provide accurate and relevant responses.",
    background_url: "/public/benefits/card-6.svg",
    icon_url: "/public/benefits/icon-2.svg",
    image_url: "/public/benefits/image-2.png",
    light: false,
  },
];
