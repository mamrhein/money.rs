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
    #[doc = "Official currency in Falkland Islands (The) [Malvinas]"]
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
    const VARIANTS: [Currency; 168_usize] = [
        Currency::AED,
        Currency::AFN,
        Currency::ALL,
        Currency::AMD,
        Currency::ANG,
        Currency::AOA,
        Currency::ARS,
        Currency::AUD,
        Currency::AWG,
        Currency::AZN,
        Currency::BAM,
        Currency::BBD,
        Currency::BDT,
        Currency::BGN,
        Currency::BHD,
        Currency::BIF,
        Currency::BMD,
        Currency::BND,
        Currency::BOB,
        Currency::BOV,
        Currency::BRL,
        Currency::BSD,
        Currency::BTN,
        Currency::BWP,
        Currency::BYN,
        Currency::BZD,
        Currency::CAD,
        Currency::CDF,
        Currency::CHE,
        Currency::CHF,
        Currency::CHW,
        Currency::CLF,
        Currency::CLP,
        Currency::CNY,
        Currency::COP,
        Currency::COU,
        Currency::CRC,
        Currency::CUC,
        Currency::CUP,
        Currency::CVE,
        Currency::CZK,
        Currency::DJF,
        Currency::DKK,
        Currency::DOP,
        Currency::DZD,
        Currency::EGP,
        Currency::ERN,
        Currency::ETB,
        Currency::EUR,
        Currency::FJD,
        Currency::FKP,
        Currency::GBP,
        Currency::GEL,
        Currency::GHS,
        Currency::GIP,
        Currency::GMD,
        Currency::GNF,
        Currency::GTQ,
        Currency::GYD,
        Currency::HKD,
        Currency::HNL,
        Currency::HRK,
        Currency::HTG,
        Currency::HUF,
        Currency::IDR,
        Currency::ILS,
        Currency::INR,
        Currency::IQD,
        Currency::IRR,
        Currency::ISK,
        Currency::JMD,
        Currency::JOD,
        Currency::JPY,
        Currency::KES,
        Currency::KGS,
        Currency::KHR,
        Currency::KMF,
        Currency::KPW,
        Currency::KRW,
        Currency::KWD,
        Currency::KYD,
        Currency::KZT,
        Currency::LAK,
        Currency::LBP,
        Currency::LKR,
        Currency::LRD,
        Currency::LSL,
        Currency::LYD,
        Currency::MAD,
        Currency::MDL,
        Currency::MGA,
        Currency::MKD,
        Currency::MMK,
        Currency::MNT,
        Currency::MOP,
        Currency::MRU,
        Currency::MUR,
        Currency::MVR,
        Currency::MWK,
        Currency::MXN,
        Currency::MXV,
        Currency::MYR,
        Currency::MZN,
        Currency::NAD,
        Currency::NGN,
        Currency::NIO,
        Currency::NOK,
        Currency::NPR,
        Currency::NZD,
        Currency::OMR,
        Currency::PAB,
        Currency::PEN,
        Currency::PGK,
        Currency::PHP,
        Currency::PKR,
        Currency::PLN,
        Currency::PYG,
        Currency::QAR,
        Currency::RON,
        Currency::RSD,
        Currency::RUB,
        Currency::RWF,
        Currency::SAR,
        Currency::SBD,
        Currency::SCR,
        Currency::SDG,
        Currency::SEK,
        Currency::SGD,
        Currency::SHP,
        Currency::SLE,
        Currency::SLL,
        Currency::SOS,
        Currency::SRD,
        Currency::SSP,
        Currency::STN,
        Currency::SVC,
        Currency::SYP,
        Currency::SZL,
        Currency::THB,
        Currency::TJS,
        Currency::TMT,
        Currency::TND,
        Currency::TOP,
        Currency::TRY,
        Currency::TTD,
        Currency::TWD,
        Currency::TZS,
        Currency::UAH,
        Currency::UGX,
        Currency::USD,
        Currency::USN,
        Currency::UYI,
        Currency::UYU,
        Currency::UYW,
        Currency::UZS,
        Currency::VED,
        Currency::VES,
        Currency::VND,
        Currency::VUV,
        Currency::WST,
        Currency::XAF,
        Currency::XCD,
        Currency::XOF,
        Currency::XPF,
        Currency::YER,
        Currency::ZAR,
        Currency::ZMW,
        Currency::ZWL,
    ];

    pub fn minor_unit(&self) -> u8 {
        match self {
            Currency::AED => 2,
            Currency::AFN => 2,
            Currency::ALL => 2,
            Currency::AMD => 2,
            Currency::ANG => 2,
            Currency::AOA => 2,
            Currency::ARS => 2,
            Currency::AUD => 2,
            Currency::AWG => 2,
            Currency::AZN => 2,
            Currency::BAM => 2,
            Currency::BBD => 2,
            Currency::BDT => 2,
            Currency::BGN => 2,
            Currency::BHD => 3,
            Currency::BIF => 0,
            Currency::BMD => 2,
            Currency::BND => 2,
            Currency::BOB => 2,
            Currency::BOV => 2,
            Currency::BRL => 2,
            Currency::BSD => 2,
            Currency::BTN => 2,
            Currency::BWP => 2,
            Currency::BYN => 2,
            Currency::BZD => 2,
            Currency::CAD => 2,
            Currency::CDF => 2,
            Currency::CHE => 2,
            Currency::CHF => 2,
            Currency::CHW => 2,
            Currency::CLF => 4,
            Currency::CLP => 0,
            Currency::CNY => 2,
            Currency::COP => 2,
            Currency::COU => 2,
            Currency::CRC => 2,
            Currency::CUC => 2,
            Currency::CUP => 2,
            Currency::CVE => 2,
            Currency::CZK => 2,
            Currency::DJF => 0,
            Currency::DKK => 2,
            Currency::DOP => 2,
            Currency::DZD => 2,
            Currency::EGP => 2,
            Currency::ERN => 2,
            Currency::ETB => 2,
            Currency::EUR => 2,
            Currency::FJD => 2,
            Currency::FKP => 2,
            Currency::GBP => 2,
            Currency::GEL => 2,
            Currency::GHS => 2,
            Currency::GIP => 2,
            Currency::GMD => 2,
            Currency::GNF => 0,
            Currency::GTQ => 2,
            Currency::GYD => 2,
            Currency::HKD => 2,
            Currency::HNL => 2,
            Currency::HRK => 2,
            Currency::HTG => 2,
            Currency::HUF => 2,
            Currency::IDR => 2,
            Currency::ILS => 2,
            Currency::INR => 2,
            Currency::IQD => 3,
            Currency::IRR => 2,
            Currency::ISK => 0,
            Currency::JMD => 2,
            Currency::JOD => 3,
            Currency::JPY => 0,
            Currency::KES => 2,
            Currency::KGS => 2,
            Currency::KHR => 2,
            Currency::KMF => 0,
            Currency::KPW => 2,
            Currency::KRW => 0,
            Currency::KWD => 3,
            Currency::KYD => 2,
            Currency::KZT => 2,
            Currency::LAK => 2,
            Currency::LBP => 2,
            Currency::LKR => 2,
            Currency::LRD => 2,
            Currency::LSL => 2,
            Currency::LYD => 3,
            Currency::MAD => 2,
            Currency::MDL => 2,
            Currency::MGA => 2,
            Currency::MKD => 2,
            Currency::MMK => 2,
            Currency::MNT => 2,
            Currency::MOP => 2,
            Currency::MRU => 2,
            Currency::MUR => 2,
            Currency::MVR => 2,
            Currency::MWK => 2,
            Currency::MXN => 2,
            Currency::MXV => 2,
            Currency::MYR => 2,
            Currency::MZN => 2,
            Currency::NAD => 2,
            Currency::NGN => 2,
            Currency::NIO => 2,
            Currency::NOK => 2,
            Currency::NPR => 2,
            Currency::NZD => 2,
            Currency::OMR => 3,
            Currency::PAB => 2,
            Currency::PEN => 2,
            Currency::PGK => 2,
            Currency::PHP => 2,
            Currency::PKR => 2,
            Currency::PLN => 2,
            Currency::PYG => 0,
            Currency::QAR => 2,
            Currency::RON => 2,
            Currency::RSD => 2,
            Currency::RUB => 2,
            Currency::RWF => 0,
            Currency::SAR => 2,
            Currency::SBD => 2,
            Currency::SCR => 2,
            Currency::SDG => 2,
            Currency::SEK => 2,
            Currency::SGD => 2,
            Currency::SHP => 2,
            Currency::SLE => 2,
            Currency::SLL => 2,
            Currency::SOS => 2,
            Currency::SRD => 2,
            Currency::SSP => 2,
            Currency::STN => 2,
            Currency::SVC => 2,
            Currency::SYP => 2,
            Currency::SZL => 2,
            Currency::THB => 2,
            Currency::TJS => 2,
            Currency::TMT => 2,
            Currency::TND => 3,
            Currency::TOP => 2,
            Currency::TRY => 2,
            Currency::TTD => 2,
            Currency::TWD => 2,
            Currency::TZS => 2,
            Currency::UAH => 2,
            Currency::UGX => 0,
            Currency::USD => 2,
            Currency::USN => 2,
            Currency::UYI => 0,
            Currency::UYU => 2,
            Currency::UYW => 4,
            Currency::UZS => 2,
            Currency::VED => 2,
            Currency::VES => 2,
            Currency::VND => 0,
            Currency::VUV => 0,
            Currency::WST => 2,
            Currency::XAF => 0,
            Currency::XCD => 2,
            Currency::XOF => 0,
            Currency::XPF => 0,
            Currency::YER => 2,
            Currency::ZAR => 2,
            Currency::ZMW => 2,
            Currency::ZWL => 2,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, form: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Unit>::fmt(&self, form)
    }
}

