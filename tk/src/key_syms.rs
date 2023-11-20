use std::convert::TryFrom;

#[allow( non_camel_case_types )]
#[derive( strum_macros::Display )]
pub enum TkKey {
    //                      name      dec        hex
    space                       ,   // 32       0x20
    exclam                      ,   // 33       0x21
    quotedbl                    ,   // 34       0x22
    numbersign                  ,   // 35       0x23
    dollar                      ,   // 36       0x24
    percent                     ,   // 37       0x25
    ampersand                   ,   // 38       0x26
    apostrophe                  ,   // 39       0x27
    parenleft                   ,   // 40       0x28
    parenright                  ,   // 41       0x29
    asterisk                    ,   // 42       0x2A
    plus                        ,   // 43       0x2B
    comma                       ,   // 44       0x2C
    minus                       ,   // 45       0x2D
    period                      ,   // 46       0x2E
    slash                       ,   // 47       0x2F
    #[strum( serialize="0" )] _0,   // 48       0x30
    #[strum( serialize="1" )] _1,   // 49       0x31
    #[strum( serialize="2" )] _2,   // 50       0x32
    #[strum( serialize="3" )] _3,   // 51       0x33
    #[strum( serialize="4" )] _4,   // 52       0x34
    #[strum( serialize="5" )] _5,   // 53       0x35
    #[strum( serialize="6" )] _6,   // 54       0x36
    #[strum( serialize="7" )] _7,   // 55       0x37
    #[strum( serialize="8" )] _8,   // 56       0x38
    #[strum( serialize="9" )] _9,   // 57       0x39
    colon                       ,   // 58       0x3A
    semicolon                   ,   // 59       0x3B
    less                        ,   // 60       0x3C
    equal                       ,   // 61       0x3D
    greater                     ,   // 62       0x3E
    question                    ,   // 63       0x3F
    at                          ,   // 64       0x40
    A                           ,   // 65       0x41
    B                           ,   // 66       0x42
    C                           ,   // 67       0x43
    D                           ,   // 68       0x44
    E                           ,   // 69       0x45
    F                           ,   // 70       0x46
    G                           ,   // 71       0x47
    H                           ,   // 72       0x48
    I                           ,   // 73       0x49
    J                           ,   // 74       0x4A
    K                           ,   // 75       0x4B
    L                           ,   // 76       0x4C
    M                           ,   // 77       0x4D
    N                           ,   // 78       0x4E
    O                           ,   // 79       0x4F
    P                           ,   // 80       0x50
    Q                           ,   // 81       0x51
    R                           ,   // 82       0x52
    S                           ,   // 83       0x53
    T                           ,   // 84       0x54
    U                           ,   // 85       0x55
    V                           ,   // 86       0x56
    W                           ,   // 87       0x57
    X                           ,   // 88       0x58
    Y                           ,   // 89       0x59
    Z                           ,   // 90       0x5A
    bracketleft                 ,   // 91       0x5B
    backslash                   ,   // 92       0x5C
    bracketright                ,   // 93       0x5D
    asciicircum                 ,   // 94       0x5E
    underscore                  ,   // 95       0x5F
    grave                       ,   // 96       0x60
    a                           ,   // 97       0x61
    b                           ,   // 98       0x62
    c                           ,   // 99       0x63
    d                           ,   //100       0x64
    e                           ,   //101       0x65
    f                           ,   //102       0x66
    g                           ,   //103       0x67
    h                           ,   //104       0x68
    i                           ,   //105       0x69
    j                           ,   //106       0x6A
    k                           ,   //107       0x6B
    l                           ,   //108       0x6C
    m                           ,   //109       0x6D
    n                           ,   //110       0x6E
    o                           ,   //111       0x6F
    p                           ,   //112       0x70
    q                           ,   //113       0x71
    r                           ,   //114       0x72
    s                           ,   //115       0x73
    t                           ,   //116       0x74
    u                           ,   //117       0x75
    v                           ,   //118       0x76
    w                           ,   //119       0x77
    x                           ,   //120       0x78
    y                           ,   //121       0x79
    z                           ,   //122       0x7A
    braceleft                   ,   //123       0x7B
    bar                         ,   //124       0x7C
    braceright                  ,   //125       0x7D
    asciitilde                  ,   //126       0x7E
    nobreakspace                ,   //160       0xA0
    exclamdown                  ,   //161       0xA1
    cent                        ,   //162       0xA2
    sterling                    ,   //163       0xA3
    currency                    ,   //164       0xA4
    yen                         ,   //165       0xA5
    brokenbar                   ,   //166       0xA6
    section                     ,   //167       0xA7
    diaeresis                   ,   //168       0xA8
    copyright                   ,   //169       0xA9
    ordfeminine                 ,   //170       0xAA
    guillemotleft               ,   //171       0xAB
    notsign                     ,   //172       0xAC
    hyphen                      ,   //173       0xAD
    registered                  ,   //174       0xAE
    macron                      ,   //175       0xAF
    degree                      ,   //176       0xB0
    plusminus                   ,   //177       0xB1
    twosuperior                 ,   //178       0xB2
    threesuperior               ,   //179       0xB3
    acute                       ,   //180       0xB4
    mu                          ,   //181       0xB5
    paragraph                   ,   //182       0xB6
    periodcentere               ,   //183       0xB7
    cedilla                     ,   //184       0xB8
    onesuperior                 ,   //185       0xB9
    masculine                   ,   //186       0xBA
    guillemotrigh               ,   //187       0xBB
    onequarter                  ,   //188       0xBC
    onehalf                     ,   //189       0xBD
    threequarters               ,   //190       0xBE
    questiondown                ,   //191       0xBF
    Agrave                      ,   //192       0xC0
    Aacute                      ,   //193       0xC1
    Acircumflex                 ,   //194       0xC2
    Atilde                      ,   //195       0xC3
    Adiaeresis                  ,   //196       0xC4
    Aring                       ,   //197       0xC5
    AE                          ,   //198       0xC6
    Ccedilla                    ,   //199       0xC7
    Egrave                      ,   //200       0xC8
    Eacute                      ,   //201       0xC9
    Ecircumflex                 ,   //202       0xCA
    Ediaeresis                  ,   //203       0xCB
    Igrave                      ,   //204       0xCC
    Iacute                      ,   //205       0xCD
    Icircumflex                 ,   //206       0xCE
    Idiaeresis                  ,   //207       0xCF
    ETH                         ,   //208       0xD0
    Ntilde                      ,   //209       0xD1
    Ograve                      ,   //210       0xD2
    Oacute                      ,   //211       0xD3
    Ocircumflex                 ,   //212       0xD4
    Otilde                      ,   //213       0xD5
    Odiaeresis                  ,   //214       0xD6
    multiply                    ,   //215       0xD7
    Oslash                      ,   //216       0xD8
    Ugrave                      ,   //217       0xD9
    Uacute                      ,   //218       0xDA
    Ucircumflex                 ,   //219       0xDB
    Udiaeresis                  ,   //220       0xDC
    Yacute                      ,   //221       0xDD
    THORN                       ,   //222       0xDE
    ssharp                      ,   //223       0xDF
    agrave                      ,   //224       0xE0
    aacute                      ,   //225       0xE1
    acircumflex                 ,   //226       0xE2
    atilde                      ,   //227       0xE3
    adiaeresis                  ,   //228       0xE4
    aring                       ,   //229       0xE5
    ae                          ,   //230       0xE6
    ccedilla                    ,   //231       0xE7
    egrave                      ,   //232       0xE8
    eacute                      ,   //233       0xE9
    ecircumflex                 ,   //234       0xEA
    ediaeresis                  ,   //235       0xEB
    igrave                      ,   //236       0xEC
    iacute                      ,   //237       0xED
    icircumflex                 ,   //238       0xEE
    idiaeresis                  ,   //239       0xEF
    eth                         ,   //240       0xF0
    ntilde                      ,   //241       0xF1
    ograve                      ,   //242       0xF2
    oacute                      ,   //243       0xF3
    ocircumflex                 ,   //244       0xF4
    otilde                      ,   //245       0xF5
    odiaeresis                  ,   //246       0xF6
    division                    ,   //247       0xF7
    oslash                      ,   //248       0xF8
    ugrave                      ,   //249       0xF9
    uacute                      ,   //250       0xFA
    ucircumflex                 ,   //251       0xFB
    udiaeresis                  ,   //252       0xFC
    yacute                      ,   //253       0xFD
    thorn                       ,   //254       0xFE
    ydiaeresis                  ,   //255       0xFF
    Aogonek                     ,   //417       0x1A1
    breve                       ,   //418       0x1A2
    Lstroke                     ,   //419       0x1A3
    Lcaron                      ,   //421       0x1A5
    Sacute                      ,   //422       0x1A6
    Scaron                      ,   //425       0x1A9
    Scedilla                    ,   //426       0x1AA
    Tcaron                      ,   //427       0x1AB
    Zacute                      ,   //428       0x1AC
    Zcaron                      ,   //430       0x1AE
    Zabovedot                   ,   //431       0x1AF
    aogonek                     ,   //433       0x1B1
    ogonek                      ,   //434       0x1B2
    lstroke                     ,   //435       0x1B3
    lcaron                      ,   //437       0x1B5
    sacute                      ,   //438       0x1B6
    caron                       ,   //439       0x1B7
    scaron                      ,   //441       0x1B9
    scedilla                    ,   //442       0x1BA
    tcaron                      ,   //443       0x1BB
    zacute                      ,   //444       0x1BC
    doubleacute                 ,   //445       0x1BD
    zcaron                      ,   //446       0x1BE
    zabovedot                   ,   //447       0x1BF
    Racute                      ,   //448       0x1C0
    Abreve                      ,   //451       0x1C3
    Lacute                      ,   //453       0x1C5
    Cacute                      ,   //454       0x1C6
    Ccaron                      ,   //456       0x1C8
    Eogonek                     ,   //458       0x1CA
    Ecaron                      ,   //460       0x1CC
    Dcaron                      ,   //463       0x1CF
    Dstroke                     ,   //464       0x1D0
    Nacute                      ,   //465       0x1D1
    Ncaron                      ,   //466       0x1D2
    Odoubleacute                ,   //469       0x1D5
    Rcaron                      ,   //472       0x1D8
    Uring                       ,   //473       0x1D9
    Udoubleacute                ,   //475       0x1DB
    Tcedilla                    ,   //478       0x1DE
    racute                      ,   //480       0x1E0
    abreve                      ,   //483       0x1E3
    lacute                      ,   //485       0x1E5
    cacute                      ,   //486       0x1E6
    ccaron                      ,   //488       0x1E8
    eogonek                     ,   //490       0x1EA
    ecaron                      ,   //492       0x1EC
    dcaron                      ,   //495       0x1EF
    dstroke                     ,   //496       0x1F0
    nacute                      ,   //497       0x1F1
    ncaron                      ,   //498       0x1F2
    odoubleacute                ,   //501       0x1F5
    rcaron                      ,   //504       0x1F8
    uring                       ,   //505       0x1F9
    udoubleacute                ,   //507       0x1FB
    tcedilla                    ,   //510       0x1FE
    abovedot                    ,   //511       0x1FF
    Hstroke                     ,   //673       0x2A1
    Hcircumflex                 ,   //678       0x2A6
    Iabovedot                   ,   //681       0x2A9
    Gbreve                      ,   //683       0x2AB
    Jcircumflex                 ,   //684       0x2AC
    hstroke                     ,   //689       0x2B1
    hcircumflex                 ,   //694       0x2B6
    idotless                    ,   //697       0x2B9
    gbreve                      ,   //699       0x2BB
    jcircumflex                 ,   //700       0x2BC
    Cabovedot                   ,   //709       0x2C5
    Ccircumflex                 ,   //710       0x2C6
    Gabovedot                   ,   //725       0x2D5
    Gcircumflex                 ,   //728       0x2D8
    Ubreve                      ,   //733       0x2DD
    Scircumflex                 ,   //734       0x2DE
    cabovedot                   ,   //741       0x2E5
    ccircumflex                 ,   //742       0x2E6
    gabovedot                   ,   //757       0x2F5
    gcircumflex                 ,   //760       0x2F8
    ubreve                      ,   //765       0x2FD
    scircumflex                 ,   //766       0x2FE
    kra                         ,   //930       0x3A2
    Rcedilla                    ,   //931       0x3A3
    Itilde                      ,   //933       0x3A5
    Lcedilla                    ,   //934       0x3A6
    Emacron                     ,   //938       0x3AA
    Gcedilla                    ,   //939       0x3AB
    Tslash                      ,   //940       0x3AC
    rcedilla                    ,   //947       0x3B3
    itilde                      ,   //949       0x3B5
    lcedilla                    ,   //950       0x3B6
    emacron                     ,   //954       0x3BA
    gcedilla                    ,   //955       0x3BB
    tslash                      ,   //956       0x3BC
    ENG                         ,   //957       0x3BD
    eng                         ,   //959       0x3BF
    Amacron                     ,   //960       0x3C0
    Iogonek                     ,   //967       0x3C7
    Eabovedot                   ,   //972       0x3CC
    Imacron                     ,   //975       0x3CF
    Ncedilla                    ,   //977       0x3D1
    Omacron                     ,   //978       0x3D2
    Kcedilla                    ,   //979       0x3D3
    Uogonek                     ,   //985       0x3D9
    Utilde                      ,   //989       0x3DD
    Umacron                     ,   //990       0x3DE
    amacron                     ,   //992       0x3E0
    iogonek                     ,   //999       0x3E7
    eabovedot                   ,   //004       0x3EC
    imacron                     ,   //007       0x3EF
    ncedilla                    ,   //009       0x3F1
    omacron                     ,   //010       0x3F2
    kcedilla                    ,   //1011      0x3F3
    uogonek                     ,   //1017      0x3F9
    utilde                      ,   //1021      0x3FD
    umacron                     ,   //1022      0x3FE
    overline                    ,   //1150      0x47E
    kana_fullstop               ,   //1185      0x4A1
    kana_openingbracket         ,   //1186      0x4A2
    kana_closingbracket         ,   //1187      0x4A3
    kana_comma                  ,   //1188      0x4A4
    kana_conjunctive            ,   //1189      0x4A5
    kana_WO                     ,   //1190      0x4A6
    kana_a                      ,   //1191      0x4A7
    kana_i                      ,   //1192      0x4A8
    kana_u                      ,   //1193      0x4A9
    kana_e                      ,   //1194      0x4AA
    kana_o                      ,   //1195      0x4AB
    kana_ya                     ,   //1196      0x4AC
    kana_yu                     ,   //1197      0x4AD
    kana_yo                     ,   //1198      0x4AE
    kana_tsu                    ,   //1199      0x4AF
    prolongedsound              ,   //1200      0x4B0
    kana_A                      ,   //1201      0x4B1
    kana_I                      ,   //1202      0x4B2
    kana_U                      ,   //1203      0x4B3
    kana_E                      ,   //1204      0x4B4
    kana_O                      ,   //1205      0x4B5
    kana_KA                     ,   //1206      0x4B6
    kana_KI                     ,   //1207      0x4B7
    kana_KU                     ,   //1208      0x4B8
    kana_KE                     ,   //1209      0x4B9
    kana_KO                     ,   //1210      0x4BA
    kana_SA                     ,   //1211      0x4BB
    kana_SHI                    ,   //1212      0x4BC
    kana_SU                     ,   //1213      0x4BD
    kana_SE                     ,   //1214      0x4BE
    kana_SO                     ,   //1215      0x4BF
    kana_TA                     ,   //1216      0x4C0
    kana_CHI                    ,   //1217      0x4C1
    kana_TSU                    ,   //1218      0x4C2
    kana_TE                     ,   //1219      0x4C3
    kana_TO                     ,   //1220      0x4C4
    kana_NA                     ,   //1221      0x4C5
    kana_NI                     ,   //1222      0x4C6
    kana_NU                     ,   //1223      0x4C7
    kana_NE                     ,   //1224      0x4C8
    kana_NO                     ,   //1225      0x4C9
    kana_HA                     ,   //1226      0x4CA
    kana_HI                     ,   //1227      0x4CB
    kana_FU                     ,   //1228      0x4CC
    kana_HE                     ,   //1229      0x4CD
    kana_HO                     ,   //1230      0x4CE
    kana_MA                     ,   //1231      0x4CF
    kana_MI                     ,   //1232      0x4D0
    kana_MU                     ,   //1233      0x4D1
    kana_ME                     ,   //1234      0x4D2
    kana_MO                     ,   //1235      0x4D3
    kana_YA                     ,   //1236      0x4D4
    kana_YU                     ,   //1237      0x4D5
    kana_YO                     ,   //1238      0x4D6
    kana_RA                     ,   //1239      0x4D7
    kana_RI                     ,   //1240      0x4D8
    kana_RU                     ,   //1241      0x4D9
    kana_RE                     ,   //1242      0x4DA
    kana_RO                     ,   //1243      0x4DB
    kana_WA                     ,   //1244      0x4DC
    kana_N                      ,   //1245      0x4DD
    voicedsound                 ,   //1246      0x4DE
    semivoicedsound             ,   //1247      0x4DF

