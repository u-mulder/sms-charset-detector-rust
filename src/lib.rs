#[cfg(test)]
mod tests {
    use crate::sms_charset_detector::*;

    const DEFAULT_GSM_SMS_TEXT: &str = "Default sms text goes here";
    const DEFAULT_UCS_SMS_TEXT: &str = "Текст дефолтного сообщения";

    #[test]
    fn is_gsm_works() {
        let sms_info = sms_info_from_text(DEFAULT_GSM_SMS_TEXT.to_string());
        assert_eq!(true, sms_info.is_gsm());
    }

    #[test]
    fn is_not_gsm_works() {
        let sms_info = sms_info_from_text(DEFAULT_UCS_SMS_TEXT.to_string());
        assert_eq!(false, sms_info.is_gsm());
    }

    #[test]
    fn parts_count_getter_works() {
        let sms_info = sms_info_from_text(DEFAULT_GSM_SMS_TEXT.to_string());
        assert_eq!(42, sms_info.parts_count());
    }
}

#[allow(dead_code)]
// mod sms_info {  // consider renaming
mod sms_charset_detector {
    /// GSM (to be exact gsm03.38) encoding is descibed here // TODO LINK TO WIKI
    enum SmsEncoding {
        GSM,
        UCS,
    }

    pub struct SmsInfo {
        text: String, 
        encoding: SmsEncoding,
        parts_data: SmsPartsData,
    }

    impl SmsInfo {
        pub fn is_gsm(&self) -> bool {
            matches!(self.encoding, SmsEncoding::GSM)
        }
    }

    const REG_EXP: &str = "";

    pub fn sms_info_from_text(text: String) -> SmsInfo {
        let encoding: SmsEncoding;
        let parts_data: SmsPartsData;
        
        // TODO
        // if text.match(REG_EXP)) {
        if is_one() {
            encoding = SmsEncoding::UCS;
            parts_data = new_parts_data_for_ucs();
        } else {
            encoding = SmsEncoding::GSM;
            parts_data = new_parts_data_for_gsm()
        }

        let mut sms_info = SmsInfo {
            text,
            encoding: encoding,
            parts_data: parts_data,
        };

        sms_info.count_parts();

        sms_info
    }

    fn is_one() -> bool {
        true
    }

    /// Default values for counting parts of GSM encoded message
    const SYMBOLS_PER_PART_GSM: u8 = 153;
    const MAX_LENGTH_GSM: u8 = 160;

    /// Default values for counting parts of UCS encoded message
    const SYMBOLS_PER_PART_UCS: u8 = 67;
    const MAX_LENGTH_UCS: u8 = 70;

    struct SmsPartsData {
        parts_count: u8,
        symbols_per_part: u8,
        max_length: u8,
    }

    fn new_parts_data_for_gsm() -> SmsPartsData {
        SmsPartsData{
            parts_count: 0,
            symbols_per_part: SYMBOLS_PER_PART_GSM,
            max_length: MAX_LENGTH_GSM,
        }
    }

    fn new_parts_data_for_ucs() -> SmsPartsData {
        SmsPartsData{
            parts_count: 0,
            symbols_per_part: SYMBOLS_PER_PART_UCS,
            max_length: MAX_LENGTH_UCS,
        }
    }
}
