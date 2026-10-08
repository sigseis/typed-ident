// =============================================================================
// USES
// =============================================================================

// -----------------------------------------------------------------------------
use crate::Cli;
use crate::generated::case_mapping::{LOWER, TITLE, UPPER};
use crate::generated::general_category::TITLECASE_LETTER;
use crate::generated::properties::{LOWERCASE, UPPERCASE};
use anyhow::{Result, anyhow};
use core::cmp::Ordering;
use std::collections::HashMap;

// =============================================================================
// CONSTANTS
// =============================================================================

// -----------------------------------------------------------------------------
const KNOWN_ODDITIES: &[&[char]] = &[
    // Uppercase Greek symbols.
    //
    // These contain no lowercase mapping - so they stay uppercase, even when
    // you call .to_lowercase() on them.
    &['ϒ', 'ϓ', 'ϔ'],
    // Uppercase mathematical letters, letter-like symbols, and symbols.
    //
    // These contain no lowercase mapping - so they stay uppercase, even when
    // you call .to_lowercase() on them.
    &[
        'ℂ', 'ℇ', 'ℋ', 'ℌ', 'ℍ', 'ℐ', 'ℑ', 'ℒ', 'ℕ', 'ℙ', 'ℚ', 'ℛ', 'ℜ', 'ℝ', 'ℤ', 'ℨ', 'ℬ', 'ℭ',
        'ℰ', 'ℱ', 'ℳ', 'ℾ', 'ℿ', 'ⅅ', '𝐀', '𝐁', '𝐂', '𝐃', '𝐄', '𝐅', '𝐆', '𝐇', '𝐈', '𝐉', '𝐊', '𝐋',
        '𝐌', '𝐍', '𝐎', '𝐏', '𝐐', '𝐑', '𝐒', '𝐓', '𝐔', '𝐕', '𝐖', '𝐗', '𝐘', '𝐙', '𝐴', '𝐵', '𝐶', '𝐷',
        '𝐸', '𝐹', '𝐺', '𝐻', '𝐼', '𝐽', '𝐾', '𝐿', '𝑀', '𝑁', '𝑂', '𝑃', '𝑄', '𝑅', '𝑆', '𝑇', '𝑈', '𝑉',
        '𝑊', '𝑋', '𝑌', '𝑍', '𝑨', '𝑩', '𝑪', '𝑫', '𝑬', '𝑭', '𝑮', '𝑯', '𝑰', '𝑱', '𝑲', '𝑳', '𝑴', '𝑵',
        '𝑶', '𝑷', '𝑸', '𝑹', '𝑺', '𝑻', '𝑼', '𝑽', '𝑾', '𝑿', '𝒀', '𝒁', '𝒜', '𝒞', '𝒟', '𝒢', '𝒥', '𝒦',
        '𝒩', '𝒪', '𝒫', '𝒬', '𝒮', '𝒯', '𝒰', '𝒱', '𝒲', '𝒳', '𝒴', '𝒵', '𝓐', '𝓑', '𝓒', '𝓓', '𝓔', '𝓕',
        '𝓖', '𝓗', '𝓘', '𝓙', '𝓚', '𝓛', '𝓜', '𝓝', '𝓞', '𝓟', '𝓠', '𝓡', '𝓢', '𝓣', '𝓤', '𝓥', '𝓦', '𝓧',
        '𝓨', '𝓩', '𝔄', '𝔅', '𝔇', '𝔈', '𝔉', '𝔊', '𝔍', '𝔎', '𝔏', '𝔐', '𝔑', '𝔒', '𝔓', '𝔔', '𝔖', '𝔗',
        '𝔘', '𝔙', '𝔚', '𝔛', '𝔜', '𝔸', '𝔹', '𝔻', '𝔼', '𝔽', '𝔾', '𝕀', '𝕁', '𝕂', '𝕃', '𝕄', '𝕆', '𝕊',
        '𝕋', '𝕌', '𝕍', '𝕎', '𝕏', '𝕐', '𝕬', '𝕭', '𝕮', '𝕯', '𝕰', '𝕱', '𝕲', '𝕳', '𝕴', '𝕵', '𝕶', '𝕷',
        '𝕸', '𝕹', '𝕺', '𝕻', '𝕼', '𝕽', '𝕾', '𝕿', '𝖀', '𝖁', '𝖂', '𝖃', '𝖄', '𝖅', '𝖠', '𝖡', '𝖢', '𝖣',
        '𝖤', '𝖥', '𝖦', '𝖧', '𝖨', '𝖩', '𝖪', '𝖫', '𝖬', '𝖭', '𝖮', '𝖯', '𝖰', '𝖱', '𝖲', '𝖳', '𝖴', '𝖵',
        '𝖶', '𝖷', '𝖸', '𝖹', '𝗔', '𝗕', '𝗖', '𝗗', '𝗘', '𝗙', '𝗚', '𝗛', '𝗜', '𝗝', '𝗞', '𝗟', '𝗠', '𝗡',
        '𝗢', '𝗣', '𝗤', '𝗥', '𝗦', '𝗧', '𝗨', '𝗩', '𝗪', '𝗫', '𝗬', '𝗭', '𝘈', '𝘉', '𝘊', '𝘋', '𝘌', '𝘍',
        '𝘎', '𝘏', '𝘐', '𝘑', '𝘒', '𝘓', '𝘔', '𝘕', '𝘖', '𝘗', '𝘘', '𝘙', '𝘚', '𝘛', '𝘜', '𝘝', '𝘞', '𝘟',
        '𝘠', '𝘡', '𝘼', '𝘽', '𝘾', '𝘿', '𝙀', '𝙁', '𝙂', '𝙃', '𝙄', '𝙅', '𝙆', '𝙇', '𝙈', '𝙉', '𝙊', '𝙋',
        '𝙌', '𝙍', '𝙎', '𝙏', '𝙐', '𝙑', '𝙒', '𝙓', '𝙔', '𝙕', '𝙰', '𝙱', '𝙲', '𝙳', '𝙴', '𝙵', '𝙶', '𝙷',
        '𝙸', '𝙹', '𝙺', '𝙻', '𝙼', '𝙽', '𝙾', '𝙿', '𝚀', '𝚁', '𝚂', '𝚃', '𝚄', '𝚅', '𝚆', '𝚇', '𝚈', '𝚉',
        '𝚨', '𝚩', '𝚪', '𝚫', '𝚬', '𝚭', '𝚮', '𝚯', '𝚰', '𝚱', '𝚲', '𝚳', '𝚴', '𝚵', '𝚶', '𝚷', '𝚸', '𝚹',
        '𝚺', '𝚻', '𝚼', '𝚽', '𝚾', '𝚿', '𝛀', '𝛢', '𝛣', '𝛤', '𝛥', '𝛦', '𝛧', '𝛨', '𝛩', '𝛪', '𝛫', '𝛬',
        '𝛭', '𝛮', '𝛯', '𝛰', '𝛱', '𝛲', '𝛳', '𝛴', '𝛵', '𝛶', '𝛷', '𝛸', '𝛹', '𝛺', '𝜜', '𝜝', '𝜞', '𝜟',
        '𝜠', '𝜡', '𝜢', '𝜣', '𝜤', '𝜥', '𝜦', '𝜧', '𝜨', '𝜩', '𝜪', '𝜫', '𝜬', '𝜭', '𝜮', '𝜯', '𝜰', '𝜱',
        '𝜲', '𝜳', '𝜴', '𝝖', '𝝗', '𝝘', '𝝙', '𝝚', '𝝛', '𝝜', '𝝝', '𝝞', '𝝟', '𝝠', '𝝡', '𝝢', '𝝣', '𝝤',
        '𝝥', '𝝦', '𝝧', '𝝨', '𝝩', '𝝪', '𝝫', '𝝬', '𝝭', '𝝮', '𝞐', '𝞑', '𝞒', '𝞓', '𝞔', '𝞕', '𝞖', '𝞗',
        '𝞘', '𝞙', '𝞚', '𝞛', '𝞜', '𝞝', '𝞞', '𝞟', '𝞠', '𝞡', '𝞢', '𝞣', '𝞤', '𝞥', '𝞦', '𝞧', '𝞨', '𝟊',
    ],
    // Uppercase enclosed label-like characters (oft used as symbols/labels).
    //
    // These contain no lowercase mapping - so they stay uppercase, even when
    // you call .to_lowercase() on them.
    &[
        '🄰', '🄱', '🄲', '🄳', '🄴', '🄵', '🄶', '🄷', '🄸', '🄹', '🄺', '🄻', '🄼', '🄽', '🄾', '🄿', '🅀', '🅁',
        '🅂', '🅃', '🅄', '🅅', '🅆', '🅇', '🅈', '🅉', '🅐', '🅑', '🅒', '🅓', '🅔', '🅕', '🅖', '🅗', '🅘', '🅙',
        '🅚', '🅛', '🅜', '🅝', '🅞', '🅟', '🅠', '🅡', '🅢', '🅣', '🅤', '🅥', '🅦', '🅧', '🅨', '🅩', '🅰', '🅱',
        '🅲', '🅳', '🅴', '🅵', '🅶', '🅷', '🅸', '🅹', '🅺', '🅻', '🅼', '🅽', '🅾', '🅿', '🆀', '🆁', '🆂', '🆃',
        '🆄', '🆅', '🆆', '🆇', '🆈', '🆉',
    ],
    // I have no idea about these characters - but I'll place them here for
    // reference. Looks like a lot of random letters, symbols, phonetic
    // characters, subscripts, superscripts, etc.
    //
    // These are lowercase, but they do not change when you call .to_uppercase()
    // on them. So they stay lowercase through case conversions.
    &[
        'ª',
        'º',
        'ĸ',
        'ƍ',
        'ƪ',
        'ƫ',
        'ƺ',
        'ƾ',
        'ȡ',
        'ȴ',
        'ȵ',
        'ȶ',
        'ȷ',
        'ȸ',
        'ȹ',
        'ɕ',
        'ɘ',
        'ɚ',
        'ɝ',
        'ɞ',
        'ɟ',
        'ɢ',
        'ɧ',
        'ɭ',
        'ɮ',
        'ɰ',
        'ɳ',
        'ɴ',
        'ɶ',
        'ɸ',
        'ɹ',
        'ɺ',
        'ɻ',
        'ɾ',
        'ɿ',
        'ʁ',
        'ʄ',
        'ʅ',
        'ʆ',
        'ʍ',
        'ʎ',
        'ʏ',
        'ʐ',
        'ʑ',
        'ʓ',
        'ʖ',
        'ʗ',
        'ʘ',
        'ʙ',
        'ʚ',
        'ʛ',
        'ʜ',
        'ʟ',
        'ʠ',
        'ʡ',
        'ʢ',
        'ʣ',
        'ʤ',
        'ʥ',
        'ʦ',
        'ʧ',
        'ʨ',
        'ʩ',
        'ʪ',
        'ʫ',
        'ʬ',
        'ʭ',
        'ʮ',
        'ʯ',
        'ʰ',
        'ʱ',
        'ʲ',
        'ʳ',
        'ʴ',
        'ʵ',
        'ʶ',
        'ʷ',
        'ʸ',
        'ˀ',
        'ˁ',
        'ˠ',
        'ˡ',
        'ˢ',
        'ˣ',
        'ˤ',
        'ͺ',
        'ϼ',
        '\u{558}',
        'ՠ',
        'ֈ',
        '\u{58b}',
        '\u{58c}',
        'ჼ',
        'ᴀ',
        'ᴁ',
        'ᴂ',
        'ᴃ',
        'ᴄ',
        'ᴅ',
        'ᴆ',
        'ᴇ',
        'ᴈ',
        'ᴉ',
        'ᴊ',
        'ᴋ',
        'ᴌ',
        'ᴍ',
        'ᴎ',
        'ᴏ',
        'ᴐ',
        'ᴑ',
        'ᴒ',
        'ᴓ',
        'ᴔ',
        'ᴕ',
        'ᴖ',
        'ᴗ',
        'ᴘ',
        'ᴙ',
        'ᴚ',
        'ᴛ',
        'ᴜ',
        'ᴝ',
        'ᴞ',
        'ᴟ',
        'ᴠ',
        'ᴡ',
        'ᴢ',
        'ᴣ',
        'ᴤ',
        'ᴥ',
        'ᴦ',
        'ᴧ',
        'ᴨ',
        'ᴩ',
        'ᴪ',
        'ᴫ',
        'ᴬ',
        'ᴭ',
        'ᴮ',
        'ᴯ',
        'ᴰ',
        'ᴱ',
        'ᴲ',
        'ᴳ',
        'ᴴ',
        'ᴵ',
        'ᴶ',
        'ᴷ',
        'ᴸ',
        'ᴹ',
        'ᴺ',
        'ᴻ',
        'ᴼ',
        'ᴽ',
        'ᴾ',
        'ᴿ',
        'ᵀ',
        'ᵁ',
        'ᵂ',
        'ᵃ',
        'ᵄ',
        'ᵅ',
        'ᵆ',
        'ᵇ',
        'ᵈ',
        'ᵉ',
        'ᵊ',
        'ᵋ',
        'ᵌ',
        'ᵍ',
        'ᵎ',
        'ᵏ',
        'ᵐ',
        'ᵑ',
        'ᵒ',
        'ᵓ',
        'ᵔ',
        'ᵕ',
        'ᵖ',
        'ᵗ',
        'ᵘ',
        'ᵙ',
        'ᵚ',
        'ᵛ',
        'ᵜ',
        'ᵝ',
        'ᵞ',
        'ᵟ',
        'ᵠ',
        'ᵡ',
        'ᵢ',
        'ᵣ',
        'ᵤ',
        'ᵥ',
        'ᵦ',
        'ᵧ',
        'ᵨ',
        'ᵩ',
        'ᵪ',
        'ᵫ',
        'ᵬ',
        'ᵭ',
        'ᵮ',
        'ᵯ',
        'ᵰ',
        'ᵱ',
        'ᵲ',
        'ᵳ',
        'ᵴ',
        'ᵵ',
        'ᵶ',
        'ᵷ',
        'ᵸ',
        'ᵺ',
        'ᵻ',
        'ᵼ',
        'ᵾ',
        'ᵿ',
        'ᶀ',
        'ᶁ',
        'ᶂ',
        'ᶃ',
        'ᶄ',
        'ᶅ',
        'ᶆ',
        'ᶇ',
        'ᶈ',
        'ᶉ',
        'ᶊ',
        'ᶋ',
        'ᶌ',
        'ᶍ',
        'ᶏ',
        'ᶐ',
        'ᶑ',
        'ᶒ',
        'ᶓ',
        'ᶔ',
        'ᶕ',
        'ᶖ',
        'ᶗ',
        'ᶘ',
        'ᶙ',
        'ᶚ',
        'ᶛ',
        'ᶜ',
        'ᶝ',
        'ᶞ',
        'ᶟ',
        'ᶠ',
        'ᶡ',
        'ᶢ',
        'ᶣ',
        'ᶤ',
        'ᶥ',
        'ᶦ',
        'ᶧ',
        'ᶨ',
        'ᶩ',
        'ᶪ',
        'ᶫ',
        'ᶬ',
        'ᶭ',
        'ᶮ',
        'ᶯ',
        'ᶰ',
        'ᶱ',
        'ᶲ',
        'ᶳ',
        'ᶴ',
        'ᶵ',
        'ᶶ',
        'ᶷ',
        'ᶸ',
        'ᶹ',
        'ᶺ',
        'ᶻ',
        'ᶼ',
        'ᶽ',
        'ᶾ',
        'ᶿ',
        'ẜ',
        'ẝ',
        'ẟ',
        'ⁱ',
        'ⁿ',
        'ₐ',
        'ₑ',
        'ₒ',
        'ₓ',
        'ₔ',
        'ₕ',
        'ₖ',
        'ₗ',
        'ₘ',
        'ₙ',
        'ₚ',
        'ₛ',
        'ₜ',
        '\u{209d}',
        '\u{209e}',
        '\u{209f}',
        'ℊ',
        'ℎ',
        'ℏ',
        'ℓ',
        'ℯ',
        'ℴ',
        'ℹ',
        'ℼ',
        'ℽ',
        'ⅆ',
        'ⅇ',
        'ⅈ',
        'ⅉ',
        'ⱱ',
        'ⱴ',
        'ⱷ',
        'ⱸ',
        'ⱹ',
        'ⱺ',
        'ⱻ',
        'ⱼ',
        'ⱽ',
        'ⳤ',
        'ꚜ',
        'ꚝ',
        'ꜰ',
        'ꜱ',
        'ꝰ',
        'ꝱ',
        'ꝲ',
        'ꝳ',
        'ꝴ',
        'ꝵ',
        'ꝶ',
        'ꝷ',
        'ꝸ',
        'ꞎ',
        'ꞕ',
        'ꞯ',
        '꟱',
        'ꟲ',
        'ꟳ',
        'ꟴ',
        'ꟸ',
        'ꟹ',
        'ꟺ',
        'ꬰ',
        'ꬱ',
        'ꬲ',
        'ꬳ',
        'ꬴ',
        'ꬵ',
        'ꬶ',
        'ꬷ',
        'ꬸ',
        'ꬹ',
        'ꬺ',
        'ꬻ',
        'ꬼ',
        'ꬽ',
        'ꬾ',
        'ꬿ',
        'ꭀ',
        'ꭁ',
        'ꭂ',
        'ꭃ',
        'ꭄ',
        'ꭅ',
        'ꭆ',
        'ꭇ',
        'ꭈ',
        'ꭉ',
        'ꭊ',
        'ꭍ',
        'ꭎ',
        'ꭏ',
        'ꭐ',
        'ꭑ',
        'ꭒ',
        'ꭔ',
        'ꭕ',
        'ꭖ',
        'ꭗ',
        'ꭘ',
        'ꭙ',
        'ꭚ',
        'ꭜ',
        'ꭝ',
        'ꭞ',
        'ꭟ',
        'ꭠ',
        'ꭡ',
        'ꭢ',
        'ꭣ',
        'ꭤ',
        'ꭥ',
        'ꭦ',
        'ꭧ',
        'ꭨ',
        'ꭩ',
        '𐞀',
        '𐞃',
        '𐞄',
        '𐞅',
        '𐞇',
        '𐞈',
        '𐞉',
        '𐞊',
        '𐞋',
        '𐞌',
        '𐞍',
        '𐞎',
        '𐞏',
        '𐞐',
        '𐞑',
        '𐞒',
        '𐞓',
        '𐞔',
        '𐞕',
        '𐞖',
        '𐞗',
        '𐞘',
        '𐞙',
        '𐞚',
        '𐞛',
        '𐞜',
        '𐞝',
        '𐞞',
        '𐞟',
        '𐞠',
        '𐞡',
        '𐞢',
        '𐞣',
        '𐞤',
        '𐞥',
        '𐞦',
        '𐞧',
        '𐞨',
        '𐞩',
        '𐞪',
        '𐞫',
        '𐞬',
        '𐞭',
        '𐞮',
        '𐞯',
        '𐞰',
        '𐞲',
        '𐞳',
        '𐞴',
        '𐞵',
        '𐞶',
        '𐞷',
        '𐞸',
        '𐞹',
        '𐞺',
        '\u{107bb}',
        '\u{107bc}',
        '\u{107bd}',
        '\u{107be}',
        '\u{107bf}',
        '𝐚',
        '𝐛',
        '𝐜',
        '𝐝',
        '𝐞',
        '𝐟',
        '𝐠',
        '𝐡',
        '𝐢',
        '𝐣',
        '𝐤',
        '𝐥',
        '𝐦',
        '𝐧',
        '𝐨',
        '𝐩',
        '𝐪',
        '𝐫',
        '𝐬',
        '𝐭',
        '𝐮',
        '𝐯',
        '𝐰',
        '𝐱',
        '𝐲',
        '𝐳',
        '𝑎',
        '𝑏',
        '𝑐',
        '𝑑',
        '𝑒',
        '𝑓',
        '𝑔',
        '𝑖',
        '𝑗',
        '𝑘',
        '𝑙',
        '𝑚',
        '𝑛',
        '𝑜',
        '𝑝',
        '𝑞',
        '𝑟',
        '𝑠',
        '𝑡',
        '𝑢',
        '𝑣',
        '𝑤',
        '𝑥',
        '𝑦',
        '𝑧',
        '𝒂',
        '𝒃',
        '𝒄',
        '𝒅',
        '𝒆',
        '𝒇',
        '𝒈',
        '𝒉',
        '𝒊',
        '𝒋',
        '𝒌',
        '𝒍',
        '𝒎',
        '𝒏',
        '𝒐',
        '𝒑',
        '𝒒',
        '𝒓',
        '𝒔',
        '𝒕',
        '𝒖',
        '𝒗',
        '𝒘',
        '𝒙',
        '𝒚',
        '𝒛',
        '𝒶',
        '𝒷',
        '𝒸',
        '𝒹',
        '𝒻',
        '𝒽',
        '𝒾',
        '𝒿',
        '𝓀',
        '𝓁',
        '𝓂',
        '𝓃',
        '𝓅',
        '𝓆',
        '𝓇',
        '𝓈',
        '𝓉',
        '𝓊',
        '𝓋',
        '𝓌',
        '𝓍',
        '𝓎',
        '𝓏',
        '𝓪',
        '𝓫',
        '𝓬',
        '𝓭',
        '𝓮',
        '𝓯',
        '𝓰',
        '𝓱',
        '𝓲',
        '𝓳',
        '𝓴',
        '𝓵',
        '𝓶',
        '𝓷',
        '𝓸',
        '𝓹',
        '𝓺',
        '𝓻',
        '𝓼',
        '𝓽',
        '𝓾',
        '𝓿',
        '𝔀',
        '𝔁',
        '𝔂',
        '𝔃',
        '𝔞',
        '𝔟',
        '𝔠',
        '𝔡',
        '𝔢',
        '𝔣',
        '𝔤',
        '𝔥',
        '𝔦',
        '𝔧',
        '𝔨',
        '𝔩',
        '𝔪',
        '𝔫',
        '𝔬',
        '𝔭',
        '𝔮',
        '𝔯',
        '𝔰',
        '𝔱',
        '𝔲',
        '𝔳',
        '𝔴',
        '𝔵',
        '𝔶',
        '𝔷',
        '𝕒',
        '𝕓',
        '𝕔',
        '𝕕',
        '𝕖',
        '𝕗',
        '𝕘',
        '𝕙',
        '𝕚',
        '𝕛',
        '𝕜',
        '𝕝',
        '𝕞',
        '𝕟',
        '𝕠',
        '𝕡',
        '𝕢',
        '𝕣',
        '𝕤',
        '𝕥',
        '𝕦',
        '𝕧',
        '𝕨',
        '𝕩',
        '𝕪',
        '𝕫',
        '𝖆',
        '𝖇',
        '𝖈',
        '𝖉',
        '𝖊',
        '𝖋',
        '𝖌',
        '𝖍',
        '𝖎',
        '𝖏',
        '𝖐',
        '𝖑',
        '𝖒',
        '𝖓',
        '𝖔',
        '𝖕',
        '𝖖',
        '𝖗',
        '𝖘',
        '𝖙',
        '𝖚',
        '𝖛',
        '𝖜',
        '𝖝',
        '𝖞',
        '𝖟',
        '𝖺',
        '𝖻',
        '𝖼',
        '𝖽',
        '𝖾',
        '𝖿',
        '𝗀',
        '𝗁',
        '𝗂',
        '𝗃',
        '𝗄',
        '𝗅',
        '𝗆',
        '𝗇',
        '𝗈',
        '𝗉',
        '𝗊',
        '𝗋',
        '𝗌',
        '𝗍',
        '𝗎',
        '𝗏',
        '𝗐',
        '𝗑',
        '𝗒',
        '𝗓',
        '𝗮',
        '𝗯',
        '𝗰',
        '𝗱',
        '𝗲',
        '𝗳',
        '𝗴',
        '𝗵',
        '𝗶',
        '𝗷',
        '𝗸',
        '𝗹',
        '𝗺',
        '𝗻',
        '𝗼',
        '𝗽',
        '𝗾',
        '𝗿',
        '𝘀',
        '𝘁',
        '𝘂',
        '𝘃',
        '𝘄',
        '𝘅',
        '𝘆',
        '𝘇',
        '𝘢',
        '𝘣',
        '𝘤',
        '𝘥',
        '𝘦',
        '𝘧',
        '𝘨',
        '𝘩',
        '𝘪',
        '𝘫',
        '𝘬',
        '𝘭',
        '𝘮',
        '𝘯',
        '𝘰',
        '𝘱',
        '𝘲',
        '𝘳',
        '𝘴',
        '𝘵',
        '𝘶',
        '𝘷',
        '𝘸',
        '𝘹',
        '𝘺',
        '𝘻',
        '𝙖',
        '𝙗',
        '𝙘',
        '𝙙',
        '𝙚',
        '𝙛',
        '𝙜',
        '𝙝',
        '𝙞',
        '𝙟',
        '𝙠',
        '𝙡',
        '𝙢',
        '𝙣',
        '𝙤',
        '𝙥',
        '𝙦',
        '𝙧',
        '𝙨',
        '𝙩',
        '𝙪',
        '𝙫',
        '𝙬',
        '𝙭',
        '𝙮',
        '𝙯',
        '𝚊',
        '𝚋',
        '𝚌',
        '𝚍',
        '𝚎',
        '𝚏',
        '𝚐',
        '𝚑',
        '𝚒',
        '𝚓',
        '𝚔',
        '𝚕',
        '𝚖',
        '𝚗',
        '𝚘',
        '𝚙',
        '𝚚',
        '𝚛',
        '𝚜',
        '𝚝',
        '𝚞',
        '𝚟',
        '𝚠',
        '𝚡',
        '𝚢',
        '𝚣',
        '𝚤',
        '𝚥',
        '\u{1d6a6}',
        '𝛂',
        '𝛃',
        '𝛄',
        '𝛅',
        '𝛆',
        '𝛇',
        '𝛈',
        '𝛉',
        '𝛊',
        '𝛋',
        '𝛌',
        '𝛍',
        '𝛎',
        '𝛏',
        '𝛐',
        '𝛑',
        '𝛒',
        '𝛓',
        '𝛔',
        '𝛕',
        '𝛖',
        '𝛗',
        '𝛘',
        '𝛙',
        '𝛚',
        '𝛜',
        '𝛝',
        '𝛞',
        '𝛟',
        '𝛠',
        '𝛡',
        '𝛼',
        '𝛽',
        '𝛾',
        '𝛿',
        '𝜀',
        '𝜁',
        '𝜂',
        '𝜃',
        '𝜄',
        '𝜅',
        '𝜆',
        '𝜇',
        '𝜈',
        '𝜉',
        '𝜊',
        '𝜋',
        '𝜌',
        '𝜍',
        '𝜎',
        '𝜏',
        '𝜐',
        '𝜑',
        '𝜒',
        '𝜓',
        '𝜔',
        '𝜖',
        '𝜗',
        '𝜘',
        '𝜙',
        '𝜚',
        '𝜛',
        '𝜶',
        '𝜷',
        '𝜸',
        '𝜹',
        '𝜺',
        '𝜻',
        '𝜼',
        '𝜽',
        '𝜾',
        '𝜿',
        '𝝀',
        '𝝁',
        '𝝂',
        '𝝃',
        '𝝄',
        '𝝅',
        '𝝆',
        '𝝇',
        '𝝈',
        '𝝉',
        '𝝊',
        '𝝋',
        '𝝌',
        '𝝍',
        '𝝎',
        '𝝐',
        '𝝑',
        '𝝒',
        '𝝓',
        '𝝔',
        '𝝕',
        '𝝰',
        '𝝱',
        '𝝲',
        '𝝳',
        '𝝴',
        '𝝵',
        '𝝶',
        '𝝷',
        '𝝸',
        '𝝹',
        '𝝺',
        '𝝻',
        '𝝼',
        '𝝽',
        '𝝾',
        '𝝿',
        '𝞀',
        '𝞁',
        '𝞂',
        '𝞃',
        '𝞄',
        '𝞅',
        '𝞆',
        '𝞇',
        '𝞈',
        '𝞊',
        '𝞋',
        '𝞌',
        '𝞍',
        '𝞎',
        '𝞏',
        '𝞪',
        '𝞫',
        '𝞬',
        '𝞭',
        '𝞮',
        '𝞯',
        '𝞰',
        '𝞱',
        '𝞲',
        '𝞳',
        '𝞴',
        '𝞵',
        '𝞶',
        '𝞷',
        '𝞸',
        '𝞹',
        '𝞺',
        '𝞻',
        '𝞼',
        '𝞽',
        '𝞾',
        '𝞿',
        '𝟀',
        '𝟁',
        '𝟂',
        '𝟄',
        '𝟅',
        '𝟆',
        '𝟇',
        '𝟈',
        '𝟉',
        '𝟋',
        '𝼀',
        '𝼁',
        '𝼂',
        '𝼃',
        '𝼄',
        '𝼅',
        '𝼆',
        '𝼇',
        '𝼈',
        '𝼉',
        '𝼋',
        '𝼌',
        '𝼍',
        '𝼎',
        '𝼏',
        '𝼐',
        '𝼑',
        '𝼒',
        '𝼓',
        '𝼔',
        '𝼕',
        '𝼖',
        '𝼗',
        '𝼘',
        '𝼙',
        '𝼚',
        '𝼛',
        '𝼜',
        '𝼝',
        '𝼞',
        '\u{1df1f}',
        '\u{1df20}',
        '\u{1df21}',
        '\u{1df22}',
        '\u{1df23}',
        '\u{1df24}',
        '𝼥',
        '𝼦',
        '𝼧',
        '𝼨',
        '𝼩',
        '𝼪',
        '\u{1df2b}',
        '\u{1df2c}',
        '\u{1df2d}',
        '\u{1df2e}',
        '\u{1df2f}',
        '\u{1df30}',
        '\u{1df31}',
        '\u{1df32}',
        '\u{1df33}',
        '\u{1df34}',
        '\u{1df35}',
        '\u{1df36}',
        '\u{1df37}',
        '\u{1df38}',
        '\u{1df39}',
        '\u{1df3a}',
        '\u{1df3b}',
        '\u{1df3c}',
        '\u{1df3d}',
        '\u{1df3e}',
        '\u{1df3f}',
        '\u{1df42}',
        '\u{1df43}',
        '\u{1df44}',
        '\u{1df45}',
        '\u{1df46}',
        '\u{1df47}',
        '\u{1df4c}',
        '\u{1df4f}',
        '\u{1df50}',
        '\u{1df53}',
        '\u{1df54}',
        '\u{1df55}',
        '\u{1df56}',
        '\u{1df57}',
        '\u{1df58}',
        '\u{1df59}',
        '\u{1df5a}',
        '\u{1df5b}',
        '\u{1df5c}',
        '\u{1df5d}',
        '\u{1df5e}',
        '\u{1df5f}',
        '\u{1df60}',
        '\u{1df61}',
        '\u{1df62}',
        '\u{1df63}',
        '\u{1df64}',
        '\u{1df65}',
        '\u{1df66}',
        '\u{1df67}',
        '\u{1df70}',
        '\u{1df71}',
        '\u{1df90}',
        '\u{1df91}',
        '\u{1df92}',
        '\u{1df93}',
        '\u{1df94}',
        '\u{1df96}',
        '\u{1dfcd}',
        '\u{1dfce}',
        '\u{1dfcf}',
        '\u{1dfd0}',
        '\u{1dfd1}',
        '\u{1dfd2}',
        '\u{1dfd3}',
        '\u{1dfd4}',
        '\u{1dfd5}',
        '\u{1dfd6}',
        '\u{1dfd7}',
        '\u{1dfd8}',
        '\u{1dfd9}',
        '\u{1dfda}',
        '\u{1dfdb}',
        '\u{1dfdc}',
        '\u{1dfdd}',
        '\u{1dfde}',
        '\u{1dfdf}',
        '\u{1dfe0}',
        '\u{1dfe1}',
        '\u{1dfe2}',
        '\u{1dfe3}',
        '\u{1dfe4}',
        '\u{1dfe5}',
        '\u{1dfe6}',
        '\u{1dfe7}',
        '\u{1dfe8}',
        '\u{1dfe9}',
        '\u{1dfea}',
        '\u{1dfeb}',
        '\u{1dfec}',
        '\u{1dfed}',
        '\u{1dfee}',
        '\u{1dfef}',
        '\u{1dff0}',
        '\u{1dff1}',
        '\u{1dff2}',
        '\u{1dff3}',
        '\u{1dff4}',
        '\u{1dff5}',
        '\u{1dff6}',
        '\u{1dff7}',
        '\u{1dff8}',
        '\u{1dff9}',
        '\u{1dffa}',
        '\u{1dffb}',
        '\u{1dffc}',
        '\u{1dffd}',
        '\u{1dffe}',
        '\u{1dfff}',
        '𞀰',
        '𞀱',
        '𞀲',
        '𞀳',
        '𞀴',
        '𞀵',
        '𞀶',
        '𞀷',
        '𞀸',
        '𞀹',
        '𞀺',
        '𞀻',
        '𞀼',
        '𞀽',
        '𞀾',
        '𞀿',
        '𞁀',
        '𞁁',
        '𞁂',
        '𞁃',
        '𞁄',
        '𞁅',
        '𞁆',
        '𞁇',
        '𞁈',
        '𞁉',
        '𞁊',
        '𞁋',
        '𞁌',
        '𞁍',
        '𞁎',
        '𞁏',
        '𞁐',
        '𞁑',
        '𞁒',
        '𞁓',
        '𞁔',
        '𞁕',
        '𞁖',
        '𞁗',
        '𞁘',
        '𞁙',
        '𞁚',
        '𞁛',
        '𞁜',
        '𞁝',
        '𞁞',
        '𞁟',
        '𞁠',
        '𞁡',
        '𞁢',
        '𞁣',
        '𞁤',
        '𞁥',
        '𞁦',
        '𞁧',
        '𞁨',
        '𞁩',
        '𞁪',
        '𞁫',
        '𞁬',
        '𞁭',
    ],
];

