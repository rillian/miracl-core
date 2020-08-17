/*
 * Copyright (c) 2012-2020 MIRACL UK Ltd.
 *
 * This file is part of MIRACL Core
 * (see https://github.com/miracl/core).
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::arch::Chunk;
use crate::bls12381::big::NLEN;

// Base Bits= 29
// bls12381 Modulus

pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFAAAB, 0xFF7FFFF, 0x14FFFFEE, 0x17FFFD62, 0xF6241EA, 0x9507B58, 0xAFD9CC3, 0x109E70A2,
    0x1764774B, 0x121A5D66, 0x12C6E9ED, 0x12FFCD34, 0x111EA3, 0xD,
];
pub const ROI: [Chunk; NLEN] = [
    0x1FFFAAAA, 0xFF7FFFF, 0x14FFFFEE, 0x17FFFD62, 0xF6241EA, 0x9507B58, 0xAFD9CC3, 0x109E70A2,
    0x1764774B, 0x121A5D66, 0x12C6E9ED, 0x12FFCD34, 0x111EA3, 0xD,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x15BEF7AE, 0x1031CD0E, 0x2DD93E8, 0x9226323, 0xE6E2CD2, 0x11684DAA, 0x1170E5DB, 0x88E25B1,
    0x1B366399, 0x1C536F47, 0xD1F9CBC, 0x278B67F, 0x1EA66A2B, 0xC,
];
pub const MCONST: Chunk = 0x1FFCFFFD;
pub const SQRTM3:[Chunk;NLEN]=[0x1AAAE,0xFD80000,0xFFFFED7,0x189FAFDA,0x1C912627,0x14945F,0xBA6AF26,0xEC3ECC4,0x13EFA3BF,0x1422F081,0x33A3655,0x12FFCD33,0x111EA3,0xD];
pub const FRA: [Chunk; NLEN] = [
    0x12235FB8, 0x83BAF6C, 0x19E04F63, 0x1D4A7AC7, 0xB9C4F67, 0x1EBC25D, 0x1D3DEC91, 0x1FA797AB,
    0x1F0FD603, 0x1016068, 0x108C6FAD, 0x5760CCF, 0x104D3BF0, 0xC,
];
pub const FRB: [Chunk; NLEN] = [
    0xDDC4AF3, 0x7BC5093, 0x1B1FB08B, 0x1AB5829A, 0x3C5F282, 0x764B8FB, 0xDBFB032, 0x10F6D8F6,
    0x1854A147, 0x1118FCFD, 0x23A7A40, 0xD89C065, 0xFC3E2B3, 0x0,
];

pub const CURVE_COF_I: isize = 0;
pub const CURVE_B_I: isize = 4;
pub const CURVE_B: [Chunk; NLEN] = [
    0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1, 0x1FFFFFF8, 0x1F96FFBF, 0x1B4805FF, 0x1D80553B, 0xC0404D0, 0x1520CCE7, 0xA6533AF,
    0x73EDA7, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x1B22C6BB, 0x19D78056, 0x1E86BBFE, 0xBD07FF2, 0x1AC586C5, 0x1D1F8B8D, 0x4168538, 0x9F2EE97,
    0xFC3688C, 0x27D4D60, 0x9A558E3, 0x32FAF28, 0x1F1D3A73, 0xB,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x6C5E7E1, 0x551194A, 0x222B903, 0x198E8945, 0xB3EDD03, 0xC659602, 0xBD8036C, 0x12BABA01,
    0x4FCF5E0, 0xBA0EC57, 0x8278C3B, 0x75541E3, 0xB3F481E, 0x4,
];

pub const CURVE_BNX: [Chunk; NLEN] = [
    0x10000, 0x10080000, 0x34, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_COF: [Chunk; NLEN] = [
    0x10001,0x10080000,0x34,0x0,0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CRU: [Chunk; NLEN] = [
    0x1FFEFFFE, 0x100FFFFF, 0x280008B, 0xFB026C4, 0x9688DE1, 0x149DF37C, 0x1FAB76CE, 0xED41EE,
    0x11BA69C6, 0x1EFBB672, 0x17C659CB, 0x0, 0x0, 0x0,
];

pub const CURVE_PXA: [Chunk; NLEN] = [
    0x121BDB8, 0x402B646, 0x16EFBF5, 0x18064D50, 0x1D1770BA, 0x5B23D71, 0xC0AD144, 0x1A9F4807,
    0x11C6E47A, 0x196E2882, 0x9820149, 0x11E1522, 0x4AA2B2F, 0x1,
];
pub const CURVE_PXB: [Chunk; NLEN] = [
    0x1D042B7E, 0xD63E82A, 0x51755F9, 0x19E22427, 0x15049334, 0x10DDEE3F, 0x186AD769, 0x1A132416,
    0x5596BD0, 0x4413A7B, 0x1F6B34E8, 0x4E33EC0, 0x1E02B605, 0x9,
];
pub const CURVE_PYA: [Chunk; NLEN] = [
    0x8B82801, 0xC9AA430, 0xB28A278, 0x15939877, 0xD12C923, 0xD34A8B0, 0xE9DB50A, 0x155197BA,
    0x1AADFD9B, 0x16D171A8, 0x3327371, 0x4FADC23, 0xE5D5277, 0x6,
];
pub const CURVE_PYB: [Chunk; NLEN] = [
    0x105F79BE, 0x15483AFF, 0x1B07686A, 0xE1A4EB9, 0x99AB3F3, 0x955AB97, 0xEBC99D2, 0xFD0B4EC,
    0x19CB3E28, 0x15E145C, 0xCAB34AC, 0x1D4E6998, 0x6C4A02, 0x3,
];

pub const CURVE_AD:[Chunk;NLEN]=[0xD584C1D,0x7A14041,0x183E5FD7,0x6DF1B41,0x81AC989,0xC0D77EC,0x1AA363A2,0xA707DCC,0x2B0EA98,0x164B6A4C,0xF5A4E80,0x771D286,0x144698A,0x0];
pub const CURVE_BD:[Chunk;NLEN]=[0xE172BE0,0xE62474C,0x1B3AA974,0x642B462,0x15EF55A2,0xA7E779,0x1C282E7,0x1E1E49E8,0x1B2016C1,0x3A9F771,0x62C4BA,0x2D10060,0xE2908D1,0x9];
pub const PC:[[Chunk;NLEN];53]=[[0xBA2D229,0xE45D174,0x134E47EA,0x1637016C,0x6B68C24,0x1F8DE126,0x1EF08F02,0xFC45906,0x1D31D79D,0x1C0F6F71,0xF47A588,0x1C4C1CE1,0xE08C248,0x3],[0x1605FB7B,0x133EF9F8,0xA177B32,0x16EE3F18,0x14866F69,0x19B001D8,0x1E5B542B,0x1BBCCF0F,0xDFA7DCC,0xE92B2D8,0x1CB63B02,0x139C0FC4,0x321DA07,0x8],[0x1E390C9E,0x1920833D,0xC9DE5F,0x12165DB8,0x11B7FA31,0xA5D7A5D,0x12659D8C,0x1007418B,0x2DD2ECB,0xAE89C79,0xB830DD4,0x179F4F88,0x9B1F8E1,0xB],[0x497E317,0xB8CC354,0xDD3A55B,0x52BE52D,0x1D1DE4FA,0xB649462,0x15D28B16,0xD9CF3EA,0xDC43B75,0xB1DF4C8,0x1EE42CCD,0x134F1F88,0xD3CF1F,0x4],[0x3F0C88E,0x65AB0C7,0x1D1D6BE7,0xF91F191,0x753339B,0x3177879,0x16C69A0B,0x1564EB69,0x13356DE5,0x6888BF2,0x1A1D0E21,0x357B7C5,0x1B81E770,0xB],[0x139ED84,0xEBF912D,0x14BB2B7,0x4A25182,0x6B2A8DA,0x110C7CE4,0x13864023,0x4C9E1F1,0x1FB11586,0x1C573295,0x1A8DC9B0,0x1FC89A52,0x16ED6553,0x6],[0xF652983,0x89E0E33,0x19CF4673,0xE1A5B95,0x8F90A08,0x15C84BF3,0x66E7B4E,0xFBB2A4F,0x15DB3CB1,0x1FBD3A55,0x744806,0x1AE627FE,0x30C3250,0xB],[0xC8895D9,0x8AA674D,0x79DF114,0x1450DE60,0x1AC18985,0x15B2CC17,0xCFC21BB,0xB424AFF,0x1499DB99,0x1F208C72,0x1990AD2C,0x333E886,0x99726A3,0x7],[0x1D9B6861,0xD9C4320,0x41C64F1,0xDC4B9C6,0x13083533,0x1944F8D9,0x1C97C6CC,0xCAD51B7,0x12D7F5E4,0x183F2AA0,0x13818274,0x1F98DB6E,0x178E7166,0xB],[0xC9EDCB0,0xBCFCED,0x25CA7F8,0x187C7A54,0xE25C958,0x1280F634,0xF95A1E3,0xE652B30,0x1BCE0324,0xE8854D0,0x7441231,0x12ECF1D8,0x154005DB,0x6],[0x13CB83BB,0x1A7778D,0x630D5BA,0x11E54DE6,0x1E86B483,0x119E3868,0x105FD597,0xB65ED50,0x1C7C17E7,0x110A3D40,0x1622EAC,0x1287565E,0x1294ED3E,0xB],[0x134649B7,0x1560B313,0x198B5BAB,0x185ABE5,0xE2C8561,0x1DAB66DA,0x17FC989,0x11145AE0,0x56B303E,0xECCC0AC,0xE024407,0x1D066681,0x1A05F2B1,0x8],[0x8ECDD0A,0xB1C268B,0x1E19400B,0xE9C9696,0x11C15931,0x99CBC79,0xDDDB7D,0x1DD2DEFA,0xF682B4,0x159D2B34,0x11DB5B8F,0x13D255A8,0x15FC13AB,0x4],[0x19A1D641,0x1BB761D3,0xE90DC11,0x4CD2557,0x18835038,0x6D33F9C,0x19ADD040,0x3AE2C26,0xCE07F8D,0xD7E3D1E,0x17A482CF,0x1B4A9F04,0x10ECF6A,0x5],[0x1DCC5A5E,0xFBECCDD,0x478B4C4,0xB72913A,0x2C580FA,0x10E6FCC1,0x2A0665B,0x1843794D,0x196E7F63,0x3A6780C,0xC2CFD6C,0x1AC95164,0xA7AC2A9,0xA],[0xEE84A3A,0x12BA24B,0x3781B3B,0x766A71E,0xDE9CEA7,0x3983157,0x62538B8,0x1335EA74,0x1570F57,0x1F02CB39,0x3CF8318,0x2D26C32,0x172CAACF,0x3],[0x1F6304A5,0x16FCD14,0x8A3C470,0x1A49788,0x982F740,0x1E77925C,0x1534290E,0x1D39D395,0x9395735,0x18283637,0x154E43DF,0x9CCCF72,0x7355F8E,0x7],[0x1532A21E,0x1CE9CAD9,0xD5E0754,0x537503E,0x106DA9BD,0x27419D9,0xAEE35AD,0xB34240C,0x1DFFDFC7,0x1A1F3D03,0x29BC757,0x4522950,0x1A8E1620,0x9],[0xDC62CD8,0x186F449C,0x1B3D7104,0xDAA487D,0x16FD0497,0x1455E146,0x15455332,0x7E2D62C,0x145B0824,0x1BE2075A,0x120EABFB,0xB15C5FD,0x1425581A,0x1],[0x1CB83E19,0x611CDD2,0x53FB73F,0x7A12CF9,0xCEACD6A,0x700588D,0x1347F299,0xDEB4E31,0x1F6F8941,0xDFF94C8,0x4DF98A,0xF4644BD,0x12962FE5,0x5],[0x82B3BFF,0xE413B76,0xC09BA79,0x155108D9,0xBF5713D,0x12C4624,0x30049B,0x19419E10,0x167041E8,0x14C729B1,0x122D1C44,0x16AB3886,0x561A5DE,0x9],[0xD21B1C,0x9E7CFD2,0xD0F7E26,0x11AD037C,0xAC62B55,0x430BFE4,0x2EA7256,0x9746B69,0xF01D5EF,0x1A5E9FD3,0x62CB98B,0x19FE335C,0xCA8D548,0x4],[0x9C8B604,0x5A2B5F3,0x10071DC1,0xA04FDFD,0x101B2B66,0xA7D4AD7,0x8E55EB7,0x11F092CB,0x15CB181D,0x1A16F975,0x13A942CE,0x121E079C,0x1E6BE4E9,0xA],[0x1475224B,0x1358F38A,0x1E6BEDE1,0x20936CA,0x7CE46BA,0x7AE9CB5,0x15A366AC,0x103AFD0C,0x1C5E673D,0x1A46251F,0xA8567D,0x1C899E22,0x1C129645,0x2],[0x1B980133,0x16CE9FAE,0x8CA9910,0x1F215A38,0x659CC6C,0x11969E20,0x16004F99,0x101A982,0x1C757B3B,0x13DF18AE,0x1CBF002B,0x1A3D9536,0x45A394A,0x1],[0xB971EF8,0xA602780,0x4847C83,0x10A38323,0x633F06C,0x87403DA,0x23B009C,0x54684D6,0x47AA7B1,0x27A9FA,0x14554258,0x372733,0x1182CAC1,0x5],[0x10074D8E,0x103E4526,0x113581B3,0x139BE836,0x1643249D,0x1F3FC88F,0x918B9AF,0x17155E18,0xC523559,0x1FF6976E,0xE463050,0x1E6DEDBD,0xB46A908,0xC],[0x1011C132,0x9B88D6,0xFEEBF3A,0x1E74B99C,0x1E61031B,0x1F20B1C4,0x4FF4460,0x196D95E9,0x13CD2FCB,0x18EA1FDC,0x37F42E3,0x6F9A37C,0x1713E479,0xC],[0xA731C30,0x1D7D575E,0x13AE9BCA,0x1EE0ABBA,0xD43B9B3,0xF3F68F2,0x1BF81A61,0x14F22B5E,0x3C42A0C,0x1D6D0A51,0x88EAF79,0x30D7B6A,0x1BBA7A1,0x7],[0x1BDBA587,0x1B872BB,0x181E8D8,0xCA4038F,0xCABE69D,0x17350F90,0x9B07A2D,0x2CCF3B8,0x1B8F3ABD,0x10F26D0D,0x1A232788,0x1B2CD097,0x1FC4018B,0x4],[0x1870FB29,0xAF26518,0x17FA4D68,0xC8AA1FD,0x842642F,0x6D36136,0x7FF40E,0x17FC77BB,0x14170A05,0x9653633,0x17A649AF,0x67570DF,0x187C8D53,0x4],[0x1FE9D6F2,0xB0FC42A,0x3D057B2,0x10F5848C,0x14F3747A,0x9E26B1,0x132D48C5,0x19457C30,0x1CE75BB8,0x13BCB59,0xCB25DF4,0x1F583779,0xAB0B9BC,0x2],[0x1633A5F0,0xD91D589,0x16A01CA6,0x1EC64D92,0x1544E203,0xE1E9D6A,0x1EF5D941,0x1A95F5B6,0x74A7D0,0xDC78535,0x8847847,0xC696D4,0x603FCA4,0xB],[0x12E8FEDB,0xDB6D767,0x4102A10,0xFF1B813,0x11ADC2EE,0x1FE9109A,0x2E1E60C,0x1F7C79CA,0x4195536,0x1510A94E,0x172BD3F8,0x1FC1FE26,0xCC03FDE,0x4],[0x10E5F4CB,0x11AAE3BD,0x11877B29,0xB5753D,0x11CF9DE4,0x11F60192,0x4702792,0x1721DD6F,0x17D42AA7,0x16C3A33A,0x1E261D46,0x11303842,0x1F86376E,0x0],[0x72DE1F6,0x6FF1206,0xC0148EE,0x1AA42C51,0xDA7D26,0x1F25C8A0,0x138B0D12,0x1ACB1463,0x142552E2,0x351DA4C,0x1D28E132,0x152CDCCD,0xCC786BA,0x0],[0xE41C696,0x4BF3AD1,0xBEA2FF8,0xACE232C,0x1AD34D6C,0x11A1F5B3,0xF43E41,0xD84A9E7,0x31223E9,0x1BB7DA34,0x15440DB5,0x9DCB023,0x14996A10,0x9],[0x1707BB33,0x14C22B8C,0xEE8F0AF,0x18F5DD36,0x143D3CD0,0x17B64AB2,0x548AD4A,0x11C9150D,0x1A11AD13,0xA4C06E7,0x96747C2,0x17449DC0,0x10D97C81,0x4],[0x1D634B8F,0xAA39D0,0xD25E011,0x5EAE1E2,0xAA205CA,0x1E6B1AB6,0x14CC93B,0xCBC4E77,0x171C40F,0x106BC0CE,0x1AC90957,0xDBB807C,0xFA1D81,0x7],[0x6ED06F7,0xFD6E099,0x5332034,0xA2F7B0E,0x480E420,0x6F93CA1,0x1F072DD2,0x129CE524,0x12BF565B,0xA9E6BB7,0x18A2F743,0x165C9E76,0x660400E,0x1],[0x173345CC,0x14CD89C2,0xE42B047,0xEC7C7,0x19B86930,0x177CD006,0x899F573,0x1B315BE0,0x16543346,0x5A2F8A4,0x10D84C51,0x18ECFFC7,0xD6B9514,0x5],[0x2561092,0x1425A94F,0x1FAEFAA5,0x12D130DE,0x1913516F,0xD446753,0xB4A303E,0x115DF9C8,0x77F94FF,0x12462862,0x1D614B07,0x103A067F,0xCCBB674,0x5],[0x1A8F6AA8,0x7C5A4E5,0xC18100,0xB853E9F,0xA5C871A,0xD9B731B,0x18A43964,0x7376C34,0x1D9C6DD0,0xD69488,0x123C0428,0x1D480B7A,0xD2F259E,0x2],[0x18913F55,0x377A45D,0xA6CD78D,0x10BD47AA,0x1D4FBC73,0xC973F53,0x1EED4C21,0xC7C27B0,0x103216F7,0x1ECA5424,0x1AA08165,0xE14DC39,0x7A55CDA,0xB],[0x15535D4A,0x1919ECEA,0x49220DA,0x1FC5EF77,0x19B4852C,0x1A8625F9,0x482AF15,0x1C98D5EB,0x4F9FB0C,0x1E8EBA66,0x686F953,0x6D8C246,0x66C8ED3,0xC],[0x15812ED9,0x7720AD0,0x77B918,0x1EB6010,0x17132B92,0x7E9031A,0x1F5FFACD,0xBDF43E9,0xEE5A437,0x15DD37FB,0xEF377E,0x1C7D4FD4,0xA3EF08B,0xB],[0x126A775C,0x8D09CC8,0x2C7EE4F,0x1538034B,0x51D5F,0x12DE2005,0x3BD774D,0x1F51A19F,0xB5EECFD,0x5674C12,0x10EEA1CD,0x1533B65F,0x6007C08,0xB],[0xAF9B7AC,0x16323BFD,0xA733880,0x71B73BF,0x15A6449F,0xC3DB787,0x20717B3,0x18CAAA1B,0x2B70152,0x1563C18C,0x7EC99BA,0x30DB65B,0xD9E5297,0x4],[0x11A5001D,0x11C8A118,0x14BB7B76,0x162BB81F,0xC916A20,0xD07E4EF,0xEC150BB,0x13E1ED37,0x1CC6D19C,0x17C1146E,0xC033244,0x8BE87C9,0x1E0E0795,0x5],[0x45F5416,0x6936CC2,0xA5EB6A,0x6C9E585,0xAF41727,0x1244F393,0xC3848F6,0x1B7BB79A,0x11D115C5,0x1C4F6DA6,0x1C8348EF,0x131CA72B,0xB7D2887,0xB],[0x1DBF67F2,0x1129C5A9,0x1E5BE247,0xAF9AC6D,0xD2ECA67,0x12EE93CE,0x1CC430D6,0xAAA35CF,0x1778C485,0xB74758A,0x1BEAAB9F,0xC81B44E,0x18DF3306,0x2],[0xE49A03D,0x17B08161,0x14A78D4C,0x84C0EC6,0x1E01F78A,0x1AB7A29,0x16729284,0x1EE6389A,0x1885C84F,0x21E1A45,0x6832F5B,0x702403C,0x162D75C2,0xC],[0x103663C1,0xA3C929D,0x3081B40,0x6D11DEC,0x12E7A07F,0x1195ADF3,0xF9BBB0C,0x1CAF1301,0x9601A6D,0x7D68757,0x14860450,0x15393164,0x112C4C3,0xB]];
pub const CURVE_ADR:[Chunk;NLEN]=[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_ADI:[Chunk;NLEN]=[0xF0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_BDR:[Chunk;NLEN]=[0x3F4,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const CURVE_BDI:[Chunk;NLEN]=[0x3F4,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0];
pub const PCR:[[Chunk;NLEN];13]=[[0xAAA5ED1,0x7155555,0x19C71C62,0x11C71A1E,0x18575709,0x8478A15,0x2A88B58,0x1CFE9D02,0x14CB14B4,0x8FAFDB0,0x1B5B7A9A,0x147199F5,0x11D6541F,0xB],[0x1FFFC71E,0x154FFFFF,0x3555549,0x5555397,0xA418147,0x635A790,0x11FE6882,0x15BEF5C1,0xF984F87,0x16BC3E44,0xC849BF3,0x17553378,0x1560BF17,0x8],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0xAAA97D6,0x11C55555,0x1671C718,0xC71C687,0xE15D5C2,0x211E285,0x10AA22D6,0x73FA740,0x532C52D,0x123EBF6C,0xED6DEA6,0x1D1C667D,0x1C759507,0x2],[0xC,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x1C718B10,0xD9B8E38,0x1712F678,0x1212F4AD,0x74524E7,0x1BE34D51,0xA1AC3A5,0x6F43C4C,0x10761B0F,0xF1C08D6,0x1EFDC10F,0x16D9EF37,0x4C9AD43,0x9],[0x1FFFC71C,0x154FFFFF,0x3555549,0x5555397,0xA418147,0x635A790,0x11FE6882,0x15BEF5C1,0xF984F87,0x16BC3E44,0xC849BF3,0x17553378,0x1560BF17,0x8],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x11C6D706,0x167E38E3,0x124BDA04,0x184BD7F1,0x1E500FC8,0x1CEC3E93,0x126FD510,0x1A940FEC,0x130F7DA5,0x183B688C,0x16693062,0x15682276,0x130477C7,0xA],[0x12,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x1FFFA8FB,0xFF7FFFF,0x14FFFFEE,0x17FFFD62,0xF6241EA,0x9507B58,0xAFD9CC3,0x109E70A2,0x1764774B,0x121A5D66,0x12C6E9ED,0x12FFCD34,0x111EA3,0xD]];
pub const PCI:[[Chunk;NLEN];13]=[[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x1FFFE38D,0x1AA7FFFF,0x11AAAAA4,0x12AAA9CB,0x520C0A3,0x31AD3C8,0x18FF3441,0x1ADF7AE0,0x7CC27C3,0x1B5E1F22,0x6424DF9,0x1BAA99BC,0xAB05F8B,0x4],[0x1FFFC71A,0x154FFFFF,0x3555549,0x5555397,0xA418147,0x635A790,0x11FE6882,0x15BEF5C1,0xF984F87,0x16BC3E44,0xC849BF3,0x17553378,0x1560BF17,0x8],[0xAAA97D6,0x11C55555,0x1671C718,0xC71C687,0xE15D5C2,0x211E285,0x10AA22D6,0x73FA740,0x532C52D,0x123EBF6C,0xED6DEA6,0x1D1C667D,0x1C759507,0x2],[0x1FFFAA9F,0xFF7FFFF,0x14FFFFEE,0x17FFFD62,0xF6241EA,0x9507B58,0xAFD9CC3,0x109E70A2,0x1764774B,0x121A5D66,0x12C6E9ED,0x12FFCD34,0x111EA3,0xD],[0x1FFFAA63,0xFF7FFFF,0x14FFFFEE,0x17FFFD62,0xF6241EA,0x9507B58,0xAFD9CC3,0x109E70A2,0x1764774B,0x121A5D66,0x12C6E9ED,0x12FFCD34,0x111EA3,0xD],[0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0],[0x1FFFE38F,0x1AA7FFFF,0x11AAAAA4,0x12AAA9CB,0x520C0A3,0x31AD3C8,0x18FF3441,0x1ADF7AE0,0x7CC27C3,0x1B5E1F22,0x6424DF9,0x1BAA99BC,0xAB05F8B,0x4],[0xAAA97BE,0x11C55555,0x1671C718,0xC71C687,0xE15D5C2,0x211E285,0x10AA22D6,0x73FA740,0x532C52D,0x123EBF6C,0xED6DEA6,0x1D1C667D,0x1C759507,0x2],[0x11C6D706,0x167E38E3,0x124BDA04,0x184BD7F1,0x1E500FC8,0x1CEC3E93,0x126FD510,0x1A940FEC,0x130F7DA5,0x183B688C,0x16693062,0x15682276,0x130477C7,0xA],[0x1FFFAA99,0xFF7FFFF,0x14FFFFEE,0x17FFFD62,0xF6241EA,0x9507B58,0xAFD9CC3,0x109E70A2,0x1764774B,0x121A5D66,0x12C6E9ED,0x12FFCD34,0x111EA3,0xD],[0x1FFFA9D3,0xFF7FFFF,0x14FFFFEE,0x17FFFD62,0xF6241EA,0x9507B58,0xAFD9CC3,0x109E70A2,0x1764774B,0x121A5D66,0x12C6E9ED,0x12FFCD34,0x111EA3,0xD],[0x1FFFA8FB,0xFF7FFFF,0x14FFFFEE,0x17FFFD62,0xF6241EA,0x9507B58,0xAFD9CC3,0x109E70A2,0x1764774B,0x121A5D66,0x12C6E9ED,0x12FFCD34,0x111EA3,0xD]];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = false;
