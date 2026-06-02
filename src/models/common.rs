use serde::{Deserialize, Serialize};

/// Language preference for UI messages shown to the user.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    English,
    Swedish,
    Norwegian,
    Danish,
    Finnish,
}

/// Standardised error code present in all `FAILED` status responses.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorCode {
    /// Authentication was rejected by the provider.
    AuthFailed,
    /// User or RP cancelled the transaction.
    Cancelled,
    /// Certificate or identity document is invalid or expired.
    InvalidId,
    /// A conflicting transaction is already in progress.
    Conflict,
    /// Unexpected internal error.
    InternalError,
    /// The session expired before completion.
    SessionTimeout,
    /// The user's device or app version is not supported.
    UnsupportedClient,
}

/// Whether the user or the relying party (RP) initiated the phone call.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallInitiator {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "RP")]
    Rp,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Country {
    #[serde(rename = "AFGHANISTAN")]
    Afghanistan,
    #[serde(rename = "ALAND_ISLANDS")]
    AlandIslands,
    #[serde(rename = "ALBANIA")]
    Albania,
    #[serde(rename = "ALGERIA")]
    Algeria,
    #[serde(rename = "AMERICAN_SAMOA")]
    AmericanSamoa,
    #[serde(rename = "ANDORRA")]
    Andorra,
    #[serde(rename = "ANGOLA")]
    Angola,
    #[serde(rename = "ANGUILLA")]
    Anguilla,
    #[serde(rename = "ANTARCTICA")]
    Antarctica,
    #[serde(rename = "ANTIGUA_AND_BARBUDA")]
    AntiguaAndBarbuda,
    #[serde(rename = "ARGENTINA")]
    Argentina,
    #[serde(rename = "ARMENIA")]
    Armenia,
    #[serde(rename = "ARUBA")]
    Aruba,
    #[serde(rename = "AUSTRALIA")]
    Australia,
    #[serde(rename = "AUSTRIA")]
    Austria,
    #[serde(rename = "AZERBAIJAN")]
    Azerbaijan,
    #[serde(rename = "BAHAMAS")]
    Bahamas,
    #[serde(rename = "BAHRAIN")]
    Bahrain,
    #[serde(rename = "BANGLADESH")]
    Bangladesh,
    #[serde(rename = "BARBADOS")]
    Barbados,
    #[serde(rename = "BELARUS")]
    Belarus,
    #[serde(rename = "BELGIUM")]
    Belgium,
    #[serde(rename = "BELIZE")]
    Belize,
    #[serde(rename = "BENIN")]
    Benin,
    #[serde(rename = "BERMUDA")]
    Bermuda,
    #[serde(rename = "BHUTAN")]
    Bhutan,
    #[serde(rename = "BOLIVIA")]
    Bolivia,
    #[serde(rename = "BONAIRE_SINT_EUSTATIUS_AND_SABA")]
    BonaireSintEustatiusAndSaba,
    #[serde(rename = "BOSNIA_AND_HERZEGOVINA")]
    BosniaAndHerzegovina,
    #[serde(rename = "BOTSWANA")]
    Botswana,
    #[serde(rename = "BOUVET_ISLAND")]
    BouvetIsland,
    #[serde(rename = "BRAZIL")]
    Brazil,
    #[serde(rename = "BRITISH_INDIAN_OCEAN_TERRITORY")]
    BritishIndianOceanTerritory,
    #[serde(rename = "BRUNEI_DARUSSALAM")]
    BruneiDarussalam,
    #[serde(rename = "BULGARIA")]
    Bulgaria,
    #[serde(rename = "BURKINA_FASO")]
    BurkinaFaso,
    #[serde(rename = "BURUNDI")]
    Burundi,
    #[serde(rename = "CABO_VERDE")]
    CaboVerde,
    #[serde(rename = "CAMBODIA")]
    Cambodia,
    #[serde(rename = "CAMEROON")]
    Cameroon,
    #[serde(rename = "CANADA")]
    Canada,
    #[serde(rename = "CAYMAN_ISLANDS")]
    CaymanIslands,
    #[serde(rename = "CENTRAL_AFRICAN_REPUBLIC")]
    CentralAfricanRepublic,
    #[serde(rename = "CHAD")]
    Chad,
    #[serde(rename = "CHILE")]
    Chile,
    #[serde(rename = "CHINA")]
    China,
    #[serde(rename = "CHRISTMAS_ISLAND")]
    ChristmasIsland,
    #[serde(rename = "COCOS_KEELING_ISLANDS")]
    CocosKeelingIslands,
    #[serde(rename = "COLOMBIA")]
    Colombia,
    #[serde(rename = "COMOROS")]
    Comoros,
    #[serde(rename = "CONGO")]
    Congo,
    #[serde(rename = "COOK_ISLANDS")]
    CookIslands,
    #[serde(rename = "COSTA_RICA")]
    CostaRica,
    #[serde(rename = "COTE_D_IVOIRE")]
    CoteDIvoire,
    #[serde(rename = "CROATIA")]
    Croatia,
    #[serde(rename = "CUBA")]
    Cuba,
    #[serde(rename = "CURACAO")]
    Curacao,
    #[serde(rename = "CYPRUS")]
    Cyprus,
    #[serde(rename = "CZECHIA")]
    Czechia,
    #[serde(rename = "DEMOCRATIC_PEOPLE_S_REPUBLIC_OF_KOREA")]
    DemocraticPeopleRepublicOfKorea,
    #[serde(rename = "DENMARK")]
    Denmark,
    #[serde(rename = "DJIBOUTI")]
    Djibouti,
    #[serde(rename = "DOMINICA")]
    Dominica,
    #[serde(rename = "DOMINICAN_REPUBLIC")]
    DominicanRepublic,
    #[serde(rename = "ECUADOR")]
    Ecuador,
    #[serde(rename = "EGYPT")]
    Egypt,
    #[serde(rename = "EL_SALVADOR")]
    ElSalvador,
    #[serde(rename = "EQUATORIAL_GUINEA")]
    EquatorialGuinea,
    #[serde(rename = "ERITREA")]
    Eritrea,
    #[serde(rename = "ESTONIA")]
    Estonia,
    #[serde(rename = "ETHIOPIA")]
    Ethiopia,
    #[serde(rename = "FALKLAND_ISLANDS_MALVINAS")]
    FalklandIslandsMalvinas,
    #[serde(rename = "FAROE_ISLANDS")]
    FaroeIslands,
    #[serde(rename = "FEDERATED_STATES_OF_MICRONESIA")]
    FederatedStatesOfMicronesia,
    #[serde(rename = "FIJI")]
    Fiji,
    #[serde(rename = "FINLAND")]
    Finland,
    #[serde(rename = "FRANCE")]
    France,
    #[serde(rename = "FRENCH_GUIANA")]
    FrenchGuiana,
    #[serde(rename = "FRENCH_POLYNESIA")]
    FrenchPolynesia,
    #[serde(rename = "FRENCH_SOUTHERN_TERRITORIES")]
    FrenchSouthernTerritories,
    #[serde(rename = "GABON")]
    Gabon,
    #[serde(rename = "GAMBIA")]
    Gambia,
    #[serde(rename = "GEORGIA")]
    Georgia,
    #[serde(rename = "GERMANY")]
    Germany,
    #[serde(rename = "GHANA")]
    Ghana,
    #[serde(rename = "GIBRALTAR")]
    Gibraltar,
    #[serde(rename = "GREECE")]
    Greece,
    #[serde(rename = "GREENLAND")]
    Greenland,
    #[serde(rename = "GRENADA")]
    Grenada,
    #[serde(rename = "GUADELOUPE")]
    Guadeloupe,
    #[serde(rename = "GUAM")]
    Guam,
    #[serde(rename = "GUATEMALA")]
    Guatemala,
    #[serde(rename = "GUERNSEY")]
    Guernsey,
    #[serde(rename = "GUINEA")]
    Guinea,
    #[serde(rename = "GUINEA_BISSAU")]
    GuineaBissau,
    #[serde(rename = "GUYANA")]
    Guyana,
    #[serde(rename = "HAITI")]
    Haiti,
    #[serde(rename = "HEARD_ISLAND_AND_MCDONALD_ISLANDS")]
    HeardIslandAndMcdonaldIslands,
    #[serde(rename = "HOLY_SEE")]
    HolySee,
    #[serde(rename = "HONDURAS")]
    Honduras,
    #[serde(rename = "HONG_KONG")]
    HongKong,
    #[serde(rename = "HUNGARY")]
    Hungary,
    #[serde(rename = "ICELAND")]
    Iceland,
    #[serde(rename = "INDIA")]
    India,
    #[serde(rename = "INDONESIA")]
    Indonesia,
    #[serde(rename = "IRAQ")]
    Iraq,
    #[serde(rename = "IRELAND")]
    Ireland,
    #[serde(rename = "ISLAMIC_REPUBLIC_OF_IRAN")]
    IslamicRepublicOfIran,
    #[serde(rename = "ISLE_OF_MAN")]
    IsleOfMan,
    #[serde(rename = "ISRAEL")]
    Israel,
    #[serde(rename = "ITALY")]
    Italy,
    #[serde(rename = "JAMAICA")]
    Jamaica,
    #[serde(rename = "JAPAN")]
    Japan,
    #[serde(rename = "JERSEY")]
    Jersey,
    #[serde(rename = "JORDAN")]
    Jordan,
    #[serde(rename = "KAZAKHSTAN")]
    Kazakhstan,
    #[serde(rename = "KENYA")]
    Kenya,
    #[serde(rename = "KIRIBATI")]
    Kiribati,
    #[serde(rename = "KUWAIT")]
    Kuwait,
    #[serde(rename = "KYRGYZSTAN")]
    Kyrgyzstan,
    #[serde(rename = "LAO_PEOPLE_S_DEMOCRATIC_REPUBLIC")]
    LaoPeopleDemocraticRepublic,
    #[serde(rename = "LATVIA")]
    Latvia,
    #[serde(rename = "LEBANON")]
    Lebanon,
    #[serde(rename = "LESOTHO")]
    Lesotho,
    #[serde(rename = "LIBERIA")]
    Liberia,
    #[serde(rename = "LIBYA")]
    Libya,
    #[serde(rename = "LIECHTENSTEIN")]
    Liechtenstein,
    #[serde(rename = "LITHUANIA")]
    Lithuania,
    #[serde(rename = "LUXEMBOURG")]
    Luxembourg,
    #[serde(rename = "MACAO")]
    Macao,
    #[serde(rename = "MACEDONIA")]
    Macedonia,
    #[serde(rename = "MADAGASCAR")]
    Madagascar,
    #[serde(rename = "MALAWI")]
    Malawi,
    #[serde(rename = "MALAYSIA")]
    Malaysia,
    #[serde(rename = "MALDIVES")]
    Maldives,
    #[serde(rename = "MALI")]
    Mali,
    #[serde(rename = "MALTA")]
    Malta,
    #[serde(rename = "MARSHALL_ISLANDS")]
    MarshallIslands,
    #[serde(rename = "MARTINIQUE")]
    Martinique,
    #[serde(rename = "MAURITANIA")]
    Mauritania,
    #[serde(rename = "MAURITIUS")]
    Mauritius,
    #[serde(rename = "MAYOTTE")]
    Mayotte,
    #[serde(rename = "MEXICO")]
    Mexico,
    #[serde(rename = "MONACO")]
    Monaco,
    #[serde(rename = "MONGOLIA")]
    Mongolia,
    #[serde(rename = "MONTENEGRO")]
    Montenegro,
    #[serde(rename = "MONTSERRAT")]
    Montserrat,
    #[serde(rename = "MOROCCO")]
    Morocco,
    #[serde(rename = "MOZAMBIQUE")]
    Mozambique,
    #[serde(rename = "MYANMAR")]
    Myanmar,
    #[serde(rename = "NAMIBIA")]
    Namibia,
    #[serde(rename = "NAURU")]
    Nauru,
    #[serde(rename = "NEPAL")]
    Nepal,
    #[serde(rename = "NETHERLANDS")]
    Netherlands,
    #[serde(rename = "NEW_CALEDONIA")]
    NewCaledonia,
    #[serde(rename = "NEW_ZEALAND")]
    NewZealand,
    #[serde(rename = "NICARAGUA")]
    Nicaragua,
    #[serde(rename = "NIGER")]
    Niger,
    #[serde(rename = "NIGERIA")]
    Nigeria,
    #[serde(rename = "NIUE")]
    Niue,
    #[serde(rename = "NORFOLK_ISLAND")]
    NorfolkIsland,
    #[serde(rename = "NORTHERN_MARIANA_ISLANDS")]
    NorthernMarianaIslands,
    #[serde(rename = "NORWAY")]
    Norway,
    #[serde(rename = "OMAN")]
    Oman,
    #[serde(rename = "PAKISTAN")]
    Pakistan,
    #[serde(rename = "PALAU")]
    Palau,
    #[serde(rename = "PALESTINE")]
    Palestine,
    #[serde(rename = "PANAMA")]
    Panama,
    #[serde(rename = "PAPUA_NEW_GUINEA")]
    PapuaNewGuinea,
    #[serde(rename = "PARAGUAY")]
    Paraguay,
    #[serde(rename = "PERU")]
    Peru,
    #[serde(rename = "PHILIPPINES")]
    Philippines,
    #[serde(rename = "PITCAIRN")]
    Pitcairn,
    #[serde(rename = "POLAND")]
    Poland,
    #[serde(rename = "PORTUGAL")]
    Portugal,
    #[serde(rename = "PUERTO_RICO")]
    PuertoRico,
    #[serde(rename = "QATAR")]
    Qatar,
    #[serde(rename = "REPUBLIC_OF_KOREA")]
    RepublicOfKorea,
    #[serde(rename = "REPUBLIC_OF_MOLDOVA")]
    RepublicOfMoldova,
    #[serde(rename = "REUNION")]
    Reunion,
    #[serde(rename = "ROMANIA")]
    Romania,
    #[serde(rename = "RUSSIAN_FEDERATION")]
    RussianFederation,
    #[serde(rename = "RWANDA")]
    Rwanda,
    #[serde(rename = "SAINT_BARTHELEMY")]
    SaintBarthelemy,
    #[serde(rename = "SAINT_HELENA_ASCENSION_AND_TRISTAN_DA_CUNHA")]
    SaintHelenaAscensionAndTristanDaCunha,
    #[serde(rename = "SAINT_KITTS_AND_NEVIS")]
    SaintKittsAndNevis,
    #[serde(rename = "SAINT_LUCIA")]
    SaintLucia,
    #[serde(rename = "SAINT_MARTIN_FRENCH_PART")]
    SaintMartinFrenchPart,
    #[serde(rename = "SAINT_PIERRE_AND_MIQUELON")]
    SaintPierreAndMiquelon,
    #[serde(rename = "SAINT_VINCENT_AND_THE_GRENADINES")]
    SaintVincentAndTheGrenadines,
    #[serde(rename = "SAMOA")]
    Samoa,
    #[serde(rename = "SAN_MARINO")]
    SanMarino,
    #[serde(rename = "SAO_TOME_AND_PRINCIPE")]
    SaoTomeAndPrincipe,
    #[serde(rename = "SAUDI_ARABIA")]
    SaudiArabia,
    #[serde(rename = "SENEGAL")]
    Senegal,
    #[serde(rename = "SERBIA")]
    Serbia,
    #[serde(rename = "SEYCHELLES")]
    Seychelles,
    #[serde(rename = "SIERRA_LEONE")]
    SierraLeone,
    #[serde(rename = "SINGAPORE")]
    Singapore,
    #[serde(rename = "SINT_MAARTEN_DUTCH_PART")]
    SintMaartenDutchPart,
    #[serde(rename = "SLOVAKIA")]
    Slovakia,
    #[serde(rename = "SLOVENIA")]
    Slovenia,
    #[serde(rename = "SOLOMON_ISLANDS")]
    SolomonIslands,
    #[serde(rename = "SOMALIA")]
    Somalia,
    #[serde(rename = "SOUTH_AFRICA")]
    SouthAfrica,
    #[serde(rename = "SOUTH_GEORGIA_AND_THE_SOUTH_SANDWICH_ISLANDS")]
    SouthGeorgiaAndTheSouthSandwichIslands,
    #[serde(rename = "SOUTH_SUDAN")]
    SouthSudan,
    #[serde(rename = "SPAIN")]
    Spain,
    #[serde(rename = "SRI_LANKA")]
    SriLanka,
    #[serde(rename = "SUDAN")]
    Sudan,
    #[serde(rename = "SURINAME")]
    Suriname,
    #[serde(rename = "SVALBARD_AND_JAN_MAYEN")]
    SvalbardAndJanMayen,
    #[serde(rename = "SWAZILAND")]
    Swaziland,
    #[serde(rename = "SWEDEN")]
    Sweden,
    #[serde(rename = "SWITZERLAND")]
    Switzerland,
    #[serde(rename = "SYRIAN_ARAB_REPUBLIC")]
    SyrianArabRepublic,
    #[serde(rename = "TAIWAN_PROVINCE_OF_CHINA")]
    TaiwanProvinceOfChina,
    #[serde(rename = "TAJIKISTAN")]
    Tajikistan,
    #[serde(rename = "THAILAND")]
    Thailand,
    #[serde(rename = "THE_DEMOCRATIC_REPUBLIC_OF_THE_CONGO")]
    TheDemocraticRepublicOfTheCongo,
    #[serde(rename = "TIMOR_LESTE")]
    TimorLeste,
    #[serde(rename = "TOGO")]
    Togo,
    #[serde(rename = "TOKELAU")]
    Tokelau,
    #[serde(rename = "TONGA")]
    Tonga,
    #[serde(rename = "TRINIDAD_AND_TOBAGO")]
    TrinidadAndTobago,
    #[serde(rename = "TUNISIA")]
    Tunisia,
    #[serde(rename = "TURKEY")]
    Turkey,
    #[serde(rename = "TURKMENISTAN")]
    Turkmenistan,
    #[serde(rename = "TURKS_AND_CAICOS_ISLANDS")]
    TurksAndCaicosIslands,
    #[serde(rename = "TUVALU")]
    Tuvalu,
    #[serde(rename = "UGANDA")]
    Uganda,
    #[serde(rename = "UKRAINE")]
    Ukraine,
    #[serde(rename = "UNITED_ARAB_EMIRATES")]
    UnitedArabEmirates,
    #[serde(rename = "UNITED_KINGDOM")]
    UnitedKingdom,
    #[serde(rename = "UNITED_REPUBLIC_OF_TANZANIA")]
    UnitedRepublicOfTanzania,
    #[serde(rename = "UNITED_STATES_MINOR_OUTLYING_ISLANDS")]
    UnitedStatesMinorOutlyingIslands,
    #[serde(rename = "UNITED_STATES_OF_AMERICA")]
    UnitedStatesOfAmerica,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "URUGUAY")]
    Uruguay,
    #[serde(rename = "UZBEKISTAN")]
    Uzbekistan,
    #[serde(rename = "VANUATU")]
    Vanuatu,
    #[serde(rename = "VENEZUELA")]
    Venezuela,
    #[serde(rename = "VIET_NAM")]
    VietNam,
    #[serde(rename = "VIRGIN_ISLANDS_BRITISH")]
    VirginIslandsBritish,
    #[serde(rename = "VIRGIN_ISLANDS_US")]
    VirginIslandsUs,
    #[serde(rename = "WALLIS_AND_FUTUNA")]
    WallisAndFutuna,
    #[serde(rename = "WESTERN_SAHARA")]
    WesternSahara,
    #[serde(rename = "YEMEN")]
    Yemen,
    #[serde(rename = "ZAMBIA")]
    Zambia,
    #[serde(rename = "ZIMBABWE")]
    Zimbabwe,
}
