use console::Color;

// TODO: Use once lock and determine colour based on distribution
#[inline(always)]
pub fn primary() -> Color {
    Color::Blue
}

pub static ASCII_ART_UBUNTU: [&str; 20] = [
    "            .-/+oossssoo+/-.            ",
    "        `:+ssssssssssssssssss+:`        ",
    "      -+ssssssssssssssssssyyssss+-      ",
    "    .ossssssssssssssssssdMMMNysssso.    ",
    "   /ssssssssssshdmmNNmmyNMMMMhssssss/   ",
    "  +ssssssssshmydMMMMMMMNddddyssssssss+  ",
    " /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/ ",
    ".ssssssssdMMMNhsssssssssshNMMMdssssssss.",
    "+sssshhhyNMMNyssssssssssssyNMMMysssssss+",
    "ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
    "ossyNMMMNyMMhsssssssssssssshmmmhssssssso",
    "+sssshhhyNMMNyssssssssssssyNMMMysssssss+",
    ".ssssssssdMMMNhsssssssssshNMMMdssssssss.",
    " /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/ ",
    "  +sssssssssdmydMMMMMMMMddddyssssssss+  ",
    "   /ssssssssssshdmNNNNmyNMMMMhssssss/   ",
    "    .ossssssssssssssssssdMMMNysssso.    ",
    "      -+sssssssssssssssssyyyssss+-      ",
    "        `:+ssssssssssssssssss+:`        ",
    "            .-/+oossssoo+/-.            ",
];
pub const ASCII_ART_UBUNTU_FILLER: &str = "                                        ";