    Arabic_comma                ,   //1452      0x5AC
    Arabic_semicolon            ,   //1467      0x5BB
    Arabic_question_mark        ,   //1471      0x5BF
    Arabic_hamza                ,   //1473      0x5C1
    Arabic_maddaonalef          ,   //1474      0x5C2
    Arabic_hamzaonalef          ,   //1475      0x5C3
    Arabic_hamzaonwaw           ,   //1476      0x5C4
    Arabic_hamzaunderalef       ,   //1477      0x5C5
    Arabic_hamzaonyeh           ,   //1478      0x5C6
    Arabic_alef                 ,   //1479      0x5C7
    Arabic_beh                  ,   //1480      0x5C8
    Arabic_tehmarbuta           ,   //1481      0x5C9
    Arabic_teh                  ,   //1482      0x5CA
    Arabic_theh                 ,   //1483      0x5CB
    Arabic_jeem                 ,   //1484      0x5CC
    Arabic_hah                  ,   //1485      0x5CD
    Arabic_khah                 ,   //1486      0x5CE
    Arabic_dal                  ,   //1487      0x5CF
    Arabic_thal                 ,   //1488      0x5D0
    Arabic_ra                   ,   //1489      0x5D1
    Arabic_zain                 ,   //1490      0x5D2
    Arabic_seen                 ,   //1491      0x5D3
    Arabic_sheen                ,   //1492      0x5D4
    Arabic_sad                  ,   //1493      0x5D5
    Arabic_dad                  ,   //1494      0x5D6
    Arabic_tah                  ,   //1495      0x5D7
    Arabic_zah                  ,   //1496      0x5D8
    Arabic_ain                  ,   //1497      0x5D9
    Arabic_ghain                ,   //1498      0x5DA
    Arabic_tatweel              ,   //1504      0x5E0
    Arabic_feh                  ,   //1505      0x5E1
    Arabic_qaf                  ,   //1506      0x5E2
    Arabic_kaf                  ,   //1507      0x5E3
    Arabic_lam                  ,   //1508      0x5E4
    Arabic_meem                 ,   //1509      0x5E5
    Arabic_noon                 ,   //1510      0x5E6
    Arabic_ha                   ,   //1511      0x5E7
    Arabic_waw                  ,   //1512      0x5E8
    Arabic_alefmaksura          ,   //1513      0x5E9
    Arabic_yeh                  ,   //1514      0x5EA
    Arabic_fathatan             ,   //1515      0x5EB
    Arabic_dammatan             ,   //1516      0x5EC
    Arabic_kasratan             ,   //1517      0x5ED
    Arabic_fatha                ,   //1518      0x5EE
    Arabic_damma                ,   //1519      0x5EF
    Arabic_kasra                ,   //1520      0x5F0
    Arabic_shadda               ,   //1521      0x5F1
    Arabic_sukun                ,   //1522      0x5F2
    Serbian_dje                 ,   //1697      0x6A1
    Macedonia_gje               ,   //1698      0x6A2
    Cyrillic_io                 ,   //1699      0x6A3
    Ukrainian_ie                ,   //1700      0x6A4
    Macedonia_dse               ,   //1701      0x6A5
    Ukrainian_i                 ,   //1702      0x6A6
    Ukrainian_yi                ,   //1703      0x6A7
    Cyrillic_je                 ,   //1704      0x6A8
    Cyrillic_lje                ,   //1705      0x6A9
    Cyrillic_nje                ,   //1706      0x6AA
    Serbian_tshe                ,   //1707      0x6AB
    Macedonia_kje               ,   //1708      0x6AC
    Ukrainian_ghe_with_upturn   ,   //1709      0x6AD
    Byelorussian_shortu         ,   //1710      0x6AE
    Cyrillic_dzhe               ,   //1711      0x6AF
    numerosign                  ,   //1712      0x6B0
    Serbian_DJE                 ,   //1713      0x6B1
    Macedonia_GJE               ,   //1714      0x6B2
    Cyrillic_IO                 ,   //1715      0x6B3
    Ukrainian_IE                ,   //1716      0x6B4
    Macedonia_DSE               ,   //1717      0x6B5
    Ukrainian_I                 ,   //1718      0x6B6
    Ukrainian_YI                ,   //1719      0x6B7
    Cyrillic_JE                 ,   //1720      0x6B8
    Cyrillic_LJE                ,   //1721      0x6B9
    Cyrillic_NJE                ,   //1722      0x6BA
    Serbian_TSHE                ,   //1723      0x6BB
    Macedonia_KJE               ,   //1724      0x6BC
    Ukrainian_GHE_WITH_UPTURN   ,   //1725      0x6BD
    Byelorussian_SHORTU         ,   //1726      0x6BE
    Cyrillic_DZHE               ,   //1727      0x6BF
    Cyrillic_yu                 ,   //1728      0x6C0
    Cyrillic_a                  ,   //1729      0x6C1
    Cyrillic_be                 ,   //1730      0x6C2
    Cyrillic_tse                ,   //1731      0x6C3
    Cyrillic_de                 ,   //1732      0x6C4
    Cyrillic_ie                 ,   //1733      0x6C5
    Cyrillic_ef                 ,   //1734      0x6C6
    Cyrillic_ghe                ,   //1735      0x6C7
    Cyrillic_ha                 ,   //1736      0x6C8
    Cyrillic_i                  ,   //1737      0x6C9
    Cyrillic_shorti             ,   //1738      0x6CA
    Cyrillic_ka                 ,   //1739      0x6CB
    Cyrillic_el                 ,   //1740      0x6CC
    Cyrillic_em                 ,   //1741      0x6CD
    Cyrillic_en                 ,   //1742      0x6CE
    Cyrillic_o                  ,   //1743      0x6CF
    Cyrillic_pe                 ,   //1744      0x6D0
    Cyrillic_ya                 ,   //1745      0x6D1
    Cyrillic_er                 ,   //1746      0x6D2
    Cyrillic_es                 ,   //1747      0x6D3
    Cyrillic_te                 ,   //1748      0x6D4
    Cyrillic_u                  ,   //1749      0x6D5
    Cyrillic_zhe                ,   //1750      0x6D6
    Cyrillic_ve                 ,   //1751      0x6D7
    Cyrillic_softsign           ,   //1752      0x6D8
    Cyrillic_yeru               ,   //1753      0x6D9
    Cyrillic_ze                 ,   //1754      0x6DA
    Cyrillic_sha                ,   //1755      0x6DB
    Cyrillic_e                  ,   //1756      0x6DC
    Cyrillic_shcha              ,   //1757      0x6DD
    Cyrillic_che                ,   //1758      0x6DE
    Cyrillic_hardsign           ,   //1759      0x6DF
    Cyrillic_YU                 ,   //1760      0x6E0
    Cyrillic_A                  ,   //1761      0x6E1
    Cyrillic_BE                 ,   //1762      0x6E2
    Cyrillic_TSE                ,   //1763      0x6E3
    Cyrillic_DE                 ,   //1764      0x6E4
    Cyrillic_IE                 ,   //1765      0x6E5
    Cyrillic_EF                 ,   //1766      0x6E6
    Cyrillic_GHE                ,   //1767      0x6E7
    Cyrillic_HA                 ,   //1768      0x6E8
    Cyrillic_I                  ,   //1769      0x6E9
    Cyrillic_SHORTI             ,   //1770      0x6EA
    Cyrillic_KA                 ,   //1771      0x6EB
    Cyrillic_EL                 ,   //1772      0x6EC
    Cyrillic_EM                 ,   //1773      0x6ED
    Cyrillic_EN                 ,   //1774      0x6EE
    Cyrillic_O                  ,   //1775      0x6EF
    Cyrillic_PE                 ,   //1776      0x6F0
    Cyrillic_YA                 ,   //1777      0x6F1
    Cyrillic_ER                 ,   //1778      0x6F2
    Cyrillic_ES                 ,   //1779      0x6F3
    Cyrillic_TE                 ,   //1780      0x6F4
    Cyrillic_U                  ,   //1781      0x6F5
    Cyrillic_ZHE                ,   //1782      0x6F6
    Cyrillic_VE                 ,   //1783      0x6F7
    Cyrillic_SOFTSIGN           ,   //1784      0x6F8
    Cyrillic_YERU               ,   //1785      0x6F9
    Cyrillic_ZE                 ,   //1786      0x6FA
    Cyrillic_SHA                ,   //1787      0x6FB
    Cyrillic_E                  ,   //1788      0x6FC
    Cyrillic_SHCHA              ,   //1789      0x6FD
    Cyrillic_CHE                ,   //1790      0x6FE
    Cyrillic_HARDSIGN           ,   //1791      0x6FF
    Greek_ALPHAaccent           ,   //1953      0x7A1
    Greek_EPSILONaccent         ,   //1954      0x7A2
    Greek_ETAaccent             ,   //1955      0x7A3
    Greek_IOTAaccent            ,   //1956      0x7A4
    Greek_IOTAdieresis          ,   //1957      0x7A5
    Greek_IOTAaccentdiaeresis   ,   //1958      0x7A6
    Greek_OMICRONaccent         ,   //1959      0x7A7
    Greek_UPSILONaccent         ,   //1960      0x7A8
    Greek_UPSILONdieresis       ,   //1961      0x7A9
    Greek_UPSILONaccentdieresis ,   //1962      0x7AA
    Greek_OMEGAaccent           ,   //1963      0x7AB
    Greek_accentdieresis        ,   //1966      0x7AE
    Greek_horizbar              ,   //1967      0x7AF
    Greek_alphaaccent           ,   //1969      0x7B1
    Greek_epsilonaccent         ,   //1970      0x7B2
    Greek_etaaccent             ,   //1971      0x7B3
    Greek_iotaaccent            ,   //1972      0x7B4
    Greek_iotadieresis          ,   //1973      0x7B5
    Greek_iotaaccentdieresis    ,   //1974      0x7B6
    Greek_omicronaccent         ,   //1975      0x7B7
    Greek_upsilonaccent         ,   //1976      0x7B8
    Greek_upsilondieresis       ,   //1977      0x7B9
    Greek_upsilonaccentdieresis ,   //1978      0x7BA
    Greek_omegaaccent           ,   //1979      0x7BB
    Greek_ALPHA                 ,   //1985      0x7C1
    Greek_BETA                  ,   //1986      0x7C2
    Greek_GAMMA                 ,   //1987      0x7C3
    Greek_DELTA                 ,   //1988      0x7C4
    Greek_EPSILON               ,   //1989      0x7C5
    Greek_ZETA                  ,   //1990      0x7C6
    Greek_ETA                   ,   //1991      0x7C7
    Greek_THETA                 ,   //1992      0x7C8
    Greek_IOTA                  ,   //1993      0x7C9
    Greek_KAPPA                 ,   //1994      0x7CA
    Greek_LAMDA                 ,   //1995      0x7CB
    Greek_MU                    ,   //1996      0x7CC
    Greek_NU                    ,   //1997      0x7CD
    Greek_XI                    ,   //1998      0x7CE
    Greek_OMICRON               ,   //1999      0x7CF
    Greek_PI                    ,   //2000      0x7D0
    Greek_RHO                   ,   //2001      0x7D1
    Greek_SIGMA                 ,   //2002      0x7D2
    Greek_TAU                   ,   //2004      0x7D4
    Greek_UPSILON               ,   //2005      0x7D5
    Greek_PHI                   ,   //2006      0x7D6
    Greek_CHI                   ,   //2007      0x7D7
    Greek_PSI                   ,   //2008      0x7D8
    Greek_OMEGA                 ,   //2009      0x7D9
    Greek_alpha                 ,   //2017      0x7E1
    Greek_beta                  ,   //2018      0x7E2
    Greek_gamma                 ,   //2019      0x7E3
    Greek_delta                 ,   //2020      0x7E4
    Greek_epsilon               ,   //2021      0x7E5
    Greek_zeta                  ,   //2022      0x7E6
    Greek_eta                   ,   //2023      0x7E7
    Greek_theta                 ,   //2024      0x7E8
    Greek_iota                  ,   //2025      0x7E9
    Greek_kappa                 ,   //2026      0x7EA
    Greek_lamda                 ,   //2027      0x7EB
    Greek_mu                    ,   //2028      0x7EC
    Greek_nu                    ,   //2029      0x7ED
    Greek_xi                    ,   //2030      0x7EE
    Greek_omicron               ,   //2031      0x7EF
    Greek_pi                    ,   //2032      0x7F0
    Greek_rho                   ,   //2033      0x7F1
    Greek_sigma                 ,   //2034      0x7F2
    Greek_finalsmallsigma       ,   //2035      0x7F3
    Greek_tau                   ,   //2036      0x7F4
    Greek_upsilon               ,   //2037      0x7F5
    Greek_phi                   ,   //2038      0x7F6
    Greek_chi                   ,   //2039      0x7F7
    Greek_psi                   ,   //2040      0x7F8
    Greek_omega                 ,   //2041      0x7F9

