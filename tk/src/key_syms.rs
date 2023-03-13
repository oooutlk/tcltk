use std::convert::TryFrom;

#[allow( non_camel_case_types )]
#[derive( strum_macros::Display )]
pub enum TkKey {
    //                      name      dec     hex
    Return                      ,
    space                       ,   // 32    0x20
    exclam                      ,   // 33    0x21
    quotedbl                    ,   // 34    0x22
    numbersign                  ,   // 35    0x23
    dollar                      ,   // 36    0x24
    percent                     ,   // 37    0x25
    ampersand                   ,   // 38    0x26
    apostrophe                  ,   // 39    0x27
    parenleft                   ,   // 40    0x28
    parenright                  ,   // 41    0x29
    asterisk                    ,   // 42    0x2A
    plus                        ,   // 43    0x2B
    comma                       ,   // 44    0x2C
    minus                       ,   // 45    0x2D
    period                      ,   // 46    0x2E
    slash                       ,   // 47    0x2F
    #[strum( serialize="0" )] _0,   // 48    0x30
    #[strum( serialize="1" )] _1,   // 49    0x31
    #[strum( serialize="2" )] _2,   // 50    0x32
    #[strum( serialize="3" )] _3,   // 51    0x33
    #[strum( serialize="4" )] _4,   // 52    0x34
    #[strum( serialize="5" )] _5,   // 53    0x35
    #[strum( serialize="6" )] _6,   // 54    0x36
    #[strum( serialize="7" )] _7,   // 55    0x37
    #[strum( serialize="8" )] _8,   // 56    0x38
    #[strum( serialize="9" )] _9,   // 57    0x39
    colon                       ,   // 58    0x3A
    semicolon                   ,   // 59    0x3B
    less                        ,   // 60    0x3C
    equal                       ,   // 61    0x3D
    greater                     ,   // 62    0x3E
    question                    ,   // 63    0x3F
    at                          ,   // 64    0x40
    A                           ,   // 65    0x41
    B                           ,   // 66    0x42
    C                           ,   // 67    0x43
    D                           ,   // 68    0x44
    E                           ,   // 69    0x45
    F                           ,   // 70    0x46
    G                           ,   // 71    0x47
    H                           ,   // 72    0x48
    I                           ,   // 73    0x49
    J                           ,   // 74    0x4A
    K                           ,   // 75    0x4B
    L                           ,   // 76    0x4C
    M                           ,   // 77    0x4D
    N                           ,   // 78    0x4E
    O                           ,   // 79    0x4F
    P                           ,   // 80    0x50
    Q                           ,   // 81    0x51
    R                           ,   // 82    0x52
    S                           ,   // 83    0x53
    T                           ,   // 84    0x54
    U                           ,   // 85    0x55
    V                           ,   // 86    0x56
    W                           ,   // 87    0x57
    X                           ,   // 88    0x58
    Y                           ,   // 89    0x59
    Z                           ,   // 90    0x5A
    bracketleft                 ,   // 91    0x5B
    backslash                   ,   // 92    0x5C
    bracketright                ,   // 93    0x5D
    asciicircum                 ,   // 94    0x5E
    underscore                  ,   // 95    0x5F
    grave                       ,   // 96    0x60
    a                           ,   // 97    0x61
    b                           ,   // 98    0x62
    c                           ,   // 99    0x63
    d                           ,   //100    0x64
    e                           ,   //101    0x65
    f                           ,   //102    0x66
    g                           ,   //103    0x67
    h                           ,   //104    0x68
    i                           ,   //105    0x69
    j                           ,   //106    0x6A
    k                           ,   //107    0x6B
    l                           ,   //108    0x6C
    m                           ,   //109    0x6D
    n                           ,   //110    0x6E
    o                           ,   //111    0x6F
    p                           ,   //112    0x70
    q                           ,   //113    0x71
    r                           ,   //114    0x72
    s                           ,   //115    0x73
    t                           ,   //116    0x74
    u                           ,   //117    0x75
    v                           ,   //118    0x76
    w                           ,   //119    0x77
    x                           ,   //120    0x78
    y                           ,   //121    0x79
    z                           ,   //122    0x7A
    braceleft                   ,   //123    0x7B
    bar                         ,   //124    0x7C
    braceright                  ,   //125    0x7D
    asciitilde                  ,   //126    0x7E
    nobreakspace                ,   //160    0xA0
    exclamdown                  ,   //161    0xA1
    cent                        ,   //162    0xA2
    sterling                    ,   //163    0xA3
    currency                    ,   //164    0xA4
    yen                         ,   //165    0xA5
    brokenbar                   ,   //166    0xA6
    section                     ,   //167    0xA7
    diaeresis                   ,   //168    0xA8
    copyright                   ,   //169    0xA9
    ordfeminine                 ,   //170    0xAA
    guillemotleft               ,   //171    0xAB
    notsign                     ,   //172    0xAC
    hyphen                      ,   //173    0xAD
    registered                  ,   //174    0xAE
    macron                      ,   //175    0xAF
    degree                      ,   //176    0xB0
    plusminus                   ,   //177    0xB1
    twosuperior                 ,   //178    0xB2
    threesuperior               ,   //179    0xB3
    acute                       ,   //180    0xB4
    mu                          ,   //181    0xB5
    paragraph                   ,   //182    0xB6
    periodcentere               ,   //183    0xB7
    cedilla                     ,   //184    0xB8
    onesuperior                 ,   //185    0xB9
    masculine                   ,   //186    0xBA
    guillemotrigh               ,   //187    0xBB
    onequarter                  ,   //188    0xBC
    onehalf                     ,   //189    0xBD
    threequarters               ,   //190    0xBE
    questiondown                ,   //191    0xBF
    Agrave                      ,   //192    0xC0
    Aacute                      ,   //193    0xC1
    Acircumflex                 ,   //194    0xC2
    Atilde                      ,   //195    0xC3
    Adiaeresis                  ,   //196    0xC4
    Aring                       ,   //197    0xC5
    AE                          ,   //198    0xC6
    Ccedilla                    ,   //199    0xC7
    Egrave                      ,   //200    0xC8
    Eacute                      ,   //201    0xC9
    Ecircumflex                 ,   //202    0xCA
    Ediaeresis                  ,   //203    0xCB
    Igrave                      ,   //204    0xCC
    Iacute                      ,   //205    0xCD
    Icircumflex                 ,   //206    0xCE
    Idiaeresis                  ,   //207    0xCF
    ETH                         ,   //208    0xD0
    Ntilde                      ,   //209    0xD1
    Ograve                      ,   //210    0xD2
    Oacute                      ,   //211    0xD3
    Ocircumflex                 ,   //212    0xD4
    Otilde                      ,   //213    0xD5
    Odiaeresis                  ,   //214    0xD6
    multiply                    ,   //215    0xD7
    Oslash                      ,   //216    0xD8
    Ugrave                      ,   //217    0xD9
    Uacute                      ,   //218    0xDA
    Ucircumflex                 ,   //219    0xDB
    Udiaeresis                  ,   //220    0xDC
    Yacute                      ,   //221    0xDD
    THORN                       ,   //222    0xDE
    ssharp                      ,   //223    0xDF
    agrave                      ,   //224    0xE0
    aacute                      ,   //225    0xE1
    acircumflex                 ,   //226    0xE2
    atilde                      ,   //227    0xE3
    adiaeresis                  ,   //228    0xE4
    aring                       ,   //229    0xE5
    ae                          ,   //230    0xE6
    ccedilla                    ,   //231    0xE7
    egrave                      ,   //232    0xE8
    eacute                      ,   //233    0xE9
    ecircumflex                 ,   //234    0xEA
    ediaeresis                  ,   //235    0xEB
    igrave                      ,   //236    0xEC
    iacute                      ,   //237    0xED
    icircumflex                 ,   //238    0xEE
    idiaeresis                  ,   //239    0xEF
    eth                         ,   //240    0xF0
    ntilde                      ,   //241    0xF1
    ograve                      ,   //242    0xF2
    oacute                      ,   //243    0xF3
    ocircumflex                 ,   //244    0xF4
    otilde                      ,   //245    0xF5
    odiaeresis                  ,   //246    0xF6
    division                    ,   //247    0xF7
    oslash                      ,   //248    0xF8
    ugrave                      ,   //249    0xF9
    uacute                      ,   //250    0xFA
    ucircumflex                 ,   //251    0xFB
    udiaeresis                  ,   //252    0xFC
    yacute                      ,   //253    0xFD
    thorn                       ,   //254    0xFE
    ydiaeresis                  ,   //255    0xFF
    Aogonek                     ,   //417    0x1A1
    breve                       ,   //418    0x1A2
    Lstroke                     ,   //419    0x1A3
    Lcaron                      ,   //421    0x1A5
    Sacute                      ,   //422    0x1A6
    Scaron                      ,   //425    0x1A9
    Scedilla                    ,   //426    0x1AA
    Tcaron                      ,   //427    0x1AB
    Zacute                      ,   //428    0x1AC
}

