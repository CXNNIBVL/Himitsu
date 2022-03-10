use crate::traits::cipher::{
    BlockCipherPrimitiveEncryption as PrimitiveEncryption,
    BlockCipherPrimitiveDecryption as PrimitiveDecryption, 
    BlockCipherPrimitiveInfo as PrimitiveInfo
};

use crate::mem;

const S_BOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
	0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
	0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
	0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
	0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
	0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
	0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
	0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
	0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
	0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
	0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
	0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
	0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
	0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
	0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
	0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16
];

const S_BOX_INV: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
	0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
	0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
	0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
	0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
	0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
	0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
	0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
	0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
	0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
	0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
	0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
	0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
	0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
	0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
	0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D
];

const MUL2: [u8; 256] = [
    0x00,0x02,0x04,0x06,0x08,0x0A,0x0C,0x0E,0x10,0x12,0x14,0x16,0x18,0x1A,0x1C,0x1E,
	0x20,0x22,0x24,0x26,0x28,0x2A,0x2C,0x2E,0x30,0x32,0x34,0x36,0x38,0x3A,0x3C,0x3E,
	0x40,0x42,0x44,0x46,0x48,0x4A,0x4C,0x4E,0x50,0x52,0x54,0x56,0x58,0x5A,0x5C,0x5E,
	0x60,0x62,0x64,0x66,0x68,0x6A,0x6C,0x6E,0x70,0x72,0x74,0x76,0x78,0x7A,0x7C,0x7E,
	0x80,0x82,0x84,0x86,0x88,0x8A,0x8C,0x8E,0x90,0x92,0x94,0x96,0x98,0x9A,0x9C,0x9E,
	0xA0,0xA2,0xA4,0xA6,0xA8,0xAA,0xAC,0xAE,0xB0,0xB2,0xB4,0xB6,0xB8,0xBA,0xBC,0xBE,
	0xC0,0xC2,0xC4,0xC6,0xC8,0xCA,0xCC,0xCE,0xD0,0xD2,0xD4,0xD6,0xD8,0xDA,0xDC,0xDE,
	0xE0,0xE2,0xE4,0xE6,0xE8,0xEA,0xEC,0xEE,0xF0,0xF2,0xF4,0xF6,0xF8,0xFA,0xFC,0xFE,
	0x1B,0x19,0x1F,0x1D,0x13,0x11,0x17,0x15,0x0B,0x09,0x0F,0x0D,0x03,0x01,0x07,0x05,
	0x3B,0x39,0x3F,0x3D,0x33,0x31,0x37,0x35,0x2B,0x29,0x2F,0x2D,0x23,0x21,0x27,0x25,
	0x5B,0x59,0x5F,0x5D,0x53,0x51,0x57,0x55,0x4B,0x49,0x4F,0x4D,0x43,0x41,0x47,0x45,
	0x7B,0x79,0x7F,0x7D,0x73,0x71,0x77,0x75,0x6B,0x69,0x6F,0x6D,0x63,0x61,0x67,0x65,
	0x9B,0x99,0x9F,0x9D,0x93,0x91,0x97,0x95,0x8B,0x89,0x8F,0x8D,0x83,0x81,0x87,0x85,
	0xBB,0xB9,0xBF,0xBD,0xB3,0xB1,0xB7,0xB5,0xAB,0xA9,0xAF,0xAD,0xA3,0xA1,0xA7,0xA5,
	0xDB,0xD9,0xDF,0xDD,0xD3,0xD1,0xD7,0xD5,0xCB,0xC9,0xCF,0xCD,0xC3,0xC1,0xC7,0xC5,
	0xFB,0xF9,0xFF,0xFD,0xF3,0xF1,0xF7,0xF5,0xEB,0xE9,0xEF,0xED,0xE3,0xE1,0xE7,0xE5
];

