//! Xterm 8-bit colors (256 supported colors), a superset of ANSI color args

// a) color rgb values come from https://github.com/jonasjacek/colors/blob/master/data.json
// b) color names taken from https://gitlab.freedesktop.org/xorg/app/rgb/raw/master/rgb.txt
// Then the closest rgb value from a) to the rgb value in b) was found, and that was selected
// as the color name. (see `color_name_picker.py`)

#[cfg(doc)]
use crate::Color;
use crate::ColorSpec;

macro_rules! XTerm {
    ($d:tt $($args:tt $name:ident)*) => {
        /// A runtime Xterm color type
        ///
        /// Can be converted from a u8 via [`From`] or [`from_args`](Self::from_code) based on the Xterm color args
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum XtermColor {
            $(
                #[doc = concat!("The runtime version of [`", stringify!($name), "`](self::", stringify!($name), ")")]
                #[doc = concat!(" repesenting the XTerm args ", stringify!($args))]
                $name,
            )*
        }

        const _: [(); core::mem::size_of::<XtermColor>()] = [(); 1];

        const _: () = {
            $(assert!(XtermColor::$name as u8 == $args);)*
        };

        $(
            /// A compile time Xterm color type
            #[doc = concat!(" repesenting the color args ", stringify!($args))]
            ///
            /// You can convert this type to [`XtermColor`] via [`From`] or [`Self::DYNAMIC`]
            /// and to [`Color`] or [`Option<Color>`] via [`From`]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
        )*

        /// Convert a literal color args to the compile time Xterm color type
        #[macro_export]
        macro_rules! xterm_from_code {
            $(($args) => { $crate::xterm::$name };)*
            ($d t:tt) => {{
                compile_error! { concat!("Invalid input, expected an unsuffixed u8 literal but got: ", stringify!($d t)) }
            }};
        }

        impl From<u8> for XtermColor {
            #[inline(always)]
            fn from(args: u8) -> Self {
                Self::from_code(args)
            }
        }

        impl From<XtermColor> for crate::Color {
            #[inline(always)]
            fn from(color: XtermColor) -> Self {
                crate::Color::Xterm(color)
            }
        }

        impl From<XtermColor> for Option<crate::Color> {
            #[inline(always)]
            fn from(color: XtermColor) -> Self {
                Some(crate::Color::Xterm(color))
            }
        }

        $(
            impl From<$name> for XtermColor {
                #[inline(always)]
                fn from(_: $name) -> Self {
                    Self::$name
                }
            }

            impl From<$name> for crate::Color {
                #[inline(always)]
                fn from(_: $name) -> Self {
                    crate::Color::Xterm($name::DYNAMIC)
                }
            }

            impl From<$name> for Option<crate::Color> {
                #[inline(always)]
                fn from(_: $name) -> Self {
                    <$name as crate::ComptimeColor>::VALUE
                }
            }

            impl crate::ComptimeColor for $name {
                const VALUE: Option<crate::Color> = Some(crate::Color::Xterm(Self::DYNAMIC));
            }
        )*

        impl XtermColor {
            /// Get a Xterm color via it's color args
            #[inline]
            pub const fn from_code(args: u8) -> Self {
                match args {
                    $($args => Self::$name,)*
                }
            }

            /// The color args of this Xterm color
            #[inline]
            pub const fn args(self) -> &'static str {
                match self {
                    $(Self::$name => $name::ARGS,)*
                }
            }

            /// The ANSI foreground color arguments
            #[inline]
            pub const fn foreground_args(self) -> &'static str {
                const FOREGROUND_ARGS: &[&'static str; 256] = &[
                    $($name::FOREGROUND_ARGS,)*
                ];

                FOREGROUND_ARGS[self as usize]
            }

            /// The ANSI background color arguments
            #[inline]
            pub const fn background_args(self) -> &'static str {
                const BACKGROUND_ARGS: &[&'static str; 256] = &[
                    $($name::BACKGROUND_ARGS,)*
                ];

                BACKGROUND_ARGS[self as usize]
            }

            /// The ANSI underline color arguments
            #[inline]
            pub const fn underline_args(self) -> &'static str {
                const UNDERLINE_ARGS: &[&'static str; 256] = &[
                    $($name::UNDERLINE_ARGS,)*
                ];

                UNDERLINE_ARGS[self as usize]
            }

            /// The foreground color sequence of this Xterm color
            #[inline]
            pub const fn foreground_escape(self) -> &'static str {
                const FOREGROUND_ESCAPE: &[&'static str; 256] = &[
                    $($name::FOREGROUND_ESCAPE,)*
                ];

                FOREGROUND_ESCAPE[self as usize]
            }

            /// The background color sequence of this Xterm color
            #[inline]
            pub const fn background_escape(self) -> &'static str {
                const BACKGROUND_ESCAPE: &[&'static str; 256] = &[
                    $($name::BACKGROUND_ESCAPE,)*
                ];

                BACKGROUND_ESCAPE[self as usize]
            }

            /// The underline color sequence of this Xterm color
            #[inline]
            pub const fn underline_escape(self) -> &'static str {
                const UNDERLINE_ESCAPE: &[&'static str; 256] = &[
                    $($name::UNDERLINE_ESCAPE,)*
                ];

                UNDERLINE_ESCAPE[self as usize]
            }
        }

        impl crate::seal::Seal for XtermColor {}
        impl ColorSpec for XtermColor {
            type Dynamic = Self;

            const KIND: crate::mode::ColorKind = crate::mode::ColorKind::Xterm;

            #[inline]
            fn into_dynamic(self) -> Self::Dynamic {
                self
            }

            #[inline]
            fn foreground_args(self) -> &'static str {
                self.foreground_args()
            }

            #[inline]
            fn background_args(self) -> &'static str {
                self.background_args()
            }

            #[inline]
            fn underline_args(self) -> &'static str {
                self.underline_args()
            }

            #[inline]
            fn foreground_escape(self) -> &'static str {
                self.foreground_escape()
            }

            #[inline]
            fn background_escape(self) -> &'static str {
                self.background_escape()
            }

            #[inline]
            fn underline_escape(self) -> &'static str {
                self.underline_escape()
            }
        }


        $(
            impl $name {
                /// The corrosponding variant on [`XtermColor`]
                pub const DYNAMIC: XtermColor = XtermColor::$name;

                /// The ANSI color args
                pub const ARGS: &'static str = concat!("5;", stringify!($args));

                /// The ANSI foreground color arguments
                pub const FOREGROUND_ARGS: &'static str = concat!("38;5;", stringify!($args));
                /// The ANSI background color arguments
                pub const BACKGROUND_ARGS: &'static str = concat!("48;5;", stringify!($args));
                /// The ANSI underline color arguments
                pub const UNDERLINE_ARGS: &'static str = concat!("58;5;", stringify!($args));

                /// The ANSI foreground color sequence
                pub const FOREGROUND_ESCAPE: &'static str = concat!("\x1b[38;5;", stringify!($args) ,"m");
                /// The ANSI background color sequence
                pub const BACKGROUND_ESCAPE: &'static str = concat!("\x1b[48;5;", stringify!($args) ,"m");
                /// The ANSI underline color sequence
                pub const UNDERLINE_ESCAPE: &'static str = concat!("\x1b[58;5;", stringify!($args) ,"m");
            }

            impl crate::seal::Seal for $name {}
            impl ColorSpec for $name {
                type Dynamic = XtermColor;

                const KIND: crate::mode::ColorKind = crate::mode::ColorKind::Xterm;

                #[inline]
                fn into_dynamic(self) -> Self::Dynamic {
                    Self::DYNAMIC
                }

                #[inline]
                fn foreground_args(self) -> &'static str {
                    Self::FOREGROUND_ARGS
                }

                #[inline]
                fn background_args(self) -> &'static str {
                    Self::BACKGROUND_ARGS
                }

                #[inline]
                fn underline_args(self) -> &'static str {
                    Self::UNDERLINE_ARGS
                }

                #[inline]
                fn foreground_escape(self) -> &'static str {
                    Self::FOREGROUND_ESCAPE
                }

                #[inline]
                fn background_escape(self) -> &'static str {
                    Self::BACKGROUND_ESCAPE
                }

                #[inline]
                fn underline_escape(self) -> &'static str {
                    Self::UNDERLINE_ESCAPE
                }
            }
        )*
    };
}