// =============================================================================
// HELPERS
// =============================================================================

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Case {
    Lowercase,
    Titlecase,
    Uncased,
    Uppercase,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Char {
    value: char,
    std: CharProperties,
    ucd: CharProperties,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CharProperties {
    is_lowercase: bool,
    is_titlecase: bool,
    is_uppercase: bool,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum MappedCase {
    Lowercase,
    Mixedcase,
    Titlecase,
    Uncased,
    Uppercase,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CaseMappings {
    lowercase: Option<MappedCase>,
    titlecase: Option<MappedCase>,
    uppercase: Option<MappedCase>,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Default)]
struct CasePairings<'a> {
    lower: CasePairingTargets<'a>,
    title: CasePairingTargets<'a>,
    uncased: CasePairingTargets<'a>,
    upper: CasePairingTargets<'a>,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Default)]
struct CasePairingTargets<'a> {
    to_lower: CasePairingTargetMappings<'a>,
    to_title: CasePairingTargetMappings<'a>,
    to_upper: CasePairingTargetMappings<'a>,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Debug, Default)]
struct CasePairingTargetMappings<'a> {
    mapped: HashMap<MappedCase, Vec<&'a Char>>,
    unmapped: Vec<&'a Char>,
}

// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Mappings {
    lowercase: Option<&'static [u32]>,
    titlecase: Option<&'static [u32]>,
    uppercase: Option<&'static [u32]>,
}

// -----------------------------------------------------------------------------
fn in_range(c: char, table: &[(u32, u32)]) -> bool {
    let c = c as u32;
    table
        .binary_search_by(|&(low, high)| {
            if low > c {
                Ordering::Greater
            } else if high < c {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
        .is_ok()
}

// -----------------------------------------------------------------------------
fn get_derived_case(c: char) -> Case {
    // Note: There's no derived titlecase property - use the general category.
    match c {
        c if in_range(c, LOWERCASE) => Case::Lowercase,
        c if in_range(c, TITLECASE_LETTER) => Case::Titlecase,
        c if in_range(c, UPPERCASE) => Case::Uppercase,
        _ => Case::Uncased,
    }
}

// -----------------------------------------------------------------------------
fn get_mapping<'a>(c: char, table: &[(u32, &'a [u32])]) -> Option<&'a [u32]> {
    let c = c as u32;
    let idx = table.binary_search_by_key(&c, |(k, _)| *k).ok()?;
    Some(table[idx].1)
}