pub struct InvalidKeySym( char );

impl TryFrom<char> for TkKey {
    type Error = InvalidKeySym;

    fn try_from( ch: char ) -> Result<TkKey, Self::Error> {
        use TkKey::*;
        match ch {
            ' '  => Ok( space               ),
            '!'  => Ok( exclam              ),
            '"'  => Ok( quotedbl            ),
            '#'  => Ok( numbersign          ),
            '$'  => Ok( dollar              ),
            '%'  => Ok( percent             ),
            '&'  => Ok( ampersand           ),
            '\'' => Ok( apostrophe          ),
            '('  => Ok( parenleft           ),
            ')'  => Ok( parenright          ),
            '*'  => Ok( asterisk            ),
            '+'  => Ok( plus                ),
            ','  => Ok( comma               ),
            '-'  => Ok( minus               ),
            '.'  => Ok( period              ),
            '/'  => Ok( slash               ),
            '0'  => Ok( _0                  ),
            '1'  => Ok( _1                  ),
            '2'  => Ok( _2                  ),
            '3'  => Ok( _3                  ),
            '4'  => Ok( _4                  ),
            '5'  => Ok( _5                  ),
            '6'  => Ok( _6                  ),
            '7'  => Ok( _7                  ),
            '8'  => Ok( _8                  ),
            '9'  => Ok( _9                  ),
            ':'  => Ok( colon               ),
            ';'  => Ok( semicolon           ),
            '<'  => Ok( less                ),
            '='  => Ok( equal               ),
            '>'  => Ok( greater             ),
            '?'  => Ok( question            ),
            '@'  => Ok( at                  ),
            'A'  => Ok( A                   ),
            'B'  => Ok( B                   ),
            'C'  => Ok( C                   ),
            'D'  => Ok( D                   ),
            'E'  => Ok( E                   ),
            'F'  => Ok( F                   ),
            'G'  => Ok( G                   ),
            'H'  => Ok( H                   ),
            'I'  => Ok( I                   ),
            'J'  => Ok( J                   ),
            'K'  => Ok( K                   ),
            'L'  => Ok( L                   ),
            'M'  => Ok( M                   ),
            'N'  => Ok( N                   ),
            'O'  => Ok( O                   ),
            'P'  => Ok( P                   ),
            'Q'  => Ok( Q                   ),
            'R'  => Ok( R                   ),
            'S'  => Ok( S                   ),
            'T'  => Ok( T                   ),
            'U'  => Ok( U                   ),
            'V'  => Ok( V                   ),
            'W'  => Ok( W                   ),
            'X'  => Ok( X                   ),
            'Y'  => Ok( Y                   ),
            'Z'  => Ok( Z                   ),
            '['  => Ok( bracketleft         ),
            '\\' => Ok( backslash           ),
            ']'  => Ok( bracketright        ),
            '^'  => Ok( asciicircum         ),
            '_'  => Ok( underscore          ),
            '`'  => Ok( grave               ),
            'a'  => Ok( a                   ),
            'b'  => Ok( b                   ),
            'c'  => Ok( c                   ),
            'd'  => Ok( d                   ),
            'e'  => Ok( e                   ),
            'f'  => Ok( f                   ),
            'g'  => Ok( g                   ),
            'h'  => Ok( h                   ),
            'i'  => Ok( i                   ),
            'j'  => Ok( j                   ),
            'k'  => Ok( k                   ),
            'l'  => Ok( l                   ),
            'm'  => Ok( m                   ),
            'n'  => Ok( n                   ),
            'o'  => Ok( o                   ),
            'p'  => Ok( p                   ),
            'q'  => Ok( q                   ),
            'r'  => Ok( r                   ),
            's'  => Ok( s                   ),
            't'  => Ok( t                   ),
            'u'  => Ok( u                   ),
            'v'  => Ok( v                   ),
            'w'  => Ok( w                   ),
            'x'  => Ok( x                   ),
            'y'  => Ok( y                   ),
            'z'  => Ok( z                   ),
            '{'  => Ok( braceleft           ),
            '|'  => Ok( bar                 ),
            '}'  => Ok( braceright          ),
            '~'  => Ok( asciitilde          ),
            _    => Err( InvalidKeySym( ch )),
        }
    }
}