const MUL3: [u8; 256] = [
    0x00,0x03,0x06,0x05,0x0C,0x0F,0x0A,0x09,0x18,0x1B,0x1E,0x1D,0x14,0x17,0x12,0x11,
	0x30,0x33,0x36,0x35,0x3C,0x3F,0x3A,0x39,0x28,0x2B,0x2E,0x2D,0x24,0x27,0x22,0x21,
	0x60,0x63,0x66,0x65,0x6C,0x6F,0x6A,0x69,0x78,0x7B,0x7E,0x7D,0x74,0x77,0x72,0x71,
	0x50,0x53,0x56,0x55,0x5C,0x5F,0x5A,0x59,0x48,0x4B,0x4E,0x4D,0x44,0x47,0x42,0x41,
	0xC0,0xC3,0xC6,0xC5,0xCC,0xCF,0xCA,0xC9,0xD8,0xDB,0xDE,0xDD,0xD4,0xD7,0xD2,0xD1,
	0xF0,0xF3,0xF6,0xF5,0xFC,0xFF,0xFA,0xF9,0xE8,0xEB,0xEE,0xED,0xE4,0xE7,0xE2,0xE1,
	0xA0,0xA3,0xA6,0xA5,0xAC,0xAF,0xAA,0xA9,0xB8,0xBB,0xBE,0xBD,0xB4,0xB7,0xB2,0xB1,
	0x90,0x93,0x96,0x95,0x9C,0x9F,0x9A,0x99,0x88,0x8B,0x8E,0x8D,0x84,0x87,0x82,0x81,
	0x9B,0x98,0x9D,0x9E,0x97,0x94,0x91,0x92,0x83,0x80,0x85,0x86,0x8F,0x8C,0x89,0x8A,
	0xAB,0xA8,0xAD,0xAE,0xA7,0xA4,0xA1,0xA2,0xB3,0xB0,0xB5,0xB6,0xBF,0xBC,0xB9,0xBA,
	0xFB,0xF8,0xFD,0xFE,0xF7,0xF4,0xF1,0xF2,0xE3,0xE0,0xE5,0xE6,0xEF,0xEC,0xE9,0xEA,
	0xCB,0xC8,0xCD,0xCE,0xC7,0xC4,0xC1,0xC2,0xD3,0xD0,0xD5,0xD6,0xDF,0xDC,0xD9,0xDA,
	0x5B,0x58,0x5D,0x5E,0x57,0x54,0x51,0x52,0x43,0x40,0x45,0x46,0x4F,0x4C,0x49,0x4A,
	0x6B,0x68,0x6D,0x6E,0x67,0x64,0x61,0x62,0x73,0x70,0x75,0x76,0x7F,0x7C,0x79,0x7A,
	0x3B,0x38,0x3D,0x3E,0x37,0x34,0x31,0x32,0x23,0x20,0x25,0x26,0x2F,0x2C,0x29,0x2A,
	0x0B,0x08,0x0D,0x0E,0x07,0x04,0x01,0x02,0x13,0x10,0x15,0x16,0x1F,0x1C,0x19,0x1A
];

const MUL9: [u8; 256] = [
    0x00,0x09,0x12,0x1B,0x24,0x2D,0x36,0x3F,0x48,0x41,0x5A,0x53,0x6C,0x65,0x7E,0x77,
	0x90,0x99,0x82,0x8B,0xB4,0xBD,0xA6,0xAF,0xD8,0xD1,0xCA,0xC3,0xFC,0xF5,0xEE,0xE7,
	0x3B,0x32,0x29,0x20,0x1F,0x16,0x0D,0x04,0x73,0x7A,0x61,0x68,0x57,0x5E,0x45,0x4C,
	0xAB,0xA2,0xB9,0xB0,0x8F,0x86,0x9D,0x94,0xE3,0xEA,0xF1,0xF8,0xC7,0xCE,0xD5,0xDC,
	0x76,0x7F,0x64,0x6D,0x52,0x5B,0x40,0x49,0x3E,0x37,0x2C,0x25,0x1A,0x13,0x08,0x01,
	0xE6,0xEF,0xF4,0xFD,0xC2,0xCB,0xD0,0xD9,0xAE,0xA7,0xBC,0xB5,0x8A,0x83,0x98,0x91,
	0x4D,0x44,0x5F,0x56,0x69,0x60,0x7B,0x72,0x05,0x0C,0x17,0x1E,0x21,0x28,0x33,0x3A,
	0xDD,0xD4,0xCF,0xC6,0xF9,0xF0,0xEB,0xE2,0x95,0x9C,0x87,0x8E,0xB1,0xB8,0xA3,0xAA,
	0xEC,0xE5,0xFE,0xF7,0xC8,0xC1,0xDA,0xD3,0xA4,0xAD,0xB6,0xBF,0x80,0x89,0x92,0x9B,
	0x7C,0x75,0x6E,0x67,0x58,0x51,0x4A,0x43,0x34,0x3D,0x26,0x2F,0x10,0x19,0x02,0x0B,
	0xD7,0xDE,0xC5,0xCC,0xF3,0xFA,0xE1,0xE8,0x9F,0x96,0x8D,0x84,0xBB,0xB2,0xA9,0xA0,
	0x47,0x4E,0x55,0x5C,0x63,0x6A,0x71,0x78,0x0F,0x06,0x1D,0x14,0x2B,0x22,0x39,0x30,
	0x9A,0x93,0x88,0x81,0xBE,0xB7,0xAC,0xA5,0xD2,0xDB,0xC0,0xC9,0xF6,0xFF,0xE4,0xED,
	0x0A,0x03,0x18,0x11,0x2E,0x27,0x3C,0x35,0x42,0x4B,0x50,0x59,0x66,0x6F,0x74,0x7D,
	0xA1,0xA8,0xB3,0xBA,0x85,0x8C,0x97,0x9E,0xE9,0xE0,0xFB,0xF2,0xCD,0xC4,0xDF,0xD6,
	0x31,0x38,0x23,0x2A,0x15,0x1C,0x07,0x0E,0x79,0x70,0x6B,0x62,0x5D,0x54,0x4F,0x46
];

