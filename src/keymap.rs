use rmk::action::KeyAction;
use rmk::{a, k, layer, mo as m};
pub(crate) const COL: usize = 23;
pub(crate) const ROW: usize = 7;
pub(crate) const NUM_LAYER: usize = 1;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    let ___ = k!(A);

    let k03 = k!(F13);
    let k04 = k!(F14);
    let k05 = k!(F15);
    let k06 = k!(F16);
    let k07 = k!(F17);
    let k08 = k!(F18);
    let k09 = k!(F19);
    let k0a = k!(F20);
    let k0b = k!(F21);
    let k0c = k!(F22);
    let k0d = k!(F23);
    let k0e = k!(F24);

    let k23 = k!(F1);
    let k24 = k!(F2);
    let k25 = k!(F3);
    let k26 = k!(F4);
    let k27 = k!(F5);
    let k28 = k!(F6);
    let k29 = k!(F7);
    let k2a = k!(F8);
    let k2b = k!(F9);
    let k2c = k!(F10);
    let k2d = k!(F11);
    let k2e = k!(F12);

    let k40 = k!(Escape);
    let k41 = k!(RGui);
    let k42 = k!(Grave);
    let k43 = k!(Kb1);
    let k44 = k!(Kb2);
    let k45 = k!(Kb3);
    let k46 = k!(Kb4);
    let k47 = k!(Kb5);
    let k48 = k!(Kb6);
    let k49 = k!(Kb7);
    let k4a = k!(Kb8);
    let k4b = k!(Kb9);
    let k4c = k!(Kb0);
    let k4d = k!(Minus);
    let k4e = k!(Equal);
    let k4f = k!(Backspace);
    let k50 = k!(Insert);
    let k51 = k!(Home);
    let k52 = k!(PageUp);
    let k53 = k!(NumLock);
    let k54 = k!(KpSlash);
    let k55 = k!(KpAsterisk);
    let k56 = k!(KpMinus);

    let k60 = k!(International5);
    let k61 = k!(MediaPlayPause);
    let k62 = k!(Tab);
    let k63 = k!(Q);
    let k64 = k!(W);
    let k65 = k!(E);
    let k66 = k!(R);
    let k67 = k!(T);
    let k68 = k!(Y);
    let k69 = k!(U);
    let k6a = k!(I);
    let k6b = k!(O);
    let k6c = k!(P);
    let k6d = k!(LeftBracket);
    let k6e = k!(RightBracket);
    let k70 = k!(Delete);
    let k71 = k!(End);
    let k72 = k!(PageDown);
    let k73 = k!(Kp7);
    let k74 = k!(Kp8);
    let k75 = k!(Kp9);
    let k76 = k!(KpPlus);

    let k80 = k!(KbVolumeDown);
    let k81 = k!(KbVolumeUp);
    let k82 = k!(CapsLock);
    let k83 = k!(A);
    let k84 = k!(S);
    let k85 = k!(D);
    let k86 = k!(F);
    let k87 = k!(G);
    let k88 = k!(H);
    let k89 = k!(J);
    let k8a = k!(K);
    let k8b = k!(L);
    let k8c = k!(Semicolon);
    let k8d = k!(Quote);
    let k8e = k!(NonusHash);
    let k8f = k!(Return);
    let k91 = k!(UP);
    let k93 = k!(Kp4);
    let k94 = k!(Kp5);
    let k95 = k!(Kp6);
    let k96 = k!(KpDot);

    let ka0 = k!(MediaPrevTrack);
    let ka1 = k!(MediaNextTrack);
    let ka2 = k!(LShift);
    let ka3 = k!(NonusBackslash);
    let ka4 = k!(Z);
    let ka5 = k!(X);
    let ka6 = k!(C);
    let ka7 = k!(V);
    let ka8 = k!(B);
    let ka9 = k!(N);
    let kaa = k!(M);
    let kab = k!(Comma);
    let kac = k!(Dot);
    let kad = k!(Slash);
    let kaf = k!(RShift);
    let kb0 = k!(Left);
    let kb1 = k!(LCtrl);
    let kb2 = k!(Right);
    let kb3 = k!(Kp1);
    let kb4 = k!(Kp2);
    let kb5 = k!(Kp3);
    let kb6 = k!(KpEnter);

    let kc0 = k!(LGui);
    let kc1 = k!(Menu);
    let kc2 = k!(LCtrl);
    let kc4 = k!(LAlt);
    let kc9 = k!(Space);
    let kcd = k!(RAlt);
    let kcf = k!(RCtrl);
    let kd1 = k!(Down);
    let kd4 = k!(Kp0);
    let kd5 = k!(KpComma);

    [layer!([
        [___,___,  ___,k03,k04,k05,k06,k07,k08,k09,k0a,k0b,k0c,k0d,k0e,___,  ___,___,___,  ___,___,___,___],
        [___,___,  ___,k23,k24,k25,k26,k27,k28,k29,k2a,k2b,k2c,k2d,k2e,___,  ___,___,___,  ___,___,___,___],

        [k40,k41,  k42,k43,k44,k45,k46,k47,k48,k49,k4a,k4b,k4c,k4d,k4e, k4f,  k50,k51,k52,  k53,k54,k55,k56],
        [k60,k61,  k62,k63,k64,k65,k66,k67,k68,k69,k6a,k6b,k6c,k6d,k6e, ___,  k70,k71,k72,  k73,k74,k75,k76],
        [k80,k81,  k82,k83,k84,k85,k86,k87,k88,k89,k8a,k8b,k8c,k8d,k8e, k8f,  ___,k91,___,  k93,k94,k95,k96],
        [ka0,ka1,  ka2,ka3,ka4,ka5,ka6,ka7,ka8,ka9,kaa,kab,kac,kad,___, kaf,  kb0,kb1,kb2,  kb3,kb4,kb5,kb6],
        [kc0,kc1,  kc2,___,kc4,___,___,___,___,kc9,___,___,___, kcd,___,kcf,  ___,kd1,___,  ___,kd4,kd5,___]
    ]),]
}