    leftradical                 ,   //2209      0x8A1
    topleftradical              ,   //2210      0x8A2
    horizconnector              ,   //2211      0x8A3
    topintegral                 ,   //2212      0x8A4
    botintegral                 ,   //2213      0x8A5
    vertconnector               ,   //2214      0x8A6
    topleftsqbracket            ,   //2215      0x8A7
    botleftsqbracket            ,   //2216      0x8A8
    toprightsqbracket           ,   //2217      0x8A9
    botrightsqbracket           ,   //2218      0x8AA
    topleftparens               ,   //2219      0x8AB
    botleftparens               ,   //2220      0x8AC
    toprightparens              ,   //2221      0x8AD
    botrightparens              ,   //2222      0x8AE
    leftmiddlecurlybrace        ,   //2223      0x8AF
    rightmiddlecurlybrace       ,   //2224      0x8B0
    topleftsummation            ,   //2225      0x8B1
    botleftsummation            ,   //2226      0x8B2
    topvertsummationconnector   ,   //2227      0x8B3
    botvertsummationconnector   ,   //2228      0x8B4
    toprightsummation           ,   //2229      0x8B5
    botrightsummation           ,   //2230      0x8B6
    rightmiddlesummation        ,   //2231      0x8B7
    lessthanequal               ,   //2236      0x8BC
    notequal                    ,   //2237      0x8BD
    greaterthanequal            ,   //2238      0x8BE
    integral                    ,   //2239      0x8BF
    therefore                   ,   //2240      0x8C0
    variation                   ,   //2241      0x8C1
    infinity                    ,   //2242      0x8C2
    nabla                       ,   //2245      0x8C5
    approximate                 ,   //2248      0x8C8
    similarequal                ,   //2249      0x8C9
    ifonlyif                    ,   //2253      0x8CD
    implies                     ,   //2254      0x8CE
    identical                   ,   //2255      0x8CF
    radical                     ,   //2262      0x8D6
    includedin                  ,   //2266      0x8DA
    includes                    ,   //2267      0x8DB
    intersection                ,   //2268      0x8DC
    union                       ,   //2269      0x8DD
    logicaland                  ,   //2270      0x8DE
    logicalor                   ,   //2271      0x8DF
    partialderivative           ,   //2287      0x8EF
    function                    ,   //2294      0x8F6
    leftarrow                   ,   //2299      0x8FB
    uparrow                     ,   //2300      0x8FC
    rightarrow                  ,   //2301      0x8FD
    downarrow                   ,   //2302      0x8FE
    blank                       ,   //2527      0x9DF
    soliddiamond                ,   //2528      0x9E0
    checkerboard                ,   //2529      0x9E1
    ht                          ,   //2530      0x9E2
    ff                          ,   //2531      0x9E3
    cr                          ,   //2532      0x9E4
    lf                          ,   //2533      0x9E5
    nl                          ,   //2536      0x9E8
    vt                          ,   //2537      0x9E9
    lowrightcorner              ,   //2538      0x9EA
    uprightcorner               ,   //2539      0x9EB
    upleftcorner                ,   //2540      0x9EC
    lowleftcorner               ,   //2541      0x9ED
    crossinglines               ,   //2542      0x9EE
    horizlinescan1              ,   //2543      0x9EF
    horizlinescan3              ,   //2544      0x9F0
    horizlinescan5              ,   //2545      0x9F1
    horizlinescan7              ,   //2546      0x9F2
    horizlinescan9              ,   //2547      0x9F3
    leftt                       ,   //2548      0x9F4
    rightt                      ,   //2549      0x9F5
    bott                        ,   //2550      0x9F6
    topt                        ,   //2551      0x9F7
    vertbar                     ,   //2552      0x9F8
    emspace                     ,   //2721      0xAA1
    enspace                     ,   //2722      0xAA2
    em3space                    ,   //2723      0xAA3
    em4space                    ,   //2724      0xAA4
    digitspace                  ,   //2725      0xAA5
    punctspace                  ,   //2726      0xAA6
    thinspace                   ,   //2727      0xAA7
    hairspace                   ,   //2728      0xAA8
    emdash                      ,   //2729      0xAA9
    endash                      ,   //2730      0xAAA
    signifblank                 ,   //2732      0xAAC
    ellipsis                    ,   //2734      0xAAE
    doubbaselinedot             ,   //2735      0xAAF
    onethird                    ,   //2736      0xAB0
    twothirds                   ,   //2737      0xAB1
    onefifth                    ,   //2738      0xAB2
    twofifths                   ,   //2739      0xAB3
    threefifths                 ,   //2740      0xAB4
    fourfifths                  ,   //2741      0xAB5
    onesixth                    ,   //2742      0xAB6
    fivesixths                  ,   //2743      0xAB7
    careof                      ,   //2744      0xAB8
    figdash                     ,   //2747      0xABB
    leftanglebracket            ,   //2748      0xABC
    decimalpoint                ,   //2749      0xABD
    rightanglebracket           ,   //2750      0xABE
    marker                      ,   //2751      0xABF
    oneeighth                   ,   //2755      0xAC3
    threeeighths                ,   //2756      0xAC4
    fiveeighths                 ,   //2757      0xAC5
    seveneighths                ,   //2758      0xAC6
    trademark                   ,   //2761      0xAC9
    signaturemark               ,   //2762      0xACA
    trademarkincircle           ,   //2763      0xACB
    leftopentriangle            ,   //2764      0xACC
    rightopentriangle           ,   //2765      0xACD
    emopencircle                ,   //2766      0xACE
    emopenrectangle             ,   //2767      0xACF
    leftsinglequotemark         ,   //2768      0xAD0
    rightsinglequotemark        ,   //2769      0xAD1
    leftdoublequotemark         ,   //2770      0xAD2
    rightdoublequotemark        ,   //2771      0xAD3
    prescription                ,   //2772      0xAD4
    permille                    ,   //2773      0xAD5
    minutes                     ,   //2774      0xAD6
    seconds                     ,   //2775      0xAD7
    latincross                  ,   //2777      0xAD9
    hexagram                    ,   //2778      0xADA
    filledrectbullet            ,   //2779      0xADB
    filledlefttribullet         ,   //2780      0xADC
    filledrighttribullet        ,   //2781      0xADD
    emfilledcircle              ,   //2782      0xADE
    emfilledrect                ,   //2783      0xADF
    enopencircbullet            ,   //2784      0xAE0
    enopensquarebullet          ,   //2785      0xAE1
    openrectbullet              ,   //2786      0xAE2
    opentribulletup             ,   //2787      0xAE3
    opentribulletdown           ,   //2788      0xAE4
    openstar                    ,   //2789      0xAE5
    enfilledcircbullet          ,   //2790      0xAE6
    enfilledsqbullet            ,   //2791      0xAE7
    filledtribulletup           ,   //2792      0xAE8
    filledtribulletdown         ,   //2793      0xAE9
    leftpointer                 ,   //2794      0xAEA
    rightpointer                ,   //2795      0xAEB
    club                        ,   //2796      0xAEC
    diamond                     ,   //2797      0xAED
    heart                       ,   //2798      0xAEE
    maltesecross                ,   //2800      0xAF0
    dagger                      ,   //2801      0xAF1
    doubledagger                ,   //2802      0xAF2
    checkmark                   ,   //2803      0xAF3
    ballotcross                 ,   //2804      0xAF4
    musicalsharp                ,   //2805      0xAF5
    musicalflat                 ,   //2806      0xAF6
    malesymbol                  ,   //2807      0xAF7
    femalesymbol                ,   //2808      0xAF8
    telephone                   ,   //2809      0xAF9
    telephonerecorder           ,   //2810      0xAFA
    phonographcopyright         ,   //2811      0xAFB
    caret                       ,   //2812      0xAFC
    singlelowquotemark          ,   //2813      0xAFD
    doublelowquotemark          ,   //2814      0xAFE
    cursor                      ,   //2815      0xAFF
    leftcaret                   ,   //2979      0xBA3
    rightcaret                  ,   //2982      0xBA6
    downcaret                   ,   //2984      0xBA8
    upcaret                     ,   //2985      0xBA9
    overbar                     ,   //3008      0xBC0
    downtack                    ,   //3010      0xBC2
    upshoe                      ,   //3011      0xBC3
    downstile                   ,   //3012      0xBC4
    underbar                    ,   //3014      0xBC6
    jot                         ,   //3018      0xBCA
    quad                        ,   //3020      0xBCC
    uptack                      ,   //3022      0xBCE
    circle                      ,   //3023      0xBCF
    upstile                     ,   //3027      0xBD3
    downshoe                    ,   //3030      0xBD6
    rightshoe                   ,   //3032      0xBD8
    leftshoe                    ,   //3034      0xBDA
    lefttack                    ,   //3036      0xBDC
    righttack                   ,   //3068      0xBFC
    hebrew_doublelowline        ,   //3295      0xCDF
    hebrew_aleph                ,   //3296      0xCE0
    hebrew_bet                  ,   //3297      0xCE1
    hebrew_gimel                ,   //3298      0xCE2
    hebrew_dalet                ,   //3299      0xCE3
    hebrew_he                   ,   //3300      0xCE4
    hebrew_waw                  ,   //3301      0xCE5
    hebrew_zain                 ,   //3302      0xCE6
    hebrew_chet                 ,   //3303      0xCE7
    hebrew_tet                  ,   //3304      0xCE8
    hebrew_yod                  ,   //3305      0xCE9
    hebrew_finalkaph            ,   //3306      0xCEA
    hebrew_kaph                 ,   //3307      0xCEB
    hebrew_lamed                ,   //3308      0xCEC
    hebrew_finalmem             ,   //3309      0xCED
    hebrew_mem                  ,   //3310      0xCEE
    hebrew_finalnun             ,   //3311      0xCEF
    hebrew_nun                  ,   //3312      0xCF0
    hebrew_samech               ,   //3313      0xCF1
    hebrew_ayin                 ,   //3314      0xCF2
    hebrew_finalpe              ,   //3315      0xCF3
    hebrew_pe                   ,   //3316      0xCF4
    hebrew_finalzade            ,   //3317      0xCF5
    hebrew_zade                 ,   //3318      0xCF6
    hebrew_qoph                 ,   //3319      0xCF7
    hebrew_resh                 ,   //3320      0xCF8
    hebrew_shin                 ,   //3321      0xCF9
    hebrew_taw                  ,   //3322      0xCFA

