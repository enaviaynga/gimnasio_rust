use freya::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TemasOscuros {
    Generico,
    Nord,
    Premium,
    Cozy,
    Espacio,
    Matrix,
}

pub fn tema_oscuro(tema_oscuro: State<TemasOscuros>) -> Theme {
    match *tema_oscuro.read() {
        TemasOscuros::Generico => oscuro_generico(),
        TemasOscuros::Nord => tema_nord_oscuro(),
        TemasOscuros::Premium => tema_premium_oscuro(),
        TemasOscuros::Cozy => tema_cozy_oscuro(),
        TemasOscuros::Espacio => tema_espacio_oscuro(),
        TemasOscuros::Matrix => tema_matrix_oscuro(),
    }
}

fn oscuro_generico() -> Theme {
    let mut tema_base = dark_theme();
    tema_base.name = "oscuro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(97, 144, 255), // Azul más suave para oscuro
        secondary: Color::from_rgb(139, 148, 158), // Gris azulado claro
        tertiary: Color::from_rgb(178, 127, 255), // Púrpura más brillante
        success: Color::from_rgb(46, 160, 100), // Verde
        warning: Color::from_rgb(255, 185, 15), // Ámbar
        error: Color::from_rgb(255, 85, 85),    // Rojo
        info: Color::from_rgb(42, 195, 235),    // Cyan
        background: Color::from_rgb(13, 17, 23), // Negro azulado muy oscuro
        surface_primary: Color::from_rgb(22, 27, 34), // Gris oscuro
        surface_secondary: Color::from_rgb(30, 36, 44), // Gris medio-oscuro
        surface_tertiary: Color::from_rgb(38, 45, 54), // Gris menos oscuro
        surface_inverse: Color::from_rgb(241, 243, 245), // Blanco humo (inverso)
        surface_inverse_secondary: Color::from_rgb(222, 226, 230), // Gris muy claro
        surface_inverse_tertiary: Color::from_rgb(206, 212, 218), // Gris claro
        border: Color::from_rgb(48, 54, 61),    // Borde oscuro
        border_focus: Color::from_rgb(97, 144, 255), // Mismo que primary
        border_disabled: Color::from_rgb(38, 45, 54), // Mismo que surface_tertiary
        text_primary: Color::from_rgb(230, 237, 243), // Blanco ligeramente gris
        text_secondary: Color::from_rgb(139, 148, 158), // Gris
        text_placeholder: Color::from_rgb(88, 96, 105), // Gris oscuro
        text_inverse: Color::from_rgb(13, 17, 23), // Mismo que background
        text_highlight: Color::from_rgb(97, 144, 255), // Mismo que primary
        focus: Color::from_af32rgb(0.5, 97, 144, 255), // RGBA con alpha 0.5
        active: Color::from_rgb(78, 121, 232),  // Azul ligeramente más oscuro que primary
        disabled: Color::from_rgb(38, 45, 54),  // Mismo que surface_tertiary
        overlay: Color::from_af32rgb(0.6, 0, 0, 0), // RGBA negro semitransparente
        shadow: Color::from_af32rgb(0.3, 0, 0, 0), // RGBA negro con opacidad media
    };
    tema_base
}

fn tema_nord_oscuro() -> Theme {
    let mut tema_base = dark_theme();
    tema_base.name = "oscuro";
    // tema_base.name = "nord_oscuro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(0, 229, 255),     // Cian neón vibrante
        secondary: Color::from_rgb(143, 188, 187), // Verde azulado pastel
        tertiary: Color::from_rgb(180, 142, 173),  // Púrpura nórdico
        success: Color::from_rgb(163, 190, 140),   // Verde salvia
        warning: Color::from_rgb(235, 203, 139),   // Amarillo cálido
        error: Color::from_rgb(191, 97, 106),      // Rojo apagado pero visible
        info: Color::from_rgb(136, 192, 208),      // Azul hielo
        background: Color::from_rgb(26, 28, 36),   // Gris pizarra muy oscuro
        surface_primary: Color::from_rgb(34, 37, 49), // Gris azulado intermedio
        surface_secondary: Color::from_rgb(42, 47, 62),
        surface_tertiary: Color::from_rgb(53, 59, 78),
        surface_inverse: Color::from_rgb(240, 244, 248),
        surface_inverse_secondary: Color::from_rgb(217, 226, 236),
        surface_inverse_tertiary: Color::from_rgb(188, 204, 220),
        border: Color::from_rgb(62, 69, 91),
        border_focus: Color::from_rgb(0, 229, 255),
        border_disabled: Color::from_rgb(42, 47, 62),
        text_primary: Color::from_rgb(245, 247, 250), // Blanco limpio
        text_secondary: Color::from_rgb(160, 174, 192), // Gris azulado legible
        text_placeholder: Color::from_rgb(113, 128, 150),
        text_inverse: Color::from_rgb(26, 28, 36),
        text_highlight: Color::from_rgb(0, 229, 255),
        focus: Color::from_af32rgb(0.4, 0, 229, 255),
        active: Color::from_rgb(0, 194, 217),
        disabled: Color::from_rgb(42, 47, 62),
        overlay: Color::from_af32rgb(0.6, 10, 12, 16),
        shadow: Color::from_af32rgb(0.4, 0, 0, 0),
    };
    tema_base
}

