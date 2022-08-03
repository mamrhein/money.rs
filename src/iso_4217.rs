// ---------------------------------------------------------------------------
// Copyright:   (c) 2022 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use core::fmt;

use crate::{AmountT, Money, Mul, Quantity, SIPrefix, Unit};

#[doc = "Unit of quantity `Money`."]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Currency {
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
             Cyprus, Estonia, European Union, Finland, France, French Guiana, \
             French Southern Territories (The), Germany, Greece, Guadeloupe, \
             Holy See (The), Ireland, Italy, Latvia, Lithuania, Luxembourg, \
             Malta, Martinique, Mayotte, Monaco, Montenegro, Netherlands \
             (The), Portugal, Réunion, Saint Barthélemy, Saint Martin (French \
             Part), Saint Pierre And Miquelon, San Marino, Slovakia, \
             Slovenia, Spain"]
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
    #[doc = "Official currency in Croatia"]
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
    #[doc = "Official currency in Korea (The Democratic People’S Republic Of)"]
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
    #[doc = "Official currency in Saint Helena, Ascension And Tristan Da Cunha"]
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
    #[doc = "Official currency in Turkey"]
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
    #[doc = "Official currency in American Samoa, Bonaire, Sint Eustatius And \
             Saba, British Indian Ocean Territory (The), Ecuador, El \
             Salvador, Guam, Haiti, Marshall Islands (The), Micronesia \
             (Federated States Of), Northern Mariana Islands (The), Palau, \
             Panama, Puerto Rico, Timor-Leste, Turks And Caicos Islands \
             (The), United States Minor Outlying Islands (The), United States \
             Of America (The), Virgin Islands (British), Virgin Islands (U.S.)"]
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
    #[doc = "Official currency in French Polynesia, New Caledonia, Wallis And \
             Futuna"]
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

impl Currency {
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