    Thai_kokai                  ,   //3489      0xDA1
    Thai_khokhai                ,   //3490      0xDA2
    Thai_khokhuat               ,   //3491      0xDA3
    Thai_khokhwai               ,   //3492      0xDA4
    Thai_khokhon                ,   //3493      0xDA5
    Thai_khorakhang             ,   //3494      0xDA6
    Thai_ngongu                 ,   //3495      0xDA7
    Thai_chochan                ,   //3496      0xDA8
    Thai_choching               ,   //3497      0xDA9
    Thai_chochang               ,   //3498      0xDAA
    Thai_soso                   ,   //3499      0xDAB
    Thai_chochoe                ,   //3500      0xDAC
    Thai_yoying                 ,   //3501      0xDAD
    Thai_dochada                ,   //3502      0xDAE
    Thai_topatak                ,   //3503      0xDAF
    Thai_thothan                ,   //3504      0xDB0
    Thai_thonangmontho          ,   //3505      0xDB1
    Thai_thophuthao             ,   //3506      0xDB2
    Thai_nonen                  ,   //3507      0xDB3
    Thai_dodek                  ,   //3508      0xDB4
    Thai_totao                  ,   //3509      0xDB5
    Thai_thothung               ,   //3510      0xDB6
    Thai_thothahan              ,   //3511      0xDB7
    Thai_thothong               ,   //3512      0xDB8
    Thai_nonu                   ,   //3513      0xDB9
    Thai_bobaimai               ,   //3514      0xDBA
    Thai_popla                  ,   //3515      0xDBB
    Thai_phophung               ,   //3516      0xDBC
    Thai_fofa                   ,   //3517      0xDBD
    Thai_phophan                ,   //3518      0xDBE
    Thai_fofan                  ,   //3519      0xDBF
    Thai_phosamphao             ,   //3520      0xDC0
    Thai_moma                   ,   //3521      0xDC1
    Thai_yoyak                  ,   //3522      0xDC2
    Thai_rorua                  ,   //3523      0xDC3
    Thai_ru                     ,   //3524      0xDC4
    Thai_loling                 ,   //3525      0xDC5
    Thai_lu                     ,   //3526      0xDC6
    Thai_wowaen                 ,   //3527      0xDC7
    Thai_sosala                 ,   //3528      0xDC8
    Thai_sorusi                 ,   //3529      0xDC9
    Thai_sosua                  ,   //3530      0xDCA
    Thai_hohip                  ,   //3531      0xDCB
    Thai_lochula                ,   //3532      0xDCC
    Thai_oang                   ,   //3533      0xDCD
    Thai_honokhuk               ,   //3534      0xDCE
    Thai_paiyannoi              ,   //3535      0xDCF
    Thai_saraa                  ,   //3536      0xDD0
    Thai_maihanakat             ,   //3537      0xDD1
    Thai_saraaa                 ,   //3538      0xDD2
    Thai_saraam                 ,   //3539      0xDD3
    Thai_sarai                  ,   //3540      0xDD4
    Thai_saraii                 ,   //3541      0xDD5
    Thai_saraue                 ,   //3542      0xDD6
    Thai_sarauee                ,   //3543      0xDD7
    Thai_sarau                  ,   //3544      0xDD8
    Thai_sarauu                 ,   //3545      0xDD9
    Thai_phinthu                ,   //3546      0xDDA
    Thai_maihanakat_maitho      ,   //3550      0xDDE
    Thai_baht                   ,   //3551      0xDDF
    Thai_sarae                  ,   //3552      0xDE0
    Thai_saraae                 ,   //3553      0xDE1
    Thai_sarao                  ,   //3554      0xDE2
    Thai_saraaimaimuan          ,   //3555      0xDE3
    Thai_saraaimaimalai         ,   //3556      0xDE4
    Thai_lakkhangyao            ,   //3557      0xDE5
    Thai_maiyamok               ,   //3558      0xDE6
    Thai_maitaikhu              ,   //3559      0xDE7
    Thai_maiek                  ,   //3560      0xDE8
    Thai_maitho                 ,   //3561      0xDE9
    Thai_maitri                 ,   //3562      0xDEA
    Thai_maichattawa            ,   //3563      0xDEB
    Thai_thanthakhat            ,   //3564      0xDEC
    Thai_nikhahit               ,   //3565      0xDED
    Thai_leksun                 ,   //3568      0xDF0
    Thai_leknung                ,   //3569      0xDF1
    Thai_leksong                ,   //3570      0xDF2
    Thai_leksam                 ,   //3571      0xDF3
    Thai_leksi                  ,   //3572      0xDF4
    Thai_lekha                  ,   //3573      0xDF5
    Thai_lekhok                 ,   //3574      0xDF6
    Thai_lekchet                ,   //3575      0xDF7
    Thai_lekpaet                ,   //3576      0xDF8
    Thai_lekkao                 ,   //3577      0xDF9
    Hangul_Kiyeog               ,   //3745      0xEA1
    Hangul_SsangKiyeog          ,   //3746      0xEA2
    Hangul_KiyeogSios           ,   //3747      0xEA3
    Hangul_Nieun                ,   //3748      0xEA4
    Hangul_NieunJieuj           ,   //3749      0xEA5
    Hangul_NieunHieuh           ,   //3750      0xEA6
    Hangul_Dikeud               ,   //3751      0xEA7
    Hangul_SsangDikeud          ,   //3752      0xEA8
    Hangul_Rieul                ,   //3753      0xEA9
    Hangul_RieulKiyeog          ,   //3754      0xEAA
    Hangul_RieulMieum           ,   //3755      0xEAB
    Hangul_RieulPieub           ,   //3756      0xEAC
    Hangul_RieulSios            ,   //3757      0xEAD
    Hangul_RieulTieut           ,   //3758      0xEAE
    Hangul_RieulPhieuf          ,   //3759      0xEAF
    Hangul_RieulHieuh           ,   //3760      0xEB0
    Hangul_Mieum                ,   //3761      0xEB1
    Hangul_Pieub                ,   //3762      0xEB2
    Hangul_SsangPieub           ,   //3763      0xEB3
    Hangul_PieubSios            ,   //3764      0xEB4
    Hangul_Sios                 ,   //3765      0xEB5
    Hangul_SsangSios            ,   //3766      0xEB6
    Hangul_Ieung                ,   //3767      0xEB7
    Hangul_Jieuj                ,   //3768      0xEB8
    Hangul_SsangJieuj           ,   //3769      0xEB9
    Hangul_Cieuc                ,   //3770      0xEBA
    Hangul_Khieuq               ,   //3771      0xEBB
    Hangul_Tieut                ,   //3772      0xEBC
    Hangul_Phieuf               ,   //3773      0xEBD
    Hangul_Hieuh                ,   //3774      0xEBE
    Hangul_A                    ,   //3775      0xEBF
    Hangul_AE                   ,   //3776      0xEC0
    Hangul_YA                   ,   //3777      0xEC1
    Hangul_YAE                  ,   //3778      0xEC2
    Hangul_EO                   ,   //3779      0xEC3
    Hangul_E                    ,   //3780      0xEC4
    Hangul_YEO                  ,   //3781      0xEC5
    Hangul_YE                   ,   //3782      0xEC6
    Hangul_O                    ,   //3783      0xEC7
    Hangul_WA                   ,   //3784      0xEC8
    Hangul_WAE                  ,   //3785      0xEC9
    Hangul_OE                   ,   //3786      0xECA
    Hangul_YO                   ,   //3787      0xECB
    Hangul_U                    ,   //3788      0xECC
    Hangul_WEO                  ,   //3789      0xECD
    Hangul_WE                   ,   //3790      0xECE
    Hangul_WI                   ,   //3791      0xECF
    Hangul_YU                   ,   //3792      0xED0
    Hangul_EU                   ,   //3793      0xED1
    Hangul_YI                   ,   //3794      0xED2
    Hangul_I                    ,   //3795      0xED3
    Hangul_J_Kiyeog             ,   //3796      0xED4
    Hangul_J_SsangKiyeog        ,   //3797      0xED5
    Hangul_J_KiyeogSios         ,   //3798      0xED6
    Hangul_J_Nieun              ,   //3799      0xED7
    Hangul_J_NieunJieuj         ,   //3800      0xED8
    Hangul_J_NieunHieuh         ,   //3801      0xED9
    Hangul_J_Dikeud             ,   //3802      0xEDA
    Hangul_J_Rieul              ,   //3803      0xEDB
    Hangul_J_RieulKiyeog        ,   //3804      0xEDC
    Hangul_J_RieulMieum         ,   //3805      0xEDD
    Hangul_J_RieulPieub         ,   //3806      0xEDE
    Hangul_J_RieulSios          ,   //3807      0xEDF
    Hangul_J_RieulTieut         ,   //3808      0xEE0
    Hangul_J_RieulPhieuf        ,   //3809      0xEE1
    Hangul_J_RieulHieuh         ,   //3810      0xEE2
    Hangul_J_Mieum              ,   //3811      0xEE3
    Hangul_J_Pieub              ,   //3812      0xEE4
    Hangul_J_PieubSios          ,   //3813      0xEE5
    Hangul_J_Sios               ,   //3814      0xEE6
    Hangul_J_SsangSios          ,   //3815      0xEE7
    Hangul_J_Ieung              ,   //3816      0xEE8
    Hangul_J_Jieuj              ,   //3817      0xEE9
    Hangul_J_Cieuc              ,   //3818      0xEEA
    Hangul_J_Khieuq             ,   //3819      0xEEB
    Hangul_J_Tieut              ,   //3820      0xEEC
    Hangul_J_Phieuf             ,   //3821      0xEED
    Hangul_J_Hieuh              ,   //3822      0xEEE
    Hangul_RieulYeorinHieuh     ,   //3823      0xEEF
    Hangul_SunkyeongeumMieum    ,   //3824      0xEF0
    Hangul_SunkyeongeumPieub    ,   //3825      0xEF1
    Hangul_PanSios              ,   //3826      0xEF2
    Hangul_KkogjiDalrinIeung    ,   //3827      0xEF3
    Hangul_SunkyeongeumPhieuf   ,   //3828      0xEF4
    Hangul_YeorinHieuh          ,   //3829      0xEF5
    Hangul_AraeA                ,   //3830      0xEF6
    Hangul_AraeAE               ,   //3831      0xEF7
    Hangul_J_PanSios            ,   //3832      0xEF8
    Hangul_J_KkogjiDalrinIeung  ,   //3833      0xEF9
    Hangul_J_YeorinHieuh        ,   //3834      0xEFA
    Korean_Won                  ,   //3839      0xEFF
    OE                          ,   //5052      0x13BC
    oe                          ,   //5053      0x13BD
    Ydiaeresis                  ,   //5054      0x13BE

