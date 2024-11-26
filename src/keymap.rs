use rmk::action::KeyAction;
use rmk::{a, k, layer, mo as m};
pub(crate) const COL: usize = 23;
pub(crate) const ROW: usize = 7;
pub(crate) const NUM_LAYER: usize = 1;

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    let ___ = k!(A);
    let k08 = k!(A);
    let k10 = k!(A);
    let k18 = k!(A);
    let k20 = k!(A);
    let k28 = k!(A);
    let k30 = k!(A);
    let k38 = k!(A);
    let k40 = k!(A);
    let k48 = k!(A);
    let k50 = k!(A);
    let k57 = k!(A);
    let k5f = k!(A);

    let k07 = k!(A);
    let k0f = k!(A);
    let k17 = k!(A);
    let k1f = k!(A);
    let k27 = k!(A);
    let k2f = k!(A);
    let k37 = k!(A);
    let k3f = k!(A);
    let k47 = k!(A);
    let k4f = k!(A);
    let k56 = k!(A);
    let k5e = k!(A);

    let k05 = k!(A);
    let k06 = k!(A);
    let k0e = k!(A);
    let k16 = k!(A);
    let k1e = k!(A);
    let k26 = k!(A);
    let k25 = k!(A);
    let k2e = k!(A);
    let k36 = k!(A);
    let k3d = k!(A);
    let k3e = k!(A);
    let k46 = k!(A);
    let k45 = k!(A);
    let k4e = k!(A);
    let k55 = k!(A);
    let k66 = k!(A);
    let k67 = k!(A);
    let k6e = k!(A);
    let k6f = k!(A);
    let k76 = k!(A);
    let k77 = k!(A);
    let k7e = k!(A);
    let k84 = k!(A);

    let k04 = k!(A);
    let k0c = k!(A);
    let k0d = k!(A);
    let k15 = k!(A);
    let k1d = k!(A);
    let k24 = k!(A);
    let k2d = k!(A);
    let k2c = k!(A);
    let k35 = k!(A);
    let k3c = k!(A);
    let k43 = k!(A);
    let k44 = k!(A);
    let k4d = k!(A);
    let k54 = k!(A);
    let k5b = k!(A);
    let k5c = k!(A);
    let k64 = k!(A);
    let k65 = k!(A);
    let k6d = k!(A);
    let k6c = k!(A);
    let k75 = k!(A);
    let k7d = k!(A);
    let k7c = k!(A);

    let k03 = k!(A);
    let k0b = k!(A);
    let k14 = k!(A);
    let k1c = k!(A);
    let k1b = k!(A);
    let k23 = k!(A);
    let k2b = k!(A);
    let k34 = k!(A);
    let k33 = k!(A);
    let k3b = k!(A);
    let k42 = k!(A);
    let k4b = k!(A);
    let k4c = k!(A);
    let k52 = k!(A);
    let k63 = k!(A);
    let k6b = k!(A);
    let k73 = k!(A);
    let k74 = k!(A);
    let k7b = k!(A);

    let k83 = k!(A);
    let k0a = k!(A);
    let k12 = k!(A);
    let k13 = k!(A);
    let k1a = k!(A);
    let k22 = k!(A);
    let k21 = k!(A);
    let k2a = k!(A);
    let k32 = k!(A);
    let k31 = k!(A);
    let k3a = k!(A);
    let k41 = k!(A);
    let k49 = k!(A);
    let k4a = k!(A);
    let k51 = k!(A);
    let k61 = k!(A);
    let k62 = k!(A);
    let k6a = k!(A);
    let k69 = k!(A);
    let k72 = k!(A);
    let k7a = k!(A);
    let k79 = k!(A);

    let k01 = k!(A);
    let k09 = k!(A);
    let k11 = k!(A);
    let k19 = k!(A);
    let k29 = k!(A);
    let k39 = k!(A);
    let k58 = k!(A);
    let k60 = k!(A);
    let k68 = k!(A);
    let k70 = k!(A);
    let k71 = k!(A);
    let k78 = k!(A);

    [layer!([
        [___,___,  ___,k08,k10,k18,k20,k28,k30,k38,k40,k48,k50,k57,k5f,    ___,  ___,___,___,  ___,___,___,___],
        [___,___,  ___,k07,k0f,k17,k1f,k27,k2f,k37,k3f,k47,k4f,k56,k5e,    ___,  ___,___,___,  ___,___,___,___],

        [k05,k06,  k0e,k16,k1e,k26,k25,k2e,k36,k3d,k3e,k46,k45,k4e,k55,  k66,    k67,k6e,k6f,  k76,k77,k7e,k84],
        [k04,k0c,  k0d,k15,k1d,k24,k2d,k2c,k35,k3c,k43,k44,k4d,k54,k5b,  k5c,    k64,k65,k6d,  k6c,k75,k7d,k7c],
        [k03,k0b,  k14,k1c,k1b,k23,k2b,k34,k33,k3b,k42,k4b,k4c,k52,___,  ___,    ___,k63,___,  k6b,k73,k74,k7b],
        [k83,k0a,  k12,k13,k1a,k22,k21,k2a,k32,k31,k3a,k41,k49,k4a,___,  k51,    k61,k62,k6a,  k69,k72,k7a,k79],
        [k01,k09,  k11,___,k19,___,___,___,k29    ,___,___,___,___,k39,___,k58,  ___,k60,___,  k68,k70,k71,k78]
    ]),]
}
