#![no_std]

pub const CLUSTER_CODE_HASHES: [[u8; 32]; 1] = [
    [
        89, 141, 121, 61, 239, 239, 54, 226,
        238, 186, 84, 169, 180, 81, 48, 228,
        202, 146, 130, 46, 29, 25, 54, 113,
        244, 144, 149, 12, 59, 133, 96, 128
    ]
];

pub const CLUSTER_PROXY_CODE_HASHES: [[u8; 32]; 1] = [
    [
        116, 103, 8, 170, 4, 229, 159, 69,
        211, 99, 234, 210, 38, 194, 7, 254,
        160, 183, 253,  223, 64, 37, 82, 25,
        181, 210, 44, 88, 154, 69, 59, 149
    ]
];

pub const CLUSTER_AGENT_CODE_HASHES: [[u8; 32]; 1] = [
    [
        252, 56, 1, 224, 84, 227, 182, 187,
        252, 160, 94, 103, 9, 167, 1, 75,
        128, 241, 6, 12, 52, 37, 119, 106,
        146, 86, 189, 47, 242, 14, 202, 230
    ]
];

pub const CKB_LUA_LIB_CODE_HASH: [u8; 32] = [237, 8, 250, 238, 140, 41, 183, 167, 194, 155, 217, 212, 149, 180, 185, 60, 194, 7, 189, 112, 202, 147, 247, 179, 86, 243, 156, 103, 126, 122, 176, 252];