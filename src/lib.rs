#![doc = include_str!("../README.md")]
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod ansi;
pub mod css;
pub mod mode;
pub mod rgb;
mod style;
mod value;
pub mod xterm;

/// A styled value, created from [`Colorize`] or [`StyledValue::new`]
#[non_exhaustive]
pub struct StyledValue<T, F = NoColor, B = NoColor, U = NoColor> {
    /// The value to style
    pub value: T,
    /// The style to use
    pub style: Style<F, B, U>,
    /// The stream to use
    pub stream: Stream,
}

/// The stream to detect when to color on
#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stream {
    /// Always color, used to pick the coloring mode at runtime for a particular value
    ///
    /// The default coloring mode for streams
    #[default]
    AlwaysColor,
    /// Never color, used to pick the coloring mode at runtime for a particular value
    NeverColor,
    /// Detect via [`std::io::stdout`] if feature `std` is enabled
    Stdout,
    /// Detect via [`std::io::stderr`] if feature `std` is enabled
    Stderr,
    /// Detect via [`std::io::stdin`] if feature `std` is enabled
    Stdin,
}

impl<T: ?Sized> Colorize for T {}
pub use value::Colorize;

pub use style::{Effect, EffectFlags, EffectFlagsIter, Style};

/// A no color placeholder type
#[derive(Debug, Clone, Copy)]
pub struct NoColor;

/// A runtime color args
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// The ANSI color type (see [`ansi`] for details)
    Ansi(ansi::AnsiColor),
    /// The Xterm color type (see [`xterm`] for details)
    Xterm(xterm::XtermColor),
    /// The CSS color type (see [`css`] for details)
    Css(css::CssColor),
    /// The Rgb color type (see [`rgb`] for details)
    Rgb(rgb::RgbColor),
}

mod seal {
    pub trait Seal {}
}

/// A sealed trait for describing ANSI color args
pub trait ColorSpec: seal::Seal {
    /// The runtime version of the color
    type Dynamic;

    #[doc(hidden)]
    const KIND: ArgsKind = ArgsKind::Unknown;

    /// Covnert to the runtime version of the color
    fn into_dynamic(self) -> Self::Dynamic;

    /// The foreground color arguments
    fn foreground_args(&self) -> &'static str;

    /// The background color arguments
    fn background_args(&self) -> &'static str;

    /// The underline color arguments
    fn underline_args(&self) -> &'static str;

    /// The foreground color sequence
    fn foreground_escape(&self) -> &'static str;

    /// The background color sequence
    fn background_escape(&self) -> &'static str;

    /// The underline color sequence
    fn underline_escape(&self) -> &'static str;
}

impl<C: ColorSpec> WriteColor for C {
    #[doc(hidden)]
    #[inline(always)]
    fn args_kind(&self) -> ArgsKind {
        C::KIND
    }

    fn fmt_foreground_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.foreground_args())
    }

    fn fmt_background_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.background_args())
    }

    fn fmt_underline_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.underline_args())
    }

    fn fmt_foreground(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.foreground_escape())
    }

    fn fmt_background(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.background_escape())
    }

    fn fmt_underline(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.underline_escape())
    }
}

/// A sealed trait for describing how to write ANSI color args
pub trait WriteColor: seal::Seal {
    #[doc(hidden)]
    #[inline(always)]
    fn args_kind(&self) -> ArgsKind {
        ArgsKind::Unknown
    }

    /// write the foreground color arguments
    fn fmt_foreground_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;

    /// write the background color arguments
    fn fmt_background_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;

    /// write the underline color arguments
    fn fmt_underline_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;

    /// write the foreground color sequence
    fn fmt_foreground(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("\x1b[")?;
        self.fmt_foreground_args(f)?;
        f.write_str("m")
    }

    /// write the background color sequence
    fn fmt_background(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("\x1b[")?;
        self.fmt_background_args(f)?;
        f.write_str("m")
    }

    /// write the underline color sequence
    fn fmt_underline(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("\x1b[58;")?;
        self.fmt_underline_args(f)?;
        f.write_str("m")
    }
}

