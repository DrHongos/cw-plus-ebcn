use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct MemberNamed {
    pub addr: String,
    pub name: String,
    pub weight: u64,
}

#[cw_serde]
pub struct MemberNamedListResponse {
    pub members: Vec<MemberNamed>,
}


#[cw_serde]
pub struct LookUpResponse {
    pub name: Option<String>,
}

#[cw_serde]
pub struct ReverseLookUpResponse {
    pub addr: Option<String>,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// The admin is the only account that can update the group state.
    /// Omit it to make the group immutable.
    pub admin: Option<String>,
    pub members: Vec<MemberNamed>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Change the admin
    UpdateAdmin { admin: Option<String> },
    /// apply a diff to the existing members.
    /// remove is applied after add, so if an address is in both, it is removed
    UpdateMembers {
        remove: Vec<String>,
        add: Vec<MemberNamed>,
    },
    /// Add a new hook to be informed of all membership changes. Must be called by Admin
    AddHook { addr: String },
    /// Remove a hook. Must be called by Admin
    RemoveHook { addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(cw_controllers::AdminResponse)]
    Admin {},
    #[returns(cw4::TotalWeightResponse)]
    TotalWeight { at_height: Option<u64> },
    #[returns(MemberNamedListResponse)]
    ListMembers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(cw4::MemberResponse)]
    Member {
        addr: String,
        at_height: Option<u64>,
    },
    #[returns(LookUpResponse)]
    LookUp {
        addr: String,
    },
    #[returns(ReverseLookUpResponse)]
    ReverseLookUp {
        name: String,
    },
    /// Shows all registered hooks.
    #[returns(cw_controllers::HooksResponse)]
    Hooks {},
}
