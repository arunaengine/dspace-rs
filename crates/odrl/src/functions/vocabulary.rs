pub struct Vocabulary;

impl Vocabulary {
    pub fn new() -> Self {
        Vocabulary
    }

    const ODRL_NS: &'static str = "http://www.w3.org/ns/odrl/2/";

    pub fn create_vocab_identifier(namespace_uri: &str, name: &str) -> String {
        format!("{}{}", namespace_uri, name)
    }

    /// The URI identifying the ODRL Version 2 Common Vocabulary is defined as:
    ///
    ///    http://www.w3.org/ns/odrl/2/
    ///
    /// This URI must be used in all encodings of ODRL policies to refer to the ODRL Common Vocabulary.
    /// A URI identifying an ODRL Common Vocabulary term is created by appending the identifier of that term to the URI of the vocabulary.
    ///
    /// For example, below are the target and play terms:
    ///
    ///    http://www.w3.org/ns/odrl/2/target
    ///    http://www.w3.org/ns/odrl/2/play
    pub fn create_odrl_vocab_item(name: &str) -> String {
        Self::create_vocab_identifier(Self::ODRL_NS, name)
    }
}