impl seal::Seal for Color {}
impl WriteColor for Color {
    #[doc(hidden)]
    #[inline(always)]
    fn args_kind(&self) -> ArgsKind {
        match self {
            Color::Ansi(_) => ArgsKind::Ansi,
            Color::Css(_) => ArgsKind::Rgb,
            Color::Xterm(_) => ArgsKind::Xterm,
            Color::Rgb(_) => ArgsKind::Rgb,
        }
    }

    fn fmt_foreground_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Color::Ansi(color) => color.fmt_foreground_args(f),
            Color::Css(color) => color.fmt_foreground_args(f),
            Color::Xterm(color) => color.fmt_foreground_args(f),
            Color::Rgb(color) => color.fmt_background_args(f),
        }
    }

    fn fmt_background_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Color::Ansi(color) => color.fmt_background_args(f),
            Color::Css(color) => color.fmt_background_args(f),
            Color::Xterm(color) => color.fmt_background_args(f),
            Color::Rgb(color) => color.fmt_background_args(f),
        }
    }

    fn fmt_underline_args(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Color::Ansi(color) => color.fmt_underline_args(f),
            Color::Css(color) => color.fmt_underline_args(f),
            Color::Xterm(color) => color.fmt_underline_args(f),
            Color::Rgb(color) => color.fmt_underline_args(f),
        }
    }

    fn fmt_foreground(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Color::Ansi(color) => color.fmt_foreground(f),
            Color::Css(color) => color.fmt_foreground(f),
            Color::Xterm(color) => color.fmt_foreground(f),
            Color::Rgb(color) => color.fmt_foreground(f),
        }
    }

    fn fmt_background(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Color::Ansi(color) => color.fmt_background(f),
            Color::Css(color) => color.fmt_background(f),
            Color::Xterm(color) => color.fmt_background(f),
            Color::Rgb(color) => color.fmt_background(f),
        }
    }

    fn fmt_underline(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Color::Ansi(color) => color.fmt_underline(f),
            Color::Css(color) => color.fmt_underline(f),
            Color::Xterm(color) => color.fmt_underline(f),
            Color::Rgb(color) => color.fmt_underline(f),
        }
    }
}

impl seal::Seal for core::convert::Infallible {}
impl WriteColor for core::convert::Infallible {
    fn fmt_foreground_args(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {}
    }

    fn fmt_background_args(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {}
    }

    fn fmt_underline_args(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {}
    }
}

#[doc(hidden)]
pub enum Kind {
    MaybeSome,
    AlwaysSome,
    NeverSome,
}

#[doc(hidden)]
pub enum ArgsKind {
    Ansi,
    Xterm,
    Rgb,
    Unknown,
}

/// An optional color type
pub trait OptionalColor {
    /// The color type
    type Color: WriteColor;

    #[doc(hidden)]
    const KIND: Kind = Kind::MaybeSome;

    /// Get the color value
    fn get(&self) -> Option<Self::Color>;
}

impl<C: WriteColor + Clone> OptionalColor for C {
    type Color = Self;

    const KIND: Kind = Kind::AlwaysSome;

    #[inline]
    fn get(&self) -> Option<Self::Color> {
        Some(self.clone())
    }
}

impl<C: OptionalColor> OptionalColor for Option<C> {
    type Color = C::Color;

    #[inline]
    fn get(&self) -> Option<Self::Color> {
        self.as_ref().and_then(C::get)
    }
}

impl OptionalColor for NoColor {
    type Color = core::convert::Infallible;

    const KIND: Kind = Kind::NeverSome;

    #[inline]
    fn get(&self) -> Option<Self::Color> {
        None
    }
}

struct Ref<'a, T: ?Sized>(&'a T);

impl<T: ?Sized + OptionalColor> OptionalColor for Ref<'_, T> {
    type Color = T::Color;

    const KIND: Kind = T::KIND;

    #[inline]
    fn get(&self) -> Option<Self::Color> {
        self.0.get()
    }
}

/// A compile time color value
pub trait ComptimeColor {
    /// The corrosponding [`Color`] value
    const VALUE: Option<Color>;
}

impl ComptimeColor for NoColor {
    const VALUE: Option<Color> = None;
}

impl From<NoColor> for Option<Color> {
    #[inline(always)]
    fn from(_value: NoColor) -> Self {
        None
    }
}
