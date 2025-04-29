use substreams::hex;
use tiny_keccak::Hasher;
use tiny_keccak::Keccak;

/// namehash("eth")
const ETH_NODE: [u8; 32] = hex!("93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae");

/// node = keccak256( parent_node ‖ label )
pub fn label_to_node(label: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak::v256();

    // abi.encodePacked(ETH_NODE, label)
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(&ETH_NODE);
    buf[32..].copy_from_slice(label);

    hasher.update(&buf);

    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    out
}