    ISO_Lock                    ,   //65025     0xFE01
    ISO_Level2_Latch            ,   //65026     0xFE02
    ISO_Level3_Shift            ,   //65027     0xFE03
    ISO_Level3_Latch            ,   //65028     0xFE04
    ISO_Level3_Lock             ,   //65029     0xFE05
    ISO_Group_Latch             ,   //65030     0xFE06
    ISO_Group_Lock              ,   //65031     0xFE07
    ISO_Next_Group              ,   //65032     0xFE08
    ISO_Next_Group_Lock         ,   //65033     0xFE09
    ISO_Prev_Group              ,   //65034     0xFE0A
    ISO_Prev_Group_Lock         ,   //65035     0xFE0B
    ISO_First_Group             ,   //65036     0xFE0C
    ISO_First_Group_Lock        ,   //65037     0xFE0D
    ISO_Last_Group              ,   //65038     0xFE0E
    ISO_Last_Group_Lock         ,   //65039     0xFE0F
    ISO_Level5_Shift            ,   //65041     0xFE11
    ISO_Level5_Latch            ,   //65042     0xFE12
    ISO_Level5_Lock             ,   //65043     0xFE13
    ISO_Left_Tab                ,   //65056     0xFE20
    ISO_Move_Line_Up            ,   //65057     0xFE21
    ISO_Move_Line_Down          ,   //65058     0xFE22
    ISO_Partial_Line_Up         ,   //65059     0xFE23
    ISO_Partial_Line_Down       ,   //65060     0xFE24
    ISO_Partial_Space_Left      ,   //65061     0xFE25
    ISO_Partial_Space_Right     ,   //65062     0xFE26
    ISO_Set_Margin_Left         ,   //65063     0xFE27
    ISO_Set_Margin_Right        ,   //65064     0xFE28
    ISO_Release_Margin_Left     ,   //65065     0xFE29
    ISO_Release_Margin_Right    ,   //65066     0xFE2A
    ISO_Release_Both_Margins    ,   //65067     0xFE2B
    ISO_Fast_Cursor_Left        ,   //65068     0xFE2C
    ISO_Fast_Cursor_Right       ,   //65069     0xFE2D
    ISO_Fast_Cursor_Up          ,   //65070     0xFE2E
    ISO_Fast_Cursor_Down        ,   //65071     0xFE2F
    ISO_Continuous_Underline    ,   //65072     0xFE30
    ISO_Discontinuous_Underline ,   //65073     0xFE31
    ISO_Emphasize               ,   //65074     0xFE32
    ISO_Center_Object           ,   //65075     0xFE33
    ISO_Enter                   ,   //65076     0xFE34
    dead_grave                  ,   //65104     0xFE50
    dead_acute                  ,   //65105     0xFE51
    dead_circumflex             ,   //65106     0xFE52
    dead_tilde                  ,   //65107     0xFE53
    dead_macron                 ,   //65108     0xFE54
    dead_breve                  ,   //65109     0xFE55
    dead_abovedot               ,   //65110     0xFE56
    dead_diaeresis              ,   //65111     0xFE57
    dead_abovering              ,   //65112     0xFE58
    dead_doubleacute            ,   //65113     0xFE59
    dead_caron                  ,   //65114     0xFE5A
    dead_cedilla                ,   //65115     0xFE5B
    dead_ogonek                 ,   //65116     0xFE5C
    dead_iota                   ,   //65117     0xFE5D
    dead_voiced_sound           ,   //65118     0xFE5E
    dead_semivoiced_sound       ,   //65119     0xFE5F
    dead_belowdot               ,   //65120     0xFE60
    dead_hook                   ,   //65121     0xFE61
    dead_horn                   ,   //65122     0xFE62
    dead_stroke                 ,   //65123     0xFE63
    dead_abovecomma             ,   //65124     0xFE64
    dead_abovereversedcomma     ,   //65125     0xFE65
    dead_doublegrave            ,   //65126     0xFE66
    dead_belowring              ,   //65127     0xFE67
    dead_belowmacron            ,   //65128     0xFE68
    dead_belowcircumflex        ,   //65129     0xFE69
    dead_belowtilde             ,   //65130     0xFE6A
    dead_belowbreve             ,   //65131     0xFE6B
    dead_belowdiaeresis         ,   //65132     0xFE6C
    dead_invertedbreve          ,   //65133     0xFE6D
    dead_belowcomma             ,   //65134     0xFE6E
    dead_currency               ,   //65135     0xFE6F
    AccessX_Enable              ,   //65136     0xFE70
    AccessX_Feedback_Enable     ,   //65137     0xFE71
    RepeatKeys_Enable           ,   //65138     0xFE72
    SlowKeys_Enable             ,   //65139     0xFE73
    BounceKeys_Enable           ,   //65140     0xFE74
    StickyKeys_Enable           ,   //65141     0xFE75
    MouseKeys_Enable            ,   //65142     0xFE76
    MouseKeys_Accel_Enable      ,   //65143     0xFE77
    Overlay1_Enable             ,   //65144     0xFE78
    Overlay2_Enable             ,   //65145     0xFE79
    AudibleBell_Enable          ,   //65146     0xFE7A
    dead_a                      ,   //65152     0xFE80
    dead_A                      ,   //65153     0xFE81
    dead_e                      ,   //65154     0xFE82
    dead_E                      ,   //65155     0xFE83
    dead_i                      ,   //65156     0xFE84
    dead_I                      ,   //65157     0xFE85
    dead_o                      ,   //65158     0xFE86
    dead_O                      ,   //65159     0xFE87
    dead_u                      ,   //65160     0xFE88
    dead_U                      ,   //65161     0xFE89
    dead_small_schwa            ,   //65162     0xFE8A
    dead_capital_schwa          ,   //65163     0xFE8B
    dead_greek                  ,   //65164     0xFE8C
    dead_lowline                ,   //65168     0xFE90
    dead_aboveverticalline      ,   //65169     0xFE91
    dead_belowverticalline      ,   //65170     0xFE92
    dead_longsolidusoverlay     ,   //65171     0xFE93
    ch                          ,   //65184     0xFEA0
    Ch                          ,   //65185     0xFEA1
    CH                          ,   //65186     0xFEA2
    c_h                         ,   //65187     0xFEA3
    C_h                         ,   //65188     0xFEA4
    C_H                         ,   //65189     0xFEA5
    First_Virtual_Screen        ,   //65232     0xFED0
    Prev_Virtual_Screen         ,   //65233     0xFED1
    Next_Virtual_Screen         ,   //65234     0xFED2
    Last_Virtual_Screen         ,   //65236     0xFED4
    Terminate_Server            ,   //65237     0xFED5
    Pointer_Left                ,   //65248     0xFEE0
    Pointer_Right               ,   //65249     0xFEE1
    Pointer_Up                  ,   //65250     0xFEE2
    Pointer_Down                ,   //65251     0xFEE3
    Pointer_UpLeft              ,   //65252     0xFEE4
    Pointer_UpRight             ,   //65253     0xFEE5
    Pointer_DownLeft            ,   //65254     0xFEE6
    Pointer_DownRight           ,   //65255     0xFEE7
    Pointer_Button_Dflt         ,   //65256     0xFEE8
    Pointer_Button1             ,   //65257     0xFEE9
    Pointer_Button2             ,   //65258     0xFEEA
    Pointer_Button3             ,   //65259     0xFEEB
    Pointer_Button4             ,   //65260     0xFEEC
    Pointer_Button5             ,   //65261     0xFEED
    Pointer_DblClick_Dflt       ,   //65262     0xFEEE
    Pointer_DblClick1           ,   //65263     0xFEEF
    Pointer_DblClick2           ,   //65264     0xFEF0
    Pointer_DblClick3           ,   //65265     0xFEF1
    Pointer_DblClick4           ,   //65266     0xFEF2
    Pointer_DblClick5           ,   //65267     0xFEF3
    Pointer_Drag_Dflt           ,   //65268     0xFEF4
    Pointer_Drag1               ,   //65269     0xFEF5
    Pointer_Drag2               ,   //65270     0xFEF6
    Pointer_Drag3               ,   //65271     0xFEF7
    Pointer_Drag4               ,   //65272     0xFEF8
    Pointer_EnableKeys          ,   //65273     0xFEF9
    Pointer_Accelerate          ,   //65274     0xFEFA
    Pointer_DfltBtnNext         ,   //65275     0xFEFB
    Pointer_DfltBtnPrev         ,   //65276     0xFEFC
    Pointer_Drag5               ,   //65277     0xFEFD