const MUL11: [u8; 256] = [
    0x00,0x0B,0x16,0x1D,0x2C,0x27,0x3A,0x31,0x58,0x53,0x4E,0x45,0x74,0x7F,0x62,0x69,
	0xB0,0xBB,0xA6,0xAD,0x9C,0x97,0x8A,0x81,0xE8,0xE3,0xFE,0xF5,0xC4,0xCF,0xD2,0xD9,
	0x7B,0x70,0x6D,0x66,0x57,0x5C,0x41,0x4A,0x23,0x28,0x35,0x3E,0x0F,0x04,0x19,0x12,
	0xCB,0xC0,0xDD,0xD6,0xE7,0xEC,0xF1,0xFA,0x93,0x98,0x85,0x8E,0xBF,0xB4,0xA9,0xA2,
	0xF6,0xFD,0xE0,0xEB,0xDA,0xD1,0xCC,0xC7,0xAE,0xA5,0xB8,0xB3,0x82,0x89,0x94,0x9F,
	0x46,0x4D,0x50,0x5B,0x6A,0x61,0x7C,0x77,0x1E,0x15,0x08,0x03,0x32,0x39,0x24,0x2F,
	0x8D,0x86,0x9B,0x90,0xA1,0xAA,0xB7,0xBC,0xD5,0xDE,0xC3,0xC8,0xF9,0xF2,0xEF,0xE4,
	0x3D,0x36,0x2B,0x20,0x11,0x1A,0x07,0x0C,0x65,0x6E,0x73,0x78,0x49,0x42,0x5F,0x54,
	0xF7,0xFC,0xE1,0xEA,0xDB,0xD0,0xCD,0xC6,0xAF,0xA4,0xB9,0xB2,0x83,0x88,0x95,0x9E,
	0x47,0x4C,0x51,0x5A,0x6B,0x60,0x7D,0x76,0x1F,0x14,0x09,0x02,0x33,0x38,0x25,0x2E,
	0x8C,0x87,0x9A,0x91,0xA0,0xAB,0xB6,0xBD,0xD4,0xDF,0xC2,0xC9,0xF8,0xF3,0xEE,0xE5,
	0x3C,0x37,0x2A,0x21,0x10,0x1B,0x06,0x0D,0x64,0x6F,0x72,0x79,0x48,0x43,0x5E,0x55,
	0x01,0x0A,0x17,0x1C,0x2D,0x26,0x3B,0x30,0x59,0x52,0x4F,0x44,0x75,0x7E,0x63,0x68,
	0xB1,0xBA,0xA7,0xAC,0x9D,0x96,0x8B,0x80,0xE9,0xE2,0xFF,0xF4,0xC5,0xCE,0xD3,0xD8,
	0x7A,0x71,0x6C,0x67,0x56,0x5D,0x40,0x4B,0x22,0x29,0x34,0x3F,0x0E,0x05,0x18,0x13,
	0xCA,0xC1,0xDC,0xD7,0xE6,0xED,0xF0,0xFB,0x92,0x99,0x84,0x8F,0xBE,0xB5,0xA8,0xA3
];

const MUL13: [u8; 256] = [
    0x00,0x0D,0x1A,0x17,0x34,0x39,0x2E,0x23,0x68,0x65,0x72,0x7F,0x5C,0x51,0x46,0x4B,
	0xD0,0xDD,0xCA,0xC7,0xE4,0xE9,0xFE,0xF3,0xB8,0xB5,0xA2,0xAF,0x8C,0x81,0x96,0x9B,
	0xBB,0xB6,0xA1,0xAC,0x8F,0x82,0x95,0x98,0xD3,0xDE,0xC9,0xC4,0xE7,0xEA,0xFD,0xF0,
	0x6B,0x66,0x71,0x7C,0x5F,0x52,0x45,0x48,0x03,0x0E,0x19,0x14,0x37,0x3A,0x2D,0x20,
	0x6D,0x60,0x77,0x7A,0x59,0x54,0x43,0x4E,0x05,0x08,0x1F,0x12,0x31,0x3C,0x2B,0x26,
	0xBD,0xB0,0xA7,0xAA,0x89,0x84,0x93,0x9E,0xD5,0xD8,0xCF,0xC2,0xE1,0xEC,0xFB,0xF6,
	0xD6,0xDB,0xCC,0xC1,0xE2,0xEF,0xF8,0xF5,0xBE,0xB3,0xA4,0xA9,0x8A,0x87,0x90,0x9D,
	0x06,0x0B,0x1C,0x11,0x32,0x3F,0x28,0x25,0x6E,0x63,0x74,0x79,0x5A,0x57,0x40,0x4D,
	0xDA,0xD7,0xC0,0xCD,0xEE,0xE3,0xF4,0xF9,0xB2,0xBF,0xA8,0xA5,0x86,0x8B,0x9C,0x91,
	0x0A,0x07,0x10,0x1D,0x3E,0x33,0x24,0x29,0x62,0x6F,0x78,0x75,0x56,0x5B,0x4C,0x41,
	0x61,0x6C,0x7B,0x76,0x55,0x58,0x4F,0x42,0x09,0x04,0x13,0x1E,0x3D,0x30,0x27,0x2A,
	0xB1,0xBC,0xAB,0xA6,0x85,0x88,0x9F,0x92,0xD9,0xD4,0xC3,0xCE,0xED,0xE0,0xF7,0xFA,
	0xB7,0xBA,0xAD,0xA0,0x83,0x8E,0x99,0x94,0xDF,0xD2,0xC5,0xC8,0xEB,0xE6,0xF1,0xFC,
	0x67,0x6A,0x7D,0x70,0x53,0x5E,0x49,0x44,0x0F,0x02,0x15,0x18,0x3B,0x36,0x21,0x2C,
	0x0C,0x01,0x16,0x1B,0x38,0x35,0x22,0x2F,0x64,0x69,0x7E,0x73,0x50,0x5D,0x4A,0x47,
	0xDC,0xD1,0xC6,0xCB,0xE8,0xE5,0xF2,0xFF,0xB4,0xB9,0xAE,0xA3,0x80,0x8D,0x9A,0x97
];