    ///
    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub const fn minor_unit(&self) -> u8 {
        match self {
            Self::AED => 2,
            Self::AFN => 2,
            Self::ALL => 2,
            Self::AMD => 2,
            Self::ANG => 2,
            Self::AOA => 2,
            Self::ARS => 2,
            Self::AUD => 2,
            Self::AWG => 2,
            Self::AZN => 2,
            Self::BAM => 2,
            Self::BBD => 2,
            Self::BDT => 2,
            Self::BGN => 2,
            Self::BHD => 3,
            Self::BIF => 0,
            Self::BMD => 2,
            Self::BND => 2,
            Self::BOB => 2,
            Self::BOV => 2,
            Self::BRL => 2,
            Self::BSD => 2,
            Self::BTN => 2,
            Self::BWP => 2,
            Self::BYN => 2,
            Self::BZD => 2,
            Self::CAD => 2,
            Self::CDF => 2,
            Self::CHE => 2,
            Self::CHF => 2,
            Self::CHW => 2,
            Self::CLF => 4,
            Self::CLP => 0,
            Self::CNY => 2,
            Self::COP => 2,
            Self::COU => 2,
            Self::CRC => 2,
            Self::CUC => 2,
            Self::CUP => 2,
            Self::CVE => 2,
            Self::CZK => 2,
            Self::DJF => 0,
            Self::DKK => 2,
            Self::DOP => 2,
            Self::DZD => 2,
            Self::EGP => 2,
            Self::ERN => 2,
            Self::ETB => 2,
            Self::EUR => 2,
            Self::FJD => 2,
            Self::FKP => 2,
            Self::GBP => 2,
            Self::GEL => 2,
            Self::GHS => 2,
            Self::GIP => 2,
            Self::GMD => 2,
            Self::GNF => 0,
            Self::GTQ => 2,
            Self::GYD => 2,
            Self::HKD => 2,
            Self::HNL => 2,
            Self::HRK => 2,
            Self::HTG => 2,
            Self::HUF => 2,
            Self::IDR => 2,
            Self::ILS => 2,
            Self::INR => 2,
            Self::IQD => 3,
            Self::IRR => 2,
            Self::ISK => 0,
            Self::JMD => 2,
            Self::JOD => 3,
            Self::JPY => 0,
            Self::KES => 2,
            Self::KGS => 2,
            Self::KHR => 2,
            Self::KMF => 0,
            Self::KPW => 2,
            Self::KRW => 0,
            Self::KWD => 3,
            Self::KYD => 2,
            Self::KZT => 2,
            Self::LAK => 2,
            Self::LBP => 2,
            Self::LKR => 2,
            Self::LRD => 2,
            Self::LSL => 2,
            Self::LYD => 3,
            Self::MAD => 2,
            Self::MDL => 2,
            Self::MGA => 2,
            Self::MKD => 2,
            Self::MMK => 2,
            Self::MNT => 2,
            Self::MOP => 2,
            Self::MRU => 2,
            Self::MUR => 2,
            Self::MVR => 2,
            Self::MWK => 2,
            Self::MXN => 2,
            Self::MXV => 2,
            Self::MYR => 2,
            Self::MZN => 2,
            Self::NAD => 2,
            Self::NGN => 2,
            Self::NIO => 2,
            Self::NOK => 2,
            Self::NPR => 2,
            Self::NZD => 2,
            Self::OMR => 3,
            Self::PAB => 2,
            Self::PEN => 2,
            Self::PGK => 2,
            Self::PHP => 2,
            Self::PKR => 2,
            Self::PLN => 2,
            Self::PYG => 0,
            Self::QAR => 2,
            Self::RON => 2,
            Self::RSD => 2,
            Self::RUB => 2,
            Self::RWF => 0,
            Self::SAR => 2,
            Self::SBD => 2,
            Self::SCR => 2,
            Self::SDG => 2,
            Self::SEK => 2,
            Self::SGD => 2,
            Self::SHP => 2,
            Self::SLE => 2,
            Self::SLL => 2,
            Self::SOS => 2,
            Self::SRD => 2,
            Self::SSP => 2,
            Self::STN => 2,
            Self::SVC => 2,
            Self::SYP => 2,
            Self::SZL => 2,
            Self::THB => 2,
            Self::TJS => 2,
            Self::TMT => 2,
            Self::TND => 3,
            Self::TOP => 2,
            Self::TRY => 2,
            Self::TTD => 2,
            Self::TWD => 2,
            Self::TZS => 2,
            Self::UAH => 2,
            Self::UGX => 0,
            Self::USD => 2,
            Self::USN => 2,
            Self::UYI => 0,
            Self::UYU => 2,
            Self::UYW => 4,
            Self::UZS => 2,
            Self::VED => 2,
            Self::VES => 2,
            Self::VND => 0,
            Self::VUV => 0,
            Self::WST => 2,
            Self::XAF => 0,
            Self::XCD => 2,
            Self::XOF => 0,
            Self::XPF => 0,
            Self::YER => 2,
            Self::ZAR => 2,
            Self::ZMW => 2,
            Self::ZWL => 2,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Unit>::fmt(self, form)
    }
}

impl Unit for Currency {
    type QuantityType = Money;

