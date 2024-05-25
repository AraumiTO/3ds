pub const CHUNK_INFO_SIZE: u64 = 6;

//>------ Primary chunk

pub const MAIN3DS: u16 = 0x4D4D;

//>------ Main Chunks

pub const MAIN_VERSION: u16 = 0x0002;
pub const MAIN_EDITOR: u16 = 0x3D3D;
pub const MAIN_KEYFRAMES: u16 = 0xB000;

//>------ sub defines of EDIT3DS

pub const EDIT_VERSION: u16 = 0x3D3E;
pub const EDIT_MATERIAL: u16 = 0xAFFF;
pub const EDIT_CONFIG1: u16 = 0x0100;
pub const EDIT_CONFIG2: u16 = 0x3E3D;
pub const EDIT_VIEW_P1: u16 = 0x7012;
pub const EDIT_VIEW_P2: u16 = 0x7011;
pub const EDIT_VIEW_P3: u16 = 0x7020;
pub const EDIT_VIEW1: u16 = 0x7001;
pub const EDIT_BACKGR: u16 = 0x1200;
pub const EDIT_AMBIENT: u16 = 0x2100;
pub const EDIT_OBJECT: u16 = 0x4000;

pub const EDIT_UNKNW01: u16 = 0x1100;
pub const EDIT_UNKNW02: u16 = 0x1201;
pub const EDIT_UNKNW03: u16 = 0x1300;
pub const EDIT_UNKNW04: u16 = 0x1400;
pub const EDIT_UNKNW05: u16 = 0x1420;
pub const EDIT_UNKNW06: u16 = 0x1450;
pub const EDIT_UNKNW07: u16 = 0x1500;
pub const EDIT_UNKNW08: u16 = 0x2200;
pub const EDIT_UNKNW09: u16 = 0x2201;
pub const EDIT_UNKNW10: u16 = 0x2210;
pub const EDIT_UNKNW11: u16 = 0x2300;
pub const EDIT_UNKNW12: u16 = 0x2302;
pub const EDIT_UNKNW13: u16 = 0x3000;
pub const EDIT_UNKNW14: u16 = 0xAFFF;

pub const MATERIAL_NAME: u16 = 0xA000;
pub const MATERIAL_TEXTURE_MAP: u16 = 0xA200;
pub const MATERIAL_TEXTURE_MAP_NAME: u16 = 0xA300;

//>------ sub defines of EDIT_OBJECT
pub const OBJ_TRIMESH: u16 = 0x4100;
pub const OBJ_LIGHT: u16 = 0x4600;
pub const OBJ_CAMERA: u16 = 0x4700;

pub const OBJ_UNKNWN01: u16 = 0x4010;
pub const OBJ_UNKNWN02: u16 = 0x4012;

//>------ sub defines of OBJ_CAMERA
pub const CAM_UNKNWN01: u16 = 0x4710;
pub const CAM_UNKNWN02: u16 = 0x4720;

//>------ sub defines of OBJ_LIGHT
pub const LIT_OFF: u16 = 0x4620;
pub const LIT_SPOT: u16 = 0x4610;
pub const LIT_UNKNWN01: u16 = 0x465A;

//>------ sub defines of OBJ_TRIMESH
pub const TRI_VERTEXL: u16 = 0x4110;
pub const TRI_FACEL2: u16 = 0x4111;
pub const TRI_FACEL1: u16 = 0x4120;
pub const TRI_SMOOTH: u16 = 0x4150;
pub const TRI_LOCAL: u16 = 0x4160;
pub const TRI_VISIBLE: u16 = 0x4165;

//>>------ sub defs of KEYF3DS

pub const KEYF_UNKNWN01: u16 = 0xB009;
pub const KEYF_UNKNWN02: u16 = 0xB00A;
pub const KEYF_FRAMES: u16 = 0xB008;
pub const KEYF_OBJDES: u16 = 0xB002;

//>>------  these define the different color chunk types
pub const COL_RGB: u16 = 0x0010;
pub const COL_TRU: u16 = 0x0011;
pub const COL_UNK: u16 = 0x0013;

//>>------ defines for viewport chunks

pub const TOP: u16 = 0x0001;
pub const BOTTOM: u16 = 0x0002;
pub const LEFT: u16 = 0x0003;
pub const RIGHT: u16 = 0x0004;
pub const FRONT: u16 = 0x0005;
pub const BACK: u16 = 0x0006;
pub const USER: u16 = 0x0007;
pub const CAMERA: u16 = 0x0008;
pub const LIGHT: u16 = 0x0009;
pub const DISABLED: u16 = 0x0010;
pub const BOGUS: u16 = 0x0011;
