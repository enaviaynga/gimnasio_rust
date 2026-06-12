use freya::prelude::*;
#[derive(Debug, PartialEq, Eq)]
pub enum TemasClaros {
    Generico,
    Premium,
    Retro,
    Pastel,
    Cyber,
    Naturaleza,
}

pub fn tema_claro(tema_claro: State<TemasClaros>) -> Theme {
    match *tema_claro.read() {
        TemasClaros::Generico => claro_generico(),
        TemasClaros::Premium => tema_premium_claro(),
        TemasClaros::Retro => tema_pastel_claro(),
        TemasClaros::Pastel => tema_cyber_claro(),
        TemasClaros::Cyber => tema_naturaleza_claro(),
        TemasClaros::Naturaleza => tema_retro_claro(),
    }
}

fn tema_claro_default() -> Theme {
    claro_generico()
}

fn claro_generico() -> Theme {
    let mut tema_base = light_theme();
    tema_base.name = "claro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(41, 98, 255),      // Azul vibrante
        secondary: Color::from_rgb(108, 117, 125),  // Gris azulado
        tertiary: Color::from_rgb(111, 66, 193),    // Púrpura
        success: Color::from_rgb(25, 135, 84),      // Verde
        warning: Color::from_rgb(255, 193, 7),      // Ámbar
        error: Color::from_rgb(220, 53, 69),        // Rojo
        info: Color::from_rgb(13, 202, 240),        // Cyan
        background: Color::from_rgb(255, 255, 255), // Blanco
        surface_primary: Color::from_rgb(255, 255, 255), // Blanco
        surface_secondary: Color::from_rgb(248, 249, 250), // Gris muy claro
        surface_tertiary: Color::from_rgb(233, 236, 239), // Gris claro
        surface_inverse: Color::from_rgb(33, 37, 41), // Casi negro (inverso)
        surface_inverse_secondary: Color::from_rgb(52, 58, 64), // Gris oscuro
        surface_inverse_tertiary: Color::from_rgb(73, 80, 87), // Gris medio-oscuro
        border: Color::from_rgb(206, 212, 218),     // Gris borde estándar
        border_focus: Color::from_rgb(41, 98, 255), // Mismo que primary
        border_disabled: Color::from_rgb(233, 236, 239), // Gris muy claro
        text_primary: Color::from_rgb(33, 37, 41),  // Casi negro
        text_secondary: Color::from_rgb(108, 117, 125), // Gris
        text_placeholder: Color::from_rgb(173, 181, 189), // Gris claro
        text_inverse: Color::from_rgb(248, 249, 250), // Blanco roto (sobre fondo oscuro)
        text_highlight: Color::from_rgb(41, 98, 255), // Mismo que primary
        focus: Color::from_af32rgb(0.4, 41, 98, 255), // Nota: RGBA (con alpha)
        active: Color::from_rgb(41, 98, 255),       // Mismo que primary
        disabled: Color::from_rgb(233, 236, 239),   // Gris fondo disabled
        overlay: Color::from_af32rgb(0.5, 0, 0, 0), // RGBA negro semitransparente
        shadow: Color::from_af32rgb(0.1, 0, 0, 0),  // RGBA negro con baja opacidad
    };
    tema_base
}

fn tema_premium_claro() -> Theme {
    let mut tema_base = light_theme();
    tema_base.name = "claro";
    // tema_base.name = "premium_claro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(24, 24, 27), // Negro elegante para botones/acentos
        secondary: Color::from_rgb(113, 113, 122),
        tertiary: Color::from_rgb(126, 34, 206),
        success: Color::from_rgb(21, 128, 61),
        warning: Color::from_rgb(161, 98, 7),
        error: Color::from_rgb(185, 28, 28),
        info: Color::from_rgb(29, 78, 216),
        background: Color::from_rgb(255, 255, 255), // Blanco puro
        surface_primary: Color::from_rgb(250, 250, 250), // Gris blanquecino sutil
        surface_secondary: Color::from_rgb(244, 244, 245),
        surface_tertiary: Color::from_rgb(228, 228, 231),
        surface_inverse: Color::from_rgb(9, 9, 11),
        surface_inverse_secondary: Color::from_rgb(24, 24, 27),
        surface_inverse_tertiary: Color::from_rgb(39, 39, 42),
        border: Color::from_rgb(228, 228, 231), // Bordes muy suaves
        border_focus: Color::from_rgb(24, 24, 27),
        border_disabled: Color::from_rgb(244, 244, 245),
        text_primary: Color::from_rgb(9, 9, 11), // Texto ultra legible
        text_secondary: Color::from_rgb(82, 82, 91),
        text_placeholder: Color::from_rgb(161, 161, 170),
        text_inverse: Color::from_rgb(250, 250, 250),
        text_highlight: Color::from_rgb(24, 24, 27),
        focus: Color::from_af32rgb(0.2, 24, 24, 27),
        active: Color::from_rgb(63, 63, 70),
        disabled: Color::from_rgb(244, 244, 245),
        overlay: Color::from_af32rgb(0.3, 0, 0, 0),
        shadow: Color::from_af32rgb(0.05, 0, 0, 0), // Sombra casi invisible
    };
    tema_base
}