XTerm! {
    $
    0 Black
    1 Red
    2 Green
    3 Yellow
    4 Blue
    5 Magenta
    6 Cyan
    7 White
    8 BrightBlack
    9 BrightRed
    10 BrightGreen
    11 BrightYellow
    12 BrightBlue
    13 BrightMagenta
    14 BrightCyan
    15 BrightWhite
    16 Gray0
    17 Navy
    18 DarkBlue
    19 Blue3
    20 MediumBlue
    21 Blue1
    22 DarkGreen
    23 Teal
    24 DeepSkyBlue4
    25 DodgerBlue4
    26 DodgerBlue3
    27 DodgerBlue2
    28 Green4
    29 SpringGreen4
    30 Turquoise4
    31 DarkCyan
    32 DeepSkyBlue3
    33 DodgerBlue
    34 ForestGreen
    35 SeaGreen
    36 Cyan4
    37 LightSeaGreen
    38 DeepSkyBlue2
    39 DeepSkyBlue
    40 Green3
    41 SpringGreen3
    42 SpringGreen2
    43 Cyan3
    44 DarkTurquoise
    45 Turquoise2
    46 Lime
    47 SpringGreen1
    48 SpringGreen
    49 MediumSpringGreen
    50 Cyan2
    51 Aqua
    52 Firebrick4
    53 DarkOrchid4
    54 Indigo
    55 Purple4
    56 Purple3
    57 BlueViolet
    58 Olive
    59 Gray37
    60 MediumPurple4
    61 SlateBlue
    62 SlateBlue3
    63 RoyalBlue1
    64 Chartreuse4
    65 DarkSeaGreen4
    66 PaleTurquoise4
    67 SteelBlue
    68 SteelBlue3
    69 CornflowerBlue
    70 OliveDrab
    71 PaleGreen4
    72 DarkSlateGray4
    73 CadetBlue
    74 SkyBlue3
    75 SteelBlue1
    76 Chartreuse3
    77 MediumSeaGreen
    78 SeaGreen3
    79 MediumAquamarine
    80 MediumTurquoise
    81 LightSkyBlue
    82 Chartreuse2
    83 LimeGreen
    84 SeaGreen2
    85 SeaGreen1
    86 Aquamarine1
    87 DarkSlateGray2
    88 DarkRed
    89 DeepPink4
    90 DarkMagenta
    91 Magenta4
    92 DarkViolet
    93 Purple2
    94 Orange4
    95 LightPink4
    96 Plum4
    97 Orchid4
    98 MediumPurple3
    99 SlateBlue1
    100 Yellow4
    101 Wheat4
    102 Gray53
    103 LightSlateGray
    104 MediumPurple
    105 LightSlateBlue
    106 OliveDrab4
    107 LemonChiffon4
    108 DarkSeaGreen
    109 Gray63
    110 LightSkyBlue3
    111 SkyBlue2
    112 LawnGreen
    113 YellowGreen
    114 PaleGreen3
    115 DarkSeaGreen3
    116 DarkSlateGray3
    117 SkyBlue1
    118 Chartreuse
    119 OliveDrab2
    120 LightGreen
    121 PaleGreen1
    122 Aquamarine
    123 DarkSlateGray1
    124 Red4
    125 Maroon4
    126 MediumVioletRed
    127 Maroon3
    128 DarkOrchid3
    129 Purple
    130 DarkGoldenrod4
    131 IndianRed3
    132 PaleVioletRed3
    133 MediumOrchid3
    134 MediumOrchid
    135 DarkOrchid1
    136 DarkGoldenrod
    137 NavajoWhite4
    138 RosyBrown
    139 Grey63
    140 MediumPurple2
    141 MediumPurple1
    142 DarkGoldenrod3
    143 DarkKhaki
    144 NavajoWhite3
    145 Gray69
    146 LightSteelBlue3
    147 LightSteelBlue
    148 OliveDrab3
    149 DarkOliveGreen3
    150 PaleGreen2
    151 Honeydew3
    152 LightCyan3
    153 LightSkyBlue1
    154 GreenYellow
    155 DarkOliveGreen2
    156 PaleGreen
    157 DarkSeaGreen2
    158 DarkSeaGreen1
    159 PaleTurquoise1
    160 Red3
    161 Crimson
    162 DeepPink3
    163 VioletRed
    164 Magenta3
    165 Magenta2
    166 DarkOrange3
    167 IndianRed
    168 HotPink3
    169 HotPink2
    170 Orchid
    171 MediumOrchid1
    172 Orange3
    173 LightSalmon3
    174 LightPink3
    175 Pink3
    176 Plum3
    177 Violet
    178 Gold3
    179 LightGoldenrod3
    180 Tan
    181 MistyRose3
    182 Thistle3
    183 Plum2
    184 Yellow3
    185 Khaki3
    186 LightGoldenrod
    187 LightYellow3
    188 Gray84
    189 LightSteelBlue1
    190 Yellow2
    191 DarkOliveGreen1
    192 Khaki2
    193 PaleGoldenrod
    194 Honeydew2
    195 LightCyan
    196 Red1
    197 DeepPink2
    198 DeepPink
    199 DeepPink1
    200 Magenta1
    201 Fuchsia
    202 OrangeRed
    203 IndianRed1
    204 VioletRed1
    205 HotPink
    206 HotPink1
    207 MediumOrchid2
    208 DarkOrange
    209 Salmon1
    210 LightCoral
    211 PaleVioletRed1
    212 Orchid2
    213 Orchid1
    214 Orange
    215 SandyBrown
    216 LightSalmon
    217 LightPink1
    218 Pink1
    219 Plum1
    220 Gold
    221 Khaki
    222 LightGoldenrod2
    223 NavajoWhite
    224 MistyRose
    225 Thistle1
    226 Yellow1
    227 LightGoldenrod1
    228 Khaki1
    229 Wheat1
    230 Cornsilk
    231 Gray100
    232 Gray3
    233 Gray7
    234 Gray11
    235 Gray15
    236 Gray19
    237 Gray23
    238 Gray27
    239 Gray30
    240 Gray34
    241 Gray38
    242 Gray42
    243 Gray46
    244 Gray50
    245 Gray54
    246 Gray58
    247 Gray62
    248 Gray66
    249 Gray70
    250 Gray74
    251 Gray78
    252 Gray81
    253 Gray85
    254 Gray89
    255 Gray93
}