// -----------------------------------------------------------------------------
fn determine_mapped_case(mapping: &[u32]) -> MappedCase {
    let mut iter = mapping
        .iter()
        .map(|c| get_derived_case(char::from_u32(*c).unwrap()))
        .filter(|c| !matches!(c, Case::Uncased));
    let case = iter.next().unwrap();
    if iter.any(|c| c != case) {
        return MappedCase::Mixedcase;
    }
    match case {
        Case::Lowercase => MappedCase::Lowercase,
        Case::Titlecase => MappedCase::Titlecase,
        Case::Uncased => MappedCase::Uncased,
        Case::Uppercase => MappedCase::Uppercase,
    }
}

// -----------------------------------------------------------------------------
impl CharProperties {
    fn case(&self) -> Case {
        if self.is_lowercase {
            Case::Lowercase
        } else if self.is_titlecase {
            Case::Titlecase
        } else if self.is_uppercase {
            Case::Uppercase
        } else {
            Case::Uncased
        }
    }
}

// -----------------------------------------------------------------------------
fn collect_valid_chars() -> Result<Vec<Char>> {
    // Compute all of the characters that we wish to analyze.
    let mut invalid_chars = Vec::default();
    let mut valid_chars = Vec::default();
    for c in char::MIN..=char::MAX {
        let c = Char {
            value: c,
            std: CharProperties {
                is_lowercase: c.is_lowercase(),
                is_titlecase: in_range(c, TITLECASE_LETTER),
                is_uppercase: c.is_uppercase(),
            },
            ucd: CharProperties {
                is_lowercase: in_range(c, LOWERCASE),
                is_titlecase: in_range(c, TITLECASE_LETTER),
                is_uppercase: in_range(c, UPPERCASE),
            },
        };
        if c.std.is_lowercase && c.std.is_uppercase {
            invalid_chars.push(c);
        } else {
            valid_chars.push(c);
        }
    }

    // We actually don't expect there to be any invalid characters. If there are
    // then we probably need to make some drastic changes to the crate.
    if !invalid_chars.is_empty() {
        return Err(anyhow!(
            "invalid characters found in UCD: {invalid_chars:#?}"
        ));
    }

    Ok(valid_chars)
}

