// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use crate::currency_info::CurrencyKey;

#[doc = "Currencies defined by ISO 4217."]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ISOCurrency {
    #[doc = "UAE Dirham"]
    #[doc = "Official currency in United Arab Emirates (The)"]
    AED,
    #[doc = "Afghani"]
    #[doc = "Official currency in Afghanistan"]
    AFN,
    #[doc = "Lek"]
    #[doc = "Official currency in Albania"]
    ALL,
    #[doc = "Armenian Dram"]
    #[doc = "Official currency in Armenia"]
    AMD,
    #[doc = "Netherlands Antillean Guilder"]
    #[doc = "Official currency in Curaçao, Sint Maarten (Dutch Part)"]
    ANG,
    #[doc = "Kwanza"]
    #[doc = "Official currency in Angola"]
    AOA,
    #[doc = "Argentine Peso"]
    #[doc = "Official currency in Argentina"]
    ARS,
    #[doc = "Australian Dollar"]
    #[doc = "Official currency in Australia, Christmas Island, Cocos \
             (Keeling) Islands (The), Heard Island And Mcdonald Islands, \
             Kiribati, Nauru, Norfolk Island, Tuvalu"]
    AUD,
    #[doc = "Aruban Florin"]
    #[doc = "Official currency in Aruba"]
    AWG,
    #[doc = "Azerbaijan Manat"]
    #[doc = "Official currency in Azerbaijan"]
    AZN,
    #[doc = "Convertible Mark"]
    #[doc = "Official currency in Bosnia And Herzegovina"]
    BAM,
    #[doc = "Barbados Dollar"]
    #[doc = "Official currency in Barbados"]
    BBD,
    #[doc = "Taka"]
    #[doc = "Official currency in Bangladesh"]
    BDT,
    #[doc = "Bulgarian Lev"]
    #[doc = "Official currency in Bulgaria"]
    BGN,
    #[doc = "Bahraini Dinar"]
    #[doc = "Official currency in Bahrain"]
    BHD,
    #[doc = "Burundi Franc"]
    #[doc = "Official currency in Burundi"]
    BIF,
    #[doc = "Bermudian Dollar"]
    #[doc = "Official currency in Bermuda"]
    BMD,
    #[doc = "Brunei Dollar"]
    #[doc = "Official currency in Brunei Darussalam"]
    BND,
    #[doc = "Boliviano"]
    #[doc = "Official currency in Bolivia (Plurinational State Of)"]
    BOB,
    #[doc = "Mvdol"]
    #[doc = "Official currency in Bolivia (Plurinational State Of)"]
    BOV,
    #[doc = "Brazilian Real"]
    #[doc = "Official currency in Brazil"]
    BRL,
    #[doc = "Bahamian Dollar"]
    #[doc = "Official currency in Bahamas (The)"]
    BSD,
    #[doc = "Ngultrum"]
    #[doc = "Official currency in Bhutan"]
    BTN,
    #[doc = "Pula"]
    #[doc = "Official currency in Botswana"]
    BWP,
    #[doc = "Belarusian Ruble"]
    #[doc = "Official currency in Belarus"]
    BYN,
    #[doc = "Belize Dollar"]
    #[doc = "Official currency in Belize"]
    BZD,
    #[doc = "Canadian Dollar"]
    #[doc = "Official currency in Canada"]
    CAD,
    #[doc = "Congolese Franc"]
    #[doc = "Official currency in Congo (The Democratic Republic Of The)"]
    CDF,
    #[doc = "WIR Euro"]
    #[doc = "Official currency in Switzerland"]
    CHE,
    #[doc = "Swiss Franc"]
    #[doc = "Official currency in Liechtenstein, Switzerland"]
    CHF,
    #[doc = "WIR Franc"]
    #[doc = "Official currency in Switzerland"]
    CHW,
    #[doc = "Unidad de Fomento"]
    #[doc = "Official currency in Chile"]
    CLF,
    #[doc = "Chilean Peso"]
    #[doc = "Official currency in Chile"]
    CLP,
    #[doc = "Yuan Renminbi"]
    #[doc = "Official currency in China"]
    CNY,
    #[doc = "Colombian Peso"]
    #[doc = "Official currency in Colombia"]
    COP,
    #[doc = "Unidad de Valor Real"]
    #[doc = "Official currency in Colombia"]
    COU,
    #[doc = "Costa Rican Colon"]
    #[doc = "Official currency in Costa Rica"]
    CRC,
    #[doc = "Peso Convertible"]
    #[doc = "Official currency in Cuba"]
    CUC,
    #[doc = "Cuban Peso"]
    #[doc = "Official currency in Cuba"]
    CUP,
    #[doc = "Cabo Verde Escudo"]
    #[doc = "Official currency in Cabo Verde"]
    CVE,
    #[doc = "Czech Koruna"]
    #[doc = "Official currency in Czechia"]
    CZK,
    #[doc = "Djibouti Franc"]
    #[doc = "Official currency in Djibouti"]
    DJF,
    #[doc = "Danish Krone"]
    #[doc = "Official currency in Denmark, Faroe Islands (The), Greenland"]
    DKK,
    #[doc = "Dominican Peso"]
    #[doc = "Official currency in Dominican Republic (The)"]
    DOP,
    #[doc = "Algerian Dinar"]
    #[doc = "Official currency in Algeria"]
    DZD,
    #[doc = "Egyptian Pound"]
    #[doc = "Official currency in Egypt"]
    EGP,
    #[doc = "Nakfa"]
    #[doc = "Official currency in Eritrea"]
    ERN,
    #[doc = "Ethiopian Birr"]
    #[doc = "Official currency in Ethiopia"]
    ETB,
    #[doc = "Euro"]
    #[doc = "Official currency in Åland Islands, Andorra, Austria, Belgium, \
             Croatia, Cyprus, Estonia, European Union, Finland, France, \
             French Guiana, French Southern Territories (The), Germany, \
             Greece, Guadeloupe, Holy See (The), Ireland, Italy, Latvia, \
             Lithuania, Luxembourg, Malta, Martinique, Mayotte, Monaco, \
             Montenegro, Netherlands (The), Portugal, Réunion, Saint \
             Barthélemy, Saint Martin (French Part), Saint Pierre And \
             Miquelon, San Marino, Slovakia, Slovenia, Spain"]
    EUR,
    #[doc = "Fiji Dollar"]
    #[doc = "Official currency in Fiji"]
    FJD,
    #[doc = "Falkland Islands Pound"]
    #[doc = "Official currency in Falkland Islands (The) (Malvinas)"]
    FKP,
    #[doc = "Pound Sterling"]
    #[doc = "Official currency in Guernsey, Isle Of Man, Jersey, United \
             Kingdom Of Great Britain And Northern Ireland (The)"]
    GBP,
    #[doc = "Lari"]
    #[doc = "Official currency in Georgia"]
    GEL,
    #[doc = "Ghana Cedi"]
    #[doc = "Official currency in Ghana"]
    GHS,
    #[doc = "Gibraltar Pound"]
    #[doc = "Official currency in Gibraltar"]
    GIP,
    #[doc = "Dalasi"]
    #[doc = "Official currency in Gambia (The)"]
    GMD,
    #[doc = "Guinean Franc"]
    #[doc = "Official currency in Guinea"]
    GNF,
    #[doc = "Quetzal"]
    #[doc = "Official currency in Guatemala"]
    GTQ,
    #[doc = "Guyana Dollar"]
    #[doc = "Official currency in Guyana"]
    GYD,
    #[doc = "Hong Kong Dollar"]
    #[doc = "Official currency in Hong Kong"]
    HKD,
    #[doc = "Lempira"]
    #[doc = "Official currency in Honduras"]
    HNL,
    #[doc = "Kuna"]
    #[doc = "Historic currency in Croatia"]
    HRK,
    #[doc = "Gourde"]
    #[doc = "Official currency in Haiti"]
    HTG,
    #[doc = "Forint"]
    #[doc = "Official currency in Hungary"]
    HUF,
    #[doc = "Rupiah"]
    #[doc = "Official currency in Indonesia"]
    IDR,
    #[doc = "New Israeli Sheqel"]
    #[doc = "Official currency in Israel"]
    ILS,
    #[doc = "Indian Rupee"]
    #[doc = "Official currency in Bhutan, India"]
    INR,
    #[doc = "Iraqi Dinar"]
    #[doc = "Official currency in Iraq"]
    IQD,
    #[doc = "Iranian Rial"]
    #[doc = "Official currency in Iran (Islamic Republic Of)"]
    IRR,
    #[doc = "Iceland Krona"]
    #[doc = "Official currency in Iceland"]
    ISK,
    #[doc = "Jamaican Dollar"]
    #[doc = "Official currency in Jamaica"]
    JMD,
    #[doc = "Jordanian Dinar"]
    #[doc = "Official currency in Jordan"]
    JOD,
    #[doc = "Yen"]
    #[doc = "Official currency in Japan"]
    JPY,
    #[doc = "Kenyan Shilling"]
    #[doc = "Official currency in Kenya"]
    KES,
    #[doc = "Som"]
    #[doc = "Official currency in Kyrgyzstan"]
    KGS,
    #[doc = "Riel"]
    #[doc = "Official currency in Cambodia"]
    KHR,
    #[doc = "Comorian Franc "]
    #[doc = "Official currency in Comoros (The)"]
    KMF,
    #[doc = "North Korean Won"]
    #[doc = "Official currency in Korea (The Democratic People’S Republic \
             Of)"]
    KPW,
    #[doc = "Won"]
    #[doc = "Official currency in Korea (The Republic Of)"]
    KRW,
    #[doc = "Kuwaiti Dinar"]
    #[doc = "Official currency in Kuwait"]
    KWD,
    #[doc = "Cayman Islands Dollar"]
    #[doc = "Official currency in Cayman Islands (The)"]
    KYD,
    #[doc = "Tenge"]
    #[doc = "Official currency in Kazakhstan"]
    KZT,
    #[doc = "Lao Kip"]
    #[doc = "Official currency in Lao People’S Democratic Republic (The)"]
    LAK,
    #[doc = "Lebanese Pound"]
    #[doc = "Official currency in Lebanon"]
    LBP,
    #[doc = "Sri Lanka Rupee"]
    #[doc = "Official currency in Sri Lanka"]
    LKR,
    #[doc = "Liberian Dollar"]
    #[doc = "Official currency in Liberia"]
    LRD,
    #[doc = "Loti"]
    #[doc = "Official currency in Lesotho"]
    LSL,
    #[doc = "Libyan Dinar"]
    #[doc = "Official currency in Libya"]
    LYD,
    #[doc = "Moroccan Dirham"]
    #[doc = "Official currency in Morocco, Western Sahara"]
    MAD,
    #[doc = "Moldovan Leu"]
    #[doc = "Official currency in Moldova (The Republic Of)"]
    MDL,
    #[doc = "Malagasy Ariary"]
    #[doc = "Official currency in Madagascar"]
    MGA,
    #[doc = "Denar"]
    #[doc = "Official currency in North Macedonia"]
    MKD,
    #[doc = "Kyat"]
    #[doc = "Official currency in Myanmar"]
    MMK,
    #[doc = "Tugrik"]
    #[doc = "Official currency in Mongolia"]
    MNT,
    #[doc = "Pataca"]
    #[doc = "Official currency in Macao"]
    MOP,
    #[doc = "Ouguiya"]
    #[doc = "Official currency in Mauritania"]
    MRU,
    #[doc = "Mauritius Rupee"]
    #[doc = "Official currency in Mauritius"]
    MUR,
    #[doc = "Rufiyaa"]
    #[doc = "Official currency in Maldives"]
    MVR,
    #[doc = "Malawi Kwacha"]
    #[doc = "Official currency in Malawi"]
    MWK,
    #[doc = "Mexican Peso"]
    #[doc = "Official currency in Mexico"]
    MXN,
    #[doc = "Mexican Unidad de Inversion (UDI)"]
    #[doc = "Official currency in Mexico"]
    MXV,
    #[doc = "Malaysian Ringgit"]
    #[doc = "Official currency in Malaysia"]
    MYR,
    #[doc = "Mozambique Metical"]
    #[doc = "Official currency in Mozambique"]
    MZN,
    #[doc = "Namibia Dollar"]
    #[doc = "Official currency in Namibia"]
    NAD,
    #[doc = "Naira"]
    #[doc = "Official currency in Nigeria"]
    NGN,
    #[doc = "Cordoba Oro"]
    #[doc = "Official currency in Nicaragua"]
    NIO,
    #[doc = "Norwegian Krone"]
    #[doc = "Official currency in Bouvet Island, Norway, Svalbard And Jan \
             Mayen"]
    NOK,
    #[doc = "Nepalese Rupee"]
    #[doc = "Official currency in Nepal"]
    NPR,
    #[doc = "New Zealand Dollar"]
    #[doc = "Official currency in Cook Islands (The), New Zealand, Niue, \
             Pitcairn, Tokelau"]
    NZD,
    #[doc = "Rial Omani"]
    #[doc = "Official currency in Oman"]
    OMR,
    #[doc = "Balboa"]
    #[doc = "Official currency in Panama"]
    PAB,
    #[doc = "Sol"]
    #[doc = "Official currency in Peru"]
    PEN,
    #[doc = "Kina"]
    #[doc = "Official currency in Papua New Guinea"]
    PGK,
    #[doc = "Philippine Peso"]
    #[doc = "Official currency in Philippines (The)"]
    PHP,
    #[doc = "Pakistan Rupee"]
    #[doc = "Official currency in Pakistan"]
    PKR,
    #[doc = "Zloty"]
    #[doc = "Official currency in Poland"]
    PLN,
    #[doc = "Guarani"]
    #[doc = "Official currency in Paraguay"]
    PYG,
    #[doc = "Qatari Rial"]
    #[doc = "Official currency in Qatar"]
    QAR,
    #[doc = "Romanian Leu"]
    #[doc = "Official currency in Romania"]
    RON,
    #[doc = "Serbian Dinar"]
    #[doc = "Official currency in Serbia"]
    RSD,
    #[doc = "Russian Ruble"]
    #[doc = "Official currency in Russian Federation (The)"]
    RUB,
    #[doc = "Rwanda Franc"]
    #[doc = "Official currency in Rwanda"]
    RWF,
    #[doc = "Saudi Riyal"]
    #[doc = "Official currency in Saudi Arabia"]
    SAR,
    #[doc = "Solomon Islands Dollar"]
    #[doc = "Official currency in Solomon Islands"]
    SBD,
    #[doc = "Seychelles Rupee"]
    #[doc = "Official currency in Seychelles"]
    SCR,
    #[doc = "Sudanese Pound"]
    #[doc = "Official currency in Sudan (The)"]
    SDG,
    #[doc = "Swedish Krona"]
    #[doc = "Official currency in Sweden"]
    SEK,
    #[doc = "Singapore Dollar"]
    #[doc = "Official currency in Singapore"]
    SGD,
    #[doc = "Saint Helena Pound"]
    #[doc = "Official currency in Saint Helena, Ascension And Tristan Da \
             Cunha"]
    SHP,
    #[doc = "Leone"]
    #[doc = "Official currency in Sierra Leone"]
    SLE,
    #[doc = "Leone"]
    #[doc = "Official currency in Sierra Leone"]
    SLL,
    #[doc = "Somali Shilling"]
    #[doc = "Official currency in Somalia"]
    SOS,
    #[doc = "Surinam Dollar"]
    #[doc = "Official currency in Suriname"]
    SRD,
    #[doc = "South Sudanese Pound"]
    #[doc = "Official currency in South Sudan"]
    SSP,
    #[doc = "Dobra"]
    #[doc = "Official currency in Sao Tome And Principe"]
    STN,
    #[doc = "El Salvador Colon"]
    #[doc = "Official currency in El Salvador"]
    SVC,
    #[doc = "Syrian Pound"]
    #[doc = "Official currency in Syrian Arab Republic"]
    SYP,
    #[doc = "Lilangeni"]
    #[doc = "Official currency in Eswatini"]
    SZL,
    #[doc = "Baht"]
    #[doc = "Official currency in Thailand"]
    THB,
    #[doc = "Somoni"]
    #[doc = "Official currency in Tajikistan"]
    TJS,
    #[doc = "Turkmenistan New Manat"]
    #[doc = "Official currency in Turkmenistan"]
    TMT,
    #[doc = "Tunisian Dinar"]
    #[doc = "Official currency in Tunisia"]
    TND,
    #[doc = "Pa’anga"]
    #[doc = "Official currency in Tonga"]
    TOP,
    #[doc = "Turkish Lira"]
    #[doc = "Official currency in Türkiye"]
    TRY,
    #[doc = "Trinidad and Tobago Dollar"]
    #[doc = "Official currency in Trinidad And Tobago"]
    TTD,
    #[doc = "New Taiwan Dollar"]
    #[doc = "Official currency in Taiwan (Province Of China)"]
    TWD,
    #[doc = "Tanzanian Shilling"]
    #[doc = "Official currency in Tanzania, United Republic Of"]
    TZS,
    #[doc = "Hryvnia"]
    #[doc = "Official currency in Ukraine"]
    UAH,
    #[doc = "Uganda Shilling"]
    #[doc = "Official currency in Uganda"]
    UGX,
    #[doc = "US Dollar"]
    #[doc = "Official currency in American Samoa, Bonaire, Sint Eustatius \
             And Saba, British Indian Ocean Territory (The), Ecuador, El \
             Salvador, Guam, Haiti, Marshall Islands (The), Micronesia \
             (Federated States Of), Northern Mariana Islands (The), Palau, \
             Panama, Puerto Rico, Timor-Leste, Turks And Caicos Islands \
             (The), United States Minor Outlying Islands (The), United \
             States Of America (The), Virgin Islands (British), Virgin \
             Islands (U.S.)"]
    USD,
    #[doc = "US Dollar (Next day)"]
    #[doc = "Official currency in United States Of America (The)"]
    USN,
    #[doc = "Uruguay Peso en Unidades Indexadas (UI)"]
    #[doc = "Official currency in Uruguay"]
    UYI,
    #[doc = "Peso Uruguayo"]
    #[doc = "Official currency in Uruguay"]
    UYU,
    #[doc = "Unidad Previsional"]
    #[doc = "Official currency in Uruguay"]
    UYW,
    #[doc = "Uzbekistan Sum"]
    #[doc = "Official currency in Uzbekistan"]
    UZS,
    #[doc = "Bolívar Soberano"]
    #[doc = "Official currency in Venezuela (Bolivarian Republic Of)"]
    VED,
    #[doc = "Bolívar Soberano"]
    #[doc = "Official currency in Venezuela (Bolivarian Republic Of)"]
    VES,
    #[doc = "Dong"]
    #[doc = "Official currency in Viet Nam"]
    VND,
    #[doc = "Vatu"]
    #[doc = "Official currency in Vanuatu"]
    VUV,
    #[doc = "Tala"]
    #[doc = "Official currency in Samoa"]
    WST,
    #[doc = "CFA Franc BEAC"]
    #[doc = "Official currency in Cameroon, Central African Republic (The), \
             Chad, Congo (The), Equatorial Guinea, Gabon"]
    XAF,
    #[doc = "East Caribbean Dollar"]
    #[doc = "Official currency in Anguilla, Antigua And Barbuda, Dominica, \
             Grenada, Montserrat, Saint Kitts And Nevis, Saint Lucia, Saint \
             Vincent And The Grenadines"]
    XCD,
    #[doc = "CFA Franc BCEAO"]
    #[doc = "Official currency in Benin, Burkina Faso, Côte D'Ivoire, \
             Guinea-Bissau, Mali, Niger (The), Senegal, Togo"]
    XOF,
    #[doc = "CFP Franc"]
    #[doc = "Official currency in French Polynesia, New Caledonia, Wallis \
             And Futuna"]
    XPF,
    #[doc = "Yemeni Rial"]
    #[doc = "Official currency in Yemen"]
    YER,
    #[doc = "Rand"]
    #[doc = "Official currency in Lesotho, Namibia, South Africa"]
    ZAR,
    #[doc = "Zambian Kwacha"]
    #[doc = "Official currency in Zambia"]
    ZMW,
    #[doc = "Zimbabwe Dollar"]
    #[doc = "Official currency in Zimbabwe"]
    ZWL,
}