const MUL14: [u8; 256] = [
    0x00,0x0e,0x1c,0x12,0x38,0x36,0x24,0x2a,0x70,0x7e,0x6c,0x62,0x48,0x46,0x54,0x5a,
	0xe0,0xee,0xfc,0xf2,0xd8,0xd6,0xc4,0xca,0x90,0x9e,0x8c,0x82,0xa8,0xa6,0xb4,0xba,
	0xdb,0xd5,0xc7,0xc9,0xe3,0xed,0xff,0xf1,0xab,0xa5,0xb7,0xb9,0x93,0x9d,0x8f,0x81,
	0x3b,0x35,0x27,0x29,0x03,0x0d,0x1f,0x11,0x4b,0x45,0x57,0x59,0x73,0x7d,0x6f,0x61,
	0xad,0xa3,0xb1,0xbf,0x95,0x9b,0x89,0x87,0xdd,0xd3,0xc1,0xcf,0xe5,0xeb,0xf9,0xf7,
	0x4d,0x43,0x51,0x5f,0x75,0x7b,0x69,0x67,0x3d,0x33,0x21,0x2f,0x05,0x0b,0x19,0x17,
	0x76,0x78,0x6a,0x64,0x4e,0x40,0x52,0x5c,0x06,0x08,0x1a,0x14,0x3e,0x30,0x22,0x2c,
	0x96,0x98,0x8a,0x84,0xae,0xa0,0xb2,0xbc,0xe6,0xe8,0xfa,0xf4,0xde,0xd0,0xc2,0xcc,
	0x41,0x4f,0x5d,0x53,0x79,0x77,0x65,0x6b,0x31,0x3f,0x2d,0x23,0x09,0x07,0x15,0x1b,
	0xa1,0xaf,0xbd,0xb3,0x99,0x97,0x85,0x8b,0xd1,0xdf,0xcd,0xc3,0xe9,0xe7,0xf5,0xfb,
	0x9a,0x94,0x86,0x88,0xa2,0xac,0xbe,0xb0,0xea,0xe4,0xf6,0xf8,0xd2,0xdc,0xce,0xc0,
	0x7a,0x74,0x66,0x68,0x42,0x4c,0x5e,0x50,0x0a,0x04,0x16,0x18,0x32,0x3c,0x2e,0x20,
	0xec,0xe2,0xf0,0xfe,0xd4,0xda,0xc8,0xc6,0x9c,0x92,0x80,0x8e,0xa4,0xaa,0xb8,0xb6,
	0x0c,0x02,0x10,0x1e,0x34,0x3a,0x28,0x26,0x7c,0x72,0x60,0x6e,0x44,0x4a,0x58,0x56,
	0x37,0x39,0x2b,0x25,0x0f,0x01,0x13,0x1d,0x47,0x49,0x5b,0x55,0x7f,0x71,0x63,0x6d,
	0xd7,0xd9,0xcb,0xc5,0xef,0xe1,0xf3,0xfd,0xa7,0xa9,0xbb,0xb5,0x9f,0x91,0x83,0x8d
];

const RCON: [u8; 256] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a,
	0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39,
	0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a,
	0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8,
	0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef,
	0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc,
	0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b,
	0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3,
	0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94,
	0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20,
	0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63, 0xc6, 0x97, 0x35,
	0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd, 0x61, 0xc2, 0x9f,
	0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d, 0x01, 0x02, 0x04,
	0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36, 0x6c, 0xd8, 0xab, 0x4d, 0x9a, 0x2f, 0x5e, 0xbc, 0x63,
	0xc6, 0x97, 0x35, 0x6a, 0xd4, 0xb3, 0x7d, 0xfa, 0xef, 0xc5, 0x91, 0x39, 0x72, 0xe4, 0xd3, 0xbd,
	0x61, 0xc2, 0x9f, 0x25, 0x4a, 0x94, 0x33, 0x66, 0xcc, 0x83, 0x1d, 0x3a, 0x74, 0xe8, 0xcb, 0x8d
];

pub const AES_BLOCKSIZE: usize = 16;
pub const AES_128_KEYLEN: usize = 16;
pub const AES_192_KEYLEN: usize = 24;
pub const AES_256_KEYLEN: usize = 32;
pub type AesBlock = [u8; AES_BLOCKSIZE];