// -----------------------------------------------------------------------------
fn compute_case_pairings(chars: &[Char]) -> Result<CasePairings<'_>> {
    let mut pairings = CasePairings::default();
    // Go over all the valid characters and inspect their case pairing
    // properties (whether or not they can change case as expected).
    for c in chars {
        // Check to see if the character can map to any other cases...
        let mappings = Mappings {
            lowercase: get_mapping(c.value, LOWER),
            titlecase: get_mapping(c.value, TITLE),
            uppercase: get_mapping(c.value, UPPER),
        };

        // If it's cased, check the mapping to see if it changes case...
        let case_mappings = CaseMappings {
            lowercase: mappings.lowercase.map(determine_mapped_case),
            titlecase: mappings.titlecase.map(determine_mapped_case),
            uppercase: mappings.uppercase.map(determine_mapped_case),
        };

        // Check the mappings for all of the characters based on the UCD case.
        let pairings = match c.ucd.case() {
            Case::Lowercase => &mut pairings.lower,
            Case::Titlecase => &mut pairings.title,
            Case::Uncased => &mut pairings.uncased,
            Case::Uppercase => &mut pairings.upper,
        };
        if let Some(mapped) = case_mappings.lowercase {
            pairings.to_lower.mapped.entry(mapped).or_default().push(c);
        } else {
            pairings.to_lower.unmapped.push(c);
        }
        if let Some(mapped) = case_mappings.titlecase {
            pairings.to_title.mapped.entry(mapped).or_default().push(c);
        } else {
            pairings.to_title.unmapped.push(c);
        }
        if let Some(mapped) = case_mappings.uppercase {
            pairings.to_upper.mapped.entry(mapped).or_default().push(c);
        } else {
            pairings.to_upper.unmapped.push(c);
        }
    }

    Ok(pairings)
}