impl Unit for Currency {
    type QuantityType = Money;

    fn iter<'a>() -> core::slice::Iter<'a, Self> {
        Self::VARIANTS.iter()
    }

    fn name(&self) -> &'static str {
        match self {
            Currency::AED => "UAE Dirham",
            Currency::AFN => "Afghani",
            Currency::ALL => "Lek",
            Currency::AMD => "Armenian Dram",
            Currency::ANG => "Netherlands Antillean Guilder",
            Currency::AOA => "Kwanza",
            Currency::ARS => "Argentine Peso",
            Currency::AUD => "Australian Dollar",
            Currency::AWG => "Aruban Florin",
            Currency::AZN => "Azerbaijan Manat",
            Currency::BAM => "Convertible Mark",
            Currency::BBD => "Barbados Dollar",
            Currency::BDT => "Taka",
            Currency::BGN => "Bulgarian Lev",
            Currency::BHD => "Bahraini Dinar",
            Currency::BIF => "Burundi Franc",
            Currency::BMD => "Bermudian Dollar",
            Currency::BND => "Brunei Dollar",
            Currency::BOB => "Boliviano",
            Currency::BOV => "Mvdol",
            Currency::BRL => "Brazilian Real",
            Currency::BSD => "Bahamian Dollar",
            Currency::BTN => "Ngultrum",
            Currency::BWP => "Pula",
            Currency::BYN => "Belarusian Ruble",
            Currency::BZD => "Belize Dollar",
            Currency::CAD => "Canadian Dollar",
            Currency::CDF => "Congolese Franc",
            Currency::CHE => "WIR Euro",
            Currency::CHF => "Swiss Franc",
            Currency::CHW => "WIR Franc",
            Currency::CLF => "Unidad de Fomento",
            Currency::CLP => "Chilean Peso",
            Currency::CNY => "Yuan Renminbi",
            Currency::COP => "Colombian Peso",
            Currency::COU => "Unidad de Valor Real",
            Currency::CRC => "Costa Rican Colon",
            Currency::CUC => "Peso Convertible",
            Currency::CUP => "Cuban Peso",
            Currency::CVE => "Cabo Verde Escudo",
            Currency::CZK => "Czech Koruna",
            Currency::DJF => "Djibouti Franc",
            Currency::DKK => "Danish Krone",
            Currency::DOP => "Dominican Peso",
            Currency::DZD => "Algerian Dinar",
            Currency::EGP => "Egyptian Pound",
            Currency::ERN => "Nakfa",
            Currency::ETB => "Ethiopian Birr",
            Currency::EUR => "Euro",
            Currency::FJD => "Fiji Dollar",
            Currency::FKP => "Falkland Islands Pound",
            Currency::GBP => "Pound Sterling",
            Currency::GEL => "Lari",
            Currency::GHS => "Ghana Cedi",
            Currency::GIP => "Gibraltar Pound",
            Currency::GMD => "Dalasi",
            Currency::GNF => "Guinean Franc",
            Currency::GTQ => "Quetzal",
            Currency::GYD => "Guyana Dollar",
            Currency::HKD => "Hong Kong Dollar",
            Currency::HNL => "Lempira",
            Currency::HRK => "Kuna",
            Currency::HTG => "Gourde",
            Currency::HUF => "Forint",
            Currency::IDR => "Rupiah",
            Currency::ILS => "New Israeli Sheqel",
            Currency::INR => "Indian Rupee",
            Currency::IQD => "Iraqi Dinar",
            Currency::IRR => "Iranian Rial",
            Currency::ISK => "Iceland Krona",
            Currency::JMD => "Jamaican Dollar",
            Currency::JOD => "Jordanian Dinar",
            Currency::JPY => "Yen",
            Currency::KES => "Kenyan Shilling",
            Currency::KGS => "Som",
            Currency::KHR => "Riel",
            Currency::KMF => "Comorian Franc ",
            Currency::KPW => "North Korean Won",
            Currency::KRW => "Won",
            Currency::KWD => "Kuwaiti Dinar",
            Currency::KYD => "Cayman Islands Dollar",
            Currency::KZT => "Tenge",
            Currency::LAK => "Lao Kip",
            Currency::LBP => "Lebanese Pound",
            Currency::LKR => "Sri Lanka Rupee",
            Currency::LRD => "Liberian Dollar",
            Currency::LSL => "Loti",
            Currency::LYD => "Libyan Dinar",
            Currency::MAD => "Moroccan Dirham",
            Currency::MDL => "Moldovan Leu",
            Currency::MGA => "Malagasy Ariary",
            Currency::MKD => "Denar",
            Currency::MMK => "Kyat",
            Currency::MNT => "Tugrik",
            Currency::MOP => "Pataca",
            Currency::MRU => "Ouguiya",
            Currency::MUR => "Mauritius Rupee",
            Currency::MVR => "Rufiyaa",
            Currency::MWK => "Malawi Kwacha",
            Currency::MXN => "Mexican Peso",
            Currency::MXV => "Mexican Unidad de Inversion (UDI)",
            Currency::MYR => "Malaysian Ringgit",
            Currency::MZN => "Mozambique Metical",
            Currency::NAD => "Namibia Dollar",
            Currency::NGN => "Naira",
            Currency::NIO => "Cordoba Oro",
            Currency::NOK => "Norwegian Krone",
            Currency::NPR => "Nepalese Rupee",
            Currency::NZD => "New Zealand Dollar",
            Currency::OMR => "Rial Omani",
            Currency::PAB => "Balboa",
            Currency::PEN => "Sol",
            Currency::PGK => "Kina",
            Currency::PHP => "Philippine Peso",
            Currency::PKR => "Pakistan Rupee",
            Currency::PLN => "Zloty",
            Currency::PYG => "Guarani",
            Currency::QAR => "Qatari Rial",
            Currency::RON => "Romanian Leu",
            Currency::RSD => "Serbian Dinar",
            Currency::RUB => "Russian Ruble",
            Currency::RWF => "Rwanda Franc",
            Currency::SAR => "Saudi Riyal",
            Currency::SBD => "Solomon Islands Dollar",
            Currency::SCR => "Seychelles Rupee",
            Currency::SDG => "Sudanese Pound",
            Currency::SEK => "Swedish Krona",
            Currency::SGD => "Singapore Dollar",
            Currency::SHP => "Saint Helena Pound",
            Currency::SLE => "Leone",
            Currency::SLL => "Leone",
            Currency::SOS => "Somali Shilling",
            Currency::SRD => "Surinam Dollar",
            Currency::SSP => "South Sudanese Pound",
            Currency::STN => "Dobra",
            Currency::SVC => "El Salvador Colon",
            Currency::SYP => "Syrian Pound",
            Currency::SZL => "Lilangeni",
            Currency::THB => "Baht",
            Currency::TJS => "Somoni",
            Currency::TMT => "Turkmenistan New Manat",
            Currency::TND => "Tunisian Dinar",
            Currency::TOP => "Pa’anga",
            Currency::TRY => "Turkish Lira",
            Currency::TTD => "Trinidad and Tobago Dollar",
            Currency::TWD => "New Taiwan Dollar",
            Currency::TZS => "Tanzanian Shilling",
            Currency::UAH => "Hryvnia",
            Currency::UGX => "Uganda Shilling",
            Currency::USD => "US Dollar",
            Currency::USN => "US Dollar (Next day)",
            Currency::UYI => "Uruguay Peso en Unidades Indexadas (UI)",
            Currency::UYU => "Peso Uruguayo",
            Currency::UYW => "Unidad Previsional",
            Currency::UZS => "Uzbekistan Sum",
            Currency::VED => "Bolívar Soberano",
            Currency::VES => "Bolívar Soberano",
            Currency::VND => "Dong",
            Currency::VUV => "Vatu",
            Currency::WST => "Tala",
            Currency::XAF => "CFA Franc BEAC",
            Currency::XCD => "East Caribbean Dollar",
            Currency::XOF => "CFA Franc BCEAO",
            Currency::XPF => "CFP Franc",
            Currency::YER => "Yemeni Rial",
            Currency::ZAR => "Rand",
            Currency::ZMW => "Zambian Kwacha",
            Currency::ZWL => "Zimbabwe Dollar",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            Currency::AED => "AED",
            Currency::AFN => "AFN",
            Currency::ALL => "ALL",
            Currency::AMD => "AMD",
            Currency::ANG => "ANG",
            Currency::AOA => "AOA",
            Currency::ARS => "ARS",
            Currency::AUD => "AUD",
            Currency::AWG => "AWG",
            Currency::AZN => "AZN",
            Currency::BAM => "BAM",
            Currency::BBD => "BBD",
            Currency::BDT => "BDT",
            Currency::BGN => "BGN",
            Currency::BHD => "BHD",
            Currency::BIF => "BIF",
            Currency::BMD => "BMD",
            Currency::BND => "BND",
            Currency::BOB => "BOB",
            Currency::BOV => "BOV",
            Currency::BRL => "BRL",
            Currency::BSD => "BSD",
            Currency::BTN => "BTN",
            Currency::BWP => "BWP",
            Currency::BYN => "BYN",
            Currency::BZD => "BZD",
            Currency::CAD => "CAD",
            Currency::CDF => "CDF",
            Currency::CHE => "CHE",
            Currency::CHF => "CHF",
            Currency::CHW => "CHW",
            Currency::CLF => "CLF",
            Currency::CLP => "CLP",
            Currency::CNY => "CNY",
            Currency::COP => "COP",
            Currency::COU => "COU",
            Currency::CRC => "CRC",
            Currency::CUC => "CUC",
            Currency::CUP => "CUP",
            Currency::CVE => "CVE",
            Currency::CZK => "CZK",
            Currency::DJF => "DJF",
            Currency::DKK => "DKK",
            Currency::DOP => "DOP",
            Currency::DZD => "DZD",
            Currency::EGP => "EGP",
            Currency::ERN => "ERN",
            Currency::ETB => "ETB",
            Currency::EUR => "EUR",
            Currency::FJD => "FJD",
            Currency::FKP => "FKP",
            Currency::GBP => "GBP",
            Currency::GEL => "GEL",
            Currency::GHS => "GHS",
            Currency::GIP => "GIP",
            Currency::GMD => "GMD",
            Currency::GNF => "GNF",
            Currency::GTQ => "GTQ",
            Currency::GYD => "GYD",
            Currency::HKD => "HKD",
            Currency::HNL => "HNL",
            Currency::HRK => "HRK",
            Currency::HTG => "HTG",
            Currency::HUF => "HUF",
            Currency::IDR => "IDR",
            Currency::ILS => "ILS",
            Currency::INR => "INR",
            Currency::IQD => "IQD",
            Currency::IRR => "IRR",
            Currency::ISK => "ISK",
            Currency::JMD => "JMD",
            Currency::JOD => "JOD",
            Currency::JPY => "JPY",
            Currency::KES => "KES",
            Currency::KGS => "KGS",
            Currency::KHR => "KHR",
            Currency::KMF => "KMF",
            Currency::KPW => "KPW",
            Currency::KRW => "KRW",
            Currency::KWD => "KWD",
            Currency::KYD => "KYD",
            Currency::KZT => "KZT",
            Currency::LAK => "LAK",
            Currency::LBP => "LBP",
            Currency::LKR => "LKR",
            Currency::LRD => "LRD",
            Currency::LSL => "LSL",
            Currency::LYD => "LYD",
            Currency::MAD => "MAD",
            Currency::MDL => "MDL",
            Currency::MGA => "MGA",
            Currency::MKD => "MKD",
            Currency::MMK => "MMK",
            Currency::MNT => "MNT",
            Currency::MOP => "MOP",
            Currency::MRU => "MRU",
            Currency::MUR => "MUR",
            Currency::MVR => "MVR",
            Currency::MWK => "MWK",
            Currency::MXN => "MXN",
            Currency::MXV => "MXV",
            Currency::MYR => "MYR",
            Currency::MZN => "MZN",
            Currency::NAD => "NAD",
            Currency::NGN => "NGN",
            Currency::NIO => "NIO",
            Currency::NOK => "NOK",
            Currency::NPR => "NPR",
            Currency::NZD => "NZD",
            Currency::OMR => "OMR",
            Currency::PAB => "PAB",
            Currency::PEN => "PEN",
            Currency::PGK => "PGK",
            Currency::PHP => "PHP",
            Currency::PKR => "PKR",
            Currency::PLN => "PLN",
            Currency::PYG => "PYG",
            Currency::QAR => "QAR",
            Currency::RON => "RON",
            Currency::RSD => "RSD",
            Currency::RUB => "RUB",
            Currency::RWF => "RWF",
            Currency::SAR => "SAR",
            Currency::SBD => "SBD",
            Currency::SCR => "SCR",
            Currency::SDG => "SDG",
            Currency::SEK => "SEK",
            Currency::SGD => "SGD",
            Currency::SHP => "SHP",
            Currency::SLE => "SLE",
            Currency::SLL => "SLL",
            Currency::SOS => "SOS",
            Currency::SRD => "SRD",
            Currency::SSP => "SSP",
            Currency::STN => "STN",
            Currency::SVC => "SVC",
            Currency::SYP => "SYP",
            Currency::SZL => "SZL",
            Currency::THB => "THB",
            Currency::TJS => "TJS",
            Currency::TMT => "TMT",
            Currency::TND => "TND",
            Currency::TOP => "TOP",
            Currency::TRY => "TRY",
            Currency::TTD => "TTD",
            Currency::TWD => "TWD",
            Currency::TZS => "TZS",
            Currency::UAH => "UAH",
            Currency::UGX => "UGX",
            Currency::USD => "USD",
            Currency::USN => "USN",
            Currency::UYI => "UYI",
            Currency::UYU => "UYU",
            Currency::UYW => "UYW",
            Currency::UZS => "UZS",
            Currency::VED => "VED",
            Currency::VES => "VES",
            Currency::VND => "VND",
            Currency::VUV => "VUV",
            Currency::WST => "WST",
            Currency::XAF => "XAF",
            Currency::XCD => "XCD",
            Currency::XOF => "XOF",
            Currency::XPF => "XPF",
            Currency::YER => "YER",
            Currency::ZAR => "ZAR",
            Currency::ZMW => "ZMW",
            Currency::ZWL => "ZWL",
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
