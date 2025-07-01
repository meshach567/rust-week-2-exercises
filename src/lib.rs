use std::str::FromStr;

// Decode hex string into Vec<u8>, return error string on failure
pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    hex::decode(hex_str).map_err(|e| format!("Failed to decode hex: {}", e))
}

// Reverse the byte order of input slice and return as Vec<u8>
pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut bytes = bytes.to_vec();
    bytes.reverse();
    bytes
}

// conversion of bytes slice to hex string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

// conversion of hex string to bytes vector
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

// little-endian byte swap for u32
pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    u32::to_le_bytes(num)
}

// Parse input string to u64, return error string if invalid
pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    u64::from_str(input).map_err(|_e| "Invalid satoshi amount".to_string())
}

pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

// Match script pattern and return corresponding ScriptType
pub fn classify_script(script: &[u8]) -> ScriptType {
    match script.len() {
        3 => match script[0] {
            0x00 => ScriptType::P2WPKH,
            _ => ScriptType::P2PKH,
        },
        _ => ScriptType::Unknown,
    }
}

// Outpoint tuple struct
pub struct Outpoint {
    pub txid: String,
    pub vout: u8,
}

// portion of the script slice (assumes pushdata starts at index 2)
pub fn read_pushdata(script: &[u8]) -> &[u8] {
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        // Return the wallet's confirmed balance
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance -= fee
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

// TODO: Add necessary derive traits
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Opcode {
    /// Its primary function is to verify signatures, ensuring that a transaction is authorized
    /// by the owner of the private key associated with the public key involved.
    /// When OP_CHECKSIG is executed in a script, it checks the signature against the provided
    /// public key and determines if they match, which confirms that the transaction is valid
    /// and can be processed further.
    /// In the context of advanced scripting mechanisms, such as Taproot or proposed changes like
    /// OP_CHECKSIG_WITHOUT_INPUT, OP_CHECKSIG can be adapted or extended to provide additional
    /// functionalities or optimize certain processe
    OpChecksig,
    /// duplicates the top item on the stack. This operation is commonly used in transaction scripts,
    /// particularly in Pay-to-PubkeyHash (P2PKH) transactions.
    /// In a typical P2PKH script, OP_DUP is used to duplicate the public key hash that's placed on
    /// the stack. This duplicated value is then used twice: once for hashing with OP_HASH160 and
    /// another time for comparison with the signature provided by the spender, ensuring that the
    /// transaction is authorized correctly. By duplicating the top stack item, OP_DUP allows both
    /// operations to occur without needing to otherwise manipulate or store additional copies of
    /// the data.
    OpDup,
    OpInvalid,
}

impl Opcode {
    //  mapping from byte to Opcode variant
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            0x00 => Err("Invalid opcode: 0x00".to_string()),
            _ => Ok(Opcode::OpInvalid),
        }
    }
}

// TODO: Add necessary derive traits
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(mut utxo: UTXO) -> UTXO {
    // Mark the UTXO as spent by setting its value to 0
    utxo.value = 0;
    utxo
}