    BackSpace                   ,   //65288     0xFF08
    Tab                         ,   //65289     0xFF09
    Linefeed                    ,   //65290     0xFF0A
    Clear                       ,   //65291     0xFF0B
    Return                      ,   //65293     0xFF0D
    Pause                       ,   //65299     0xFF13
    Scroll_Lock                 ,   //65300     0xFF14
    Sys_Req                     ,   //65301     0xFF15
    Escape                      ,   //65307     0xFF1B
    Multi_key                   ,   //65312     0xFF20
    Kanji                       ,   //65313     0xFF21
    Muhenkan                    ,   //65314     0xFF22
    Henkan_Mode                 ,   //65315     0xFF23
    Romaji                      ,   //65316     0xFF24
    Hiragana                    ,   //65317     0xFF25
    Katakana                    ,   //65318     0xFF26
    Hiragana_Katakana           ,   //65319     0xFF27
    Zenkaku                     ,   //65320     0xFF28
    Hankaku                     ,   //65321     0xFF29
    Zenkaku_Hankaku             ,   //65322     0xFF2A
    Touroku                     ,   //65323     0xFF2B
    Massyo                      ,   //65324     0xFF2C
    Kana_Lock                   ,   //65325     0xFF2D
    Kana_Shift                  ,   //65326     0xFF2E
    Eisu_Shift                  ,   //65327     0xFF2F
    Eisu_toggle                 ,   //65328     0xFF30
    Hangul                      ,   //65329     0xFF31
    Hangul_Start                ,   //65330     0xFF32
    Hangul_End                  ,   //65331     0xFF33
    Hangul_Hanja                ,   //65332     0xFF34
    Hangul_Jamo                 ,   //65333     0xFF35
    Hangul_Romaja               ,   //65334     0xFF36
    Codeinput                   ,   //65335     0xFF37
    Hangul_Jeonja               ,   //65336     0xFF38
    Hangul_Banja                ,   //65337     0xFF39
    Hangul_PreHanja             ,   //65338     0xFF3A
    Hangul_PostHanja            ,   //65339     0xFF3B
    SingleCandidate             ,   //65340     0xFF3C
    MultipleCandidate           ,   //65341     0xFF3D
    PreviousCandidate           ,   //65342     0xFF3E
    Hangul_Special              ,   //65343     0xFF3F
    Home                        ,   //65360     0xFF50
    Left                        ,   //65361     0xFF51
    Up                          ,   //65362     0xFF52
    Right                       ,   //65363     0xFF53
    Down                        ,   //65364     0xFF54
    Prior                       ,   //65365     0xFF55
    Next                        ,   //65366     0xFF56
    End                         ,   //65367     0xFF57
    Begin                       ,   //65368     0xFF58
    Win_L                       ,   //65371     0xFF5B
    Win_R                       ,   //65372     0xFF5C
    App                         ,   //65373     0xFF5D
    Select                      ,   //65376     0xFF60
    Print                       ,   //65377     0xFF61
    Execute                     ,   //65378     0xFF62
    Insert                      ,   //65379     0xFF63
    Undo                        ,   //65381     0xFF65
    Redo                        ,   //65382     0xFF66
    Menu                        ,   //65383     0xFF67
    Find                        ,   //65384     0xFF68
    Cancel                      ,   //65385     0xFF69
    Help                        ,   //65386     0xFF6A
    Break                       ,   //65387     0xFF6B
    Mode_switch                 ,   //65406     0xFF7E
    Num_Lock                    ,   //65407     0xFF7F
    KP_Space                    ,   //65408     0xFF80
    KP_Tab                      ,   //65417     0xFF89
    KP_Enter                    ,   //65421     0xFF8D
    KP_F1                       ,   //65425     0xFF91
    KP_F2                       ,   //65426     0xFF92
    KP_F3                       ,   //65427     0xFF93
    KP_F4                       ,   //65428     0xFF94
    KP_Home                     ,   //65429     0xFF95
    KP_Left                     ,   //65430     0xFF96
    KP_Up                       ,   //65431     0xFF97
    KP_Right                    ,   //65432     0xFF98
    KP_Down                     ,   //65433     0xFF99
    KP_Prior                    ,   //65434     0xFF9A
    KP_Next                     ,   //65435     0xFF9B
    KP_End                      ,   //65436     0xFF9C
    KP_Begin                    ,   //65437     0xFF9D
    KP_Insert                   ,   //65438     0xFF9E
    KP_Delete                   ,   //65439     0xFF9F
    KP_Multiply                 ,   //65450     0xFFAA
    KP_Add                      ,   //65451     0xFFAB
    KP_Separator                ,   //65452     0xFFAC
    KP_Subtract                 ,   //65453     0xFFAD
    KP_Decimal                  ,   //65454     0xFFAE
    KP_Divide                   ,   //65455     0xFFAF
    KP_0                        ,   //65456     0xFFB0
    KP_1                        ,   //65457     0xFFB1
    KP_2                        ,   //65458     0xFFB2
    KP_3                        ,   //65459     0xFFB3
    KP_4                        ,   //65460     0xFFB4
    KP_5                        ,   //65461     0xFFB5
    KP_6                        ,   //65462     0xFFB6
    KP_7                        ,   //65463     0xFFB7
    KP_8                        ,   //65464     0xFFB8
    KP_9                        ,   //65465     0xFFB9
    KP_Equal                    ,   //65469     0xFFBD
    F1                          ,   //65470     0xFFBE
    F2                          ,   //65471     0xFFBF
    F3                          ,   //65472     0xFFC0
    F4                          ,   //65473     0xFFC1
    F5                          ,   //65474     0xFFC2
    F6                          ,   //65475     0xFFC3
    F7                          ,   //65476     0xFFC4
    F8                          ,   //65477     0xFFC5
    F9                          ,   //65478     0xFFC6
    F10                         ,   //65479     0xFFC7
    F11                         ,   //65480     0xFFC8
    F12                         ,   //65481     0xFFC9
    F13                         ,   //65482     0xFFCA
    F14                         ,   //65483     0xFFCB
    F15                         ,   //65484     0xFFCC
    F16                         ,   //65485     0xFFCD
    F17                         ,   //65486     0xFFCE
    F18                         ,   //65487     0xFFCF
    F19                         ,   //65488     0xFFD0
    F20                         ,   //65489     0xFFD1
    F21                         ,   //65490     0xFFD2
    F22                         ,   //65491     0xFFD3
    F23                         ,   //65492     0xFFD4
    F24                         ,   //65493     0xFFD5
    F25                         ,   //65494     0xFFD6
    F26                         ,   //65495     0xFFD7
    F27                         ,   //65496     0xFFD8
    F28                         ,   //65497     0xFFD9
    F29                         ,   //65498     0xFFDA
    F30                         ,   //65499     0xFFDB
    F31                         ,   //65500     0xFFDC
    F32                         ,   //65501     0xFFDD
    F33                         ,   //65502     0xFFDE
    F34                         ,   //65503     0xFFDF
    F35                         ,   //65504     0xFFE0
    Shift_L                     ,   //65505     0xFFE1
    Shift_R                     ,   //65506     0xFFE2
    Control_L                   ,   //65507     0xFFE3
    Control_R                   ,   //65508     0xFFE4
    Caps_Lock                   ,   //65509     0xFFE5
    Shift_Lock                  ,   //65510     0xFFE6
    Meta_L                      ,   //65511     0xFFE7
    Meta_R                      ,   //65512     0xFFE8
    Alt_L                       ,   //65513     0xFFE9
    Alt_R                       ,   //65514     0xFFEA
    Super_L                     ,   //65515     0xFFEB
    Super_R                     ,   //65516     0xFFEC
    Hyper_L                     ,   //65517     0xFFED
    Hyper_R                     ,   //65518     0xFFEE
    braille_dot_1               ,   //65521     0xFFF1
    braille_dot_2               ,   //65522     0xFFF2
    braille_dot_3               ,   //65523     0xFFF3
    braille_dot_4               ,   //65524     0xFFF4
    braille_dot_5               ,   //65525     0xFFF5
    braille_dot_6               ,   //65526     0xFFF6
    braille_dot_7               ,   //65527     0xFFF7
    braille_dot_8               ,   //65528     0xFFF8
    braille_dot_9               ,   //65529     0xFFF9
    braille_dot_10              ,   //65530     0xFFFA
    Delete                      ,   //65535     0xFFFF