// -----------------------------------------------------------------------------
fn print_summary(chars: &[Char]) {
    println!("Total Characters: {}", chars.len());
    println!(
        "  Total Cased Characters: {}",
        chars
            .iter()
            .filter(|c| c.ucd.case() != Case::Uncased)
            .count()
    );
    println!(
        "    Lowercase: Std({}), Ucd({})",
        chars
            .iter()
            .filter(|c| c.std.case() == Case::Lowercase)
            .count(),
        chars
            .iter()
            .filter(|c| c.ucd.case() == Case::Lowercase)
            .count()
    );
    println!(
        "    Titlecase: Std({}), Ucd({})",
        chars
            .iter()
            .filter(|c| c.std.case() == Case::Titlecase)
            .count(),
        chars
            .iter()
            .filter(|c| c.ucd.case() == Case::Titlecase)
            .count()
    );
    println!(
        "    Uppercase: Std({}), Ucd({})",
        chars
            .iter()
            .filter(|c| c.std.case() == Case::Uppercase)
            .count(),
        chars
            .iter()
            .filter(|c| c.ucd.case() == Case::Uppercase)
            .count()
    );
    println!(
        "  Total Uncased Characters: {}",
        chars
            .iter()
            .filter(|c| c.ucd.case() == Case::Uncased)
            .count()
    );
    println!();
}

