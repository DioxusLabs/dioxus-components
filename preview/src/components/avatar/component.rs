use dioxus::prelude::*;
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::avatar::{self, AvatarState};
use dioxus_primitives::merge_attributes;

#[css_module("/src/components/avatar/style.css")]
struct Styles;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarImageSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl AvatarImageSize {
    fn to_class(self) -> &'static str {
        match self {
            AvatarImageSize::Small => Styles::dx_avatar_sm.inner,
            AvatarImageSize::Medium => Styles::dx_avatar_md.inner,
            AvatarImageSize::Large => Styles::dx_avatar_lg.inner,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

impl AvatarShape {
    fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => Styles::dx_avatar_circle.inner,
            AvatarShape::Rounded => Styles::dx_avatar_rounded.inner,
        }
    }
}

/// The props for the [`Avatar`] root component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Callback when image loads successfully.
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load.
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes.
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let class = format!(
        "{} {} {}",
        Styles::dx_avatar,
        props.size.to_class(),
        props.shape.to_class()
    );
    let base = attributes!(span {
        class
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        avatar::Avatar {
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            attributes: merged,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarImageProps {
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    pub src: String,

    #[props(default)]
    pub alt: String,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    let base = attributes!(img {
        class: Styles::dx_avatar_image,
        draggable: "false",
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        avatar::AvatarImage {
            id: props.id,
            src: props.src,
            alt: props.alt,
            attributes: merged,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AvatarFallbackProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    pub children: Element,
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    let base = attributes!(span {
        class: Styles::dx_avatar_fallback,
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        avatar::AvatarFallback {
            attributes: merged,
            {props.children}
        }
    }
}

/// The props for the [`ImageAvatar`] convenience component.
#[derive(Props, Clone, PartialEq)]
pub struct ImageAvatarProps {
    /// The image source URL.
    pub src: String,

    /// The image alt text.
    #[props(default)]
    pub alt: String,

    /// Callback when image loads successfully.
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load.
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes.
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The fallback content shown while the image is loading or if it fails to load.
    pub children: Element,
}

#[component]
pub fn ImageAvatar(props: ImageAvatarProps) -> Element {
    rsx! {
        Avatar {
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            size: props.size,
            shape: props.shape,
            attributes: props.attributes,
            AvatarImage {
                src: props.src,
                alt: props.alt,
            }
            AvatarFallback {
                {props.children}
            }
        }
    }
}