    SunFA_Grave                 ,   //268828416 0x1005FF00
    SunFA_Circum                ,   //268828417 0x1005FF01
    SunFA_Tilde                 ,   //268828418 0x1005FF02
    SunFA_Acute                 ,   //268828419 0x1005FF03
    SunFA_Diaeresis             ,   //268828420 0x1005FF04
    SunFA_Cedilla               ,   //268828421 0x1005FF05
    SunF36                      ,   //268828432 0x1005FF10
    SunF37                      ,   //268828433 0x1005FF11
    SunSys_Req                  ,   //268828512 0x1005FF60
    SunProps                    ,   //268828528 0x1005FF70
    SunFront                    ,   //268828529 0x1005FF71
    SunCopy                     ,   //268828530 0x1005FF72
    SunOpen                     ,   //268828531 0x1005FF73
    SunPaste                    ,   //268828532 0x1005FF74
    SunCut                      ,   //268828533 0x1005FF75
    SunPowerSwitch              ,   //268828534 0x1005FF76
    SunAudioLowerVolume         ,   //268828535 0x1005FF77
    SunAudioMute                ,   //268828536 0x1005FF78
    SunAudioRaiseVolume         ,   //268828537 0x1005FF79
    SunVideoDegauss             ,   //268828538 0x1005FF7A
    SunVideoLowerBrightness     ,   //268828539 0x1005FF7B
    SunVideoRaiseBrightness     ,   //268828540 0x1005FF7C
    SunPowerSwitchShift         ,   //268828541 0x1005FF7D
    XF86BrightnessAuto          ,   //268964084 0x100810F4
    XF86DisplayOff              ,   //268964085 0x100810F5
    XF86Info                    ,   //268964198 0x10081166
    XF86AspectRatio             ,   //268964215 0x10081177
    XF86DVD                     ,   //268964229 0x10081185
    XF86Audio                   ,   //268964232 0x10081188
    XF86ChannelUp               ,   //268964242 0x10081192
    XF86ChannelDown             ,   //268964243 0x10081193
    XF86Break                   ,   //268964251 0x1008119B
    XF86VideoPhone              ,   //268964256 0x100811A0
    XF86ZoomReset               ,   //268964260 0x100811A4
    XF86Editor                  ,   //268964262 0x100811A6
    XF86GraphicsEditor          ,   //268964264 0x100811A8
    XF86Presentation            ,   //268964265 0x100811A9
    XF86Database                ,   //268964266 0x100811AA
    XF86Voicemail               ,   //268964268 0x100811AC
    XF86Addressbook             ,   //268964269 0x100811AD
    XF86DisplayToggle           ,   //268964271 0x100811AF
    XF86SpellCheck              ,   //268964272 0x100811B0
    XF86ContextMenu             ,   //268964278 0x100811B6
    XF86MediaRepeat             ,   //268964279 0x100811B7
    XF8610ChannelsUp            ,   //268964280 0x100811B8
    XF8610ChannelsDown          ,   //268964281 0x100811B9
    XF86Images                  ,   //268964282 0x100811BA
    XF86NotificationCenter      ,   //268964284 0x100811BC
    XF86PickupPhone             ,   //268964285 0x100811BD
    XF86HangupPhone             ,   //268964286 0x100811BE
    XF86Fn                      ,   //268964304 0x100811D0
    XF86Fn_Esc                  ,   //268964305 0x100811D1
    XF86FnRightShift            ,   //268964325 0x100811E5
    XF86Numeric0                ,   //268964352 0x10081200
    XF86Numeric1                ,   //268964353 0x10081201
    XF86Numeric2                ,   //268964354 0x10081202
    XF86Numeric3                ,   //268964355 0x10081203
    XF86Numeric4                ,   //268964356 0x10081204
    XF86Numeric5                ,   //268964357 0x10081205
    XF86Numeric6                ,   //268964358 0x10081206
    XF86Numeric7                ,   //268964359 0x10081207
    XF86Numeric8                ,   //268964360 0x10081208
    XF86Numeric9                ,   //268964361 0x10081209
    XF86NumericStar             ,   //268964362 0x1008120A
    XF86NumericPound            ,   //268964363 0x1008120B
    XF86NumericA                ,   //268964364 0x1008120C
    XF86NumericB                ,   //268964365 0x1008120D
    XF86NumericC                ,   //268964366 0x1008120E
    XF86NumericD                ,   //268964367 0x1008120F
    XF86CameraFocus             ,   //268964368 0x10081210
    XF86WPSButton               ,   //268964369 0x10081211
    XF86CameraZoomIn            ,   //268964373 0x10081215
    XF86CameraZoomOut           ,   //268964374 0x10081216
    XF86CameraUp                ,   //268964375 0x10081217
    XF86CameraDown              ,   //268964376 0x10081218
    XF86CameraLeft              ,   //268964377 0x10081219
    XF86CameraRight             ,   //268964378 0x1008121A
    XF86AttendantOn             ,   //268964379 0x1008121B
    XF86AttendantOff            ,   //268964380 0x1008121C
    XF86AttendantToggle         ,   //268964381 0x1008121D
    XF86LightsToggle            ,   //268964382 0x1008121E
    XF86ALSToggle               ,   //268964400 0x10081230
    XF86Buttonconfig            ,   //268964416 0x10081240
    XF86Taskmanager             ,   //268964417 0x10081241
    XF86Journal                 ,   //268964418 0x10081242
    XF86ControlPanel            ,   //268964419 0x10081243
    XF86AppSelect               ,   //268964420 0x10081244
    XF86Screensaver             ,   //268964421 0x10081245
    XF86VoiceCommand            ,   //268964422 0x10081246
    XF86Assistant               ,   //268964423 0x10081247
    XF86EmojiPicker             ,   //268964425 0x10081249
    XF86Dictate                 ,   //268964426 0x1008124A
    XF86BrightnessMin           ,   //268964432 0x10081250
    XF86BrightnessMax           ,   //268964433 0x10081251
    XF86KbdInputAssistPrev      ,   //268964448 0x10081260
    XF86KbdInputAssistNext      ,   //268964449 0x10081261
    XF86KbdInputAssistPrevgroup ,   //268964450 0x10081262
    XF86KbdInputAssistNextgroup ,   //268964451 0x10081263
    XF86KbdInputAssistAccept    ,   //268964452 0x10081264
    XF86KbdInputAssistCancel    ,   //268964453 0x10081265
    XF86RightUp                 ,   //268964454 0x10081266
    XF86RightDown               ,   //268964455 0x10081267
    XF86LeftUp                  ,   //268964456 0x10081268
    XF86LeftDown                ,   //268964457 0x10081269
    XF86RootMenu                ,   //268964458 0x1008126A
    XF86MediaTopMenu            ,   //268964459 0x1008126B
    XF86Numeric11               ,   //268964460 0x1008126C
    XF86Numeric12               ,   //268964461 0x1008126D
    XF86AudioDesc               ,   //268964462 0x1008126E
    XF863DMode                  ,   //268964463 0x1008126F
    XF86NextFavorite            ,   //268964464 0x10081270
    XF86StopRecord              ,   //268964465 0x10081271
    XF86PauseRecord             ,   //268964466 0x10081272
    XF86VOD                     ,   //268964467 0x10081273
    XF86Unmute                  ,   //268964468 0x10081274
    XF86FastReverse             ,   //268964469 0x10081275
    XF86SlowReverse             ,   //268964470 0x10081276
    XF86Data                    ,   //268964471 0x10081277
    XF86OnScreenKeyboard        ,   //268964472 0x10081278
    XF86PrivacyScreenToggle     ,   //268964473 0x10081279
    XF86SelectiveScreenshot     ,   //268964474 0x1008127A
    XF86Macro1                  ,   //268964496 0x10081290
    XF86Macro2                  ,   //268964497 0x10081291
    XF86Macro3                  ,   //268964498 0x10081292
    XF86Macro4                  ,   //268964499 0x10081293
    XF86Macro5                  ,   //268964500 0x10081294
    XF86Macro6                  ,   //268964501 0x10081295
    XF86Macro7                  ,   //268964502 0x10081296
    XF86Macro8                  ,   //268964503 0x10081297
    XF86Macro9                  ,   //268964504 0x10081298
    XF86Macro10                 ,   //268964505 0x10081299
    XF86Macro11                 ,   //268964506 0x1008129A
    XF86Macro12                 ,   //268964507 0x1008129B
    XF86Macro13                 ,   //268964508 0x1008129C
    XF86Macro14                 ,   //268964509 0x1008129D
    XF86Macro15                 ,   //268964510 0x1008129E
    XF86Macro16                 ,   //268964511 0x1008129F
    XF86Macro17                 ,   //268964512 0x100812A0
    XF86Macro18                 ,   //268964513 0x100812A1
    XF86Macro19                 ,   //268964514 0x100812A2
    XF86Macro20                 ,   //268964515 0x100812A3
    XF86Macro21                 ,   //268964516 0x100812A4
    XF86Macro22                 ,   //268964517 0x100812A5
    XF86Macro23                 ,   //268964518 0x100812A6
    XF86Macro24                 ,   //268964519 0x100812A7
    XF86Macro25                 ,   //268964520 0x100812A8
    XF86Macro26                 ,   //268964521 0x100812A9
    XF86Macro27                 ,   //268964522 0x100812AA
    XF86Macro28                 ,   //268964523 0x100812AB
    XF86Macro29                 ,   //268964524 0x100812AC
    XF86Macro30                 ,   //268964525 0x100812AD
    XF86MacroRecordStart        ,   //268964528 0x100812B0
    XF86MacroRecordStop         ,   //268964529 0x100812B1
    XF86MacroPresetCycle        ,   //268964530 0x100812B2
    XF86MacroPreset1            ,   //268964531 0x100812B3
    XF86MacroPreset2            ,   //268964532 0x100812B4
    XF86MacroPreset3            ,   //268964533 0x100812B5
    XF86KbdLcdMenu1             ,   //268964536 0x100812B8
    XF86KbdLcdMenu2             ,   //268964537 0x100812B9
    XF86KbdLcdMenu3             ,   //268964538 0x100812BA
    XF86KbdLcdMenu4             ,   //268964539 0x100812BB
    XF86KbdLcdMenu5             ,   //268964540 0x100812BC
    XF86Switch_VT_1             ,   //269024769 0x1008FE01
    XF86Switch_VT_2             ,   //269024770 0x1008FE02
    XF86Switch_VT_3             ,   //269024771 0x1008FE03
    XF86Switch_VT_4             ,   //269024772 0x1008FE04
    XF86Switch_VT_5             ,   //269024773 0x1008FE05
    XF86Switch_VT_6             ,   //269024774 0x1008FE06
    XF86Switch_VT_7             ,   //269024775 0x1008FE07
    XF86Switch_VT_8             ,   //269024776 0x1008FE08
    XF86Switch_VT_9             ,   //269024777 0x1008FE09
    XF86Switch_VT_10            ,   //269024778 0x1008FE0A
    XF86Switch_VT_11            ,   //269024779 0x1008FE0B
    XF86Switch_VT_12            ,   //269024780 0x1008FE0C