// -----------------------------------------------------------------------------
fn print_case_pairings_section(section: &str, mappings: &CasePairingTargetMappings) {
    println!(
        "    {section}: Lower({}), Mixed({}), Title({}), Uncased({}), Upper({}), Unmapped({})",
        mappings
            .mapped
            .get(&MappedCase::Lowercase)
            .map(|v| v.len())
            .unwrap_or_default(),
        mappings
            .mapped
            .get(&MappedCase::Mixedcase)
            .map(|v| v.len())
            .unwrap_or_default(),
        mappings
            .mapped
            .get(&MappedCase::Titlecase)
            .map(|v| v.len())
            .unwrap_or_default(),
        mappings
            .mapped
            .get(&MappedCase::Uncased)
            .map(|v| v.len())
            .unwrap_or_default(),
        mappings
            .mapped
            .get(&MappedCase::Uppercase)
            .map(|v| v.len())
            .unwrap_or_default(),
        mappings.unmapped.len(),
    );
}

// -----------------------------------------------------------------------------
fn print_case_pairing_targets(section: &str, targets: &CasePairingTargets) {
    println!("  {section} Pairings:");
    print_case_pairings_section("ToLowercase", &targets.to_lower);
    print_case_pairings_section("ToTitlecase", &targets.to_title);
    print_case_pairings_section("ToUppercase", &targets.to_upper);
}

