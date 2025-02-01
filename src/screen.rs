// 10 spaces to shift the smoke grid on screen
pub const SHIFT: &str = "          ";
pub const SHIFT_MINUS_ONE: &str = "         ";
const CLEAR: &str = "\x1B[2J";
const FAKE_SMOKE: &str = r#"
                       .
                        `:.
                          `:.
                  .:'     ,::
                 .:'      ;:'
                 ::      ;:'
                  :    .:'
                   `.  :."#;
const MUG: &str = r#"          _________________________
         : _ _ _ _ _ _ _ _ _ _ _ _ :
     ,---:".".".".".".".".".".".".":
    : ,'"`::.:.:.:.:.:.:.:.:.:.:.::'
    `.`.  `:-===-===-===-===-===-:'
      `.`-._:                   :
        `-.__`.               ,'
    ,--------`"`-------------'--------.
     `"--.__                   __.--"'
            `""-------------""'
"#;


pub fn clear_screen() {
    print!("{}", CLEAR);
}

pub fn print_mug() {
    print!("{}", MUG);
}

pub fn print_fake_smoke() {
    print!("{}", FAKE_SMOKE);
}