    XF86Ungrab                  ,   //269024800 0x1008FE20
    XF86ClearGrab               ,   //269024801 0x1008FE21
    XF86Next_VMode              ,   //269024802 0x1008FE22
    XF86Prev_VMode              ,   //269024803 0x1008FE23
    XF86LogWindowTree           ,   //269024804 0x1008FE24
    XF86LogGrabInfo             ,   //269024805 0x1008FE25
    XF86ModeLock                ,   //269025025 0x1008FF01
    XF86MonBrightnessUp         ,   //269025026 0x1008FF02
    XF86MonBrightnessDown       ,   //269025027 0x1008FF03
    XF86KbdLightOnOff           ,   //269025028 0x1008FF04
    XF86KbdBrightnessUp         ,   //269025029 0x1008FF05
    XF86KbdBrightnessDown       ,   //269025030 0x1008FF06
    XF86MonBrightnessCycle      ,   //269025031 0x1008FF07
    XF86Standby                 ,   //269025040 0x1008FF10
    XF86AudioLowerVolume        ,   //269025041 0x1008FF11
    XF86AudioMute               ,   //269025042 0x1008FF12
    XF86AudioRaiseVolume        ,   //269025043 0x1008FF13
    XF86AudioPlay               ,   //269025044 0x1008FF14
    XF86AudioStop               ,   //269025045 0x1008FF15
    XF86AudioPrev               ,   //269025046 0x1008FF16
    XF86AudioNext               ,   //269025047 0x1008FF17
    XF86HomePage                ,   //269025048 0x1008FF18
    XF86Mail                    ,   //269025049 0x1008FF19
    XF86Start                   ,   //269025050 0x1008FF1A
    XF86Search                  ,   //269025051 0x1008FF1B
    XF86AudioRecord             ,   //269025052 0x1008FF1C
    XF86Calculator              ,   //269025053 0x1008FF1D
    XF86Memo                    ,   //269025054 0x1008FF1E
    XF86ToDoList                ,   //269025055 0x1008FF1F
    XF86Calendar                ,   //269025056 0x1008FF20
    XF86PowerDown               ,   //269025057 0x1008FF21
    XF86ContrastAdjust          ,   //269025058 0x1008FF22
    XF86RockerUp                ,   //269025059 0x1008FF23
    XF86RockerDown              ,   //269025060 0x1008FF24
    XF86RockerEnter             ,   //269025061 0x1008FF25
    XF86Back                    ,   //269025062 0x1008FF26
    XF86Forward                 ,   //269025063 0x1008FF27
    XF86Stop                    ,   //269025064 0x1008FF28
    XF86Refresh                 ,   //269025065 0x1008FF29
    XF86PowerOff                ,   //269025066 0x1008FF2A
    XF86WakeUp                  ,   //269025067 0x1008FF2B
    XF86Eject                   ,   //269025068 0x1008FF2C
    XF86ScreenSaver             ,   //269025069 0x1008FF2D
    XF86WWW                     ,   //269025070 0x1008FF2E
    XF86Sleep                   ,   //269025071 0x1008FF2F
    XF86Favorites               ,   //269025072 0x1008FF30
    XF86AudioPause              ,   //269025073 0x1008FF31
    XF86AudioMedia              ,   //269025074 0x1008FF32
    XF86MyComputer              ,   //269025075 0x1008FF33
    XF86VendorHome              ,   //269025076 0x1008FF34
    XF86LightBulb               ,   //269025077 0x1008FF35
    XF86Shop                    ,   //269025078 0x1008FF36
    XF86History                 ,   //269025079 0x1008FF37
    XF86OpenURL                 ,   //269025080 0x1008FF38
    XF86AddFavorite             ,   //269025081 0x1008FF39
    XF86HotLinks                ,   //269025082 0x1008FF3A
    XF86BrightnessAdjust        ,   //269025083 0x1008FF3B
    XF86Finance                 ,   //269025084 0x1008FF3C
    XF86Community               ,   //269025085 0x1008FF3D
    XF86AudioRewind             ,   //269025086 0x1008FF3E
    XF86BackForward             ,   //269025087 0x1008FF3F
    XF86Launch0                 ,   //269025088 0x1008FF40
    XF86Launch1                 ,   //269025089 0x1008FF41
    XF86Launch2                 ,   //269025090 0x1008FF42
    XF86Launch3                 ,   //269025091 0x1008FF43
    XF86Launch4                 ,   //269025092 0x1008FF44
    XF86Launch5                 ,   //269025093 0x1008FF45
    XF86Launch6                 ,   //269025094 0x1008FF46
    XF86Launch7                 ,   //269025095 0x1008FF47
    XF86Launch8                 ,   //269025096 0x1008FF48
    XF86Launch9                 ,   //269025097 0x1008FF49
    XF86LaunchA                 ,   //269025098 0x1008FF4A
    XF86LaunchB                 ,   //269025099 0x1008FF4B
    XF86LaunchC                 ,   //269025100 0x1008FF4C
    XF86LaunchD                 ,   //269025101 0x1008FF4D
    XF86LaunchE                 ,   //269025102 0x1008FF4E
    XF86LaunchF                 ,   //269025103 0x1008FF4F
    XF86ApplicationLeft         ,   //269025104 0x1008FF50
    XF86ApplicationRight        ,   //269025105 0x1008FF51
    XF86Book                    ,   //269025106 0x1008FF52
    XF86CD                      ,   //269025107 0x1008FF53
    XF86Calculater              ,   //269025108 0x1008FF54
    XF86Clear                   ,   //269025109 0x1008FF55
    XF86Close                   ,   //269025110 0x1008FF56
    XF86Copy                    ,   //269025111 0x1008FF57
    XF86Cut                     ,   //269025112 0x1008FF58
    XF86Display                 ,   //269025113 0x1008FF59
    XF86DOS                     ,   //269025114 0x1008FF5A
    XF86Documents               ,   //269025115 0x1008FF5B
    XF86Excel                   ,   //269025116 0x1008FF5C
    XF86Explorer                ,   //269025117 0x1008FF5D
    XF86Game                    ,   //269025118 0x1008FF5E
    XF86Go                      ,   //269025119 0x1008FF5F
    XF86iTouch                  ,   //269025120 0x1008FF60
    XF86LogOff                  ,   //269025121 0x1008FF61
    XF86Market                  ,   //269025122 0x1008FF62
    XF86Meeting                 ,   //269025123 0x1008FF63
    XF86MenuKB                  ,   //269025125 0x1008FF65
    XF86MenuPB                  ,   //269025126 0x1008FF66
    XF86MySites                 ,   //269025127 0x1008FF67
    XF86New                     ,   //269025128 0x1008FF68
    XF86News                    ,   //269025129 0x1008FF69
    XF86OfficeHome              ,   //269025130 0x1008FF6A
    XF86Open                    ,   //269025131 0x1008FF6B
    XF86Option                  ,   //269025132 0x1008FF6C
    XF86Paste                   ,   //269025133 0x1008FF6D
    XF86Phone                   ,   //269025134 0x1008FF6E
    XF86Q                       ,   //269025136 0x1008FF70
    XF86Reply                   ,   //269025138 0x1008FF72
    XF86Reload                  ,   //269025139 0x1008FF73
    XF86RotateWindows           ,   //269025140 0x1008FF74
    XF86RotationPB              ,   //269025141 0x1008FF75
    XF86RotationKB              ,   //269025142 0x1008FF76
    XF86Save                    ,   //269025143 0x1008FF77
    XF86ScrollUp                ,   //269025144 0x1008FF78
    XF86ScrollDown              ,   //269025145 0x1008FF79
    XF86ScrollClick             ,   //269025146 0x1008FF7A
    XF86Send                    ,   //269025147 0x1008FF7B
    XF86Spell                   ,   //269025148 0x1008FF7C
    XF86SplitScreen             ,   //269025149 0x1008FF7D
    XF86Support                 ,   //269025150 0x1008FF7E
    XF86TaskPane                ,   //269025151 0x1008FF7F
    XF86Terminal                ,   //269025152 0x1008FF80
    XF86Tools                   ,   //269025153 0x1008FF81
    XF86Travel                  ,   //269025154 0x1008FF82
    XF86UserPB                  ,   //269025156 0x1008FF84
    XF86User1KB                 ,   //269025157 0x1008FF85
    XF86User2KB                 ,   //269025158 0x1008FF86
    XF86Video                   ,   //269025159 0x1008FF87
    XF86WheelButton             ,   //269025160 0x1008FF88
    XF86Word                    ,   //269025161 0x1008FF89
    XF86Xfer                    ,   //269025162 0x1008FF8A
    XF86ZoomIn                  ,   //269025163 0x1008FF8B
    XF86ZoomOut                 ,   //269025164 0x1008FF8C
    XF86Away                    ,   //269025165 0x1008FF8D
    XF86Messenger               ,   //269025166 0x1008FF8E
    XF86WebCam                  ,   //269025167 0x1008FF8F
    XF86MailForward             ,   //269025168 0x1008FF90
    XF86Pictures                ,   //269025169 0x1008FF91
    XF86Music                   ,   //269025170 0x1008FF92
    XF86Battery                 ,   //269025171 0x1008FF93
    XF86Bluetooth               ,   //269025172 0x1008FF94
    XF86WLAN                    ,   //269025173 0x1008FF95
    XF86UWB                     ,   //269025174 0x1008FF96
    XF86AudioForward            ,   //269025175 0x1008FF97
    XF86AudioRepeat             ,   //269025176 0x1008FF98
    XF86AudioRandomPlay         ,   //269025177 0x1008FF99
    XF86Subtitle                ,   //269025178 0x1008FF9A
    XF86AudioCycleTrack         ,   //269025179 0x1008FF9B
    XF86CycleAngle              ,   //269025180 0x1008FF9C
    XF86FrameBack               ,   //269025181 0x1008FF9D
    XF86FrameForward            ,   //269025182 0x1008FF9E
    XF86Time                    ,   //269025183 0x1008FF9F
    XF86Select                  ,   //269025184 0x1008FFA0
    XF86View                    ,   //269025185 0x1008FFA1
    XF86TopMenu                 ,   //269025186 0x1008FFA2
    XF86Red                     ,   //269025187 0x1008FFA3
    XF86Green                   ,   //269025188 0x1008FFA4
    XF86Yellow                  ,   //269025189 0x1008FFA5
    XF86Blue                    ,   //269025190 0x1008FFA6
    XF86Suspend                 ,   //269025191 0x1008FFA7
    XF86Hibernate               ,   //269025192 0x1008FFA8
    XF86TouchpadToggle          ,   //269025193 0x1008FFA9
    XF86TouchpadOn              ,   //269025200 0x1008FFB0
    XF86TouchpadOff             ,   //269025201 0x1008FFB1
    XF86AudioMicMute            ,   //269025202 0x1008FFB2
    XF86Keyboard                ,   //269025203 0x1008FFB3
    XF86WWAN                    ,   //269025204 0x1008FFB4
    XF86RFKill                  ,   //269025205 0x1008FFB5
    XF86AudioPreset             ,   //269025206 0x1008FFB6
    XF86RotationLockToggle      ,   //269025207 0x1008FFB7
    XF86FullScreen              ,   //269025208 0x1008FFB8


}

pub struct InvalidKeySym( char );

impl TryFrom<char> for TkKey {
    type Error = InvalidKeySym;

    fn try_from( ch: char ) -> Result<TkKey, Self::Error> {
        match ch {
            ' '  => Ok( TkKey::space        ),
            '!'  => Ok( TkKey::exclam       ),
            '"'  => Ok( TkKey::quotedbl     ),
            '#'  => Ok( TkKey::numbersign   ),
            '$'  => Ok( TkKey::dollar       ),
            '%'  => Ok( TkKey::percent      ),
            '&'  => Ok( TkKey::ampersand    ),
            '\'' => Ok( TkKey::apostrophe   ),
            '('  => Ok( TkKey::parenleft    ),
            ')'  => Ok( TkKey::parenright   ),
            '*'  => Ok( TkKey::asterisk     ),
            '+'  => Ok( TkKey::plus         ),
            ','  => Ok( TkKey::comma        ),
            '-'  => Ok( TkKey::minus        ),
            '.'  => Ok( TkKey::period       ),
            '/'  => Ok( TkKey::slash        ),
            '0'  => Ok( TkKey::_0           ),
            '1'  => Ok( TkKey::_1           ),
            '2'  => Ok( TkKey::_2           ),
            '3'  => Ok( TkKey::_3           ),
            '4'  => Ok( TkKey::_4           ),
            '5'  => Ok( TkKey::_5           ),
            '6'  => Ok( TkKey::_6           ),
            '7'  => Ok( TkKey::_7           ),
            '8'  => Ok( TkKey::_8           ),
            '9'  => Ok( TkKey::_9           ),
            ':'  => Ok( TkKey::colon        ),
            ';'  => Ok( TkKey::semicolon    ),
            '<'  => Ok( TkKey::less         ),
            '='  => Ok( TkKey::equal        ),
            '>'  => Ok( TkKey::greater      ),
            '?'  => Ok( TkKey::question     ),
            '@'  => Ok( TkKey::at           ),
            'A'  => Ok( TkKey::A            ),
            'B'  => Ok( TkKey::B            ),
            'C'  => Ok( TkKey::C            ),
            'D'  => Ok( TkKey::D            ),
            'E'  => Ok( TkKey::E            ),
            'F'  => Ok( TkKey::F            ),
            'G'  => Ok( TkKey::G            ),
            'H'  => Ok( TkKey::H            ),
            'I'  => Ok( TkKey::I            ),
            'J'  => Ok( TkKey::J            ),
            'K'  => Ok( TkKey::K            ),
            'L'  => Ok( TkKey::L            ),
            'M'  => Ok( TkKey::M            ),
            'N'  => Ok( TkKey::N            ),
            'O'  => Ok( TkKey::O            ),
            'P'  => Ok( TkKey::P            ),
            'Q'  => Ok( TkKey::Q            ),
            'R'  => Ok( TkKey::R            ),
            'S'  => Ok( TkKey::S            ),
            'T'  => Ok( TkKey::T            ),
            'U'  => Ok( TkKey::U            ),
            'V'  => Ok( TkKey::V            ),
            'W'  => Ok( TkKey::W            ),
            'X'  => Ok( TkKey::X            ),
            'Y'  => Ok( TkKey::Y            ),
            'Z'  => Ok( TkKey::Z            ),
            '['  => Ok( TkKey::bracketleft  ),
            '\\' => Ok( TkKey::backslash    ),
            ']'  => Ok( TkKey::bracketright ),
            '^'  => Ok( TkKey::asciicircum  ),
            '_'  => Ok( TkKey::underscore   ),
            '`'  => Ok( TkKey::grave        ),
            'a'  => Ok( TkKey::a            ),
            'b'  => Ok( TkKey::b            ),
            'c'  => Ok( TkKey::c            ),
            'd'  => Ok( TkKey::d            ),
            'e'  => Ok( TkKey::e            ),
            'f'  => Ok( TkKey::f            ),
            'g'  => Ok( TkKey::g            ),
            'h'  => Ok( TkKey::h            ),
            'i'  => Ok( TkKey::i            ),
            'j'  => Ok( TkKey::j            ),
            'k'  => Ok( TkKey::k            ),
            'l'  => Ok( TkKey::l            ),
            'm'  => Ok( TkKey::m            ),
            'n'  => Ok( TkKey::n            ),
            'o'  => Ok( TkKey::o            ),
            'p'  => Ok( TkKey::p            ),
            'q'  => Ok( TkKey::q            ),
            'r'  => Ok( TkKey::r            ),
            's'  => Ok( TkKey::s            ),
            't'  => Ok( TkKey::t            ),
            'u'  => Ok( TkKey::u            ),
            'v'  => Ok( TkKey::v            ),
            'w'  => Ok( TkKey::w            ),
            'x'  => Ok( TkKey::x            ),
            'y'  => Ok( TkKey::y            ),
            'z'  => Ok( TkKey::z            ),
            '{'  => Ok( TkKey::braceleft    ),
            '|'  => Ok( TkKey::bar          ),
            '}'  => Ok( TkKey::braceright   ),
            '~'  => Ok( TkKey::asciitilde   ),
            _    => Err( InvalidKeySym( ch )),
        }
    }
}