// -----------------------------------------------------------------------------
fn print_case_pairings(pairings: &CasePairings) {
    println!("Case Pairing Data:");
    print_case_pairing_targets("Lowercase", &pairings.lower);
    print_case_pairing_targets("Titlecase", &pairings.title);
    print_case_pairing_targets("Uncased", &pairings.uncased);
    print_case_pairing_targets("Uppercase", &pairings.upper);
    println!()
}

// -----------------------------------------------------------------------------
fn print_unknown_pairing_data(section: &str, chars: &[&Char], mapping: &[(u32, &[u32])]) {
    let unknown: Vec<char> = chars
        .iter()
        .map(|c| c.value)
        .filter(|c| !KNOWN_ODDITIES.iter().any(|o| o.contains(&c)))
        .collect();
    println!("      {section}: {unknown:?}");
    for c in chars {
        if KNOWN_ODDITIES.iter().any(|o| o.contains(&c.value)) {
            continue;
        }
        let mapping = get_mapping(c.value, mapping).unwrap();
        let mapped: String = mapping
            .iter()
            .map(|c| char::from_u32(*c).unwrap())
            .collect();
        println!(
            "        '{character}' ({value:05X}; {value}) maps to {mapped} ({mapping:?})",
            character = c.value,
            value = c.value as u32,
        );
    }
}