/// Aes Encryption and Decryption provider
pub struct Aes { 
	cfg: AesCfg,
}

impl Aes {
	/// Create a new Aes instance
	pub fn new(key: &[u8]) -> Self {
		Self { cfg: aes_configuration(key) }
	}
}

impl PrimitiveInfo for Aes {
    const BLOCKSIZE: usize = AES_BLOCKSIZE;
    const KEYLEN_MIN: usize = AES_128_KEYLEN;
    const KEYLEN_MAX: usize = AES_256_KEYLEN;
}

impl PrimitiveEncryption<AES_BLOCKSIZE> for Aes {

    fn encrypt(&self, state: &mut AesBlock, xor_pre: Option<&AesBlock>, xor_post: Option<&AesBlock>) {

		if let Some(block) = xor_pre {
			mem::xor_buffers(state.as_mut(), block.as_ref());
		}

		add_roundkey(state.as_mut(), &self.cfg.expanded_key[0..16]);

		for i in 0..self.cfg.rounds - 1 {

			sub_bytes_enc(state.as_mut());
			shift_rows_enc(state.as_mut());
			mix_columns_enc(state.as_mut());
			
			let start = 16 * (i + 1);
			let end = start + 16;
			add_roundkey(state.as_mut(), &self.cfg.expanded_key[start..end]);
		} 

		sub_bytes_enc(state.as_mut());
		shift_rows_enc(state.as_mut());
		
		let index = self.cfg.expanded_key.len() - 16; 
		add_roundkey(state.as_mut(), &self.cfg.expanded_key[index..]);

		if let Some(block) = xor_post {
			mem::xor_buffers(state.as_mut(), block.as_ref());	
		}
    }
}

impl PrimitiveDecryption<AES_BLOCKSIZE> for Aes {

    fn decrypt(&self, state: &mut AesBlock, xor_pre: Option<&AesBlock>, xor_post: Option<&AesBlock>) {

		if let Some(block) = xor_pre {
			mem::xor_buffers(state.as_mut(), block.as_ref());	
		}

		let index = self.cfg.expanded_key.len() - 16; 
		add_roundkey(state.as_mut(), &self.cfg.expanded_key[index..]);
		sub_bytes_dec(state.as_mut());
		shift_rows_dec(state.as_mut());

		for i in (0..self.cfg.rounds - 1).rev() {
			let start = 16 * (i + 1);
			let end = start + 16;
			add_roundkey(state.as_mut(), &self.cfg.expanded_key[start..end]);

			mix_columns_dec(state.as_mut());
			sub_bytes_dec(state.as_mut());
			shift_rows_dec(state.as_mut());
		}

		add_roundkey(state.as_mut(), &self.cfg.expanded_key[0..16]);

		if let Some(block) = xor_post {
			mem::xor_buffers(state.as_mut(), block.as_ref());	
		}
    }
}

struct AesCfg {
    expanded_key: Vec<u8>,
    rounds: usize,
}

fn aes_configuration(key: &[u8]) -> AesCfg {
	let (expanded_key, rounds) = key_expansion(key);
    AesCfg { expanded_key, rounds }
}

fn key_expansion_gcon(k: &mut [u8; 4]) {
	// Apply S_BOX
	k[0] = S_BOX[k[0] as usize];
	k[1] = S_BOX[k[1] as usize];
	k[2] = S_BOX[k[2] as usize];
	k[3] = S_BOX[k[3] as usize];
}

fn key_expansion_rcon(k: &mut [u8; 4], iteration: usize) {
	k.rotate_left(1);
	key_expansion_gcon(k);
	k[0] ^= RCON[iteration];
}

/// Returns the expanded key and the number of rounds
fn key_expansion(key: &[u8]) -> (Vec<u8>, usize) {

	let (
		rounds, 
		copy, 
		acc_key_len
	) = match key.len() {
		0..=AES_128_KEYLEN => { (10, key.len(), AES_128_KEYLEN) },

		17..=AES_192_KEYLEN => { (12, key.len(), AES_192_KEYLEN) },

		25..=AES_256_KEYLEN => { (14, key.len(), AES_256_KEYLEN) },

		_ => { (14, AES_256_KEYLEN, AES_256_KEYLEN) }
	};

	let capacity = (rounds + 1) * AES_BLOCKSIZE;
	let mut expanded_key = Vec::with_capacity(capacity);
	expanded_key.extend(&key[0..copy]);

	while expanded_key.len() != acc_key_len { expanded_key.push(0); }

	let mut rcon_iteration = 1;
	let mut bytes_generated = acc_key_len;
	while bytes_generated != capacity {

		let mut tmp = [0u8;4];
		tmp.copy_from_slice(&expanded_key[expanded_key.len() - 4..]);

		if 	expanded_key.len() % 16 == 0 && expanded_key.len() % 32 != 0 && capacity == 15 * AES_BLOCKSIZE {
			key_expansion_gcon(&mut tmp);
		}

		if expanded_key.len() % acc_key_len == 0 {
			key_expansion_rcon(&mut tmp, rcon_iteration);
			rcon_iteration += 1;
		}

		let ix = expanded_key.len() - acc_key_len;
		mem::xor_buffers(&mut tmp, &expanded_key[ix..ix + 4]);

		expanded_key.extend(tmp);
		bytes_generated += 4;
	}

	(expanded_key, rounds)
}