fn tema_pastel_claro() -> Theme {
    let mut tema_base = light_theme();
    tema_base.name = "claro";
    // tema_base.name = "pastel_claro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(167, 139, 250), // Lavanda pastel activo
        secondary: Color::from_rgb(156, 163, 175), // Gris suave
        tertiary: Color::from_rgb(244, 143, 177), // Rosa pastel
        success: Color::from_rgb(167, 243, 208), // Menta / Verde pastel
        warning: Color::from_rgb(253, 230, 138), // Amarillo pastel
        error: Color::from_rgb(254, 202, 202),   // Coral / Rojo pastel
        info: Color::from_rgb(191, 219, 254),    // Azul cielo pastel
        background: Color::from_rgb(250, 249, 252), // Blanco con un toque lavanda muy sutil
        surface_primary: Color::from_rgb(255, 255, 255), // Blanco puro para tarjetas
        surface_secondary: Color::from_rgb(243, 241, 247), // Gris-lavanda claro
        surface_tertiary: Color::from_rgb(233, 230, 239),
        surface_inverse: Color::from_rgb(46, 41, 51), // Morado oscuro inverso
        surface_inverse_secondary: Color::from_rgb(61, 55, 68),
        surface_inverse_tertiary: Color::from_rgb(80, 72, 89),
        border: Color::from_rgb(226, 221, 235), // Bordes muy sutiles
        border_focus: Color::from_rgb(167, 139, 250),
        border_disabled: Color::from_rgb(243, 241, 247),
        text_primary: Color::from_rgb(59, 53, 65), // Gris oscuro purpúreo (más suave que el negro)
        text_secondary: Color::from_rgb(120, 113, 128), // Gris medio
        text_placeholder: Color::from_rgb(175, 168, 182),
        text_inverse: Color::from_rgb(250, 249, 252),
        text_highlight: Color::from_rgb(167, 139, 250),
        focus: Color::from_af32rgb(0.3, 167, 139, 250),
        active: Color::from_rgb(139, 92, 246), // Un punto más oscuro para el click
        disabled: Color::from_rgb(243, 241, 247),
        overlay: Color::from_af32rgb(0.4, 46, 41, 51),
        shadow: Color::from_af32rgb(0.06, 167, 139, 250), // Sombra con un tinte de color
    };
    tema_base
}

fn tema_cyber_claro() -> Theme {
    let mut tema_base = light_theme();
    tema_base.name = "claro";
    // tema_base.name = "cyber_claro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(0, 184, 212), // Cian ultra brillante
        secondary: Color::from_rgb(100, 116, 139),
        tertiary: Color::from_rgb(213, 0, 249), // Neón Magenta
        success: Color::from_rgb(0, 200, 83),   // Verde lima
        warning: Color::from_rgb(255, 214, 0),  // Amarillo puro
        error: Color::from_rgb(255, 23, 68),    // Rojo vibrante
        info: Color::from_rgb(41, 121, 255),
        background: Color::from_rgb(241, 245, 249), // Gris Slate claro (estilo Tailwind)
        surface_primary: Color::from_rgb(255, 255, 255),
        surface_secondary: Color::from_rgb(226, 232, 240), // Gris medio
        surface_tertiary: Color::from_rgb(203, 213, 225),
        surface_inverse: Color::from_rgb(15, 23, 42), // Slate 900 (Azul/Negro muy oscuro)
        surface_inverse_secondary: Color::from_rgb(30, 41, 59),
        surface_inverse_tertiary: Color::from_rgb(51, 65, 85),
        border: Color::from_rgb(148, 163, 184), // Bordes marcados, estilo cómic/brutalista
        border_focus: Color::from_rgb(0, 184, 212),
        border_disabled: Color::from_rgb(226, 232, 240),
        text_primary: Color::from_rgb(15, 23, 42), // Texto casi negro puro
        text_secondary: Color::from_rgb(71, 85, 105),
        text_placeholder: Color::from_rgb(148, 163, 184),
        text_inverse: Color::from_rgb(241, 245, 249),
        text_highlight: Color::from_rgb(0, 184, 212),
        focus: Color::from_af32rgb(0.4, 0, 184, 212),
        active: Color::from_rgb(0, 151, 167),
        disabled: Color::from_rgb(226, 232, 240),
        overlay: Color::from_af32rgb(0.5, 15, 23, 42),
        shadow: Color::from_af32rgb(0.15, 15, 23, 42), // Sombras bien definidas
    };
    tema_base
}