// -----------------------------------------------------------------------------
fn print_unknown_pairings_section(
    section: &str,
    mappings: &CasePairingTargetMappings,
    mapping: &[(u32, &[u32])],
    source_case: Case,
    target_case: Case,
) {
    println!("    {section}:");
    if target_case != Case::Lowercase {
        print_unknown_pairing_data(
            "Lowercase",
            mappings
                .mapped
                .get(&MappedCase::Lowercase)
                .map(Vec::as_slice)
                .unwrap_or_default(),
            mapping,
        );
    }
    print_unknown_pairing_data(
        "Mixedcase",
        mappings
            .mapped
            .get(&MappedCase::Mixedcase)
            .map(Vec::as_slice)
            .unwrap_or_default(),
        mapping,
    );
    if target_case != Case::Titlecase {
        print_unknown_pairing_data(
            "Titlecase",
            mappings
                .mapped
                .get(&MappedCase::Titlecase)
                .map(Vec::as_slice)
                .unwrap_or_default(),
            mapping,
        );
    }
    if source_case != target_case && source_case != Case::Uncased {
        let unknown: Vec<char> = mappings
            .unmapped
            .iter()
            .map(|c| c.value)
            .filter(|c| !KNOWN_ODDITIES.iter().any(|o| o.contains(&c)))
            .collect();
        println!("      Unmapped: {unknown:?}");
        for c in &mappings.unmapped {
            if KNOWN_ODDITIES.iter().any(|o| o.contains(&c.value)) {
                continue;
            }
            println!(
                "        '{character}' ({value:05X}; {value})",
                character = c.value,
                value = c.value as u32
            );
        }
    }
    print_unknown_pairing_data(
        "Uncased",
        mappings
            .mapped
            .get(&MappedCase::Uncased)
            .map(Vec::as_slice)
            .unwrap_or_default(),
        mapping,
    );
    if target_case != Case::Uppercase {
        print_unknown_pairing_data(
            "Uppercase",
            mappings
                .mapped
                .get(&MappedCase::Uppercase)
                .map(Vec::as_slice)
                .unwrap_or_default(),
            mapping,
        );
    }
}

// -----------------------------------------------------------------------------
fn print_unknown_pairing_targets(section: &str, targets: &CasePairingTargets, case: Case) {
    println!("  {section} Pairings:");
    print_unknown_pairings_section(
        "ToLowercase",
        &targets.to_lower,
        LOWER,
        case,
        Case::Lowercase,
    );
    print_unknown_pairings_section(
        "ToUppercase",
        &targets.to_upper,
        UPPER,
        case,
        Case::Uppercase,
    );
}

// -----------------------------------------------------------------------------
fn print_unknown_pairings(pairings: &CasePairings) {
    println!("Exploring Unknown Pairings: (See analyze_tables.rs for a list of KNOWN_ODDITIES)");
    print_unknown_pairing_targets("Lowercase", &pairings.lower, Case::Lowercase);
    print_unknown_pairing_targets("Titlecase", &pairings.title, Case::Titlecase);
    print_unknown_pairing_targets("Uncased", &pairings.uncased, Case::Uncased);
    print_unknown_pairing_targets("Uppercase", &pairings.upper, Case::Uppercase);
}

// =============================================================================
// COMMAND
// =============================================================================

// -----------------------------------------------------------------------------
pub fn analyze_tables(_cli: &Cli) -> Result<()> {
    let chars = collect_valid_chars()?;
    let pairings = compute_case_pairings(&chars)?;
    print_summary(&chars);
    print_case_pairings(&pairings);
    print_unknown_pairings(&pairings);
    Ok(())
}