impl ISOCurrency {
    const VARIANTS: [Self; 168_usize] = [
        Self::AED,
        Self::AFN,
        Self::ALL,
        Self::AMD,
        Self::ANG,
        Self::AOA,
        Self::ARS,
        Self::AUD,
        Self::AWG,
        Self::AZN,
        Self::BAM,
        Self::BBD,
        Self::BDT,
        Self::BGN,
        Self::BHD,
        Self::BIF,
        Self::BMD,
        Self::BND,
        Self::BOB,
        Self::BOV,
        Self::BRL,
        Self::BSD,
        Self::BTN,
        Self::BWP,
        Self::BYN,
        Self::BZD,
        Self::CAD,
        Self::CDF,
        Self::CHE,
        Self::CHF,
        Self::CHW,
        Self::CLF,
        Self::CLP,
        Self::CNY,
        Self::COP,
        Self::COU,
        Self::CRC,
        Self::CUC,
        Self::CUP,
        Self::CVE,
        Self::CZK,
        Self::DJF,
        Self::DKK,
        Self::DOP,
        Self::DZD,
        Self::EGP,
        Self::ERN,
        Self::ETB,
        Self::EUR,
        Self::FJD,
        Self::FKP,
        Self::GBP,
        Self::GEL,
        Self::GHS,
        Self::GIP,
        Self::GMD,
        Self::GNF,
        Self::GTQ,
        Self::GYD,
        Self::HKD,
        Self::HNL,
        Self::HRK,
        Self::HTG,
        Self::HUF,
        Self::IDR,
        Self::ILS,
        Self::INR,
        Self::IQD,
        Self::IRR,
        Self::ISK,
        Self::JMD,
        Self::JOD,
        Self::JPY,
        Self::KES,
        Self::KGS,
        Self::KHR,
        Self::KMF,
        Self::KPW,
        Self::KRW,
        Self::KWD,
        Self::KYD,
        Self::KZT,
        Self::LAK,
        Self::LBP,
        Self::LKR,
        Self::LRD,
        Self::LSL,
        Self::LYD,
        Self::MAD,
        Self::MDL,
        Self::MGA,
        Self::MKD,
        Self::MMK,
        Self::MNT,
        Self::MOP,
        Self::MRU,
        Self::MUR,
        Self::MVR,
        Self::MWK,
        Self::MXN,
        Self::MXV,
        Self::MYR,
        Self::MZN,
        Self::NAD,
        Self::NGN,
        Self::NIO,
        Self::NOK,
        Self::NPR,
        Self::NZD,
        Self::OMR,
        Self::PAB,
        Self::PEN,
        Self::PGK,
        Self::PHP,
        Self::PKR,
        Self::PLN,
        Self::PYG,
        Self::QAR,
        Self::RON,
        Self::RSD,
        Self::RUB,
        Self::RWF,
        Self::SAR,
        Self::SBD,
        Self::SCR,
        Self::SDG,
        Self::SEK,
        Self::SGD,
        Self::SHP,
        Self::SLE,
        Self::SLL,
        Self::SOS,
        Self::SRD,
        Self::SSP,
        Self::STN,
        Self::SVC,
        Self::SYP,
        Self::SZL,
        Self::THB,
        Self::TJS,
        Self::TMT,
        Self::TND,
        Self::TOP,
        Self::TRY,
        Self::TTD,
        Self::TWD,
        Self::TZS,
        Self::UAH,
        Self::UGX,
        Self::USD,
        Self::USN,
        Self::UYI,
        Self::UYU,
        Self::UYW,
        Self::UZS,
        Self::VED,
        Self::VES,
        Self::VND,
        Self::VUV,
        Self::WST,
        Self::XAF,
        Self::XCD,
        Self::XOF,
        Self::XPF,
        Self::YER,
        Self::ZAR,
        Self::ZMW,
        Self::ZWL,
    ];