/// Xor round key into state
fn add_roundkey(state: &mut[u8], key: &[u8]) {
	mem::xor_buffers(state, key);
}

/// Substitute with SBOX
fn sub_bytes_enc(state: &mut [u8]) {
	for i in 0..16 {
		state[i] = S_BOX[state[i] as usize];
	}
}

fn shift_rows_enc(state: &mut [u8]) {
	// Row 1	a b c d -> b c d a
	state.swap(1, 5);
	state.swap(5, 13);
	state.swap(5, 9);

	// Row 2	a b c d -> c d a b
	state.swap(2, 10);
	state.swap(6, 14);

	// Row 3	a b c d -> d a b c
	state.swap(3, 15);
	state.swap(7, 15);
	state.swap(11, 15);
}

fn mix_columns_enc(state: &mut [u8]) {

	let mut tmp = [0u8;16];

	tmp[0] = MUL2[state[0] as usize] ^ MUL3[state[1] as usize] ^ state[2] ^ state[3];
	tmp[1] = state[0] ^ MUL2[state[1] as usize] ^ MUL3[state[2] as usize] ^ state[3];
	tmp[2] = state[0] ^ state[1] ^ MUL2[state[2] as usize] ^ MUL3[state[3] as usize];
	tmp[3] = MUL3[state[0] as usize] ^ state[1] ^ state[2] ^ MUL2[state[3] as usize];

	tmp[4] = MUL2[state[4] as usize] ^ MUL3[state[5] as usize] ^ state[6] ^ state[7];
	tmp[5] = state[4] ^ MUL2[state[5] as usize] ^ MUL3[state[6] as usize] ^ state[7];
	tmp[6] = state[4] ^ state[5] ^ MUL2[state[6] as usize] ^ MUL3[state[7] as usize];
	tmp[7] = MUL3[state[4] as usize] ^ state[5] ^ state[6] ^ MUL2[state[7] as usize];

	tmp[8] = MUL2[state[8] as usize] ^ MUL3[state[9] as usize] ^ state[10] ^ state[11];
	tmp[9] = state[8] ^ MUL2[state[9] as usize] ^ MUL3[state[10] as usize] ^ state[11];
	tmp[10] = state[8] ^ state[9] ^ MUL2[state[10] as usize] ^ MUL3[state[11] as usize];
	tmp[11] = MUL3[state[8] as usize] ^ state[9] ^ state[10] ^ MUL2[state[11] as usize];

	tmp[12] = MUL2[state[12] as usize] ^ MUL3[state[13] as usize] ^ state[14] ^ state[15];
	tmp[13] = state[12] ^ MUL2[state[13] as usize] ^ MUL3[state[14] as usize] ^ state[15];
	tmp[14] = state[12] ^ state[13] ^ MUL2[state[14] as usize] ^ MUL3[state[15] as usize];
	tmp[15] = MUL3[state[12] as usize] ^ state[13] ^ state[14] ^ MUL2[state[15] as usize];

	state.copy_from_slice(&tmp);
}

fn sub_bytes_dec(state: &mut [u8]) {
	for i in 0..16 {
		state[i] = S_BOX_INV[state[i] as usize];
	}
}

fn shift_rows_dec(state: &mut [u8]) {
	// Row 1	b c d a -> a b c d
	state.swap(1, 13);
	state.swap(5, 13);
	state.swap(9, 13);
	
	// Row 2	c d a b -> a b c d
	state.swap(2, 10);
	state.swap(6, 14);
	
	// Row 3	d a b c -> a b c d
	state.swap(3, 7);
	state.swap(7, 11);
	state.swap(11, 15);
}

