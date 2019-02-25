use openimageio_sys as sys;

#[repr(u8)]
pub enum BaseType {
    Unknown = sys::OIIO_TypeDesc_BaseType_Unknown as u8,
    None = sys::OIIO_TypeDesc_BaseType_None as u8,
    //UChar=sys::OIIO_TypeDesc_BaseType_UChar as u8,
    UInt8 = sys::OIIO_TypeDesc_BaseType_UInt8 as u8,
    //Char=sys::OIIO_TypeDesc_BaseType_Char as u8,
    Int8 = sys::OIIO_TypeDesc_BaseType_Int8 as u8,
    //UShort=sys::OIIO_TypeDesc_BaseType_UShort as u8,
    UInt16 = sys::OIIO_TypeDesc_BaseType_UInt16 as u8,
    //Short=sys::OIIO_TypeDesc_BaseType_Short as u8,
    Int16 = sys::OIIO_TypeDesc_BaseType_Int16 as u8,
    //UInt=sys::OIIO_TypeDesc_BaseType_UInt as u8,
    UInt32 = sys::OIIO_TypeDesc_BaseType_UInt32 as u8,
    //Int=sys::OIIO_TypeDesc_BaseType_Int as u8,
    Int32 = sys::OIIO_TypeDesc_BaseType_Int32 as u8,
    //ULongLong=sys::OIIO_TypeDesc_BaseType_ULongLong as u8,
    UInt64 = sys::OIIO_TypeDesc_BaseType_UInt64 as u8,
    //LongLong=sys::OIIO_TypeDesc_BaseType_LongLong as u8,
    Int64 = sys::OIIO_TypeDesc_BaseType_Int64 as u8,
    Half = sys::OIIO_TypeDesc_BaseType_Half as u8,
    Float = sys::OIIO_TypeDesc_BaseType_Float as u8,
    Double = sys::OIIO_TypeDesc_BaseType_Double as u8,
    String = sys::OIIO_TypeDesc_BaseType_String as u8,
    Ptr = sys::OIIO_TypeDesc_BaseType_Ptr as u8,
}

#[repr(u8)]
pub enum Aggregate {
    Scalar = sys::OIIO_TypeDesc_Aggregate_Scalar as u8,
    Vec2 = sys::OIIO_TypeDesc_Aggregate_Vec2 as u8,
    Vec3 = sys::OIIO_TypeDesc_Aggregate_Vec3 as u8,
    Vec4 = sys::OIIO_TypeDesc_Aggregate_Vec4 as u8,
    Matrix33 = sys::OIIO_TypeDesc_Aggregate_Matrix33 as u8,
    Matrix44 = sys::OIIO_TypeDesc_Aggregate_Matrix44 as u8,
}

#[repr(u8)]
pub enum VecSemantics {
    /// No semantic hints
    NoSemantics = sys::OIIO_TypeDesc_VecSemantics_NoSemantics as u8,
    /// Color
    Color = sys::OIIO_TypeDesc_VecSemantics_Color as u8,
    /// Spatial location
    Point = sys::OIIO_TypeDesc_VecSemantics_Point as u8,
    /// Spatial direction
    Vector = sys::OIIO_TypeDesc_VecSemantics_Vector as u8,
    /// Surface normal
    Normal = sys::OIIO_TypeDesc_VecSemantics_Normal as u8,
    /// SMPTE timecode (should be int\[2\])
    Timecode = sys::OIIO_TypeDesc_VecSemantics_Timecode as u8,
    /// SMPTE keycode (should be int\[7\])
    Keycode = sys::OIIO_TypeDesc_VecSemantics_Keycode as u8,
    /// paired numerator and denominator
    Rational = sys::OIIO_TypeDesc_VecSemantics_Rational as u8,
}

/// Data type descriptions.
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct TypeDesc(pub(crate) sys::OIIO_TypeDesc);

impl TypeDesc {
    pub const UNKNOWN: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Unknown as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const FLOAT: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const DOUBLE: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Double as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const COLOR: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Vec3 as u8,
        vecsemantics: VecSemantics::Color as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const POINT: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Vec3 as u8,
        vecsemantics: VecSemantics::Point as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const VECTOR: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Vec3 as u8,
        vecsemantics: VecSemantics::Vector as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const NORMAL: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Vec3 as u8,
        vecsemantics: VecSemantics::Normal as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const MATRIX33: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Matrix33 as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const MATRIX44: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Matrix44 as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const MATRIX: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Matrix44 as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const STRING: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::String as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const I8: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Int8 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const U8: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::UInt8 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const I16: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Int16 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const U16: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::UInt16 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const I32: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Int32 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const U32: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::UInt32 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const BYTE: TypeDesc = Self::I8;
    pub const UBYTE: TypeDesc = Self::U8;
    pub const SHORT: TypeDesc = Self::I16;
    pub const USHORT: TypeDesc = Self::U16;
    pub const INT: TypeDesc = Self::I32;
    pub const UINT: TypeDesc = Self::U32;

    pub const HALF: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Half as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });

    pub const TIMECODE: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::UInt32 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::Timecode as u8,
        reserved: 0,
        arraylen: 2,
    });

    pub const KEYCODE: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Int32 as u8,
        aggregate: Aggregate::Scalar as u8,
        vecsemantics: VecSemantics::Keycode as u8,
        reserved: 0,
        arraylen: 7,
    });

    pub const FLOAT4: TypeDesc = TypeDesc(sys::OIIO_TypeDesc {
        basetype: BaseType::Float as u8,
        aggregate: Aggregate::Vec4 as u8,
        vecsemantics: VecSemantics::NoSemantics as u8,
        reserved: 0,
        arraylen: 0,
    });
}

pub trait ImageData {
    const DESC: TypeDesc;
}

impl ImageData for f32 {
    const DESC: TypeDesc = TypeDesc::FLOAT;
}

impl ImageData for u16 {
    const DESC: TypeDesc = TypeDesc::U16;
}

impl ImageData for i16 {
    const DESC: TypeDesc = TypeDesc::I16;
}

impl ImageData for u8 {
    const DESC: TypeDesc = TypeDesc::U8;
}

impl ImageData for i8 {
    const DESC: TypeDesc = TypeDesc::I8;
}
