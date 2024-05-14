
use std::collections::HashMap;
use lazy_static::lazy_static;


// hashmap with constant DATA of my garammer
lazy_static! {
    pub static ref BRABBLE: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("⏳☝💨".to_string(), "LUI".to_string());
        m.insert("➕☝💨💻".to_string(), "AUIPC".to_string());
        m.insert("🦘".to_string(), "JAL".to_string());
        m.insert("🦘🔗".to_string(), "JALR".to_string());
        m.insert("🌳🙆‍♂️".to_string(), "BEQ".to_string());
        m.insert("🌳🙅‍♂️".to_string(), "BNE".to_string());
        m.insert("🌳🤏🖋️".to_string(), "BLS".to_string());
        m.insert("🌳💁‍♂️🖋️".to_string(), "BGE".to_string());
        m.insert("🌳🤏🔏".to_string(), "BLSU".to_string());
        m.insert("🌳💁‍♂️🔏".to_string(), "BGEU".to_string());
        m.insert("⏳👾🖋️".to_string(), "LB".to_string());
        m.insert("⏳👾👾🖋️".to_string(), "LH".to_string());
        m.insert("⏳👾👾👾👾🖋️".to_string(), "LW".to_string());
        m.insert("⏳👾🔏".to_string(), "LBU".to_string());
        m.insert("⏳👾👾🔏".to_string(), "LHU".to_string());
        m.insert("🏪👾".to_string(), "SB".to_string());
        m.insert("🏪👾👾".to_string(), "SH".to_string());
        m.insert("🏪👾👾👾👾".to_string(), "SW".to_string());
        m.insert("➕💨".to_string(), "ADDI".to_string());
        m.insert("🪑🤏💨".to_string(), "SLTI".to_string());
        m.insert("🪑🤏💨🔏".to_string(), "SLTIU".to_string());
        m.insert("🦻💨".to_string(), "XORI".to_string());
        m.insert("👂💨".to_string(), "ORI".to_string());
        m.insert("🤗💨".to_string(), "ANDI".to_string());
        m.insert("👈💨".to_string(), "SLLI".to_string());
        m.insert("👉💨".to_string(), "SRLI".to_string());
        m.insert("👉🧮💨".to_string(), "SRAI".to_string());
        m.insert("➕".to_string(), "ADD".to_string());
        m.insert("➖".to_string(), "SUB".to_string());
        m.insert("👈".to_string(), "SLL".to_string());
        m.insert("🪑🤏".to_string(), "SLT".to_string());
        m.insert("🪑🤏🔏".to_string(), "SLTU".to_string());
        m.insert("🦻".to_string(), "XOR".to_string());
        m.insert("👉".to_string(), "SRL".to_string());
        m.insert("👉🧮".to_string(), "SRA".to_string());
        m.insert("👂".to_string(), "OR".to_string());
        m.insert("🤗".to_string(), "AND".to_string());
        m.insert("🚧".to_string(), "FENCE".to_string());
        m.insert("🚧ℹ️".to_string(), "FENCE.I".to_string());
        m.insert("⏸".to_string(), "PAUSE".to_string());
        m.insert("✋".to_string(), "PAUSE".to_string());
        m.insert("📞".to_string(), "ECALL".to_string());
        m.insert("⏹".to_string(), "EBREAK".to_string());
        m.insert("☢️📰✍️".to_string(), "CSRRW".to_string());
        m.insert("🪑📰".to_string(), "CSRRS".to_string());
        m.insert("🧼📰".to_string(), "CSRRC".to_string());
        m.insert("☢️📰✍️💨".to_string(), "CSRRWI".to_string());
        m.insert("🪑📰💨".to_string(), "CSRRSI".to_string());
        m.insert("🧼📰💨".to_string(), "CSRRCI".to_string());
        m
    };
}


