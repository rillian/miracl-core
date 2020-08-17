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

/* Fixed Data in ROM - Field and Curve parameters */


package org.miracl.core.SECP256K1;

public class ROM
{

// Base Bits= 56
public static final long[] Modulus= {0xFFFFFEFFFFFC2FL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFL};
public static final long[] ROI= {0xFFFFFEFFFFFC2EL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFL};
public static final long[] R2modp= {0xA1000000000000L,0x7A2000E90L,0x1L,0x0L,0x0L};
public static final long MConst= 0x38091DD2253531L;
public static final long[] SQRTm3= {0x8D27AE1CD5F852L,0x6D15DA14ECD47DL,0xC2A797962CC61FL,0x3507F1DF233770L,0xA2D2BA9L};

public static final int CURVE_Cof_I= 1;
public static final long[] CURVE_Cof= {0x1L,0x0L,0x0L,0x0L,0x0L};
public static final int CURVE_B_I= 7;
public static final long[] CURVE_B= {0x7L,0x0L,0x0L,0x0L,0x0L};
public static final long[] CURVE_Order= {0xD25E8CD0364141L,0xDCE6AF48A03BBFL,0xFFFFFFFFFEBAAEL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFL};
public static final long[] CURVE_Gx= {0xF2815B16F81798L,0xFCDB2DCE28D959L,0x95CE870B07029BL,0xF9DCBBAC55A062L,0x79BE667EL};
public static final long[] CURVE_Gy= {0x47D08FFB10D4B8L,0xB448A68554199CL,0xFC0E1108A8FD17L,0x26A3C4655DA4FBL,0x483ADA77L};

public static final long[] CURVE_Ad= {0x5447C01A444533L,0xD363CB6F0E5D40L,0x58F0F5D272E953L,0xDD661ADCA08A55L,0x3F8731ABL};
public static final long[] CURVE_Bd= {0x6EBL,0x0L,0x0L,0x0L,0x0L};
public static final long[][] PC= {{0x38E38DAAAAA88CL,0x8E38E38E38E38EL,0xE38E38E38E38E3L,0x38E38E38E38E38L,0x8E38E38EL},{0xCBD0B53D9DD262L,0x6144037C40314EL,0xDECA25CAECE450L,0x23F234E6E2A413L,0x534C328DL},{0xFF1044F17C6581L,0xD2FC0BF63B92DFL,0xCEA7FD44C5D595L,0xBC321D5B9F315L,0x7D3D4C8L},{0x38E38DAAAAA8C7L,0x8E38E38E38E38EL,0xE38E38E38E38E3L,0x38E38E38E38E38L,0x8E38E38EL},{0x2A56612A8C6D14L,0x6B641F5E41BBC5L,0xD51B54225406D3L,0x4383DC1DF7C4B2L,0xEDADC6F6L},{0xE6B745781EB49BL,0x409542F8487D9FL,0xCBB7B640DD86CDL,0x3D94918A9CA34CL,0xD3577119L},{0xBDA12F38E38D84L,0x2F684BDA12F684L,0x4BDA12F684BDA1L,0x12F684BDA12F68L,0x2F684BDAL},{0x65E85A9ECEE931L,0x30A201BE2018A7L,0xEF6512E5767228L,0x91F91A73715209L,0x29A61946L},{0xFC90FC201D71A3L,0xB046D686DA6FDFL,0x4B12A0A6D5647AL,0xD5CB7C0FA9D0A5L,0xC75E0C32L},{0x2F684B8E38E23CL,0x4BDA12F684BDA1L,0x12F684BDA12F68L,0x84BDA12F684BDAL,0x4BDA12F6L},{0xBF8192BFD2A76FL,0x21162F0D6299A7L,0x3FA8FE337E0A3DL,0x6545CA2CF3A70CL,0x6484AA71L},{0xB425D2685C2573L,0xC1BFC8E8D978DFL,0x632722C2989467L,0xB8BDB49FD5E9E6L,0x7A06534BL},{0xFFFFFEFFFFF93BL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFFFFFFFL,0xFFFFFFFFL}};

}

