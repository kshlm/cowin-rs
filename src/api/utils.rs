pub(crate) mod serde_date {
    use chrono::NaiveDate;
    use serde::{self, Serializer};

    const FORMAT: &str = "%d-%m-%Y";

    pub(crate) fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }
}

pub(crate) fn opti16_display(s: &Option<i16>) -> i16 {
    s.unwrap_or_default()
}