    fn iter<'a>() -> core::slice::Iter<'a, Self> {
        Self::VARIANTS.iter()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::AED => "UAE Dirham",
            Self::AFN => "Afghani",
            Self::ALL => "Lek",
            Self::AMD => "Armenian Dram",
            Self::ANG => "Netherlands Antillean Guilder",
            Self::AOA => "Kwanza",
            Self::ARS => "Argentine Peso",
            Self::AUD => "Australian Dollar",
            Self::AWG => "Aruban Florin",
            Self::AZN => "Azerbaijan Manat",
            Self::BAM => "Convertible Mark",
            Self::BBD => "Barbados Dollar",
            Self::BDT => "Taka",
            Self::BGN => "Bulgarian Lev",
            Self::BHD => "Bahraini Dinar",
            Self::BIF => "Burundi Franc",
            Self::BMD => "Bermudian Dollar",
            Self::BND => "Brunei Dollar",
            Self::BOB => "Boliviano",
            Self::BOV => "Mvdol",
            Self::BRL => "Brazilian Real",
            Self::BSD => "Bahamian Dollar",
            Self::BTN => "Ngultrum",
            Self::BWP => "Pula",
            Self::BYN => "Belarusian Ruble",
            Self::BZD => "Belize Dollar",
            Self::CAD => "Canadian Dollar",
            Self::CDF => "Congolese Franc",
            Self::CHE => "WIR Euro",
            Self::CHF => "Swiss Franc",
            Self::CHW => "WIR Franc",
            Self::CLF => "Unidad de Fomento",
            Self::CLP => "Chilean Peso",
            Self::CNY => "Yuan Renminbi",
            Self::COP => "Colombian Peso",
            Self::COU => "Unidad de Valor Real",
            Self::CRC => "Costa Rican Colon",
            Self::CUC => "Peso Convertible",
            Self::CUP => "Cuban Peso",
            Self::CVE => "Cabo Verde Escudo",
            Self::CZK => "Czech Koruna",
            Self::DJF => "Djibouti Franc",
            Self::DKK => "Danish Krone",
            Self::DOP => "Dominican Peso",
            Self::DZD => "Algerian Dinar",
            Self::EGP => "Egyptian Pound",
            Self::ERN => "Nakfa",
            Self::ETB => "Ethiopian Birr",
            Self::EUR => "Euro",
            Self::FJD => "Fiji Dollar",
            Self::FKP => "Falkland Islands Pound",
            Self::GBP => "Pound Sterling",
            Self::GEL => "Lari",
            Self::GHS => "Ghana Cedi",
            Self::GIP => "Gibraltar Pound",
            Self::GMD => "Dalasi",
            Self::GNF => "Guinean Franc",
            Self::GTQ => "Quetzal",
            Self::GYD => "Guyana Dollar",
            Self::HKD => "Hong Kong Dollar",
            Self::HNL => "Lempira",
            Self::HRK => "Kuna",
            Self::HTG => "Gourde",
            Self::HUF => "Forint",
            Self::IDR => "Rupiah",
            Self::ILS => "New Israeli Sheqel",
            Self::INR => "Indian Rupee",
            Self::IQD => "Iraqi Dinar",
            Self::IRR => "Iranian Rial",
            Self::ISK => "Iceland Krona",
            Self::JMD => "Jamaican Dollar",
            Self::JOD => "Jordanian Dinar",
            Self::JPY => "Yen",
            Self::KES => "Kenyan Shilling",
            Self::KGS => "Som",
            Self::KHR => "Riel",
            Self::KMF => "Comorian Franc ",
            Self::KPW => "North Korean Won",
            Self::KRW => "Won",
            Self::KWD => "Kuwaiti Dinar",
            Self::KYD => "Cayman Islands Dollar",
            Self::KZT => "Tenge",
            Self::LAK => "Lao Kip",
            Self::LBP => "Lebanese Pound",
            Self::LKR => "Sri Lanka Rupee",
            Self::LRD => "Liberian Dollar",
            Self::LSL => "Loti",
            Self::LYD => "Libyan Dinar",
            Self::MAD => "Moroccan Dirham",
            Self::MDL => "Moldovan Leu",
            Self::MGA => "Malagasy Ariary",
            Self::MKD => "Denar",
            Self::MMK => "Kyat",
            Self::MNT => "Tugrik",
            Self::MOP => "Pataca",
            Self::MRU => "Ouguiya",
            Self::MUR => "Mauritius Rupee",
            Self::MVR => "Rufiyaa",
            Self::MWK => "Malawi Kwacha",
            Self::MXN => "Mexican Peso",
            Self::MXV => "Mexican Unidad de Inversion (UDI)",
            Self::MYR => "Malaysian Ringgit",
            Self::MZN => "Mozambique Metical",
            Self::NAD => "Namibia Dollar",
            Self::NGN => "Naira",
            Self::NIO => "Cordoba Oro",
            Self::NOK => "Norwegian Krone",
            Self::NPR => "Nepalese Rupee",
            Self::NZD => "New Zealand Dollar",
            Self::OMR => "Rial Omani",
            Self::PAB => "Balboa",
            Self::PEN => "Sol",
            Self::PGK => "Kina",
            Self::PHP => "Philippine Peso",
            Self::PKR => "Pakistan Rupee",
            Self::PLN => "Zloty",
            Self::PYG => "Guarani",
            Self::QAR => "Qatari Rial",
            Self::RON => "Romanian Leu",
            Self::RSD => "Serbian Dinar",
            Self::RUB => "Russian Ruble",
            Self::RWF => "Rwanda Franc",
            Self::SAR => "Saudi Riyal",
            Self::SBD => "Solomon Islands Dollar",
            Self::SCR => "Seychelles Rupee",
            Self::SDG => "Sudanese Pound",
            Self::SEK => "Swedish Krona",
            Self::SGD => "Singapore Dollar",
            Self::SHP => "Saint Helena Pound",
            Self::SLE | Self::SLL => "Leone",
            Self::SOS => "Somali Shilling",
            Self::SRD => "Surinam Dollar",
            Self::SSP => "South Sudanese Pound",
            Self::STN => "Dobra",
            Self::SVC => "El Salvador Colon",
            Self::SYP => "Syrian Pound",
            Self::SZL => "Lilangeni",
            Self::THB => "Baht",
            Self::TJS => "Somoni",
            Self::TMT => "Turkmenistan New Manat",
            Self::TND => "Tunisian Dinar",
            Self::TOP => "Pa’anga",
            Self::TRY => "Turkish Lira",
            Self::TTD => "Trinidad and Tobago Dollar",
            Self::TWD => "New Taiwan Dollar",
            Self::TZS => "Tanzanian Shilling",
            Self::UAH => "Hryvnia",
            Self::UGX => "Uganda Shilling",
            Self::USD => "US Dollar",
            Self::USN => "US Dollar (Next day)",
            Self::UYI => "Uruguay Peso en Unidades Indexadas (UI)",
            Self::UYU => "Peso Uruguayo",
            Self::UYW => "Unidad Previsional",
            Self::UZS => "Uzbekistan Sum",
            Self::VED | Self::VES => "Bolívar Soberano",
            Self::VND => "Dong",
            Self::VUV => "Vatu",
            Self::WST => "Tala",
            Self::XAF => "CFA Franc BEAC",
            Self::XCD => "East Caribbean Dollar",
            Self::XOF => "CFA Franc BCEAO",
            Self::XPF => "CFP Franc",
            Self::YER => "Yemeni Rial",
            Self::ZAR => "Rand",
            Self::ZMW => "Zambian Kwacha",
            Self::ZWL => "Zimbabwe Dollar",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            Self::AED => "AED",
            Self::AFN => "AFN",
            Self::ALL => "ALL",
            Self::AMD => "AMD",
            Self::ANG => "ANG",
            Self::AOA => "AOA",
            Self::ARS => "ARS",
            Self::AUD => "AUD",
            Self::AWG => "AWG",
            Self::AZN => "AZN",
            Self::BAM => "BAM",
            Self::BBD => "BBD",
            Self::BDT => "BDT",
            Self::BGN => "BGN",
            Self::BHD => "BHD",
            Self::BIF => "BIF",
            Self::BMD => "BMD",
            Self::BND => "BND",
            Self::BOB => "BOB",
            Self::BOV => "BOV",
            Self::BRL => "BRL",
            Self::BSD => "BSD",
            Self::BTN => "BTN",
            Self::BWP => "BWP",
            Self::BYN => "BYN",
            Self::BZD => "BZD",
            Self::CAD => "CAD",
            Self::CDF => "CDF",
            Self::CHE => "CHE",
            Self::CHF => "CHF",
            Self::CHW => "CHW",
            Self::CLF => "CLF",
            Self::CLP => "CLP",
            Self::CNY => "CNY",
            Self::COP => "COP",
            Self::COU => "COU",
            Self::CRC => "CRC",
            Self::CUC => "CUC",
            Self::CUP => "CUP",
            Self::CVE => "CVE",
            Self::CZK => "CZK",
            Self::DJF => "DJF",
            Self::DKK => "DKK",
            Self::DOP => "DOP",
            Self::DZD => "DZD",
            Self::EGP => "EGP",
            Self::ERN => "ERN",
            Self::ETB => "ETB",
            Self::EUR => "EUR",
            Self::FJD => "FJD",
            Self::FKP => "FKP",
            Self::GBP => "GBP",
            Self::GEL => "GEL",
            Self::GHS => "GHS",
            Self::GIP => "GIP",
            Self::GMD => "GMD",
            Self::GNF => "GNF",
            Self::GTQ => "GTQ",
            Self::GYD => "GYD",
            Self::HKD => "HKD",
            Self::HNL => "HNL",
            Self::HRK => "HRK",
            Self::HTG => "HTG",
            Self::HUF => "HUF",
            Self::IDR => "IDR",
            Self::ILS => "ILS",
            Self::INR => "INR",
            Self::IQD => "IQD",
            Self::IRR => "IRR",
            Self::ISK => "ISK",
            Self::JMD => "JMD",
            Self::JOD => "JOD",
            Self::JPY => "JPY",
            Self::KES => "KES",
            Self::KGS => "KGS",
            Self::KHR => "KHR",
            Self::KMF => "KMF",
            Self::KPW => "KPW",
            Self::KRW => "KRW",
            Self::KWD => "KWD",
            Self::KYD => "KYD",
            Self::KZT => "KZT",
            Self::LAK => "LAK",
            Self::LBP => "LBP",
            Self::LKR => "LKR",
            Self::LRD => "LRD",
            Self::LSL => "LSL",
            Self::LYD => "LYD",
            Self::MAD => "MAD",
            Self::MDL => "MDL",
            Self::MGA => "MGA",
            Self::MKD => "MKD",
            Self::MMK => "MMK",
            Self::MNT => "MNT",
            Self::MOP => "MOP",
            Self::MRU => "MRU",
            Self::MUR => "MUR",
            Self::MVR => "MVR",
            Self::MWK => "MWK",
            Self::MXN => "MXN",
            Self::MXV => "MXV",
            Self::MYR => "MYR",
            Self::MZN => "MZN",
            Self::NAD => "NAD",
            Self::NGN => "NGN",
            Self::NIO => "NIO",
            Self::NOK => "NOK",
            Self::NPR => "NPR",
            Self::NZD => "NZD",
            Self::OMR => "OMR",
            Self::PAB => "PAB",
            Self::PEN => "PEN",
            Self::PGK => "PGK",
            Self::PHP => "PHP",
            Self::PKR => "PKR",
            Self::PLN => "PLN",
            Self::PYG => "PYG",
            Self::QAR => "QAR",
            Self::RON => "RON",
            Self::RSD => "RSD",
            Self::RUB => "RUB",
            Self::RWF => "RWF",
            Self::SAR => "SAR",
            Self::SBD => "SBD",
            Self::SCR => "SCR",
            Self::SDG => "SDG",
            Self::SEK => "SEK",
            Self::SGD => "SGD",
            Self::SHP => "SHP",
            Self::SLE => "SLE",
            Self::SLL => "SLL",
            Self::SOS => "SOS",
            Self::SRD => "SRD",
            Self::SSP => "SSP",
            Self::STN => "STN",
            Self::SVC => "SVC",
            Self::SYP => "SYP",
            Self::SZL => "SZL",
            Self::THB => "THB",
            Self::TJS => "TJS",
            Self::TMT => "TMT",
            Self::TND => "TND",
            Self::TOP => "TOP",
            Self::TRY => "TRY",
            Self::TTD => "TTD",
            Self::TWD => "TWD",
            Self::TZS => "TZS",
            Self::UAH => "UAH",
            Self::UGX => "UGX",
            Self::USD => "USD",
            Self::USN => "USN",
            Self::UYI => "UYI",
            Self::UYU => "UYU",
            Self::UYW => "UYW",
            Self::UZS => "UZS",
            Self::VED => "VED",
            Self::VES => "VES",
            Self::VND => "VND",
            Self::VUV => "VUV",
            Self::WST => "WST",
            Self::XAF => "XAF",
            Self::XCD => "XCD",
            Self::XOF => "XOF",
            Self::XPF => "XPF",
            Self::YER => "YER",
            Self::ZAR => "ZAR",
            Self::ZMW => "ZMW",
            Self::ZWL => "ZWL",
        }
    }

    fn si_prefix(&self) -> Option<SIPrefix> {
        None
    }
}

impl Mul<Currency> for AmountT {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: Currency) -> Self::Output {
        Money::new(self, rhs)
    }
}

impl Mul<AmountT> for Currency {
    type Output = Money;

    #[inline(always)]
    fn mul(self, rhs: AmountT) -> Self::Output {
        Money::new(rhs, self)
    }
}