fn tema_naturaleza_claro() -> Theme {
    let mut tema_base = light_theme();
    tema_base.name = "claro";
    // tema_base.name = "naturaleza_claro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(40, 91, 64), // Verde bosque elegante
        secondary: Color::from_rgb(130, 120, 110), // Gris tierra
        tertiary: Color::from_rgb(194, 119, 83), // Terracota / Óxido
        success: Color::from_rgb(46, 125, 50),
        warning: Color::from_rgb(245, 158, 11),
        error: Color::from_rgb(198, 40, 40),
        info: Color::from_rgb(0, 131, 143),
        background: Color::from_rgb(247, 246, 242), // Blanco cálido / Arena
        surface_primary: Color::from_rgb(255, 255, 255),
        surface_secondary: Color::from_rgb(239, 237, 230), // Arena más oscuro
        surface_tertiary: Color::from_rgb(228, 224, 213),
        surface_inverse: Color::from_rgb(43, 40, 38), // Marrón ceniza muy oscuro
        surface_inverse_secondary: Color::from_rgb(61, 57, 54),
        surface_inverse_tertiary: Color::from_rgb(84, 79, 75),
        border: Color::from_rgb(213, 207, 193),
        border_focus: Color::from_rgb(40, 91, 64),
        border_disabled: Color::from_rgb(239, 237, 230),
        text_primary: Color::from_rgb(43, 40, 38), // Marrón oscuro en vez de negro
        text_secondary: Color::from_rgb(108, 100, 95),
        text_placeholder: Color::from_rgb(166, 156, 149),
        text_inverse: Color::from_rgb(247, 246, 242),
        text_highlight: Color::from_rgb(40, 91, 64),
        focus: Color::from_af32rgb(0.3, 40, 91, 64),
        active: Color::from_rgb(27, 66, 45),
        disabled: Color::from_rgb(239, 237, 230),
        overlay: Color::from_af32rgb(0.4, 43, 40, 38),
        shadow: Color::from_af32rgb(0.08, 43, 40, 38),
    };
    tema_base
}

fn tema_retro_claro() -> Theme {
    let mut tema_base = light_theme();
    tema_base.name = "claro";
    // tema_base.name = "retro_claro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(217, 119, 6), // Ámbar tostado
        secondary: Color::from_rgb(115, 115, 115),
        tertiary: Color::from_rgb(13, 148, 136), // Teal oscuro
        success: Color::from_rgb(22, 163, 74),
        warning: Color::from_rgb(234, 179, 8),
        error: Color::from_rgb(220, 38, 38),
        info: Color::from_rgb(37, 99, 235),
        background: Color::from_rgb(253, 251, 247), // Color papel / marfil calientito
        surface_primary: Color::from_rgb(255, 255, 255),
        surface_secondary: Color::from_rgb(244, 241, 234), // Crema medio
        surface_tertiary: Color::from_rgb(234, 229, 219),
        surface_inverse: Color::from_rgb(64, 61, 57), // Carbón inverso
        surface_inverse_secondary: Color::from_rgb(87, 83, 78),
        surface_inverse_tertiary: Color::from_rgb(120, 113, 108),
        border: Color::from_rgb(221, 214, 203),
        border_focus: Color::from_rgb(217, 119, 6),
        border_disabled: Color::from_rgb(244, 241, 234),
        text_primary: Color::from_rgb(67, 56, 202), // Un toque de azul tinta muy oscuro para el texto
        text_secondary: Color::from_rgb(120, 113, 108), // Café grisáceo
        text_placeholder: Color::from_rgb(168, 162, 158),
        text_inverse: Color::from_rgb(253, 251, 247),
        text_highlight: Color::from_rgb(217, 119, 6),
        focus: Color::from_af32rgb(0.3, 217, 119, 6),
        active: Color::from_rgb(180, 83, 9),
        disabled: Color::from_rgb(244, 241, 234),
        overlay: Color::from_af32rgb(0.4, 64, 61, 57),
        shadow: Color::from_af32rgb(0.07, 64, 61, 57),
    };
    tema_base
}