fn tema_premium_oscuro() -> Theme {
    let mut tema_base = dark_theme();
    tema_base.name = "oscuro";
    // tema_base.name = "premium_oscuro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(255, 255, 255), // El blanco es el protagonista
        secondary: Color::from_rgb(161, 161, 170), // Gris zinc
        tertiary: Color::from_rgb(168, 85, 247), // Morado elegante
        success: Color::from_rgb(34, 197, 94),
        warning: Color::from_rgb(234, 179, 8),
        error: Color::from_rgb(239, 68, 68),
        info: Color::from_rgb(59, 130, 246),
        background: Color::from_rgb(9, 9, 11), // Negro casi absoluto (Zinc 950)
        surface_primary: Color::from_rgb(24, 24, 27), // Zinc 900
        surface_secondary: Color::from_rgb(39, 39, 42),
        surface_tertiary: Color::from_rgb(63, 63, 70),
        surface_inverse: Color::from_rgb(250, 250, 250),
        surface_inverse_secondary: Color::from_rgb(244, 244, 245),
        surface_inverse_tertiary: Color::from_rgb(228, 228, 231),
        border: Color::from_rgb(39, 39, 42), // Bordes muy finos y oscuros
        border_focus: Color::from_rgb(255, 255, 255),
        border_disabled: Color::from_rgb(24, 24, 27),
        text_primary: Color::from_rgb(250, 250, 250),
        text_secondary: Color::from_rgb(161, 161, 170),
        text_placeholder: Color::from_rgb(113, 113, 122),
        text_inverse: Color::from_rgb(9, 9, 11),
        text_highlight: Color::from_rgb(255, 255, 255),
        focus: Color::from_af32rgb(0.3, 255, 255, 255),
        active: Color::from_rgb(228, 228, 231),
        disabled: Color::from_rgb(39, 39, 42),
        overlay: Color::from_af32rgb(0.7, 0, 0, 0),
        shadow: Color::from_af32rgb(0.5, 0, 0, 0),
    };
    tema_base
}

fn tema_cozy_oscuro() -> Theme {
    let mut tema_base = dark_theme();
    tema_base.name = "oscuro";
    // tema_base.name = "cozy_oscuro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(255, 171, 145),   // Coral suave
        secondary: Color::from_rgb(210, 201, 192), // Gris cálido / Arena
        tertiary: Color::from_rgb(206, 147, 216),  // Lavanda
        success: Color::from_rgb(129, 199, 132),
        warning: Color::from_rgb(255, 213, 79),
        error: Color::from_rgb(229, 115, 115),
        info: Color::from_rgb(128, 222, 234),
        background: Color::from_rgb(32, 28, 26), // Marrón/Negro sepia
        surface_primary: Color::from_rgb(44, 38, 35), // Superficie cálida
        surface_secondary: Color::from_rgb(56, 49, 45),
        surface_tertiary: Color::from_rgb(72, 63, 58),
        surface_inverse: Color::from_rgb(247, 245, 243),
        surface_inverse_secondary: Color::from_rgb(235, 230, 225),
        surface_inverse_tertiary: Color::from_rgb(214, 206, 198),
        border: Color::from_rgb(69, 59, 54),
        border_focus: Color::from_rgb(255, 171, 145),
        border_disabled: Color::from_rgb(44, 38, 35),
        text_primary: Color::from_rgb(245, 240, 235), // Blanco hueso (no lastima los ojos)
        text_secondary: Color::from_rgb(186, 174, 166),
        text_placeholder: Color::from_rgb(138, 125, 117),
        text_inverse: Color::from_rgb(32, 28, 26),
        text_highlight: Color::from_rgb(255, 171, 145),
        focus: Color::from_af32rgb(0.4, 255, 171, 145),
        active: Color::from_rgb(244, 143, 117),
        disabled: Color::from_rgb(56, 49, 45),
        overlay: Color::from_af32rgb(0.6, 20, 15, 15),
        shadow: Color::from_af32rgb(0.3, 0, 0, 0),
    };
    tema_base
}

