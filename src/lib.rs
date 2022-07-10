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
mod sms_info {
    enum SmsEncoding {
        GSM,
        UCS,
    }

    /// Default values for counting parts of GSM encoded message
    const SYMBOLS_PER_PART_GSM: u8 = 153;
    const MAX_LENGTH_GSM: u8 = 160;

    /// Default values for counting parts of UCS encoded message
    const SYMBOLS_PER_PART_UCS: u8 = 67;
    const MAX_LENGTH_UCS: u8 = 70;

    struct SmsPartsInfo {
        parts_count: u8,
        symbols_per_part: u8,
        max_length: u8,
    }

    fn new_gsm_parts_info() -> SmsPartsInfo {
        SmsPartsInfo {
            parts_count: 0,
            symbols_per_part: SYMBOLS_PER_PART_GSM,
            max_length: MAX_LENGTH_GSM,
        }
    }

    fn new_ucs_parts_info() -> SmsPartsInfo {
        SmsPartsInfo {
            parts_count: 0,
            symbols_per_part: SYMBOLS_PER_PART_UCS,
            max_length: MAX_LENGTH_UCS,
        }
    }
}