fn mix_columns_dec(state: &mut [u8]) {

	let mut tmp = [0u8;16];

	tmp[0] = MUL14[state[0 ] as usize ] ^ MUL11[state[1 ] as usize ] ^ MUL13[state[2 ] as usize ] ^ MUL9[state[3 ] as usize ];
	tmp[1] = MUL9[state[0 ] as usize ] ^ MUL14[state[1 ] as usize ] ^ MUL11[state[2 ] as usize ] ^ MUL13[state[3 ] as usize ];
	tmp[2] = MUL13[state[0 ] as usize ] ^ MUL9[state[1 ] as usize ] ^ MUL14[state[2 ] as usize ] ^ MUL11[state[3 ] as usize ];
	tmp[3] = MUL11[state[0 ] as usize ] ^ MUL13[state[1 ] as usize ] ^ MUL9[state[2 ] as usize ] ^ MUL14[state[3 ] as usize ];

	tmp[4] = MUL14[state[4 ] as usize ] ^ MUL11[state[5 ] as usize ] ^ MUL13[state[6 ] as usize ] ^ MUL9[state[7 ] as usize ];
	tmp[5] = MUL9[state[4 ] as usize ] ^ MUL14[state[5 ] as usize ] ^ MUL11[state[6 ] as usize ] ^ MUL13[state[7 ] as usize ];
	tmp[6] = MUL13[state[4 ] as usize ] ^ MUL9[state[5 ] as usize ] ^ MUL14[state[6 ] as usize ] ^ MUL11[state[7 ] as usize ];
	tmp[7] = MUL11[state[4 ] as usize ] ^ MUL13[state[5 ] as usize ] ^ MUL9[state[6 ] as usize ] ^ MUL14[state[7 ] as usize ];

	tmp[8] = MUL14[state[8 ] as usize ] ^ MUL11[state[9 ] as usize ] ^ MUL13[state[10 ] as usize ] ^ MUL9[state[11 ] as usize ];
	tmp[9] = MUL9[state[8 ] as usize ] ^ MUL14[state[9 ] as usize ] ^ MUL11[state[10 ] as usize ] ^ MUL13[state[11 ] as usize ];
	tmp[10] = MUL13[state[8 ] as usize ] ^ MUL9[state[9 ] as usize ] ^ MUL14[state[10 ] as usize ] ^ MUL11[state[11 ] as usize ];
	tmp[11] = MUL11[state[8 ] as usize ] ^ MUL13[state[9 ] as usize ] ^ MUL9[state[10 ] as usize ] ^ MUL14[state[11 ] as usize ];

	tmp[12] = MUL14[state[12 ] as usize ] ^ MUL11[state[13 ] as usize ] ^ MUL13[state[14 ] as usize ] ^ MUL9[state[15 ] as usize ];
	tmp[13] = MUL9[state[12 ] as usize ] ^ MUL14[state[13 ] as usize ] ^ MUL11[state[14 ] as usize ] ^ MUL13[state[15 ] as usize ];
	tmp[14] = MUL13[state[12 ] as usize ] ^ MUL9[state[13 ] as usize ] ^ MUL14[state[14 ] as usize ] ^ MUL11[state[15 ] as usize ];
	tmp[15] = MUL11[state[12 ] as usize ] ^ MUL13[state[13 ] as usize ] ^ MUL9[state[14 ] as usize ] ^ MUL14[state[15 ] as usize ];

	state.copy_from_slice(&tmp);
}

#[cfg(test)]
mod tests {

	use super::*;

	fn decode(s: &str) -> Vec<u8> {
		use crate::encode::HexEncoder;
		HexEncoder::builder().decode(s)
	}

	#[test]
	fn test_key_expansion_16byte() {

		let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
		let expected_str = 
		"00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		9b 98 98 c9 f9 fb fb aa 9b 98 98 c9 f9 fb fb aa 
		90 97 34 50 69 6c cf fa f2 f4 57 33 0b 0f ac 99 
		ee 06 da 7b 87 6a 15 81 75 9e 42 b2 7e 91 ee 2b 
		7f 2e 2b 88 f8 44 3e 09 8d da 7c bb f3 4b 92 90 
		ec 61 4b 85 14 25 75 8c 99 ff 09 37 6a b4 9b a7 
		21 75 17 87 35 50 62 0b ac af 6b 3c c6 1b f0 9b 
		0e f9 03 33 3b a9 61 38 97 06 0a 04 51 1d fa 9f 
		b1 d4 d8 e2 8a 7d b9 da 1d 7b b3 de 4c 66 49 41 
		b4 ef 5b cb 3e 92 e2 11 23 e9 51 cf 6f 8f 18 8e";

		let (key, expected) = (decode(key_str), decode(expected_str));
		let (expanded, _ ) = key_expansion(&key);

		assert_eq!(expanded, expected);
	}

	#[test]
	fn test_key_expansion_24byte() {

		let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
		let expected_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		00 00 00 00 00 00 00 00 62 63 63 63 62 63 63 63 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		9b 98 98 c9 f9 fb fb aa 9b 98 98 c9 f9 fb fb aa 
		9b 98 98 c9 f9 fb fb aa 90 97 34 50 69 6c cf fa 
		f2 f4 57 33 0b 0f ac 99 90 97 34 50 69 6c cf fa 
		c8 1d 19 a9 a1 71 d6 53 53 85 81 60 58 8a 2d f9 
		c8 1d 19 a9 a1 71 d6 53 7b eb f4 9b da 9a 22 c8 
		89 1f a3 a8 d1 95 8e 51 19 88 97 f8 b8 f9 41 ab 
		c2 68 96 f7 18 f2 b4 3f 91 ed 17 97 40 78 99 c6 
		59 f0 0e 3e e1 09 4f 95 83 ec bc 0f 9b 1e 08 30 
		0a f3 1f a7 4a 8b 86 61 13 7b 88 5f f2 72 c7 ca 
		43 2a c8 86 d8 34 c0 b6 d2 c7 df 11 98 4c 59 70";

		let (key, expected) = (decode(key_str), decode(expected_str));
		let (expanded, _ ) = key_expansion(&key);

		assert_eq!(expanded, expected);
	}