fn tema_espacio_oscuro() -> Theme {
    let mut tema_base = dark_theme();
    tema_base.name = "oscuro";
    // tema_base.name = "espacio_oscuro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(129, 140, 248), // Azul índigo brillante
        secondary: Color::from_rgb(148, 163, 184),
        tertiary: Color::from_rgb(232, 121, 249), // Orquídea / Fuchsia
        success: Color::from_rgb(52, 211, 153),
        warning: Color::from_rgb(251, 191, 36),
        error: Color::from_rgb(248, 113, 113),
        info: Color::from_rgb(56, 189, 248),
        background: Color::from_rgb(11, 15, 30), // Azul noche profundo (Abyss)
        surface_primary: Color::from_rgb(21, 26, 48), // Capa intermedia marina
        surface_secondary: Color::from_rgb(30, 37, 68),
        surface_tertiary: Color::from_rgb(47, 56, 95),
        surface_inverse: Color::from_rgb(241, 245, 249),
        surface_inverse_secondary: Color::from_rgb(203, 213, 225),
        surface_inverse_tertiary: Color::from_rgb(148, 163, 184),
        border: Color::from_rgb(38, 47, 84),
        border_focus: Color::from_rgb(129, 140, 248),
        border_disabled: Color::from_rgb(21, 26, 48),
        text_primary: Color::from_rgb(248, 250, 252), // Blanco gélido
        text_secondary: Color::from_rgb(148, 163, 184), // Gris espacial
        text_placeholder: Color::from_rgb(100, 116, 139),
        text_inverse: Color::from_rgb(11, 15, 30),
        text_highlight: Color::from_rgb(129, 140, 248),
        focus: Color::from_af32rgb(0.4, 129, 140, 248),
        active: Color::from_rgb(99, 102, 241),
        disabled: Color::from_rgb(30, 37, 68),
        overlay: Color::from_af32rgb(0.6, 5, 7, 16),
        shadow: Color::from_af32rgb(0.4, 0, 0, 0),
    };
    tema_base
}

fn tema_matrix_oscuro() -> Theme {
    let mut tema_base = dark_theme();
    tema_base.name = "oscuro";
    // tema_base.name = "matrix_oscuro";
    tema_base.colors = ColorsSheet {
        primary: Color::from_rgb(0, 255, 65),   // Verde fósforo clásico
        secondary: Color::from_rgb(0, 143, 17), // Verde oscuro complementario
        tertiary: Color::from_rgb(0, 200, 115), // Verde menta/hacker
        success: Color::from_rgb(0, 255, 65),
        warning: Color::from_rgb(212, 255, 0),
        error: Color::from_rgb(255, 0, 60),
        info: Color::from_rgb(0, 229, 255),
        background: Color::from_rgb(5, 5, 5), // Negro casi absoluto
        surface_primary: Color::from_rgb(14, 18, 14), // Negro con un sutil tinte verde
        surface_secondary: Color::from_rgb(24, 31, 24),
        surface_tertiary: Color::from_rgb(36, 48, 36),
        surface_inverse: Color::from_rgb(230, 245, 230),
        surface_inverse_secondary: Color::from_rgb(190, 215, 190),
        surface_inverse_tertiary: Color::from_rgb(150, 180, 150),
        border: Color::from_rgb(24, 50, 24), // Bordes verde oscuro muy sutiles
        border_focus: Color::from_rgb(0, 255, 65),
        border_disabled: Color::from_rgb(14, 18, 14),
        text_primary: Color::from_rgb(215, 255, 220), // Blanco-verde muy brillante
        text_secondary: Color::from_rgb(0, 180, 30),  // Texto secundario verde terminal
        text_placeholder: Color::from_rgb(0, 90, 15),
        text_inverse: Color::from_rgb(5, 5, 5),
        text_highlight: Color::from_rgb(0, 255, 65),
        focus: Color::from_af32rgb(0.3, 0, 255, 65),
        active: Color::from_rgb(0, 200, 45),
        disabled: Color::from_rgb(24, 31, 24),
        overlay: Color::from_af32rgb(0.7, 0, 10, 0), // Overlay con tinte verde
        shadow: Color::from_af32rgb(0.2, 0, 255, 65), // Resplandor verde en vez de sombra negra
    };
    tema_base
}