    pub(crate) const fn info<'a>(&self) -> (&'a str, &'a str, u8) {
        match self {
            ISOCurrency::AED => ("AED", "UAE Dirham", 2),
            ISOCurrency::AFN => ("AFN", "Afghani", 2),
            ISOCurrency::ALL => ("ALL", "Lek", 2),
            ISOCurrency::AMD => ("AMD", "Armenian Dram", 2),
            ISOCurrency::ANG => ("ANG", "Netherlands Antillean Guilder", 2),
            ISOCurrency::AOA => ("AOA", "Kwanza", 2),
            ISOCurrency::ARS => ("ARS", "Argentine Peso", 2),
            ISOCurrency::AUD => ("AUD", "Australian Dollar", 2),
            ISOCurrency::AWG => ("AWG", "Aruban Florin", 2),
            ISOCurrency::AZN => ("AZN", "Azerbaijan Manat", 2),
            ISOCurrency::BAM => ("BAM", "Convertible Mark", 2),
            ISOCurrency::BBD => ("BBD", "Barbados Dollar", 2),
            ISOCurrency::BDT => ("BDT", "Taka", 2),
            ISOCurrency::BGN => ("BGN", "Bulgarian Lev", 2),
            ISOCurrency::BHD => ("BHD", "Bahraini Dinar", 3),
            ISOCurrency::BIF => ("BIF", "Burundi Franc", 0),
            ISOCurrency::BMD => ("BMD", "Bermudian Dollar", 2),
            ISOCurrency::BND => ("BND", "Brunei Dollar", 2),
            ISOCurrency::BOB => ("BOB", "Boliviano", 2),
            ISOCurrency::BOV => ("BOV", "Mvdol", 2),
            ISOCurrency::BRL => ("BRL", "Brazilian Real", 2),
            ISOCurrency::BSD => ("BSD", "Bahamian Dollar", 2),
            ISOCurrency::BTN => ("BTN", "Ngultrum", 2),
            ISOCurrency::BWP => ("BWP", "Pula", 2),
            ISOCurrency::BYN => ("BYN", "Belarusian Ruble", 2),
            ISOCurrency::BZD => ("BZD", "Belize Dollar", 2),
            ISOCurrency::CAD => ("CAD", "Canadian Dollar", 2),
            ISOCurrency::CDF => ("CDF", "Congolese Franc", 2),
            ISOCurrency::CHE => ("CHE", "WIR Euro", 2),
            ISOCurrency::CHF => ("CHF", "Swiss Franc", 2),
            ISOCurrency::CHW => ("CHW", "WIR Franc", 2),
            ISOCurrency::CLF => ("CLF", "Unidad de Fomento", 4),
            ISOCurrency::CLP => ("CLP", "Chilean Peso", 0),
            ISOCurrency::CNY => ("CNY", "Yuan Renminbi", 2),
            ISOCurrency::COP => ("COP", "Colombian Peso", 2),
            ISOCurrency::COU => ("COU", "Unidad de Valor Real", 2),
            ISOCurrency::CRC => ("CRC", "Costa Rican Colon", 2),
            ISOCurrency::CUC => ("CUC", "Peso Convertible", 2),
            ISOCurrency::CUP => ("CUP", "Cuban Peso", 2),
            ISOCurrency::CVE => ("CVE", "Cabo Verde Escudo", 2),
            ISOCurrency::CZK => ("CZK", "Czech Koruna", 2),
            ISOCurrency::DJF => ("DJF", "Djibouti Franc", 0),
            ISOCurrency::DKK => ("DKK", "Danish Krone", 2),
            ISOCurrency::DOP => ("DOP", "Dominican Peso", 2),
            ISOCurrency::DZD => ("DZD", "Algerian Dinar", 2),
            ISOCurrency::EGP => ("EGP", "Egyptian Pound", 2),
            ISOCurrency::ERN => ("ERN", "Nakfa", 2),
            ISOCurrency::ETB => ("ETB", "Ethiopian Birr", 2),
            ISOCurrency::EUR => ("EUR", "Euro", 2),
            ISOCurrency::FJD => ("FJD", "Fiji Dollar", 2),
            ISOCurrency::FKP => ("FKP", "Falkland Islands Pound", 2),
            ISOCurrency::GBP => ("GBP", "Pound Sterling", 2),
            ISOCurrency::GEL => ("GEL", "Lari", 2),
            ISOCurrency::GHS => ("GHS", "Ghana Cedi", 2),
            ISOCurrency::GIP => ("GIP", "Gibraltar Pound", 2),
            ISOCurrency::GMD => ("GMD", "Dalasi", 2),
            ISOCurrency::GNF => ("GNF", "Guinean Franc", 0),
            ISOCurrency::GTQ => ("GTQ", "Quetzal", 2),
            ISOCurrency::GYD => ("GYD", "Guyana Dollar", 2),
            ISOCurrency::HKD => ("HKD", "Hong Kong Dollar", 2),
            ISOCurrency::HNL => ("HNL", "Lempira", 2),
            ISOCurrency::HRK => ("HRK", "Kuna", 2),
            ISOCurrency::HTG => ("HTG", "Gourde", 2),
            ISOCurrency::HUF => ("HUF", "Forint", 2),
            ISOCurrency::IDR => ("IDR", "Rupiah", 2),
            ISOCurrency::ILS => ("ILS", "New Israeli Sheqel", 2),
            ISOCurrency::INR => ("INR", "Indian Rupee", 2),
            ISOCurrency::IQD => ("IQD", "Iraqi Dinar", 3),
            ISOCurrency::IRR => ("IRR", "Iranian Rial", 2),
            ISOCurrency::ISK => ("ISK", "Iceland Krona", 0),
            ISOCurrency::JMD => ("JMD", "Jamaican Dollar", 2),
            ISOCurrency::JOD => ("JOD", "Jordanian Dinar", 3),
            ISOCurrency::JPY => ("JPY", "Yen", 0),
            ISOCurrency::KES => ("KES", "Kenyan Shilling", 2),
            ISOCurrency::KGS => ("KGS", "Som", 2),
            ISOCurrency::KHR => ("KHR", "Riel", 2),
            ISOCurrency::KMF => ("KMF", "Comorian Franc ", 0),
            ISOCurrency::KPW => ("KPW", "North Korean Won", 2),
            ISOCurrency::KRW => ("KRW", "Won", 0),
            ISOCurrency::KWD => ("KWD", "Kuwaiti Dinar", 3),
            ISOCurrency::KYD => ("KYD", "Cayman Islands Dollar", 2),
            ISOCurrency::KZT => ("KZT", "Tenge", 2),
            ISOCurrency::LAK => ("LAK", "Lao Kip", 2),
            ISOCurrency::LBP => ("LBP", "Lebanese Pound", 2),
            ISOCurrency::LKR => ("LKR", "Sri Lanka Rupee", 2),
            ISOCurrency::LRD => ("LRD", "Liberian Dollar", 2),
            ISOCurrency::LSL => ("LSL", "Loti", 2),
            ISOCurrency::LYD => ("LYD", "Libyan Dinar", 3),
            ISOCurrency::MAD => ("MAD", "Moroccan Dirham", 2),
            ISOCurrency::MDL => ("MDL", "Moldovan Leu", 2),
            ISOCurrency::MGA => ("MGA", "Malagasy Ariary", 2),
            ISOCurrency::MKD => ("MKD", "Denar", 2),
            ISOCurrency::MMK => ("MMK", "Kyat", 2),
            ISOCurrency::MNT => ("MNT", "Tugrik", 2),
            ISOCurrency::MOP => ("MOP", "Pataca", 2),
            ISOCurrency::MRU => ("MRU", "Ouguiya", 2),
            ISOCurrency::MUR => ("MUR", "Mauritius Rupee", 2),
            ISOCurrency::MVR => ("MVR", "Rufiyaa", 2),
            ISOCurrency::MWK => ("MWK", "Malawi Kwacha", 2),
            ISOCurrency::MXN => ("MXN", "Mexican Peso", 2),
            ISOCurrency::MXV => {
                ("MXV", "Mexican Unidad de Inversion (UDI)", 2)
            }
            ISOCurrency::MYR => ("MYR", "Malaysian Ringgit", 2),
            ISOCurrency::MZN => ("MZN", "Mozambique Metical", 2),
            ISOCurrency::NAD => ("NAD", "Namibia Dollar", 2),
            ISOCurrency::NGN => ("NGN", "Naira", 2),
            ISOCurrency::NIO => ("NIO", "Cordoba Oro", 2),
            ISOCurrency::NOK => ("NOK", "Norwegian Krone", 2),
            ISOCurrency::NPR => ("NPR", "Nepalese Rupee", 2),
            ISOCurrency::NZD => ("NZD", "New Zealand Dollar", 2),
            ISOCurrency::OMR => ("OMR", "Rial Omani", 3),
            ISOCurrency::PAB => ("PAB", "Balboa", 2),
            ISOCurrency::PEN => ("PEN", "Sol", 2),
            ISOCurrency::PGK => ("PGK", "Kina", 2),
            ISOCurrency::PHP => ("PHP", "Philippine Peso", 2),
            ISOCurrency::PKR => ("PKR", "Pakistan Rupee", 2),
            ISOCurrency::PLN => ("PLN", "Zloty", 2),
            ISOCurrency::PYG => ("PYG", "Guarani", 0),
            ISOCurrency::QAR => ("QAR", "Qatari Rial", 2),
            ISOCurrency::RON => ("RON", "Romanian Leu", 2),
            ISOCurrency::RSD => ("RSD", "Serbian Dinar", 2),
            ISOCurrency::RUB => ("RUB", "Russian Ruble", 2),
            ISOCurrency::RWF => ("RWF", "Rwanda Franc", 0),
            ISOCurrency::SAR => ("SAR", "Saudi Riyal", 2),
            ISOCurrency::SBD => ("SBD", "Solomon Islands Dollar", 2),
            ISOCurrency::SCR => ("SCR", "Seychelles Rupee", 2),
            ISOCurrency::SDG => ("SDG", "Sudanese Pound", 2),
            ISOCurrency::SEK => ("SEK", "Swedish Krona", 2),
            ISOCurrency::SGD => ("SGD", "Singapore Dollar", 2),
            ISOCurrency::SHP => ("SHP", "Saint Helena Pound", 2),
            ISOCurrency::SLE => ("SLE", "Leone", 2),
            ISOCurrency::SLL => ("SLL", "Leone", 2),
            ISOCurrency::SOS => ("SOS", "Somali Shilling", 2),
            ISOCurrency::SRD => ("SRD", "Surinam Dollar", 2),
            ISOCurrency::SSP => ("SSP", "South Sudanese Pound", 2),
            ISOCurrency::STN => ("STN", "Dobra", 2),
            ISOCurrency::SVC => ("SVC", "El Salvador Colon", 2),
            ISOCurrency::SYP => ("SYP", "Syrian Pound", 2),
            ISOCurrency::SZL => ("SZL", "Lilangeni", 2),
            ISOCurrency::THB => ("THB", "Baht", 2),
            ISOCurrency::TJS => ("TJS", "Somoni", 2),
            ISOCurrency::TMT => ("TMT", "Turkmenistan New Manat", 2),
            ISOCurrency::TND => ("TND", "Tunisian Dinar", 3),
            ISOCurrency::TOP => ("TOP", "Pa’anga", 2),
            ISOCurrency::TRY => ("TRY", "Turkish Lira", 2),
            ISOCurrency::TTD => ("TTD", "Trinidad and Tobago Dollar", 2),
            ISOCurrency::TWD => ("TWD", "New Taiwan Dollar", 2),
            ISOCurrency::TZS => ("TZS", "Tanzanian Shilling", 2),
            ISOCurrency::UAH => ("UAH", "Hryvnia", 2),
            ISOCurrency::UGX => ("UGX", "Uganda Shilling", 0),
            ISOCurrency::USD => ("USD", "US Dollar", 2),
            ISOCurrency::USN => ("USN", "US Dollar (Next day)", 2),
            ISOCurrency::UYI => {
                ("UYI", "Uruguay Peso en Unidades Indexadas (UI)", 0)
            }
            ISOCurrency::UYU => ("UYU", "Peso Uruguayo", 2),
            ISOCurrency::UYW => ("UYW", "Unidad Previsional", 4),
            ISOCurrency::UZS => ("UZS", "Uzbekistan Sum", 2),
            ISOCurrency::VED => ("VED", "Bolívar Soberano", 2),
            ISOCurrency::VES => ("VES", "Bolívar Soberano", 2),
            ISOCurrency::VND => ("VND", "Dong", 0),
            ISOCurrency::VUV => ("VUV", "Vatu", 0),
            ISOCurrency::WST => ("WST", "Tala", 2),
            ISOCurrency::XAF => ("XAF", "CFA Franc BEAC", 0),
            ISOCurrency::XCD => ("XCD", "East Caribbean Dollar", 2),
            ISOCurrency::XOF => ("XOF", "CFA Franc BCEAO", 0),
            ISOCurrency::XPF => ("XPF", "CFP Franc", 0),
            ISOCurrency::YER => ("YER", "Yemeni Rial", 2),
            ISOCurrency::ZAR => ("ZAR", "Rand", 2),
            ISOCurrency::ZMW => ("ZMW", "Zambian Kwacha", 2),
            ISOCurrency::ZWL => ("ZWL", "Zimbabwe Dollar", 2),
        }
    }

    /// Returns the `CurrencyKey` of `self`.
    pub(crate) const fn key(&self) -> CurrencyKey {
        CurrencyKey::from_ascii(self.symbol())
    }

    /// Returns the symbol used to represent `self`.
    pub const fn symbol<'a>(&self) -> &'a str {
        self.info().0
    }

    /// Returns the name of `self`.
    pub const fn name<'a>(&self) -> &'a str {
        self.info().1
    }

    /// Returns the minor unit of `self`.
    pub const fn minor_unit(&self) -> u8 {
        self.info().2
    }

    /// Returns the `ISOCurrency` instance having the given key.
    pub(crate) fn from_key<'a>(key: CurrencyKey) -> Option<&'a Self> {
        Self::VARIANTS.iter().find(|x| x.key() == key)
    }

    /// Returns the `ISOCurrency` instance having the given symbol.
    pub(crate) fn from_symbol<'a>(sym: &str) -> Option<&'a Self> {
        Self::VARIANTS.iter().find(|x| x.symbol() == sym)
    }
}
