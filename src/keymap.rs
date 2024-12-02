use rmk::action::KeyAction;
use rmk::{a, k, layer, mo as m};
pub(crate) const COL: usize = 23;
pub(crate) const ROW: usize = 7;
pub(crate) const NUM_LAYER: usize = 1;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    let ___ = k!(A);

    let k03 = k!(A);
    let k04 = k!(A);
    let k05 = k!(A);
    let k06 = k!(A);
    let k07 = k!(A);
    let k08 = k!(A);
    let k09 = k!(A);
    let k0a = k!(A);
    let k0b = k!(A);
    let k0c = k!(A);
    let k0d = k!(A);
    let k0e = k!(A);

    let k23 = k!(A);
    let k24 = k!(A);
    let k25 = k!(A);
    let k26 = k!(A);
    let k27 = k!(A);
    let k28 = k!(A);
    let k29 = k!(A);
    let k2a = k!(A);
    let k2b = k!(A);
    let k2c = k!(A);
    let k2d = k!(A);
    let k2e = k!(A);

    let k40 = k!(A);
    let k41 = k!(A);
    let k42 = k!(A);
    let k43 = k!(A);
    let k44 = k!(A);
    let k45 = k!(A);
    let k46 = k!(A);
    let k47 = k!(A);
    let k48 = k!(A);
    let k49 = k!(A);
    let k4a = k!(A);
    let k4b = k!(A);
    let k4c = k!(A);
    let k4d = k!(A);
    let k4e = k!(A);
    let k4f = k!(A);
    let k50 = k!(A);
    let k51 = k!(A);
    let k52 = k!(A);
    let k53 = k!(A);
    let k54 = k!(A);
    let k55 = k!(A);
    let k56 = k!(A);

    let k60 = k!(A);
    let k61 = k!(A);
    let k62 = k!(A);
    let k63 = k!(A);
    let k64 = k!(A);
    let k65 = k!(A);
    let k66 = k!(A);
    let k67 = k!(A);
    let k68 = k!(A);
    let k69 = k!(A);
    let k6a = k!(A);
    let k6b = k!(A);
    let k6c = k!(A);
    let k6d = k!(A);
    let k6e = k!(A);
    let k70 = k!(A);
    let k71 = k!(A);
    let k72 = k!(A);
    let k73 = k!(A);
    let k74 = k!(A);
    let k75 = k!(A);
    let k76 = k!(A);

    let k80 = k!(A);
    let k81 = k!(A);
    let k82 = k!(A);
    let k83 = k!(A);
    let k84 = k!(A);
    let k85 = k!(A);
    let k86 = k!(A);
    let k87 = k!(A);
    let k88 = k!(A);
    let k89 = k!(A);
    let k8a = k!(A);
    let k8b = k!(A);
    let k8c = k!(A);
    let k8d = k!(A);
    let k8e = k!(A);
    let k8f = k!(A);
    let k91 = k!(A);
    let k93 = k!(A);
    let k94 = k!(A);
    let k95 = k!(A);
    let k96 = k!(A);

    let ka0 = k!(A);
    let ka1 = k!(A);
    let ka2 = k!(A);
    let ka3 = k!(A);
    let ka4 = k!(A);
    let ka5 = k!(A);
    let ka6 = k!(A);
    let ka7 = k!(A);
    let ka8 = k!(A);
    let ka9 = k!(A);
    let kaa = k!(A);
    let kab = k!(A);
    let kac = k!(A);
    let kad = k!(A);
    let kaf = k!(A);
    let kb0 = k!(A);
    let kb1 = k!(A);
    let kb2 = k!(A);
    let kb3 = k!(A);
    let kb4 = k!(A);
    let kb5 = k!(A);
    let kb6 = k!(A);

    let kc0 = k!(A);
    let kc1 = k!(A);
    let kc2 = k!(A);
    let kc4 = k!(A);
    let kc9 = k!(A);
    let kcd = k!(A);
    let kcf = k!(A);
    let kd1 = k!(A);
    let kd4 = k!(A);
    let kd5 = k!(A);

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