	#[test]
	fn test_key_expansion_32byte() {

		let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
		let expected_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		aa fb fb fb aa fb fb fb aa fb fb fb aa fb fb fb 
		6f 6c 6c cf 0d 0f 0f ac 6f 6c 6c cf 0d 0f 0f ac 
		7d 8d 8d 6a d7 76 76 91 7d 8d 8d 6a d7 76 76 91 
		53 54 ed c1 5e 5b e2 6d 31 37 8e a2 3c 38 81 0e 
		96 8a 81 c1 41 fc f7 50 3c 71 7a 3a eb 07 0c ab 
		9e aa 8f 28 c0 f1 6d 45 f1 c6 e3 e7 cd fe 62 e9 
		2b 31 2b df 6a cd dc 8f 56 bc a6 b5 bd bb aa 1e 
		64 06 fd 52 a4 f7 90 17 55 31 73 f0 98 cf 11 19 
		6d bb a9 0b 07 76 75 84 51 ca d3 31 ec 71 79 2f 
		e7 b0 e8 9c 43 47 78 8b 16 76 0b 7b 8e b9 1a 62 
		74 ed 0b a1 73 9b 7e 25 22 51 ad 14 ce 20 d4 3b 
		10 f8 0a 17 53 bf 72 9c 45 c9 79 e7 cb 70 63 85 ";

		let (key, expected) = (decode(key_str), decode(expected_str));
		let (expanded, _ ) = key_expansion(&key);

		assert_eq!(expanded, expected);
	}

	#[test]
	fn test_key_expansion_bigger_32byte() {

		let key_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00";
		let expected_str = "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
		62 63 63 63 62 63 63 63 62 63 63 63 62 63 63 63 
		aa fb fb fb aa fb fb fb aa fb fb fb aa fb fb fb 
		6f 6c 6c cf 0d 0f 0f ac 6f 6c 6c cf 0d 0f 0f ac 
		7d 8d 8d 6a d7 76 76 91 7d 8d 8d 6a d7 76 76 91 
		53 54 ed c1 5e 5b e2 6d 31 37 8e a2 3c 38 81 0e 
		96 8a 81 c1 41 fc f7 50 3c 71 7a 3a eb 07 0c ab 
		9e aa 8f 28 c0 f1 6d 45 f1 c6 e3 e7 cd fe 62 e9 
		2b 31 2b df 6a cd dc 8f 56 bc a6 b5 bd bb aa 1e 
		64 06 fd 52 a4 f7 90 17 55 31 73 f0 98 cf 11 19 
		6d bb a9 0b 07 76 75 84 51 ca d3 31 ec 71 79 2f 
		e7 b0 e8 9c 43 47 78 8b 16 76 0b 7b 8e b9 1a 62 
		74 ed 0b a1 73 9b 7e 25 22 51 ad 14 ce 20 d4 3b 
		10 f8 0a 17 53 bf 72 9c 45 c9 79 e7 cb 70 63 85 ";

		let (key, expected) = (decode(key_str), decode(expected_str));
		let (expanded, _ ) = key_expansion(&key);

		assert_eq!(expanded, expected);
	}

	#[test]
	fn test_add_roundkey() {

		let key = decode("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f");
		let mut state = decode("00 11 22 33 44 55 66 77 88 99 aa bb cc dd ee ff");
		let expected = decode("00102030405060708090a0b0c0d0e0f0");

		add_roundkey(&mut state, &key);

		assert_eq!(expected, state)
	}

	#[test]
	fn test_sub_bytes_enc() {
		let mut state = decode("00 10 20 30 40 50 60 70 80 90 a0 b0 c0 d0 e0 f0");
		let expected =  decode("63cab7040953d051cd60e0e7ba70e18c");
		sub_bytes_enc(&mut state);
		assert_eq!(expected, state);
	}

	#[test]
	fn test_shift_rows_enc() {
		let mut state = decode("63cab7040953d051cd60e0e7ba70e18c");
		let expected = decode("6353e08c0960e104cd70b751bacad0e7");
		shift_rows_enc(&mut state);
		assert_eq!(expected, state);
	}

	#[test]
	fn test_mix_columns_enc() {
		let mut state = decode("6353e08c0960e104cd70b751bacad0e7");
		let expected = decode("5f72641557f5bc92f7be3b291db9f91a");
		mix_columns_enc(&mut state);
		assert_eq!(expected, state);
	}

	#[test]
	fn test_sub_bytes_dec() {

		let mut state = decode("7a9f102789d5f50b2beffd9f3dca4ea7");
		let expected = decode("bd6e7c3df2b5779e0b61216e8b10b689");

		sub_bytes_dec(&mut state);
		assert_eq!(expected, state);
	}

	#[test]
	fn test_shift_rows_dec() {
		let mut state = decode("7ad5fda789ef4e272bca100b3d9ff59f");
		let expected = decode("7a9f102789d5f50b2beffd9f3dca4ea7");
		shift_rows_dec(&mut state);
		assert_eq!(expected, state);
	}

	#[test]
	fn test_mix_columns_dec() {
		let mut state = decode("bd6e7c3df2b5779e0b61216e8b10b689");
		let expected = decode("4773b91ff72f354361cb018ea1e6cf2c");
		mix_columns_dec(&mut state);
		assert_eq!(expected, state);
	}
}
