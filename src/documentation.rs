//! # celq Manual
#![doc = include_str!("../docs/manual.md")]
//!
//! # Guides
//!
//! ## [Installation Guide](`crate::installation_guide`)
//!
//! ## [Comparison with other tools](`crate::comparison_with_other_tools`)

macro_rules! guide_links {
    (installation_guide) => {
        "### [Comparison with other tools](`crate::comparison_with_other_tools`)"
    };
    (comparison_with_other_tools) => {
        "### [Installation Guide](`crate::installation_guide`)"
    };
}

macro_rules! guide_module {
    ($name:ident, $guide:ident, $file:literal) => {
        #[doc = concat!(
            include_str!($file),
            "\n\n## Guides\n\n",
            guide_links!($guide)
        )]
        pub mod $name {}
    };
}

guide_module!(installation_guide, installation_guide, "../docs/installation_guide.md");
guide_module!(
    comparison_with_other_tools,
    comparison_with_other_tools,
    "../docs/comparison_with_other_tools.md"
);
